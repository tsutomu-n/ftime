# ftime v0.1.0 Test Plan

## 1. Unit Tests (Logic)
*   **Time Bucketing:**
    *   Mock current time and verify files fall into correct buckets (Active vs Today vs Week).
    *   Verify boundary conditions (e.g., exactly 1 hour ago).
*   **Sorting:**
    *   Verify files are strictly sorted by `mtime` descending.

## 2. Integration Tests (CLI)
*   **Default Behavior:**
    *   Run `ftime` in a folder with mixed file ages. Verify buckets appear.
*   **Filtering:**
    *   Create `.hidden_file`. Verify it does not appear by default.
    *   Run `ftime -H`. Verify it appears.
*   **History Folding:**
    *   Create >21 old files. Verify "History" bucket shows only count/summary by default.
    *   Run `ftime -a`. Verify all history files are listed.
*   **Pipe Behavior:**
    *   Run `ftime | cat`. Verify output is plain text (no color codes) and tab-separated.

## 3. Edge Cases
*   **Empty Directory:** Should output nothing (or "No recent files found" message) without crashing.
*   **Permission Denied:** Run on a folder with a locked file inside. Should skip file without crashing.
*   **Broken Symlink:** Should list the link itself without crashing.
*   **Symlink Display:** (TTY) Shows `name -> target` with the link colored and target dimmed.
