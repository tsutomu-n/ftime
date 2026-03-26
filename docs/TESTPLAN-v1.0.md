# ftime v1.0.0 Test Plan
Last updated: 2026-03-25

## 1. Unit Tests
*   Bucket boundaries for Active / Today / This Week / History.
*   Relative time strings, including future `Skew` values.
*   Absolute timestamp formatting with timezone offset.
*   Sorting stability (`mtime` DESC, `name` ASC on ties).
*   Timezone offset helper format.
*   TTY time-tone classification: bucket heatmap and `Skew` precedence.

## 2. Integration Tests
*   `-H` is rejected.
*   Dotfiles are included by default and excluded by `--exclude-dots`.
*   History collapses by default and expands with `-a`.
*   Pipe output remains two-column TSV.
*   `-A/--absolute` changes pipe and TTY time output to `YYYY-MM-DD HH:MM:SS (UTC±HH:MM)`.
*   TTY shows size column.
*   TTY shows `Skew` and `Current Timezone: UTC±HH:MM`.
*   `NO_COLOR` keeps the text contract while removing ANSI escape codes.
*   JSON includes `size` for regular files and omits it for directories.

## 3. Tooling Gates
```bash
cargo check
CI=true timeout 30 cargo test --all-features
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

## 4. Release Validation
*   `cargo run -- --help` shows `-A, --absolute` and `--exclude-dots`.
*   `cargo run -- --help` does not show `-H, --hidden`.
*   `cargo run -- --version` prints `ftime 1.0.0`.
*   After publishing `v1.0.0`, `scripts/install.sh` and `scripts/install.ps1` install a binary whose `--help` matches the v1 contract.
