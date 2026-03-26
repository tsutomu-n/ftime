# ftime v1.0.1 Release Notes
Date: 2026-03-26

## Summary
v1.0.1 is a patch release focused on clearer absolute timestamps and a safer latest-release installation path.

## Public Contract
- Dotfiles are included by default.
- `--exclude-dots` is the opt-out flag for dotfiles.
- `-H/--hidden` is not part of the public v1.0.1 CLI.

## New Behavior
- Added `-A/--absolute` for `YYYY-MM-DD HH:MM:SS (UTC±HH:MM)` timestamps in TTY and pipe output.
- TTY output now includes a size column.
- Future mtimes are rendered as `+Ns [Skew]` / `+Nm [Skew]`.
- TTY output appends `Current Timezone: UTC±HH:MM`.
- TTY time column uses a bucket-aware heatmap.
- JSON now includes optional `size` for regular files.
- GitHub Releases now publish stable installer assets such as `ftime-install.sh` and stable latest-download binary asset names.

## Install Guidance
- GitHub Releases expose stable installer assets via `releases/latest/download/ftime-install.sh` and `ftime-install.ps1`.
- Developers who want the current checkout should use `cargo install --path . --force`.

## Canonical Docs
- `docs/SPEC-v1.0.md`
- `docs/TESTPLAN-v1.0.md`
- `docs/CLI.md`
