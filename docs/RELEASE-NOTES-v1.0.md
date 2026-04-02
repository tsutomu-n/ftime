# ftime v1.0.4 Release Notes
Date: 2026-04-02

## Summary
v1.0.4 is a release-fix patch for install and self-update paths on top of the current public v1 CLI.

## Changes in v1.0.4
- Self-update now resolves the latest tag before downloading the installer asset.
- Unix and PowerShell installers now resolve the latest tag before downloading versioned platform assets.
- Install and self-update no longer depend on unversioned `latest/download` binary asset aliases.

## Current v1 Contract Snapshot
- Dotfiles are included by default.
- `--exclude-dots` is the opt-out flag for dotfiles.
- `-H/--hidden` is not part of the public v1.0.4 CLI.
- TTY output uses bucketed sections plus `name | size | time`.
- Pipe output remains plain two-column TSV: `<path>\t<time>`.
- `--check-update` checks the latest published release without installing it.
- `--self-update` installs the latest published release in place.
- Windows PowerShell installer no longer defaults to `.cargo\bin`.
- Windows PowerShell installer uses `%LOCALAPPDATA%\Programs\ftime\bin`.
- GitHub Releases installer now documents that Rust is not required.

## Canonical Docs
- `docs/SPEC-v1.0.md`
- `docs/TESTPLAN-v1.0.md`
- `docs/CLI.md`
