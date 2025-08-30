# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2025-08-28
### Changed
- **BREAKING**: `ftime` without arguments now shows file listing (like `ls`) instead of help
- Short help moved to `ftime --help-short`

### Improved
- More intuitive behavior - matches user expectations from other CLI tools

## [0.2.0] - 2025-08-28
### Added
- Short help display when no arguments provided
- User-friendly error messages for directory not found

### Changed
- Default behavior without arguments now shows help instead of listing files
- `--help` now shows detailed help (previously default behavior)

## [0.1.0] - 2025-08-28
### Added
- Initial release of `ftime-list.sh`.
- Shows `modified`, `created`, and `name` columns.
- Local timezone by default; override with `FTL_TZ`.
- Simple patterns (e.g., `md` → `*.md`) and multiple OR filters.
- Optional color on the name column (TTY auto / force via `FTL_FORCE_COLOR=1`).
- Legend line (disable via `FTL_NO_LEGEND=1`).
