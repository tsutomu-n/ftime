# ftime = file time / mtime-oriented recent listing

English | [日本語](docs/README-ja.md) | [中文](docs/README-zh.md)

`ftime` is a read-only CLI that shows only the first level of a directory, sorts entries by `mtime`, and groups them into time buckets so you can see what changed recently without recursive noise.

[![release](https://github.com/tsutomu-n/ftime/actions/workflows/release.yml/badge.svg)](https://github.com/tsutomu-n/ftime/actions/workflows/release.yml)

- Read-only, depth-1 scan
- Buckets: `Active` / `Today` / `This Week` / `History`
- TTY output for humans, plain text / JSON Lines for scripts

## Install

### GitHub Releases (recommended)
Fetches the latest published installer from GitHub Releases. This installs the latest published release, not unreleased `main`.

#### macOS / Linux
```bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.sh | bash
```
#### Windows (PowerShell)
```bash
powershell -ExecutionPolicy Bypass -Command "iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.ps1 -UseBasicParsing | iex"
```

### From source
Requires Rust/Cargo 1.92+.

```bash
cargo install --path . --force
hash -r
ftime --version
```

Windows installer currently targets x86_64 / AMD64.

Uninstall steps are documented in `## Uninstall`, including custom install directories.

## Quick Usage

```bash
ftime
ftime ~/project
ftime --exclude-dots
ftime --ext rs,toml
ftime --json
```

Common flags:

- `-a, --all`: expand `History` in TTY mode
- `-A, --absolute`: show absolute local timestamps like `2026-03-16 20:49:28 (UTC+09:00)`
- `--check-update`: report whether a newer published release is available
- `--self-update`: update the current installed binary to the latest published release
- `--exclude-dots`: hide dotfiles
- `--no-ignore`: disable built-in and file-based ignore rules

## Update

```bash
ftime --check-update
ftime --self-update
```

Typical output:

```text
update available: 1.0.0 -> 1.0.1
ftime updated 1.0.0 -> 1.0.1 in /home/tn/.local/bin
ftime is already up to date at 1.0.0 in /home/tn/.local/bin
ftime now points to 1.0.0 (was 1.0.2) in /home/tn/.local/bin
```

When invoked via a symlink, `ftime --self-update` updates that symlink directory.

If your current binary predates `--self-update`, reinstall once from the latest GitHub Releases installer.

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
