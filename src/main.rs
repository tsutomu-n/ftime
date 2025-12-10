mod engine;
mod model;
mod util;
mod view;

use anyhow::{bail, Context, Result};
use clap::Parser;
use engine::{bucketize, scan_dir, ScanOptions};
use std::env;
use std::fs;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "ftime",
    version,
    about = "Recent file viewer with time buckets"
)]
struct Cli {
    /// Emit JSON Lines output
    #[cfg(feature = "json")]
    #[arg(long = "json")]
    json: bool,

    /// Disable ignore rules (built-in and ~/.ftimeignore)
    #[arg(long = "no-ignore")]
    no_ignore: bool,

    /// Disable best-effort labels (e.g., Fresh)
    #[arg(long = "no-labels")]
    no_labels: bool,

    /// Show full History bucket
    #[arg(short = 'a', long = "all")]
    show_all_history: bool,

    /// Filter files by comma-separated extensions (case-insensitive)
    #[arg(long = "ext")]
    ext: Option<String>,

    /// Show Nerd Font icons (opt-in)
    #[arg(short = 'I', long = "icons")]
    use_icons: bool,

    /// Include dotfiles
    #[arg(short = 'H', long = "hidden")]
    include_hidden: bool,

    /// Target directory (defaults to current directory)
    path: Option<PathBuf>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let path = match cli.path {
        Some(p) => p,
        None => env::current_dir().context("failed to obtain current directory")?,
    };

    let meta = std::fs::metadata(&path)
        .with_context(|| format!("failed to read metadata for {}", path.display()))?;
    if !meta.is_dir() {
        bail!("{} is not a directory", path.display());
    }

    if env::var_os("NO_COLOR").is_some() {
        colored::control::set_override(false);
    }

    let scan_opts = ScanOptions {
        include_hidden: cli.include_hidden,
        ext_filter: cli.ext.as_ref().map(|s| {
            s.split(',')
                .map(|p| p.trim().to_lowercase())
                .filter(|x| !x.is_empty())
                .collect()
        }),
        no_ignore: cli.no_ignore,
        ignore_patterns: if cli.no_ignore {
            Vec::new()
        } else {
            load_ignore_patterns()
        },
        local_ignore_patterns: if cli.no_ignore {
            Vec::new()
        } else {
            load_local_ignore(&path)
        },
        no_labels: cli.no_labels,
    };

    let scan = scan_dir(&path, &scan_opts)?;
    let bucketed = bucketize(&scan.entries, scan.now, &scan_opts);
    let force_tty = env::var_os("FTIME_FORCE_TTY").is_some();

    #[cfg(feature = "json")]
    if cli.json {
        return view::json::render(&bucketed, scan.now, &path);
    }

    if force_tty || std::io::stdout().is_terminal() {
        view::tty::render(
            &bucketed,
            scan.now,
            &path,
            cli.show_all_history,
            cli.use_icons,
        )?;
    } else {
        view::text::render(&scan.entries, scan.now, &path)?;
    }
    Ok(())
}

fn load_ignore_patterns() -> Vec<String> {
    if let Some(path) = env::var_os("FTIME_IGNORE") {
        return read_ignore_file(PathBuf::from(path));
    }
    if let Some(home) = env::var_os("HOME") {
        let default = PathBuf::from(home).join(".ftimeignore");
        return read_ignore_file(default);
    }
    Vec::new()
}

fn load_local_ignore(root: &Path) -> Vec<String> {
    let candidate = root.join(".ftimeignore");
    read_ignore_file(candidate)
}

fn read_ignore_file(path: PathBuf) -> Vec<String> {
    let Ok(contents) = fs::read_to_string(path) else {
        return Vec::new();
    };
    contents
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .collect()
}
