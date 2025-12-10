#[cfg(feature = "json")]
use chrono::{DateTime, Utc};
#[cfg(feature = "json")]
use serde::Serialize;

use crate::engine::Bucketed;
use crate::model::{FileEntry, TimeBucket};
use crate::util::time::relative_time;
use anyhow::Result;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::SystemTime;

#[cfg_attr(not(feature = "json"), allow(dead_code))]
pub fn render(buckets: &Bucketed, now: SystemTime, base: &Path) -> Result<()> {
    #[cfg(not(feature = "json"))]
    {
        // featureが無効なら呼ばれない想定
        unreachable!("json feature not enabled");
    }

    #[cfg(feature = "json")]
    {
        let mut writer = BufWriter::new(std::io::stdout());
        for (bucket, entries) in [
            (TimeBucket::Active, &buckets.active),
            (TimeBucket::Today, &buckets.today),
            (TimeBucket::ThisWeek, &buckets.week),
            (TimeBucket::History, &buckets.history),
        ] {
            for entry in entries {
                let record = JsonEntry::from_entry(entry, bucket, now, base)?;
                let line = serde_json::to_string(&record)?;
                writeln!(writer, "{line}")?;
            }
        }
        writer.flush()?;
        Ok(())
    }
}

#[cfg(feature = "json")]
#[derive(Serialize)]
struct JsonEntry {
    // フィールド順・名前は後方互換のため凍結。変更はメジャーバージョンのみ。
    path: String,
    bucket: String,
    mtime: String,
    relative_time: String,
    is_dir: bool,
    is_symlink: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    symlink_target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[cfg(feature = "json")]
impl JsonEntry {
    fn from_entry(
        entry: &FileEntry,
        bucket: TimeBucket,
        now: SystemTime,
        base: &Path,
    ) -> Result<Self> {
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

        let mtime_dt: DateTime<Utc> = DateTime::<Utc>::from(entry.mtime);
        Ok(Self {
            path,
            bucket: bucket_key(bucket),
            mtime: mtime_dt.to_rfc3339(),
            relative_time: relative_time(now, entry.mtime),
            is_dir: entry.is_dir,
            is_symlink: entry.is_symlink,
            symlink_target,
            label: entry.label.map(|l| match l {
                crate::model::Label::Fresh => "fresh".to_string(),
            }),
        })
    }
}

#[cfg(feature = "json")]
fn bucket_key(bucket: TimeBucket) -> String {
    match bucket {
        TimeBucket::Active => "active",
        TimeBucket::Today => "today",
        TimeBucket::ThisWeek => "this_week",
        TimeBucket::History => "history",
    }
    .to_string()
}
