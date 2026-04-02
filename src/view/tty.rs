use crate::engine::Bucketed;
use crate::model::{FileEntry, TimeBucket};
use crate::util::time::{absolute_time, current_timezone_offset, relative_time};
#[cfg(feature = "icons")]
use crate::view::icon::NerdIconProvider;
use crate::view::icon::{DefaultIconProvider, IconProvider};
use anyhow::Result;
use colored::Colorize;
use std::path::Path;
use std::time::SystemTime;

const LIMIT: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimeTone {
    Skew,
    Active,
    Today,
    ThisWeek,
    History,
}

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
    let tz_offset = current_timezone_offset();

    render_bucket(
        &header(provider.as_ref(), TimeBucket::Active, bucket_title(TimeBucket::Active)),
        &buckets.active,
        TimeBucket::Active,
        now,
        base,
        use_absolute,
    );
    render_bucket(
        &header(provider.as_ref(), TimeBucket::Today, bucket_title(TimeBucket::Today)),
        &buckets.today,
        TimeBucket::Today,
        now,
        base,
        use_absolute,
    );
    render_bucket(
        &header(
            provider.as_ref(),
            TimeBucket::ThisWeek,
            bucket_title(TimeBucket::ThisWeek),
        ),
        &buckets.week,
        TimeBucket::ThisWeek,
        now,
        base,
        use_absolute,
    );

    if show_all_history || buckets.history.len() <= LIMIT {
        render_bucket(
            &header(
                provider.as_ref(),
                TimeBucket::History,
                bucket_title(TimeBucket::History),
            ),
            &buckets.history,
            TimeBucket::History,
            now,
            base,
            use_absolute,
        );
    } else if !buckets.history.is_empty() {
        println!(
            "{} ({} files hidden)",
            header(
                provider.as_ref(),
                TimeBucket::History,
                bucket_title(TimeBucket::History),
            ),
            buckets.history.len()
        );
    }

    println!("Current Timezone: {}", tz_offset.dimmed());

    Ok(())
}

fn render_bucket(
    header: &str,
    entries: &[FileEntry],
    bucket: TimeBucket,
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
        let display_time = style_time_text(bucket, &time_str);
        println!(
            "  • {} | {} | {}{}",
            format_name(entry, base),
            format_size(entry.size),
            display_time,
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

fn classify_time_tone(bucket: TimeBucket, time_str: &str) -> TimeTone {
    if time_str.contains("[Skew]") {
        TimeTone::Skew
    } else {
        match bucket {
            TimeBucket::Active => TimeTone::Active,
            TimeBucket::Today => TimeTone::Today,
            TimeBucket::ThisWeek => TimeTone::ThisWeek,
            TimeBucket::History => TimeTone::History,
        }
    }
}

fn style_time_text(bucket: TimeBucket, time_str: &str) -> String {
    match classify_time_tone(bucket, time_str) {
        TimeTone::Skew => time_str.yellow().bold().to_string(),
        TimeTone::Active => time_str.green().bold().to_string(),
        TimeTone::Today => time_str.normal().to_string(),
        TimeTone::ThisWeek => time_str.truecolor(180, 180, 180).to_string(),
        TimeTone::History => time_str.truecolor(100, 100, 100).to_string(),
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

fn bucket_title(bucket: TimeBucket) -> &'static str {
    match bucket {
        TimeBucket::Active => "Active",
        TimeBucket::Today => "Today",
        TimeBucket::ThisWeek => "This Week",
        TimeBucket::History => "History",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_time_tone_prefers_skew_over_bucket_colors() {
        assert_eq!(
            classify_time_tone(TimeBucket::History, "+5m [Skew]"),
            TimeTone::Skew
        );
    }

    #[test]
    fn classify_time_tone_uses_bucket_heatmap_for_non_skew_values() {
        assert_eq!(
            classify_time_tone(TimeBucket::Active, "just now"),
            TimeTone::Active
        );
        assert_eq!(
            classify_time_tone(TimeBucket::Today, "2 hours ago"),
            TimeTone::Today
        );
        assert_eq!(
            classify_time_tone(TimeBucket::ThisWeek, "3 days ago"),
            TimeTone::ThisWeek
        );
        assert_eq!(
            classify_time_tone(TimeBucket::History, "2026-03-01"),
            TimeTone::History
        );
    }

    #[test]
    fn bucket_titles_are_concise_and_match_readme_language() {
        assert_eq!(bucket_title(TimeBucket::Active), "Active");
        assert_eq!(bucket_title(TimeBucket::Today), "Today");
        assert_eq!(bucket_title(TimeBucket::ThisWeek), "This Week");
        assert_eq!(bucket_title(TimeBucket::History), "History");
    }
}
