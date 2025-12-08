use crate::model::FileEntry;
use crate::util::time::relative_time;
use anyhow::Result;
use std::path::Path;
use std::time::SystemTime;

pub fn render(entries: &[FileEntry], now: SystemTime, base: &Path) -> Result<()> {
    for entry in entries {
        let path = entry
            .path
            .strip_prefix(base)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| entry.name.clone());
        println!("{}\t{}", path, relative_time(now, entry.mtime));
    }
    Ok(())
}
