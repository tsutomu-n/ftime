use crate::model::{FileEntry, TimeBucket};
use crate::util::time::{classify_bucket, classify_label};
use anyhow::{Context, Result};
use std::fs::{self, ReadDir};
use std::path::Path;
use std::time::SystemTime;

pub struct ScanOptions {
    pub include_hidden: bool,
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

    let default_ignores = if opts.no_ignore {
        Vec::new()
    } else {
        vec![".DS_Store".to_string(), "Thumbs.db".to_string()]
    };

    for entry in read_dir {
        let Ok(entry) = entry else { continue };
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy().to_string();

        if !opts.no_ignore
            && is_ignored(
                &name,
                &entry.path(),
                path,
                &opts.ignore_patterns,
                &opts.local_ignore_patterns,
                &default_ignores,
            )
        {
            continue;
        }
        if !opts.include_hidden && name.starts_with('.') {
            continue;
        }

        let metadata = match fs::symlink_metadata(entry.path()) {
            Ok(m) => m,
            Err(_) => continue, // skip permission errors silently
        };
        let mtime = match metadata.modified() {
            Ok(t) => t,
            Err(_) => continue,
        };

        let is_dir = metadata.is_dir();
        let is_symlink = metadata.file_type().is_symlink();
        let symlink_target = if is_symlink {
            fs::read_link(entry.path()).ok()
        } else {
            None
        };
        // Extensionフィルタ（ファイルのみ対象、ディレクトリやシンボリックリンク先は除外）
        if let Some(exts) = &opts.ext_filter {
            if metadata.is_file() {
                let ext = entry
                    .path()
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_lowercase());
                if !ext
                    .as_ref()
                    .map(|e| exts.iter().any(|x| x == e))
                    .unwrap_or(false)
                {
                    continue;
                }
            } else {
                // 非ファイルはスキップ
                continue;
            }
        }

        entries.push(FileEntry {
            path: entry.path(),
            name,
            is_dir,
            mtime,
            is_symlink,
            symlink_target,
            label: None,
        });
    }

    // sort by mtime descending, tie-break by name ascending
    entries.sort_by(|a, b| b.mtime.cmp(&a.mtime).then_with(|| a.name.cmp(&b.name)));

    Ok(ScanResult { entries, now })
}

fn is_ignored(
    name: &str,
    full_path: &Path,
    root: &Path,
    user_patterns: &[String],
    local_patterns: &[String],
    default_patterns: &[String],
) -> bool {
    let rel = full_path.strip_prefix(root).ok();
    let rel_str = rel.map(|p| p.to_string_lossy().to_string());

    for pat in default_patterns
        .iter()
        .chain(user_patterns.iter())
        .chain(local_patterns.iter())
    {
        let has_slash = pat.contains('/');
        if has_slash {
            if let Some(rel) = &rel_str {
                if glob_match(pat, rel) {
                    return true;
                }
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
    use filetime::{set_file_mtime, FileTime};
    use std::fs::File;
    use std::path::PathBuf;
    use std::time::{Duration, SystemTime};
    use tempfile::tempdir;

    #[test]
    fn scan_skips_hidden_by_default() -> Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("visible"))?;
        File::create(dir.path().join(".hidden"))?;
        let opts = ScanOptions {
            include_hidden: false,
            ext_filter: None,
            no_ignore: false,
            ignore_patterns: Vec::new(),
            no_labels: false,
            local_ignore_patterns: Vec::new(),
        };
        let res = scan_dir(dir.path(), &opts)?;
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

        let opts = ScanOptions {
            include_hidden: false,
            ext_filter: None,
            no_ignore: false,
            ignore_patterns: Vec::new(),
            no_labels: false,
            local_ignore_patterns: Vec::new(),
        };
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
        let opts = ScanOptions {
            include_hidden: false,
            ext_filter: None,
            no_ignore: false,
            ignore_patterns: Vec::new(),
            no_labels: false,
            local_ignore_patterns: Vec::new(),
        };
        let b = bucketize(&entries, now, &opts);
        assert_eq!(b.active.len(), 1);
        assert_eq!(b.today.len(), 1);
        assert_eq!(b.week.len(), 1);
        assert_eq!(b.history.len(), 1);
    }
}
