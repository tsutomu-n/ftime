# ftime

English | [日本語](docs/README-ja.md) | [中文](docs/README-zh.md)

![header](https://capsule-render.vercel.app/api?type=waving&color=0:0f2027,50:203a43,100:2c5364&height=300&text=ftime&fontSize=150&fontFamily=Acme&fontColor=e2e2e2&animation=fadeIn)

A tiny, read-only CLI that lists recently modified files and directories in time buckets (depth 1).

[![release](https://github.com/tsutomu-n/ftime/actions/workflows/release.yml/badge.svg)](https://github.com/tsutomu-n/ftime/actions/workflows/release.yml)

## Features
- 4 time buckets by `mtime`: Active (<1h) / Today / This Week (<7d) / History
- TTY output: color + buckets, History collapsed by default (`--all`), max 20 items per bucket
- Pipe/redirect output: tab-separated plain text (no headers, no colors, no icons)
- JSON Lines: `--json` (default build)
- Filters: `--ext`, ignore rules (`~/.ftimeignore`, `<PATH>/.ftimeignore`, `FTIME_IGNORE`, `--no-ignore`)

## Quickstart
```bash
ftime
```

## Install
### GitHub Releases (recommended)
```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/install.sh | bash

# Windows (PowerShell)
powershell -ExecutionPolicy Bypass -Command "iwr https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/install.ps1 -UseBasicParsing | iex"
```

### crates.io (when published)
```bash
cargo install ftime
```

### From source (build + install)
Requires Rust/Cargo 1.85+ (edition 2024).

```bash
cargo install --path .
ftime --version
```

> `cargo build --release` only builds `target/release/ftime` and does not add it to your `PATH`.

## Usage
```bash
ftime [OPTIONS] [PATH]
```

Common options:
- `-a, --all`     Show History bucket (TTY mode)
- `-H, --hidden`  Include dotfiles
- `--ext rs,toml` Filter by extensions (files only)
- `--json`        JSON Lines output (if built with default features)
- `--no-ignore`   Disable ignore rules
- `--no-labels`   Disable best-effort labels (e.g. Fresh)

## Docs
- Japanese README: `docs/README-ja.md`
- Chinese README: `docs/README-zh.md`
- User guide (Japanese): `docs/USER-GUIDE-ja.md`
- CLI contract: `docs/CLI.md`

## License
MIT (see `LICENSE`)
