# ftime v0.1.0 Behavior Specification

## 1. Scope
*   **Target:** Local filesystem visualization.
*   **Mode:** Read-only CLI tool.
*   **Limitations:**
    *   No Git integration (v0.1).
    *   No recursive scanning (depth=1 only).
    *   No heuristic provenance labels (Fresh/Imported logic is for v0.2).

## 2. Input Model
*   **Source:** Single directory path provided via CLI argument. (Defaults to current directory `.`).
*   **Entry Handling:**
    *   Symlinks: Follow metadata of the link itself (`lstat`), not target.
    *   Broken Links: Treat as regular files (do not panic).
    *   Permissions: Skip entries with permission errors silently (or log to stderr if verbose).
    *   Default ignore (always skipped): `.DS_Store`, `Thumbs.db`（`--hidden` でも除外）

## 3. Time Bucketing Logic
Files are sorted by `mtime` (descending) and grouped into buckets. Evaluation order is strictly top-to-bottom.

| Bucket Name | Condition |
| :--- | :--- |
| **🔥 Active Context** | `now - mtime < 1 hour` |
| **☕ Today's Session** | `mtime >= Today 00:00:00` (Local Time) |
| **📅 This Week** | `now - mtime < 7 days` |
| **💤 History** | Everything else |

*   **Display Limit:** Max **20 items** per bucket. If exceeded, show top 20 and append a summary line (e.g., `... and 42 more items`).
*   **Empty Buckets:** Do not display headers for empty buckets.
*   **Symlinks:** Show as `name -> target` and color the source name yellow. If target resolution fails, display `<unresolved>`.

## 4. Sorting Strategy
1.  Collect all valid entries in the target directory.
2.  Sort all entries by `mtime` DESC (newest first). When `mtime` is equal, sort by `name` ASC for stability.
3.  Distribute into buckets preserving the sort order.

## 5. Output Format (TTY Mode)
When `stdout` is a terminal:
*   **Headers:** Display bucket icon and name (e.g., `🔥 Active Context (< 1h)`).
*   **Entries:**
    *   Format: `<padding> <icon> <filename> <padding> <relative_time>`; symlinks include `-> target`.
    *   Directory distinction: Append `/` to directory names and apply **Bold Blue** color. Symlinks are Yellow, targets are dimmed.
    *   Time format: `just now` (<60s), `1 min ago`, `12 mins ago`, `3 hours ago`, `Yesterday`, `YYYY-MM-DD`.
*   **Empty Directory:** If no entries are found, print `No recent files found`.
*   **History:** By default, collapse "History" bucket (show only count, e.g., `💤 History (128 files hidden)`). Expand if `--all` is set.

## 6. Output Format (Pipe/File Mode)
When `stdout` is **NOT** a terminal:
*   **Disable:** All colors, headers, icons, and bucket groupings.
*   **Content:** List all files (sorted by mtime desc). Symlink targets are not shown;ディレクトリも末尾`/`なしでパスのみ。
*   **Format:** `<path>\t<relative_time>` (Tab-separated).
*   **Limit:** Do NOT apply the 20-item limit (output all).

## 7. Output Format (JSON Mode)
*   Triggered by `--json`.
*   Emits one JSON object per line (JSON Lines).
*   Fields (**frozen for compatibility**):
    * `path`: string（可能なら基準ディレクトリ相対）
    * `bucket`: `"active" | "today" | "this_week" | "history"`
    * `mtime`: string (RFC3339, UTC)
    * `relative_time`: string（TTY/pipeと同じ表記）
    * `is_dir`: bool
    * `is_symlink`: bool
    * `symlink_target`: string|null（可能なら相対）
*   Colors/icons/20件制限は無効。TTY/非TTYに依存しない。

## 8. Filtering
*   **Hidden Files:** Ignore entries starting with `.` by default. Include them if `--hidden` is passed.
*   **Extension Filter:** `--ext ext1,ext2` で拡張子ホワイトリスト（case-insensitive）。対象はファイルのみで、ディレクトリ/拡張子なしファイルは除外される。

## 9. Environment Overrides
*   `NO_COLOR`: Disable color output when set. **最優先**で適用する（TTY強制より優先）。
*   `FTIME_FORCE_TTY`: Force TTY mode (bucketed layout) even when stdout is not a terminal。色の有無は `NO_COLOR` に従う。
