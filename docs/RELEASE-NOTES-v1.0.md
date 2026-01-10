# ftime v1.0.0 Release Notes
Date: 2026-01-10

## Summary
v1.0.0 is the stability release for the FS edition. CLI and output formats are frozen, and the v1.0 specification is now the source of truth.

## Compatibility Policy
- CLI flags and behavior are frozen in v1.0; changes require a major version.
- Output formats (TTY/pipe/JSON) are stable.
- JSON field set is frozen for compatibility.
- No experimental options exist in v1.0.

## Requirements
- Rust/Cargo 1.85+ (edition 2024).

## Output Contracts
- JSON Lines (`--json`): one object per line with fields:
  `path`, `bucket`, `mtime`, `relative_time`, `is_dir`, `is_symlink`, `symlink_target`, `label`
- `symlink_target` and `label` are only emitted when applicable.
- `NO_COLOR` disables color output even if set to an empty string (intentional divergence from no-color.org).

## Documentation Updates
- `docs/SPEC-v1.0.md` and `docs/TESTPLAN-v1.0.md` added as the canonical spec/test plan.
- CLI contract updated for v1.0 freeze and compatibility policy.
