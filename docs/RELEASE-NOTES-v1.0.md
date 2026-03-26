# ftime v1.0.0 Release Notes
Date: 2026-03-25

## Summary
v1.0.0 is the first stable public release of the current `ftime` CLI. It promotes the current `main` behavior to the published release line.

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

## Install Guidance
- `scripts/install.sh` and `scripts/install.ps1` continue to install the latest published release.
- Developers who want the current checkout should use `cargo install --path . --force`.

## Canonical Docs
- `docs/SPEC-v1.0.md`
- `docs/TESTPLAN-v1.0.md`
- `docs/CLI.md`
