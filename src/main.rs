mod engine;
mod model;
mod util;
mod view;

use anyhow::{bail, Context, Result};
use clap::Parser;
use engine::{bucketize, scan_dir, ScanOptions};
use std::env;
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
    /// Show full History bucket
    #[arg(short = 'a', long = "all")]
    show_all_history: bool,

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

    let path = cli
        .path
        .unwrap_or_else(|| env::current_dir().expect("current_dir unavailable"));

    let meta = std::fs::metadata(&path)
        .with_context(|| format!("failed to read metadata for {}", path.display()))?;
    if !meta.is_dir() {
        bail!("{} is not a directory", path.display());
    }

    if env::var_os("NO_COLOR").is_some() {
        colored::control::set_override(false);
    }

    let scan = scan_dir(
        &path,
        &ScanOptions {
            include_hidden: cli.include_hidden,
        },
    )?;
    let bucketed = bucketize(&scan.entries, scan.now);
    let force_tty = env::var_os("FTIME_FORCE_TTY").is_some();

    if force_tty || std::io::stdout().is_terminal() {
        view::tty::render(&bucketed, scan.now, &path, cli.show_all_history)?;
    } else {
        view::text::render(&scan.entries, scan.now, &path)?;
    }
    Ok(())
}
