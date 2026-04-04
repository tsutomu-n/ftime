# ftime v1.0.5 Release Notes
Date: 2026-04-03

## Summary
v1.0.5 adds TTY-only child activity hints for directory rows, closes the remaining coverage gaps around that feature, and aligns the current public v1 docs with the shipped behavior.

## Changes in v1.0.5
- TTY directory rows may now show `[child: active]` or `[child: today]` when a direct child is hotter than the directory itself.
- Child activity hints remain advisory only: they do not reclassify the parent bucket and never appear in plain text or JSON output.
- Added CLI coverage for symlink directory rows and unreadable directories so child activity hints stay suppressed in those cases.
- Updated the current v1 docs so README / spec / CLI references match the shipped child activity hint behavior.

## Current v1 Contract Snapshot
- Dotfiles are included by default.
- `--exclude-dots` is the opt-out flag for dotfiles.
- `-H/--hidden` is not part of the public v1.0.5 CLI.
- TTY output uses bucketed sections plus `name | size | time`.
- TTY directory rows may append advisory child activity hints without changing the parent bucket or sort order.
- Pipe output remains plain two-column TSV: `<path>\t<time>`.
- `--check-update` checks the latest published release without installing it.
- `--self-update` installs the latest published release in place.
- Windows PowerShell installer no longer defaults to `.cargo\bin`.
- Windows PowerShell installer uses `%LOCALAPPDATA%\Programs\ftime\bin`.
- GitHub Releases installer now documents that Rust is not required.

## Canonical Docs
- `README.md`
- `docs/USER-GUIDE-ja.md`
- `docs/CLI.md`
