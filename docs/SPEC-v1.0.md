# ftime v1.0.5 Behavior Specification
Last updated: 2026-04-03

## 1. Scope
*   **Target:** Local filesystem visualization.
*   **Mode:** Read-only CLI tool.
*   **Limitations:**
    *   No Git integration.
    *   No recursive scanning (depth=1 only).
    *   Heuristic provenance labels remain limited to `Fresh`.

## 2. Input Model
*   **Source:** Single directory path provided via CLI argument. Defaults to current directory `.`.
*   **Entry Handling:**
    *   Symlinks use metadata of the link itself (`lstat`-style behavior), not the target.
    *   Broken links do not panic and may render as `<unresolved>` in TTY output.
    *   Per-entry permission or metadata failures are skipped silently.
    *   Built-in ignore entries are `.DS_Store` and `Thumbs.db`; they are disabled only by `--no-ignore`.
    *   Dotfiles are included by default and can be excluded with `--exclude-dots`.

## 3. Time Buckets
Evaluation order is fixed: Active → Today → This Week → History.

| Bucket | Condition |
| :--- | :--- |
| **Active** | `now - mtime < 1 hour`, or `mtime` is in the future |
| **Today** | `mtime >= Today 00:00:00` (local time) and not Active |
| **This Week** | `now - mtime < 7 days` and not Today |
| **History** | Everything else |

Future mtimes are rendered as `+Ns [Skew]` or `+Nm [Skew]`.

## 4. Output Contracts
### TTY
*   Bucketed output with icons/colors.
*   Entry format is `name | size | time`.
*   `time` is relative by default, or absolute with `-A/--absolute`.
*   Time column uses a bucket-aware heatmap.
*   `Skew` styling takes precedence over bucket heatmap.
*   Directory rows may append `[child: active]` or `[child: today]` when a direct child is hotter than the directory itself.
*   Child activity hints never reclassify the parent bucket.
*   History is collapsed by default; `-a/--all` expands it.
*   A `Current Timezone: UTC±HH:MM` footer is appended.

### Pipe / redirected output
*   Plain two-column TSV: `<path>\t<time>`.
*   No colors, headers, icons, bucket grouping, or item limit.
*   `time` is relative by default, or `YYYY-MM-DD HH:MM:SS (UTC±HH:MM)` with `-A/--absolute`.
*   This hint is TTY-only and does not appear in pipe or JSON output.

### JSON Lines
*   Triggered by `--json`.
*   One object per line.
*   Stable fields:
    * `path`
    * `bucket`
    * `mtime`
    * `relative_time`
    * `is_dir`
    * `is_symlink`
*   Optional fields:
    * `symlink_target`
    * `label`
    * `size` (regular files only)

## 5. Filters and Flags
*   `-a, --all`: Expand History in TTY mode.
*   `-A, --absolute`: Render absolute local timestamps in TTY and pipe modes.
*   `--exclude-dots`: Exclude dotfiles from scan results.
*   `--ext ext1,ext2`: File-only extension whitelist, case-insensitive.
*   `--no-ignore`: Disable built-in ignores and ignore files.
*   `--no-labels`: Disable labels such as `Fresh`.
*   `-I, --icons`: Opt-in Nerd Font icons when the binary supports the `icons` feature.

`-H/--hidden` is not part of the public v1.0.5 CLI contract.

## 6. Environment
*   `NO_COLOR`: Disable colors even if set to an empty string.
*   `FTIME_FORCE_TTY`: Force TTY layout even when stdout is not a terminal.
*   `FTIME_IGNORE`: Override the global ignore file path.

## 7. Compatibility Policy
*   v1.0.5 freezes the current public CLI contract for future compatibility.
*   The public v1.0.5 contract includes dotfiles by default and uses `--exclude-dots` as the opt-out flag.
*   Pipe output remains 2 columns.
*   JSON remains JSON Lines and may omit optional fields when not applicable.
