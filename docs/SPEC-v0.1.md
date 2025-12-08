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

## 4. Sorting Strategy
1.  Collect all valid entries in the target directory.
2.  Sort all entries by `mtime` DESC (newest first).
3.  Distribute into buckets preserving the sort order.

## 5. Output Format (TTY Mode)
When `stdout` is a terminal:
*   **Headers:** Display bucket icon and name (e.g., `🔥 Active Context (< 1h)`).
*   **Entries:**
    *   Format: `<padding> <icon> <filename> <padding> <relative_time>`
    *   Directory distinction: Append `/` to directory names and apply **Bold Blue** color.
    *   Time format: `12 mins ago`, `3 hours ago`, `Yesterday`, `YYYY-MM-DD`.
*   **History:** By default, collapse "History" bucket (show only count, e.g., `💤 History (128 files hidden)`). Expand if `--all` is set.

## 6. Output Format (Pipe/File Mode)
When `stdout` is **NOT** a terminal:
*   **Disable:** All colors, headers, icons, and bucket groupings.
*   **Content:** List all files (sorted by mtime desc).
*   **Format:** `<path>\t<relative_time>` (Tab-separated).
*   **Limit:** Do NOT apply the 20-item limit (output all).

## 7. Filtering
*   **Hidden Files:** Ignore entries starting with `.` by default. Include them if `--hidden` is passed.