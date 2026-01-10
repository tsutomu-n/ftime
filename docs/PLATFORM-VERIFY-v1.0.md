# Platform Verification Report v1.0
Date: 2026-01-10

## Environment
- OS: Linux (pop-os 22.04)
- Kernel: 6.17.4-76061704-generic
- Arch: x86_64

## Verification Steps (Linux)
### 1) TTY vs non-TTY output
Commands:
- `FTIME_FORCE_TTY=1 NO_COLOR=1 cargo run --quiet -- <temp_dir>`
- `cargo run --quiet -- <temp_dir>`

Observed:
- `FTIME_FORCE_TTY=1` in a non-TTY environment produces bucketed TTY output.
- Default non-TTY output is TSV (`<path>\t<relative_time>`).

### 2) Symlink handling
Setup:
- `file` (regular file)
- `link` -> `file`
- `broken` -> `missing`

Observed (TTY forced):
- `link` renders as `link -> file`
- `broken` renders as `broken -> missing`

Note:
- Spec updated: `read_link` success prints the target string without existence check.

### 3) Time bucket boundaries / DST
Observed:
- Newly created files show `just now` as expected.
Pending:
- DST/local-time boundary behavior is not validated in this environment.

### 4) Permission errors
Pending:
- Unable to reproduce a per-entry permission error in this environment; not validated.

## Cross-Platform Status
- Linux: partially verified (TTY/non-TTY, symlink behavior).
- macOS: not verified.
