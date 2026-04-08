# `ftime` = files by time

English | [日本語](docs/README-ja.md) | [中文](docs/README-zh.md)

`ftime` is a read-only CLI for answering one question quickly:

> What changed in this folder recently?

The name stands for `files by time`. It scans only the first level of a directory, sorts entries by `mtime`, and groups them into time buckets so you can recover context without recursive noise.

[![release](https://github.com/tsutomu-n/ftime/actions/workflows/release.yml/badge.svg)](https://github.com/tsutomu-n/ftime/actions/workflows/release.yml)

[![demo_ftime](assets/demo_ftime.gif)](assets/demo_ftime.mp4)

- Read-only by design: no delete, rename, or write operations
- Depth-1 only: see the current folder, not the whole tree
- Buckets: `Active` / `Today` / `This Week` / `History`
- Human-first bucket view by default
- hidden files stay visible by default while hidden directories stay hidden
- Use `--plain` or `--json` when you want machine-oriented output

## Why `ftime`?

Use it when you want to:

- clean up `~/Downloads`
- check build output in `./target`
- inspect a log or sync folder
- answer "did anything change here?" in seconds

## Common examples

```bash
ftime
ftime ~/Downloads
ftime -a
ftime --all-history
ftime --hide-dots
ftime --plain
ftime --json | jq -r '.path'
```

`--json` emits one JSON object per line, so it works well with `jq` and other scripts.

## Example output

```text
Active (1)
  [FIL]  Cargo.toml  2.1 KiB          12s

Today (1)
  [FIL]  README.md   8.4 KiB          2h

This Week (1)
  [DIR]  docs/        <dir>          3d

History (1)
  [DIR]  target/      <dir>  2026-03-16
```

Directories show `<dir>` and symlinks show `<lnk>` in the size column.
Pass `--hints` when you want directory child activity hints such as `[child: today]`.
Use `--plain` or `--json` when you need full machine-readable values without human-view truncation or color.

## Install

### GitHub Releases (recommended)

macOS / Linux:

```bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.sh | bash
```

Windows (PowerShell):

```bash
powershell -ExecutionPolicy Bypass -Command "iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.ps1 -UseBasicParsing | iex"
```

### crates.io

```bash
cargo install ftime --locked
ftime --version
```

For custom install paths, from-source installs, `--check-update`, `--self-update`, and uninstall steps, see [docs/INSTALL.md](docs/INSTALL.md).

## Learn More

- [Command guide](docs/COMMANDS.md)
- [Install and update guide](docs/INSTALL.md)
- [CLI contract](docs/CLI.md)
- [日本語](docs/README-ja.md)
- [中文](docs/README-zh.md)
- [Japanese docs router](docs/ftime-overview-ja.md)
- [User guide (Japanese)](docs/USER-GUIDE-ja.md)

## License

MIT (see `LICENSE`)
