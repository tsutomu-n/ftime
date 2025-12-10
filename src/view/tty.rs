use crate::engine::Bucketed;
use crate::model::{FileEntry, TimeBucket};
use crate::util::time::relative_time;
#[cfg(feature = "icons")]
use crate::view::icon::NerdIconProvider;
use crate::view::icon::{DefaultIconProvider, IconProvider};
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
    use_icons: bool,
) -> Result<()> {
    if buckets.total() == 0 {
        println!("No recent files found");
        return Ok(());
    }

    let provider = select_provider(use_icons);

    render_bucket(
        &header(
            provider.as_ref(),
            TimeBucket::Active,
            "Active Context (< 1h)",
        ),
        &buckets.active,
        now,
        base,
    );
    render_bucket(
        &header(provider.as_ref(), TimeBucket::Today, "Today's Session"),
        &buckets.today,
        now,
        base,
    );
    render_bucket(
        &header(provider.as_ref(), TimeBucket::ThisWeek, "This Week"),
        &buckets.week,
        now,
        base,
    );

    if show_all_history {
        render_bucket(
            &header(provider.as_ref(), TimeBucket::History, "History"),
            &buckets.history,
            now,
            base,
        );
    } else if !buckets.history.is_empty() {
        println!(
            "{} ({} files hidden)",
            header(provider.as_ref(), TimeBucket::History, "History"),
            buckets.history.len()
        );
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

fn header(provider: &dyn IconProvider, bucket: TimeBucket, label: &str) -> String {
    let icon = provider.bucket_icon(bucket);
    if icon.is_empty() {
        label.to_string()
    } else {
        format!("{icon} {label}")
    }
}

fn select_provider(use_icons: bool) -> Box<dyn IconProvider> {
    #[cfg(feature = "icons")]
    {
        if use_icons {
            return Box::new(NerdIconProvider);
        }
    }
    #[cfg(not(feature = "icons"))]
    let _ = use_icons;
    Box::new(DefaultIconProvider)
}
