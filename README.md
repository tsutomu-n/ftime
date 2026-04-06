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
ftime ./target
ftime /var/log/app
ftime -a
ftime --all-history
ftime --hide-dots
ftime --files-only --ext rs,toml
ftime --plain
ftime --json | jq -r '.path'
```

`--json` emits one JSON object per line, so it works well with `jq` and other scripts.

## Design philosophy

`ftime` is built for Context Recovery. Its main job is to rebuild the recent working context of one folder quickly, not just extract the single newest path.

Time buckets act as cognitive scaffolding: `Active`, `Today`, `This Week`, and `History` separate recent activity into chunks that are easier to scan than one flat time-sorted listing.

## Non-goals

`ftime` is not a replacement for `fd`, `find`, `eza`, or `git status`.

- Recursive search is not the primary goal. Use `fd` or `find` when you need to search deep trees.
- Rich VCS state inspection is not the primary goal. Use `git status` when you need tracked and untracked state.
- Destructive actions are out of scope. `ftime` stays read-only.
- General-purpose one-shot extraction is not the primary goal. `ftime` first helps you recover context, then makes the next thing to inspect easier to spot.

## Example output

```text
Active (1)
  Cargo.toml  2.1 KiB          12s

Today (1)
  README.md   8.4 KiB          2h

This Week (1)
  docs/           —           3d [child: today]

History (1)
  target/         —   2026-03-16 [child: active]
```

Directories show `—` in the size column.
Directory rows may show a child activity hint when a direct child is more recent than the directory itself.
The hint is advisory only: the parent directory keeps its own bucket and sort position based on the directory's `mtime`.
Human output aligns columns by Unicode display width, so Japanese/full-width names do not skew the `size` and `time` columns.
When a human-view name gets too long, `ftime` truncates only the displayed name while keeping the full name in `--plain` and `--json`.
Symlink rows keep `name  size  time` aligned and show `-> target` as a suffix after the time column.

## Tool fit

| Tool | Strong at | Where `ftime` differs |
| --- | --- | --- |
| `ls -lt` | quick sorted listing | no recency buckets |
| `eza` | rich file listing with metadata | no built-in time buckets |
| `fd` | recursive search and filters | recursive by design |
| `bat` | reading file contents | not a folder activity view |
| `ftime` | context recovery in one folder | buckets + size at a glance |

## Install

### GitHub Releases (recommended)

Fetches the latest published installer from GitHub Releases. This installs the latest published release, not unreleased `main`.
Rust is not required for the GitHub Releases installer.

#### macOS / Linux

```bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.sh | bash
```

#### Windows (PowerShell)

```bash
powershell -ExecutionPolicy Bypass -Command "iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.ps1 -UseBasicParsing | iex"
```

Default Windows install dir: `%LOCALAPPDATA%\Programs\ftime\bin`.

Windows installer currently targets x86_64 / AMD64.

### crates.io

Uses the published crate from crates.io.

```bash
cargo install ftime --locked
ftime --version
```

### From source (for unreleased main)

Requires Rust/Cargo 1.92+.

```bash
cargo install --path . --force
hash -r
ftime --version
```

Uninstall steps are documented in `## Uninstall`, including custom install directories.

Common flags:

- `-a, --all`: show hidden files and hidden directories
- `--all-history`: expand the History bucket
- `--hide-dots`: hide all hidden entries
- `--ext`: focus on selected regular file extensions while keeping directory context
- `--files-only`: only show regular files
- `-A, --absolute`: show absolute local timestamps like `2026-03-16 20:49:28 (UTC+09:00)`
- `--plain`: emit `path<TAB>bucket<TAB>time`
- `--json`: emit one JSON object per line for scripts
- `--no-hints`: disable `[child: ...]` hints
- `--color <auto|always|never>`: control ANSI color in human output
- `--check-update`: report whether a newer published release is available
- `--self-update`: update the current installed binary to the latest published release
- `--no-ignore`: temporarily disable ignore rules to verify what was filtered out

## Update

```bash
ftime --check-update
ftime --self-update
```

Typical output:

```text
update available: 2.0.1 -> 2.0.2
ftime updated 2.0.1 -> 2.0.2 in /home/tn/.local/bin
ftime is already up to date at 2.0.1 in /home/tn/.local/bin
ftime now points to 2.0.2 (was 2.0.1) in /home/tn/.local/bin
```

When invoked via a symlink, `ftime --self-update` updates that symlink directory.

## Uninstall

### GitHub Releases install

#### macOS / Linux

```bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh | bash
```

If you installed to a custom directory, pass the same location again:

```bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh | env INSTALL_DIR=/custom/bin bash
```

#### Windows PowerShell

```powershell
powershell -ExecutionPolicy Bypass -Command "iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1 -UseBasicParsing | iex"
```

```powershell
powershell -ExecutionPolicy Bypass -Command "& ([scriptblock]::Create((iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1 -UseBasicParsing).Content)) -InstallDir 'C:\custom\bin'"
```

### `cargo install` / `cargo install --path .`

```bash
cargo uninstall ftime
```

## Learn More

- [日本語](docs/README-ja.md)
- [中文](docs/README-zh.md)
- [Japanese docs router](docs/ftime-overview-ja.md)
- [User guide (Japanese)](docs/USER-GUIDE-ja.md)
- [CLI contract](docs/CLI.md)

## License

MIT (see `LICENSE`)
