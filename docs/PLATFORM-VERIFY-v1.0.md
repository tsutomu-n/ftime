# Platform Verification Report v1.0
Date: 2026-01-10

## Environment
- OS: Linux (pop-os 22.04)
- Kernel: 6.17.4-76061704-generic
- Arch: x86_64

## Verification Steps (Linux)
### 1) TTY vs non-TTY output
Commands:
- `script -q -c "NO_COLOR=1 cargo run --quiet -- <temp_dir>" /tmp/ftime_tty_script.txt`
- `cargo run --quiet -- <temp_dir>`

Observed:
- TTY output shows bucket headers and grouped entries (confirmed via `script`).
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
Commands:
- `TZ=America/New_York cargo test util::time::tests::test_classify_bucket_around_dst_transition -- --nocapture`

Observed:
- DST transition unit test passes under `TZ=America/New_York` (2026-01-10).
- Local midnight boundary check (example):
  - `after_midnight` → `bucket: "today"`
  - `before_midnight` → `bucket: "this_week"`

### 4) Permission errors
Attempt:
- Created a file with mode `000` under a temp directory and scanned it.

Observed:
- Entry still appears in output; metadata retrieval did not fail.

Pending:
- Per-entry permission error could not be reproduced in this environment.

## Cross-Platform Status
- Linux: partially verified (TTY/non-TTY, symlink behavior, DST unit test via TZ override; permission error not reproduced).
- macOS: not verified.
