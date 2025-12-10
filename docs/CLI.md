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
| `-H` | `--hidden` | Include hidden files (starting with `.`). |
| `-h` | `--help` | Print help message. |
| `-V` | `--version` | Print version information. |

## 4. Environment Variables
*   `NO_COLOR`: If present (regardless of value), disable color output.
*   `FTIME_FORCE_TTY`: If present, force TTY-style grouped/color output even when stdout is piped or redirected.

## 5. Exit Codes
*   `0`: Success.
*   `1`: General error (e.g., directory not found, permission denied on target root).

## 6. Usage Examples
```bash
# Basic usage
ftime

# Scan specific directory
ftime ~/Downloads

# Show everything including dotfiles and history
ftime -a -H

# Pipe usage (outputs plain text)
ftime | grep ".rs"
```
