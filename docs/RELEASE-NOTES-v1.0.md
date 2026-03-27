# ftime v1.0.0 Release Notes
Date: 2026-03-27

## Summary
v1.0.0 is the first public release of the current CLI contract.

## Public Contract
- Dotfiles are included by default.
- `--exclude-dots` is the opt-out flag for dotfiles.
- `-H/--hidden` is not part of the public v1.0.0 CLI.

## New Behavior
- Added `-A/--absolute` for `YYYY-MM-DD HH:MM:SS (UTC±HH:MM)` timestamps in TTY and pipe output.
- TTY output now includes a size column.
- Future mtimes are rendered as `+Ns [Skew]` / `+Nm [Skew]`.
- TTY output appends `Current Timezone: UTC±HH:MM`.
- TTY time column uses a bucket-aware heatmap.
- JSON now includes optional `size` for regular files.
- GitHub Releases now publish stable installer assets such as `ftime-install.sh` and stable latest-download binary asset names.
- Added `--self-update` to refresh the current installed binary in place via the latest installer asset.
- `--self-update` rejects Cargo build outputs for direct, cross-target, and custom profile layouts.
- `--self-update` prefers the invoked symlink path when resolving the install directory.
- `--self-update` now reports whether the installed version changed, stayed current, or now points to a renumbered release.

## Install Guidance
- GitHub Releases expose stable installer assets via `releases/latest/download/ftime-install.sh` and `ftime-install.ps1`.
- Developers who want the current checkout should use `cargo install --path . --force`.

## Canonical Docs
- `docs/SPEC-v1.0.md`
- `docs/TESTPLAN-v1.0.md`
- `docs/CLI.md`
