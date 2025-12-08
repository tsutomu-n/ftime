use crate::model::{FileEntry, TimeBucket};
use crate::util::time::classify_bucket;
use anyhow::{Context, Result};
use std::fs::{self, ReadDir};
use std::path::Path;
use std::time::SystemTime;

pub struct ScanOptions {
    pub include_hidden: bool,
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
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy().to_string();
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
        entries.push(FileEntry {
            path: entry.path(),
            name,
            is_dir,
            mtime,
            is_symlink,
        });
    }

    // sort by mtime descending
    entries.sort_by(|a, b| b.mtime.cmp(&a.mtime));

    Ok(ScanResult { entries, now })
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
        };
        let res = scan_dir(dir.path(), &opts)?;
        assert_eq!(res.entries.len(), 1);
        assert_eq!(res.entries[0].name, "visible");
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
        };
        let entries = vec![
            mk(10),            // active
            mk(4000),          // today (~1.1h)
            mk(2 * 24 * 3600), // week
            mk(8 * 24 * 3600), // history
        ];
        let b = bucketize(&entries, now);
        assert_eq!(b.active.len(), 1);
        assert_eq!(b.today.len(), 1);
        assert_eq!(b.week.len(), 1);
        assert_eq!(b.history.len(), 1);
    }
}
