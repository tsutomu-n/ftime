# ftime v2.0.0 Release Notes
Date: 2026-03-25

## Summary
v2.0.0 is the first breaking release after the v1.x contract. It promotes the current `main` CLI behavior to the published release line.

## Breaking Changes
- Removed `-H/--hidden`.
- Dotfiles are now included by default.
- Added `--exclude-dots` to opt out of dotfiles.

## New Behavior
- Added `-A/--absolute` for timezone-aware absolute timestamps in TTY and pipe output.
- TTY output now includes a size column.
- Future mtimes are rendered as `+Ns [Skew]` / `+Nm [Skew]`.
- TTY output appends `Current Timezone: ±HHMM`.
- TTY time column uses a bucket-aware heatmap.
- JSON now includes optional `size` for regular files.

## Install Guidance
- `scripts/install.sh` and `scripts/install.ps1` continue to install the latest published release.
- Developers who want the current checkout should use `cargo install --path . --force`.

## Canonical Docs
- `docs/SPEC-v2.0.md`
- `docs/TESTPLAN-v2.0.md`
- `docs/CLI.md`
