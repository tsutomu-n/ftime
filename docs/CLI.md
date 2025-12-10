# ftime v0.1.0 CLI Contract

## 1. Command Signature
```bash
ftime [OPTIONS] [PATH]
```

## 2. Arguments
*   `[PATH]` (Optional):
    *   Target directory to scan.
    *   Default: `.` (Current directory).
    *   Constraint: Must be a directory. If a file is passed, exit with error code 1.

## 3. Options
| Flag | Long Flag | Description |
| :--- | :--- | :--- |
| `-a` | `--all` | Expand the "History" bucket (TTY mode only). |
| `-I` | `--icons` | Show Nerd Font icons in bucket headers (requires binary built with `--features icons`; otherwise falls back to default emoji). |
| `-H` | `--hidden` | Include hidden files (starting with `.`). |
| `-h` | `--help` | Print help message. |
| `-V` | `--version` | Print version information. |

## 4. Environment Variables
*   `NO_COLOR`: If present (regardless of value), disable color output. **Always takes precedence** over other coloring decisions.
*   `FTIME_FORCE_TTY`: If present, force TTY-style grouped output even when stdout is piped or redirected. Coloring still obeys `NO_COLOR`.
*   Nerd Fonts: To see Nerd Font glyphs with `--icons`, build the binary with `cargo build --features icons` (or install via a package that enables the `icons` feature) and use a terminal configured with a Nerd Font. Without the font or feature, output gracefully falls back to the default emoji headers.

## 5. Exit Codes
*   `0`: Success.
*   `1`: General error (e.g., directory not found, permission denied on target root, target is a file). Per-entry I/O エラーはスキップし処理継続する。

## 6. Usage Examples
```bash
# Basic usage
ftime

# Scan specific directory
ftime ~/Downloads

# Show everything including dotfiles and history
ftime -a -H

# Build with Nerd Font icons feature and enable icons at runtime
cargo build --features icons
./target/debug/ftime --icons

# Pipe usage (outputs plain text)
ftime | grep ".rs"
```
