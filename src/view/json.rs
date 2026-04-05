#[cfg(feature = "json")]
use serde::Serialize;

use crate::model::FileEntry;
use crate::util::time::{classify_bucket, relative_time, utc_rfc3339};
use anyhow::Result;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::SystemTime;

#[cfg_attr(not(feature = "json"), allow(dead_code))]
pub fn render(entries: &[FileEntry], now: SystemTime, base: &Path) -> Result<()> {
    #[cfg(not(feature = "json"))]
    {
        unreachable!("json feature not enabled");
    }

    #[cfg(feature = "json")]
    {
        let mut writer = BufWriter::new(std::io::stdout());
        for entry in entries {
            let record = JsonEntry::from_entry(entry, now, base);
            let line = serde_json::to_string(&record)?;
            writeln!(writer, "{line}")?;
        }
        writer.flush()?;
        Ok(())
    }
}

#[cfg(feature = "json")]
#[derive(Serialize)]
struct JsonEntry {
    path: String,
    bucket: String,
    mtime: String,
    relative_time: String,
    is_dir: bool,
    is_symlink: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symlink_target: Option<String>,
}

#[cfg(feature = "json")]
impl JsonEntry {
    fn from_entry(entry: &FileEntry, now: SystemTime, base: &Path) -> Self {
        let path = entry
            .path
            .strip_prefix(base)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| entry.name.clone());

        let symlink_target = entry.symlink_target.as_ref().map(|p| {
            p.strip_prefix(base)
                .map(|pp| pp.display().to_string())
                .unwrap_or_else(|_| p.display().to_string())
        });

        Self {
            path,
            bucket: classify_bucket(now, entry.mtime).key().to_string(),
            mtime: utc_rfc3339(entry.mtime),
            relative_time: relative_time(now, entry.mtime),
            is_dir: entry.is_dir(),
            is_symlink: entry.is_symlink(),
            size: entry.size,
            symlink_target,
        }
    }
}
