# ftime Install and Update Guide

This document covers installation, update, and uninstall flows. For output behavior and validation rules, see [CLI.md](CLI.md).

## GitHub Releases (recommended)

Fetches the latest published installer from GitHub Releases. This installs the latest published release, not unreleased `main`.

### macOS / Linux

```bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.sh | bash
```

### Windows (PowerShell)

```powershell
powershell -ExecutionPolicy Bypass -Command "iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.ps1 -UseBasicParsing | iex"
```

Default Windows install dir: `%LOCALAPPDATA%\Programs\ftime\bin`.

## crates.io

```bash
cargo install ftime --locked
ftime --version
```

## From source (for unreleased main)

Requires Rust/Cargo 1.92+.

```bash
cargo install --path . --force
hash -r
ftime --version
```

## Update

```bash
ftime --check-update
ftime --self-update
```

Typical output:

```text
update available: <current> -> <latest>
ftime updated <from> -> <to> in /home/user/.local/bin
ftime is already up to date at <current> in /home/user/.local/bin
ftime now points to <to> (was <from>) in /home/user/.local/bin
```

When invoked via a symlink, `ftime --self-update` updates that symlink directory.

## Uninstall

### GitHub Releases install

macOS / Linux:

```bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh | bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh | env INSTALL_DIR=/custom/bin bash
```

Windows PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -Command "iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1 -UseBasicParsing | iex"
powershell -ExecutionPolicy Bypass -Command "& ([scriptblock]::Create((iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1 -UseBasicParsing).Content)) -InstallDir 'C:\custom\bin'"
```

### `cargo install` / `cargo install --path .`

```bash
cargo uninstall ftime
```
