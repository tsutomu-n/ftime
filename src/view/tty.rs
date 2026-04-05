use crate::engine::{Bucketed, ScanOptions, ScanStats, dir_child_activity_hint};
use crate::model::{ChildActivityHint, FileEntry, TimeBucket};
use crate::util::time::{absolute_time, relative_time};
#[cfg(feature = "icons")]
use crate::view::icon::NerdIconProvider;
use crate::view::icon::{DefaultIconProvider, IconProvider};
use anyhow::Result;
use clap::ValueEnum;
use colored::Colorize;
use std::io::IsTerminal;
use std::path::Path;
use std::time::SystemTime;

const ACTIVE_LIMIT: usize = 20;
const TODAY_LIMIT: usize = 20;
const WEEK_LIMIT: usize = 20;
const HISTORY_LIMIT: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

#[derive(Clone, Copy)]
pub struct RenderOptions<'a> {
    pub now: SystemTime,
    pub base: &'a Path,
    pub show_all_history: bool,
    pub use_icons: bool,
    pub use_absolute: bool,
    pub color_mode: ColorMode,
    pub scan_opts: &'a ScanOptions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimeTone {
    Skew,
    Active,
    Today,
    ThisWeek,
    History,
}

pub fn render(buckets: &Bucketed, stats: &ScanStats, options: RenderOptions<'_>) -> Result<()> {
    colored::control::set_override(should_colorize(options.color_mode));

    if buckets.total() == 0 {
        println!("No matching entries");
        if let Some(filters) = filters_summary(options.scan_opts) {
            println!("{filters}");
        }
        return Ok(());
    }

    render_bucket(
        buckets.active.as_slice(),
        TimeBucket::Active,
        ACTIVE_LIMIT,
        false,
        options,
    );
    render_bucket(
        buckets.today.as_slice(),
        TimeBucket::Today,
        TODAY_LIMIT,
        false,
        options,
    );
    render_bucket(
        buckets.week.as_slice(),
        TimeBucket::ThisWeek,
        WEEK_LIMIT,
        false,
        options,
    );
    render_bucket(
        buckets.history.as_slice(),
        TimeBucket::History,
        HISTORY_LIMIT,
        options.show_all_history,
        options,
    );

    if stats.skipped_unreadable > 0 {
        println!("Skipped {} unreadable entries", stats.skipped_unreadable);
    }

    Ok(())
}

fn render_bucket(
    entries: &[FileEntry],
    bucket: TimeBucket,
    preview_limit: usize,
    show_all: bool,
    options: RenderOptions<'_>,
) {
    if entries.is_empty() {
        return;
    }

    let shown = if show_all || entries.len() <= preview_limit {
        entries.len()
    } else {
        preview_limit
    };

    println!(
        "{}",
        style_header(
            bucket,
            &bucket_header(bucket, shown, entries.len(), show_all),
            options.use_icons
        )
    );

    for entry in &entries[..shown] {
        let time_str = if options.use_absolute {
            absolute_time(entry.mtime)
        } else {
            relative_time(options.now, entry.mtime)
        };
        let child_hint =
            format_child_activity_hint_suffix(entry, options.now, bucket, options.scan_opts);
        println!(
            "  {}  {}  {}{}",
            format_name(entry, options.base),
            format_size(entry.size),
            style_time_text(bucket, &time_str),
            child_hint
        );
    }

    println!();
}

fn bucket_header(bucket: TimeBucket, shown: usize, total: usize, show_all: bool) -> String {
    if show_all || shown == total {
        format!("{} ({total})", bucket.title())
    } else {
        format!("{} ({shown}/{total})", bucket.title())
    }
}

fn style_header(bucket: TimeBucket, header: &str, use_icons: bool) -> String {
    let icon = icon_prefix(bucket, use_icons);
    let text = if icon.is_empty() {
        header.to_string()
    } else {
        format!("{icon} {header}")
    };
    match bucket {
        TimeBucket::Active => text.green().bold().to_string(),
        TimeBucket::Today => text.bold().to_string(),
        TimeBucket::ThisWeek => text.truecolor(180, 180, 180).bold().to_string(),
        TimeBucket::History => text.truecolor(130, 130, 130).bold().to_string(),
    }
}

fn format_name(entry: &FileEntry, base: &Path) -> String {
    let rel = entry
        .path
        .strip_prefix(base)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| entry.name.clone());

    if entry.is_dir() {
        format!("{rel}/")
    } else if entry.is_symlink() {
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
        format!("{rel} -> {target}")
    } else {
        rel
    }
}

fn classify_time_tone(bucket: TimeBucket, time_str: &str) -> TimeTone {
    if time_str.contains("[skew]") {
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
        TimeTone::History => time_str.truecolor(130, 130, 130).to_string(),
    }
}

fn format_child_activity_hint_suffix(
    entry: &FileEntry,
    now: SystemTime,
    bucket: TimeBucket,
    scan_opts: &ScanOptions,
) -> String {
    if !scan_opts.use_hints || !entry.is_dir() || entry.is_symlink() {
        return String::new();
    }

    dir_child_activity_hint(&entry.path, now, bucket, scan_opts)
        .map(format_child_activity_hint)
        .unwrap_or_default()
}

fn format_child_activity_hint(hint: ChildActivityHint) -> String {
    match hint {
        ChildActivityHint::Active => " [child: active]".dimmed().to_string(),
        ChildActivityHint::Today => " [child: today]".dimmed().to_string(),
    }
}

fn format_size(size: Option<u64>) -> String {
    let Some(size) = size else {
        return "—".to_string();
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

fn filters_summary(scan_opts: &ScanOptions) -> Option<String> {
    let dots = match scan_opts.dot_mode {
        crate::engine::DotMode::Default => "default".to_string(),
        crate::engine::DotMode::All => "all".to_string(),
        crate::engine::DotMode::None => "none".to_string(),
    };

    let mut parts = vec![format!("dots={dots}")];
    parts.push(format!(
        "ignore={}",
        if scan_opts.use_ignore { "on" } else { "off" }
    ));

    if let Some(exts) = &scan_opts.ext_filter {
        parts.push(format!("ext={}", exts.join(",")));
    }
    if scan_opts.files_only {
        parts.push("type=files-only".to_string());
    }

    if parts == ["dots=default".to_string(), "ignore=on".to_string()] {
        None
    } else {
        Some(format!("filters: {}", parts.join(", ")))
    }
}

fn should_colorize(mode: ColorMode) -> bool {
    match mode {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => {
            std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none()
        }
    }
}

fn icon_prefix(bucket: TimeBucket, use_icons: bool) -> &'static str {
    #[cfg(feature = "icons")]
    {
        if use_icons {
            let provider = NerdIconProvider;
            return provider.bucket_icon(bucket);
        }
    }

    let _ = use_icons;
    let provider = DefaultIconProvider;
    provider.bucket_icon(bucket)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_time_tone_prefers_skew_over_bucket_colors() {
        assert_eq!(
            classify_time_tone(TimeBucket::History, "+5m [skew]"),
            TimeTone::Skew
        );
    }

    #[test]
    fn classify_time_tone_uses_bucket_heatmap_for_non_skew_values() {
        assert_eq!(
            classify_time_tone(TimeBucket::Active, "12s"),
            TimeTone::Active
        );
        assert_eq!(classify_time_tone(TimeBucket::Today, "2h"), TimeTone::Today);
        assert_eq!(
            classify_time_tone(TimeBucket::ThisWeek, "3d"),
            TimeTone::ThisWeek
        );
        assert_eq!(
            classify_time_tone(TimeBucket::History, "2026-03-01"),
            TimeTone::History
        );
    }

    #[test]
    fn bucket_headers_show_preview_counts() {
        assert_eq!(
            bucket_header(TimeBucket::History, 5, 7, false),
            "History (5/7)"
        );
        assert_eq!(bucket_header(TimeBucket::Active, 3, 3, false), "Active (3)");
    }
}
