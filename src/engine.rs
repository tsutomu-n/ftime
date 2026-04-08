use crate::model::{ChildActivityHint, EntryKind, FileEntry, TimeBucket};
use crate::util::ignore::load_local_ignore;
use crate::util::time::classify_bucket;
use anyhow::{Context, Result};
use std::fs::{self, DirEntry, Metadata, ReadDir};
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DotMode {
    Default,
    All,
    None,
}

pub struct ScanOptions {
    pub dot_mode: DotMode,
    pub use_ignore: bool,
    pub ignore_patterns: Vec<String>,
    pub local_ignore_patterns: Vec<String>,
    pub ext_filter: Option<Vec<String>>,
    pub files_only: bool,
    pub show_hints: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ScanStats {
    pub total_raw_entries: usize,
    pub visible_entries: usize,
    pub skipped_unreadable: usize,
    pub filtered_hidden: usize,
    pub filtered_ignored: usize,
    pub filtered_ext: usize,
    pub filtered_type: usize,
}

pub struct ScanResult {
    pub entries: Vec<FileEntry>,
    pub now: SystemTime,
    pub stats: ScanStats,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FilterDecision {
    Include,
    Hidden,
    Ignored,
    Ext,
    Type,
}

pub fn scan_dir(path: &Path, opts: &ScanOptions) -> Result<ScanResult> {
    let now = SystemTime::now();
    let read_dir: ReadDir = fs::read_dir(path)
        .with_context(|| format!("failed to read directory {}", path.display()))?;
    let mut entries = Vec::new();
    let mut stats = ScanStats::default();

    for entry in read_dir {
        let Ok(entry) = entry else {
            stats.skipped_unreadable += 1;
            continue;
        };
        stats.total_raw_entries += 1;

        let full_path = entry.path();
        let metadata = match fs::symlink_metadata(&full_path) {
            Ok(m) => m,
            Err(_) => {
                stats.skipped_unreadable += 1;
                continue;
            }
        };

        match should_include_entry(
            &entry,
            &full_path,
            path,
            &metadata,
            opts,
            &opts.local_ignore_patterns,
        ) {
            FilterDecision::Include => {}
            FilterDecision::Hidden => {
                stats.filtered_hidden += 1;
                continue;
            }
            FilterDecision::Ignored => {
                stats.filtered_ignored += 1;
                continue;
            }
            FilterDecision::Ext => {
                stats.filtered_ext += 1;
                continue;
            }
            FilterDecision::Type => {
                stats.filtered_type += 1;
                continue;
            }
        }

        let name = entry.file_name().to_string_lossy().to_string();
        let mtime = match metadata.modified() {
            Ok(t) => t,
            Err(_) => {
                stats.skipped_unreadable += 1;
                continue;
            }
        };

        let kind = if metadata.file_type().is_symlink() {
            EntryKind::Symlink
        } else if metadata.is_dir() {
            EntryKind::Dir
        } else {
            EntryKind::File
        };

        let size = matches!(kind, EntryKind::File).then_some(metadata.len());
        let symlink_target = matches!(kind, EntryKind::Symlink)
            .then(|| fs::read_link(&full_path).ok())
            .flatten();

        entries.push(FileEntry {
            path: full_path,
            name,
            kind,
            mtime,
            size,
            symlink_target,
        });
        stats.visible_entries += 1;
    }

    entries.sort_by(|a, b| b.mtime.cmp(&a.mtime).then_with(|| a.name.cmp(&b.name)));

    Ok(ScanResult {
        entries,
        now,
        stats,
    })
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
    if !parent_scan_opts.show_hints || !dir_path.is_dir() {
        return None;
    }

    let local_ignore_patterns = if parent_scan_opts.use_ignore {
        load_local_ignore(dir_path)
    } else {
        Vec::new()
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

        if should_include_entry(
            &entry,
            &child_path,
            dir_path,
            &metadata,
            parent_scan_opts,
            &local_ignore_patterns,
        ) != FilterDecision::Include
        {
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
) -> FilterDecision {
    let file_name = entry.file_name();
    let name = file_name.to_string_lossy();
    let is_hidden = name.starts_with('.');

    if is_hidden {
        match opts.dot_mode {
            DotMode::All => {}
            DotMode::None => return FilterDecision::Hidden,
            DotMode::Default => {
                if metadata.is_dir() && !metadata.file_type().is_symlink() {
                    return FilterDecision::Hidden;
                }
            }
        }
    }

    if opts.use_ignore
        && is_ignored(
            &name,
            full_path,
            root,
            &opts.ignore_patterns,
            local_ignore_patterns,
            &default_ignore_patterns(),
        )
    {
        return FilterDecision::Ignored;
    }

    if opts.files_only && !metadata.is_file() {
        return FilterDecision::Type;
    }

    if let Some(exts) = &opts.ext_filter
        && metadata.is_file()
    {
        let ext = full_path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase());
        if !ext
            .as_ref()
            .map(|e| exts.iter().any(|x| x == e))
            .unwrap_or(false)
        {
            return FilterDecision::Ext;
        }
    }

    FilterDecision::Include
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

fn glob_match(pattern: &str, text: &str) -> bool {
    glob_match_inner(pattern.as_bytes(), text.as_bytes())
}

fn glob_match_inner(p: &[u8], t: &[u8]) -> bool {
    if p.is_empty() {
        return t.is_empty();
    }
    match p[0] {
        b'*' => (0..=t.len()).any(|i| glob_match_inner(&p[1..], &t[i..])),
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

pub fn bucketize(entries: &[FileEntry], now: SystemTime) -> Bucketed {
    let mut active = Vec::new();
    let mut today = Vec::new();
    let mut week = Vec::new();
    let mut history = Vec::new();

    for entry in entries {
        match classify_bucket(now, entry.mtime) {
            TimeBucket::Active => active.push(entry.clone()),
            TimeBucket::Today => today.push(entry.clone()),
            TimeBucket::ThisWeek => week.push(entry.clone()),
            TimeBucket::History => history.push(entry.clone()),
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
            dot_mode: DotMode::Default,
            ext_filter: None,
            use_ignore: true,
            ignore_patterns: Vec::new(),
            local_ignore_patterns: Vec::new(),
            files_only: false,
            show_hints: true,
        }
    }

    #[test]
    fn scan_default_dot_mode_hides_hidden_directories_only() -> Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("visible"))?;
        File::create(dir.path().join(".hidden"))?;
        fs::create_dir(dir.path().join(".hidden_dir"))?;

        let res = scan_dir(dir.path(), &scan_options())?;
        let names: Vec<&str> = res
            .entries
            .iter()
            .map(|entry| entry.name.as_str())
            .collect();

        assert!(names.contains(&"visible"));
        assert!(names.contains(&".hidden"));
        assert!(!names.contains(&".hidden_dir"));
        assert_eq!(res.stats.filtered_hidden, 1);
        Ok(())
    }

    #[test]
    fn scan_dot_modes_all_and_none_behave_as_expected() -> Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join(".hidden"))?;
        fs::create_dir(dir.path().join(".hidden_dir"))?;

        let all = scan_dir(
            dir.path(),
            &ScanOptions {
                dot_mode: DotMode::All,
                ..scan_options()
            },
        )?;
        assert_eq!(all.entries.len(), 2);

        let none = scan_dir(
            dir.path(),
            &ScanOptions {
                dot_mode: DotMode::None,
                ..scan_options()
            },
        )?;
        assert_eq!(none.entries.len(), 0);
        assert_eq!(none.stats.filtered_hidden, 2);
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

        let res = scan_dir(dir.path(), &scan_options())?;
        let names: Vec<&str> = res.entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["a", "b"]);
        Ok(())
    }

    #[test]
    fn ext_filter_only_applies_to_regular_files() -> Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("keep.rs"))?;
        File::create(dir.path().join("drop.txt"))?;
        fs::create_dir(dir.path().join("docs"))?;

        let res = scan_dir(
            dir.path(),
            &ScanOptions {
                ext_filter: Some(vec!["rs".to_string()]),
                ..scan_options()
            },
        )?;
        let names: Vec<&str> = res.entries.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"keep.rs"));
        assert!(names.contains(&"docs"));
        assert!(!names.contains(&"drop.txt"));
        assert_eq!(res.stats.filtered_ext, 1);
        Ok(())
    }

    #[test]
    fn files_only_filters_non_files() -> Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("keep.rs"))?;
        fs::create_dir(dir.path().join("docs"))?;

        let res = scan_dir(
            dir.path(),
            &ScanOptions {
                files_only: true,
                ..scan_options()
            },
        )?;
        assert_eq!(res.entries.len(), 1);
        assert_eq!(res.entries[0].name, "keep.rs");
        assert_eq!(res.stats.filtered_type, 1);
        Ok(())
    }

    #[test]
    fn bucketize_groups_correctly() {
        let now = SystemTime::now();
        let mk = |delta_secs: u64| FileEntry {
            path: PathBuf::from("x"),
            name: "x".to_string(),
            kind: EntryKind::File,
            mtime: now - Duration::from_secs(delta_secs),
            size: Some(0),
            symlink_target: None,
        };
        let entries = vec![mk(10), mk(4000), mk(2 * 24 * 3600), mk(8 * 24 * 3600)];
        let b = bucketize(&entries, now);
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
            dot_mode: DotMode::None,
            ..scan_options()
        };

        assert_eq!(
            dir_child_activity_hint(dir.path(), now, TimeBucket::History, &opts),
            None
        );
        Ok(())
    }
}
