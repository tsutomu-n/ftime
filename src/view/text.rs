use crate::model::FileEntry;
use crate::util::time::{absolute_time, classify_bucket, relative_time};
use anyhow::Result;
use std::path::Path;
use std::time::SystemTime;

pub fn render(
    entries: &[FileEntry],
    now: SystemTime,
    base: &Path,
    use_absolute: bool,
) -> Result<()> {
    for entry in entries {
        let path = entry
            .path
            .strip_prefix(base)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| entry.name.clone());
        let time_str = if use_absolute {
            absolute_time(entry.mtime)
        } else {
            relative_time(now, entry.mtime)
        };
        let bucket = classify_bucket(now, entry.mtime);
        println!("{path}\t{}\t{time_str}", bucket.key());
    }
    Ok(())
}
