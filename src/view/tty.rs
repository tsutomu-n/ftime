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

#[derive(Debug, Clone)]
struct RenderedRow {
    bucket: TimeBucket,
    name: String,
    size: String,
    time: String,
    suffix: String,
    is_dir: bool,
}

#[derive(Debug, Clone)]
struct RenderedBucket {
    bucket: TimeBucket,
    header: String,
    rows: Vec<RenderedRow>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct ColumnWidths {
    name: usize,
    size: usize,
    time: usize,
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

    let mut rendered = Vec::new();
    push_rendered_bucket(
        &mut rendered,
        buckets.active.as_slice(),
        TimeBucket::Active,
        ACTIVE_LIMIT,
        false,
        options,
    );
    push_rendered_bucket(
        &mut rendered,
        buckets.today.as_slice(),
        TimeBucket::Today,
        TODAY_LIMIT,
        false,
        options,
    );
    push_rendered_bucket(
        &mut rendered,
        buckets.week.as_slice(),
        TimeBucket::ThisWeek,
        WEEK_LIMIT,
        false,
        options,
    );
    push_rendered_bucket(
        &mut rendered,
        buckets.history.as_slice(),
        TimeBucket::History,
        HISTORY_LIMIT,
        options.show_all_history,
        options,
    );

    let widths = column_widths(&rendered);
    for bucket in &rendered {
        render_bucket(bucket, widths, options.use_icons);
    }

    if stats.skipped_unreadable > 0 {
        println!("Skipped {} unreadable entries", stats.skipped_unreadable);
    }

    Ok(())
}

fn push_rendered_bucket(
    out: &mut Vec<RenderedBucket>,
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

    let rows = entries[..shown]
        .iter()
        .map(|entry| render_row(entry, bucket, options))
        .collect();

    out.push(RenderedBucket {
        bucket,
        header: bucket_header(bucket, shown, entries.len(), show_all),
        rows,
    });
}

fn render_bucket(bucket: &RenderedBucket, widths: ColumnWidths, use_icons: bool) {
    println!("{}", style_header(bucket.bucket, &bucket.header, use_icons));

    for row in &bucket.rows {
        let name = format!("{:<width$}", row.name, width = widths.name);
        let size = format!("{:>width$}", row.size, width = widths.size);
        let time = format!("{:>width$}", row.time, width = widths.time);
        let suffix = if row.suffix.is_empty() {
            String::new()
        } else {
            format!(" {}", row.suffix)
        };

        println!(
            "  {}  {}  {}{}",
            style_name(&name, row),
            size,
            style_time_text(row.bucket, &time),
            suffix
        );
    }

    println!();
}

fn render_row(entry: &FileEntry, bucket: TimeBucket, options: RenderOptions<'_>) -> RenderedRow {
    let time = if options.use_absolute {
        absolute_time(entry.mtime)
    } else {
        relative_time(options.now, entry.mtime)
    };

    RenderedRow {
        bucket,
        name: format_name(entry, options.base),
        size: format_size(entry.size),
        time,
        suffix: format_suffix(entry, options.base, options.now, bucket, options.scan_opts),
        is_dir: entry.is_dir(),
    }
}

fn column_widths(buckets: &[RenderedBucket]) -> ColumnWidths {
    let mut widths = ColumnWidths::default();

    for bucket in buckets {
        for row in &bucket.rows {
            widths.name = widths.name.max(display_width(&row.name));
            widths.size = widths.size.max(display_width(&row.size));
            widths.time = widths.time.max(display_width(&row.time));
        }
    }

    widths
}

fn display_width(text: &str) -> usize {
    text.chars().count()
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
        TimeBucket::ThisWeek => text.cyan().bold().to_string(),
        TimeBucket::History => text.to_string(),
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
    } else {
        rel
    }
}

fn format_suffix(
    entry: &FileEntry,
    base: &Path,
    now: SystemTime,
    bucket: TimeBucket,
    scan_opts: &ScanOptions,
) -> String {
    if entry.is_symlink() {
        return format_symlink_target_suffix(entry, base);
    }

    format_child_activity_hint_suffix(entry, now, bucket, scan_opts)
}

fn format_symlink_target_suffix(entry: &FileEntry, base: &Path) -> String {
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
    format!("-> {target}")
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
        TimeTone::ThisWeek => time_str.cyan().to_string(),
        TimeTone::History => time_str.to_string(),
    }
}

fn style_name(text: &str, row: &RenderedRow) -> String {
    if row.is_dir {
        text.bold().to_string()
    } else {
        text.to_string()
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
        ChildActivityHint::Active => "[child: active]".to_string(),
        ChildActivityHint::Today => "[child: today]".to_string(),
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

    #[test]
    fn column_widths_use_the_longest_visible_row_across_buckets() {
        let buckets = vec![
            RenderedBucket {
                bucket: TimeBucket::Today,
                header: "Today (1)".to_string(),
                rows: vec![RenderedRow {
                    bucket: TimeBucket::Today,
                    name: "README.md".to_string(),
                    size: "1.2 KiB".to_string(),
                    time: "2h".to_string(),
                    suffix: String::new(),
                    is_dir: false,
                }],
            },
            RenderedBucket {
                bucket: TimeBucket::History,
                header: "History (1)".to_string(),
                rows: vec![RenderedRow {
                    bucket: TimeBucket::History,
                    name: "link_to_readme".to_string(),
                    size: "—".to_string(),
                    time: "2026-03-01".to_string(),
                    suffix: "-> README.md".to_string(),
                    is_dir: false,
                }],
            },
        ];

        assert_eq!(
            column_widths(&buckets),
            ColumnWidths {
                name: "link_to_readme".chars().count(),
                size: "1.2 KiB".chars().count(),
                time: "2026-03-01".chars().count(),
            }
        );
    }

    #[test]
    fn semantic_palette_is_theme_safe() {
        colored::control::set_override(true);

        assert_eq!(
            style_header(TimeBucket::ThisWeek, "This Week (1)", false),
            "\u{1b}[1;36mThis Week (1)\u{1b}[0m"
        );
        assert_eq!(
            style_header(TimeBucket::History, "History (1)", false),
            "History (1)"
        );
        assert_eq!(
            style_time_text(TimeBucket::ThisWeek, "2d"),
            "\u{1b}[36m2d\u{1b}[0m"
        );
        assert_eq!(
            style_time_text(TimeBucket::History, "2026-03-01"),
            "2026-03-01"
        );

        colored::control::set_override(false);
    }
}
