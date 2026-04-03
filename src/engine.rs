use crate::model::{ChildActivityHint, FileEntry, TimeBucket};
use crate::util::ignore::load_local_ignore;
use crate::util::time::{classify_bucket, classify_label};
use anyhow::{Context, Result};
use std::fs::{self, DirEntry, Metadata, ReadDir};
use std::path::Path;
use std::time::SystemTime;

pub struct ScanOptions {
    pub exclude_dots: bool,
    pub ext_filter: Option<Vec<String>>,
    pub no_ignore: bool,
    pub ignore_patterns: Vec<String>,
    pub no_labels: bool,
    pub local_ignore_patterns: Vec<String>,
}

pub struct ScanResult {
    pub entries: Vec<FileEntry>,
    pub now: SystemTime,
}

pub fn scan_dir(path: &Path, opts: &ScanOptions) -> Result<ScanResult> {
    let now = SystemTime::now();
    let read_dir: ReadDir = fs::read_dir(path)
        .with_context(|| format!("failed to read directory {}", path.display()))?;
    let mut entries = Vec::new();

    for entry in read_dir {
        let Ok(entry) = entry else { continue };
        let full_path = entry.path();
        let metadata = match fs::symlink_metadata(&full_path) {
            Ok(m) => m,
            Err(_) => continue, // skip permission errors silently
        };
        if !should_include_entry(
            &entry,
            &full_path,
            path,
            &metadata,
            opts,
            &opts.local_ignore_patterns,
        ) {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        let mtime = match metadata.modified() {
            Ok(t) => t,
            Err(_) => continue,
        };

        let is_dir = metadata.is_dir();
        let size = metadata.is_file().then_some(metadata.len());
        let is_symlink = metadata.file_type().is_symlink();
        let symlink_target = if is_symlink {
            fs::read_link(&full_path).ok()
        } else {
            None
        };

        entries.push(FileEntry {
            path: full_path,
            name,
            is_dir,
            mtime,
            size,
            is_symlink,
            symlink_target,
            label: None,
        });
    }

    // sort by mtime descending, tie-break by name ascending
    entries.sort_by(|a, b| b.mtime.cmp(&a.mtime).then_with(|| a.name.cmp(&b.name)));

    Ok(ScanResult { entries, now })
}

pub fn bucket_heat(bucket: TimeBucket) -> u8 {
    match bucket {
        TimeBucket::History => 0,
        TimeBucket::ThisWeek => 1,
        TimeBucket::Today => 2,
        TimeBucket::Active => 3,
    }
}

pub fn dir_child_activity_hint(
    dir_path: &Path,
    now: SystemTime,
    parent_bucket: TimeBucket,
    parent_scan_opts: &ScanOptions,
) -> Option<ChildActivityHint> {
    if !dir_path.is_dir() {
        return None;
    }

    let local_ignore_patterns = if parent_scan_opts.no_ignore {
        Vec::new()
    } else {
        load_local_ignore(dir_path)
    };

    let iter = fs::read_dir(dir_path).ok()?;
    let mut hottest = None;

    for item in iter {
        let Ok(entry) = item else { continue };
        let child_path = entry.path();
        let metadata = match fs::symlink_metadata(&child_path) {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };

        if metadata.file_type().is_symlink() {
            continue;
        }

        if !should_include_entry(
            &entry,
            &child_path,
            dir_path,
            &metadata,
            parent_scan_opts,
            &local_ignore_patterns,
        ) {
            continue;
        }

        let mtime = match metadata.modified() {
            Ok(mtime) => mtime,
            Err(_) => continue,
        };
        let child_bucket = classify_bucket(now, mtime);

        hottest = Some(match hottest {
            Some(current) if bucket_heat(current) >= bucket_heat(child_bucket) => current,
            _ => child_bucket,
        });

        if child_bucket == TimeBucket::Active {
            break;
        }
    }

    match hottest {
        Some(TimeBucket::Active)
            if bucket_heat(TimeBucket::Active) > bucket_heat(parent_bucket) =>
        {
            Some(ChildActivityHint::Active)
        }
        Some(TimeBucket::Today) if bucket_heat(TimeBucket::Today) > bucket_heat(parent_bucket) => {
            Some(ChildActivityHint::Today)
        }
        _ => None,
    }
}

fn should_include_entry(
    entry: &DirEntry,
    full_path: &Path,
    root: &Path,
    metadata: &Metadata,
    opts: &ScanOptions,
    local_ignore_patterns: &[String],
) -> bool {
    let file_name = entry.file_name();
    let name = file_name.to_string_lossy();

    if opts.exclude_dots && name.starts_with('.') {
        return false;
    }

    if !opts.no_ignore
        && is_ignored(
            &name,
            full_path,
            root,
            &opts.ignore_patterns,
            local_ignore_patterns,
            &default_ignore_patterns(),
        )
    {
        return false;
    }

    if let Some(exts) = &opts.ext_filter {
        if metadata.is_file() {
            let ext = full_path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_lowercase());
            if !ext
                .as_ref()
                .map(|e| exts.iter().any(|x| x == e))
                .unwrap_or(false)
            {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn default_ignore_patterns() -> [&'static str; 2] {
    [".DS_Store", "Thumbs.db"]
}

fn is_ignored(
    name: &str,
    full_path: &Path,
    root: &Path,
    user_patterns: &[String],
    local_patterns: &[String],
    default_patterns: &[&str],
) -> bool {
    let rel = full_path.strip_prefix(root).ok();
    let rel_str = rel.map(|p| p.to_string_lossy().to_string());

    for pat in default_patterns
        .iter()
        .copied()
        .chain(user_patterns.iter().map(String::as_str))
        .chain(local_patterns.iter().map(String::as_str))
    {
        let has_slash = pat.contains('/');
        if has_slash {
            if let Some(rel) = &rel_str
                && glob_match(pat, rel)
            {
                return true;
            }
        } else if glob_match(pat, name) {
            return true;
        }
    }
    false
}

// 簡易グロブ: * 任意長、? 1文字、その他はリテラル。バックスラッシュエスケープなし。
fn glob_match(pattern: &str, text: &str) -> bool {
    glob_match_inner(pattern.as_bytes(), text.as_bytes())
}

fn glob_match_inner(p: &[u8], t: &[u8]) -> bool {
    if p.is_empty() {
        return t.is_empty();
    }
    match p[0] {
        b'*' => {
            // *: 0文字以上
            (0..=t.len()).any(|i| glob_match_inner(&p[1..], &t[i..]))
        }
        b'?' => {
            if t.is_empty() {
                false
            } else {
                glob_match_inner(&p[1..], &t[1..])
            }
        }
        c => {
            if t.first() == Some(&c) {
                glob_match_inner(&p[1..], &t[1..])
            } else {
                false
            }
        }
    }
}

pub fn bucketize(entries: &[FileEntry], now: SystemTime, opts: &ScanOptions) -> Bucketed {
    let mut active = Vec::new();
    let mut today = Vec::new();
    let mut week = Vec::new();
    let mut history = Vec::new();

    for entry in entries {
        let mut e = entry.clone();
        if !opts.no_labels && e.label.is_none() {
            e.label = classify_label(now, e.mtime);
        }
        match classify_bucket(now, e.mtime) {
            TimeBucket::Active => active.push(e),
            TimeBucket::Today => today.push(e),
            TimeBucket::ThisWeek => week.push(e),
            TimeBucket::History => history.push(e),
        }
    }

    Bucketed {
        active,
        today,
        week,
        history,
    }
}

pub struct Bucketed {
    pub active: Vec<FileEntry>,
    pub today: Vec<FileEntry>,
    pub week: Vec<FileEntry>,
    pub history: Vec<FileEntry>,
}

impl Bucketed {
    pub fn total(&self) -> usize {
        self.active.len() + self.today.len() + self.week.len() + self.history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ChildActivityHint;
    use filetime::{FileTime, set_file_mtime};
    use std::fs::File;
    use std::path::PathBuf;
    use std::time::{Duration, SystemTime};
    use tempfile::tempdir;

    fn scan_options() -> ScanOptions {
        ScanOptions {
            exclude_dots: false,
            ext_filter: None,
            no_ignore: false,
            ignore_patterns: Vec::new(),
            no_labels: false,
            local_ignore_patterns: Vec::new(),
        }
    }

    #[test]
    fn scan_includes_hidden_by_default_and_excludes_with_flag() -> Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("visible"))?;
        File::create(dir.path().join(".hidden"))?;
        let opts = scan_options();
        let res = scan_dir(dir.path(), &opts)?;
        assert_eq!(res.entries.len(), 2);
        assert!(res.entries.iter().any(|entry| entry.name == "visible"));
        assert!(res.entries.iter().any(|entry| entry.name == ".hidden"));

        let res = scan_dir(
            dir.path(),
            &ScanOptions {
                exclude_dots: true,
                ..opts
            },
        )?;
        assert_eq!(res.entries.len(), 1);
        assert_eq!(res.entries[0].name, "visible");
        Ok(())
    }

    #[test]
    fn scan_sorts_by_mtime_then_name() -> Result<()> {
        let dir = tempdir()?;
        let a_path = dir.path().join("a");
        let b_path = dir.path().join("b");
        File::create(&b_path)?;
        File::create(&a_path)?;

        let t = SystemTime::now() - Duration::from_secs(60);
        let ft = FileTime::from_system_time(t);
        set_file_mtime(&a_path, ft)?;
        set_file_mtime(&b_path, ft)?;

        let opts = scan_options();
        let res = scan_dir(dir.path(), &opts)?;
        let names: Vec<&str> = res.entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["a", "b"]);
        Ok(())
    }

    #[test]
    fn bucketize_groups_correctly() {
        let now = SystemTime::now();
        let mk = |delta_secs: u64| FileEntry {
            path: PathBuf::from("x"),
            name: "x".to_string(),
            is_dir: false,
            mtime: now - Duration::from_secs(delta_secs),
            size: Some(0),
            is_symlink: false,
            symlink_target: None,
            label: None,
        };
        let entries = vec![
            mk(10),            // active
            mk(4000),          // today (~1.1h)
            mk(2 * 24 * 3600), // week
            mk(8 * 24 * 3600), // history
        ];
        let opts = scan_options();
        let b = bucketize(&entries, now, &opts);
        assert_eq!(b.active.len(), 1);
        assert_eq!(b.today.len(), 1);
        assert_eq!(b.week.len(), 1);
        assert_eq!(b.history.len(), 1);
    }

    #[test]
    fn bucket_heat_orders_hotter_buckets_first() {
        assert!(bucket_heat(TimeBucket::Active) > bucket_heat(TimeBucket::Today));
        assert!(bucket_heat(TimeBucket::Today) > bucket_heat(TimeBucket::ThisWeek));
        assert!(bucket_heat(TimeBucket::ThisWeek) > bucket_heat(TimeBucket::History));
    }

    #[test]
    fn dir_child_activity_hint_returns_active_for_hotter_history_parent() -> Result<()> {
        let dir = tempdir()?;
        let child = dir.path().join("artifact.bin");
        File::create(&child)?;
        let now = SystemTime::now();
        set_file_mtime(
            &child,
            FileTime::from_system_time(now - Duration::from_secs(30)),
        )?;

        assert_eq!(
            dir_child_activity_hint(dir.path(), now, TimeBucket::History, &scan_options()),
            Some(ChildActivityHint::Active)
        );
        Ok(())
    }

    #[test]
    fn dir_child_activity_hint_returns_today_for_hotter_history_parent() -> Result<()> {
        let dir = tempdir()?;
        let child = dir.path().join("notes.md");
        File::create(&child)?;
        let now = SystemTime::now();
        set_file_mtime(
            &child,
            FileTime::from_system_time(now - Duration::from_secs(2 * 3600)),
        )?;

        assert_eq!(
            dir_child_activity_hint(dir.path(), now, TimeBucket::History, &scan_options()),
            Some(ChildActivityHint::Today)
        );
        Ok(())
    }

    #[test]
    fn dir_child_activity_hint_suppresses_equal_or_cooler_buckets() -> Result<()> {
        let dir = tempdir()?;
        let child = dir.path().join("old.log");
        File::create(&child)?;
        let now = SystemTime::now();

        set_file_mtime(
            &child,
            FileTime::from_system_time(now - Duration::from_secs(2 * 3600)),
        )?;
        assert_eq!(
            dir_child_activity_hint(dir.path(), now, TimeBucket::Today, &scan_options()),
            None
        );

        set_file_mtime(
            &child,
            FileTime::from_system_time(now - Duration::from_secs(2 * 24 * 3600)),
        )?;
        assert_eq!(
            dir_child_activity_hint(dir.path(), now, TimeBucket::History, &scan_options()),
            None
        );

        set_file_mtime(
            &child,
            FileTime::from_system_time(now - Duration::from_secs(30)),
        )?;
        assert_eq!(
            dir_child_activity_hint(dir.path(), now, TimeBucket::Active, &scan_options()),
            None
        );
        Ok(())
    }

    #[test]
    fn dir_child_activity_hint_ignores_hidden_children_when_requested() -> Result<()> {
        let dir = tempdir()?;
        let hidden = dir.path().join(".cache");
        File::create(&hidden)?;
        let now = SystemTime::now();
        set_file_mtime(
            &hidden,
            FileTime::from_system_time(now - Duration::from_secs(30)),
        )?;

        let opts = ScanOptions {
            exclude_dots: true,
            ..scan_options()
        };

        assert_eq!(
            dir_child_activity_hint(dir.path(), now, TimeBucket::History, &opts),
            None
        );
        Ok(())
    }
}
