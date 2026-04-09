mod engine;
mod model;
mod util;
mod view;

use anyhow::{Context, Result, bail};
use clap::Parser;
use engine::{DotMode, ScanOptions, bucketize, scan_dir};
use std::env;
use std::path::PathBuf;
use std::process;
use util::ignore::{load_ignore_patterns, load_local_ignore};
use view::tty::ColorMode;

#[derive(Parser, Debug)]
#[command(
    name = "ftime",
    version,
    about = "files by time: a read-only File Time CLI",
    after_help = "Default output is the human bucket view. Use --plain or --json for script-friendly output."
)]
struct Cli {
    /// Emit plain TSV output
    #[arg(long = "plain")]
    plain: bool,

    /// Emit JSON Lines output
    #[cfg(feature = "json")]
    #[arg(long = "json")]
    json: bool,

    /// Show hidden files and hidden directories
    #[arg(short = 'a', long = "all")]
    all: bool,

    /// Hide all hidden entries
    #[arg(long = "hide-dots")]
    hide_dots: bool,

    /// Disable ignore rules (built-in, FTIME_IGNORE, ~/.ftimeignore, and local .ftimeignore)
    #[arg(long = "no-ignore")]
    no_ignore: bool,

    /// Filter regular files by comma-separated extensions (case-insensitive)
    #[arg(long = "ext")]
    ext: Option<String>,

    /// Only show regular files
    #[arg(long = "files-only")]
    files_only: bool,

    /// Expand the History bucket
    #[arg(long = "all-history")]
    all_history: bool,

    /// Emit absolute local timestamps with UTC offset instead of relative time
    #[arg(short = 'A', long = "absolute")]
    absolute_time: bool,

    /// Show directory child activity hints
    #[arg(long = "hints")]
    hints: bool,

    /// Color handling for human output
    #[arg(long = "color", value_enum, default_value_t = ColorMode::Auto)]
    color: ColorMode,

    /// Show Nerd Font icons (opt-in)
    #[arg(short = 'I', long = "icons")]
    use_icons: bool,

    /// Check whether a newer published release is available
    #[arg(long = "check-update")]
    check_update: bool,

    /// Update the current installed binary to the latest published release
    #[arg(long = "self-update")]
    self_update: bool,

    /// Target directory (defaults to current directory)
    path: Option<PathBuf>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    if cli.self_update || cli.check_update {
        if cli.self_update && cli.check_update {
            bail!("--self-update and --check-update cannot be combined");
        }

        let update_flag = update_flag_name(&cli);

        if has_scan_options(&cli) {
            bail!("{update_flag} cannot be combined with scan options or PATH");
        }

        #[cfg(feature = "json")]
        if cli.json {
            bail!("{update_flag} cannot be combined with scan options or PATH");
        }

        if cli.plain {
            bail!("{update_flag} cannot be combined with scan options or PATH");
        }

        return if cli.self_update {
            util::update::self_update()
        } else {
            util::update::check_for_update()
        };
    }

    validate_output_flags(&cli)?;

    let path = match cli.path {
        Some(p) => p,
        None => env::current_dir().context("failed to obtain current directory")?,
    };

    let meta = std::fs::metadata(&path)
        .with_context(|| format!("failed to read metadata for {}", path.display()))?;
    if !meta.is_dir() {
        bail!("{} is not a directory", path.display());
    }

    let dot_mode = if cli.all {
        DotMode::All
    } else if cli.hide_dots {
        DotMode::None
    } else {
        DotMode::Default
    };

    let use_ignore = !cli.no_ignore;
    let scan_opts = ScanOptions {
        dot_mode,
        ext_filter: cli.ext.as_ref().map(|s| {
            s.split(',')
                .map(|p| p.trim().trim_start_matches('.').to_lowercase())
                .filter(|x| !x.is_empty())
                .collect()
        }),
        use_ignore,
        ignore_patterns: if use_ignore {
            load_ignore_patterns()
        } else {
            Vec::new()
        },
        local_ignore_patterns: if use_ignore {
            load_local_ignore(&path)
        } else {
            Vec::new()
        },
        files_only: cli.files_only,
        show_hints: cli.hints,
    };

    let scan = scan_dir(&path, &scan_opts)?;

    #[cfg(feature = "json")]
    if cli.json {
        return view::json::render(&scan.entries, scan.now, &path);
    }

    if cli.plain {
        return view::text::render(&scan.entries, scan.now, &path, cli.absolute_time);
    }

    let bucketed = bucketize(&scan.entries, scan.now);
    view::tty::render(
        &bucketed,
        &scan.stats,
        view::tty::RenderOptions {
            now: scan.now,
            base: &path,
            show_all_history: cli.all_history,
            use_icons: cli.use_icons,
            use_absolute: cli.absolute_time,
            color_mode: cli.color,
            scan_opts: &scan_opts,
        },
    )?;
    Ok(())
}

fn validate_output_flags(cli: &Cli) -> Result<()> {
    #[cfg(feature = "json")]
    if cli.plain && cli.json {
        bail!("--plain and --json cannot be combined");
    }

    if cli.all && cli.hide_dots {
        bail!("-a and --hide-dots cannot be combined");
    }

    #[cfg(feature = "json")]
    if cli.json
        && (cli.absolute_time
            || cli.all_history
            || cli.hints
            || cli.use_icons
            || cli.color != ColorMode::Auto)
    {
        bail!("--json cannot be combined with human-only flags");
    }

    if cli.plain && (cli.all_history || cli.hints || cli.use_icons || cli.color != ColorMode::Auto)
    {
        bail!("--plain cannot be combined with human-only flags");
    }

    Ok(())
}

fn update_flag_name(cli: &Cli) -> &'static str {
    if cli.self_update {
        "--self-update"
    } else {
        "--check-update"
    }
}

fn has_scan_options(cli: &Cli) -> bool {
    cli.path.is_some()
        || cli.plain
        || cli.no_ignore
        || cli.all
        || cli.hide_dots
        || cli.ext.is_some()
        || cli.files_only
        || cli.all_history
        || cli.hints
        || cli.use_icons
        || cli.absolute_time
        || cli.color != ColorMode::Auto
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_cli(args: &[&str]) -> Cli {
        Cli::parse_from(args)
    }

    #[test]
    fn has_scan_options_is_false_for_update_only_flags() {
        let cli = parse_cli(&["ftime", "--check-update"]);
        assert!(!has_scan_options(&cli));
    }

    #[test]
    fn has_scan_options_detects_path_argument() {
        let cli = parse_cli(&["ftime", "--self-update", "."]);
        assert!(has_scan_options(&cli));
    }

    #[test]
    fn has_scan_options_detects_scan_flags() {
        let cli = parse_cli(&["ftime", "--check-update", "--hide-dots"]);
        assert!(has_scan_options(&cli));
    }

    #[test]
    fn update_flag_name_prefers_active_update_mode() {
        let cli = parse_cli(&["ftime", "--check-update"]);
        assert_eq!(update_flag_name(&cli), "--check-update");
    }
}
