use crate::engine::Bucketed;
use crate::model::FileEntry;
use crate::util::time::relative_time;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;
use std::time::SystemTime;

const LIMIT: usize = 20;

pub fn render(
    buckets: &Bucketed,
    now: SystemTime,
    base: &Path,
    show_all_history: bool,
) -> Result<()> {
    if buckets.total() == 0 {
        println!("No recent files found");
        return Ok(());
    }

    render_bucket("🔥 Active Context (< 1h)", &buckets.active, now, base);
    render_bucket("☕ Today's Session", &buckets.today, now, base);
    render_bucket("📅 This Week", &buckets.week, now, base);

    if show_all_history {
        render_bucket("💤 History", &buckets.history, now, base);
    } else if !buckets.history.is_empty() {
        println!("💤 History ({} files hidden)", buckets.history.len());
    }

    Ok(())
}

fn render_bucket(header: &str, entries: &[FileEntry], now: SystemTime, base: &Path) {
    if entries.is_empty() {
        return;
    }
    println!("{}", header.bold());

    let (list, remainder) = if entries.len() > LIMIT {
        (&entries[..LIMIT], Some(entries.len() - LIMIT))
    } else {
        (entries, None)
    };

    for entry in list {
        println!(
            "  • {}  {}",
            format_name(entry, base),
            relative_time(now, entry.mtime)
        );
    }

    if let Some(rest) = remainder {
        println!("  ... and {} more items", rest);
    }
    println!();
}

fn format_name(entry: &FileEntry, base: &Path) -> String {
    let rel = entry
        .path
        .strip_prefix(base)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| entry.name.clone());

    if entry.is_dir {
        format!("{}/", rel).bold().blue().to_string()
    } else if entry.is_symlink {
        let target = entry
            .symlink_target
            .as_ref()
            .and_then(|p| p.strip_prefix(base).ok().map(|pp| pp.display().to_string()))
            .unwrap_or_else(|| {
                entry
                    .symlink_target
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "<unresolved>".to_string())
            });
        format!("{} -> {}", rel.normal().yellow(), target.dimmed())
    } else {
        rel.normal().to_string()
    }
}
