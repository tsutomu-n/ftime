# ftime v1.0.3 Release Notes
Date: 2026-04-02

## Summary
v1.0.3 is a docs-and-contract cleanup patch on top of the current public v1 CLI.

## Changes in v1.0.3
- TTY bucket headers now use `Active`, `Today`, `This Week`, and `History`.
- README examples now match the shipped `• name | size | time` TTY output.
- Japanese extended docs now match the current public v1 contract and sample output.
- The canonical CLI contract doc is English-only again.

## Current v1 Contract Snapshot
- Dotfiles are included by default.
- `--exclude-dots` is the opt-out flag for dotfiles.
- `-H/--hidden` is not part of the public v1.0.3 CLI.
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
