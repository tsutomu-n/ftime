use crate::engine::Bucketed;
use crate::model::{FileEntry, TimeBucket};
use crate::util::time::{absolute_time, relative_time};
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
    use_absolute: bool,
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
        use_absolute,
    );
    render_bucket(
        &header(provider.as_ref(), TimeBucket::Today, "Today's Session"),
        &buckets.today,
        now,
        base,
        use_absolute,
    );
    render_bucket(
        &header(provider.as_ref(), TimeBucket::ThisWeek, "This Week"),
        &buckets.week,
        now,
        base,
        use_absolute,
    );

    if show_all_history || buckets.history.len() <= LIMIT {
        render_bucket(
            &header(provider.as_ref(), TimeBucket::History, "History"),
            &buckets.history,
            now,
            base,
            use_absolute,
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

fn render_bucket(
    header: &str,
    entries: &[FileEntry],
    now: SystemTime,
    base: &Path,
    use_absolute: bool,
) {
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
        let label = format_label(entry);
        let time_str = if use_absolute {
            absolute_time(entry.mtime)
        } else {
            relative_time(now, entry.mtime)
        };
        println!(
            "  • {} | {} | {}{}",
            format_name(entry, base),
            format_size(entry.size),
            time_str,
            label
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

fn format_label(entry: &FileEntry) -> String {
    match entry.label {
        Some(crate::model::Label::Fresh) => "  ✨ Fresh".to_string(),
        None => "".to_string(),
    }
}

fn format_size(size: Option<u64>) -> String {
    let Some(size) = size else {
        return "-".to_string();
    };

    if size < 1024 {
        return format!("{size} B");
    }

    const UNITS: [&str; 4] = ["KiB", "MiB", "GiB", "TiB"];
    let mut value = size as f64;
    let mut unit = "B";

    for next in UNITS {
        value /= 1024.0;
        unit = next;
        if value < 1024.0 {
            break;
        }
    }

    if value >= 10.0 {
        format!("{value:.0} {unit}")
    } else {
        format!("{value:.1} {unit}")
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
