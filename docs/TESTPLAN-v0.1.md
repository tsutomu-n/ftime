# ftime v0.1.0 Test Plan
最終更新: 2025-12-10 / 実装状態: v0.2相当（JSON・ext・ignore・Freshラベル込み）

## 1. Unit Tests (Logic)
*   **Time Bucketing:**
    *   Mock current time and verify files fall into correct buckets (Active vs Today vs Week).
    *   Verify boundary conditions (e.g., exactly 1 hour ago).
*   **Relative Time Strings:**
    *   <60s: `just now`
    *   1 min: `1 min ago`
    *   N mins (N>1): `{N} mins ago`
*   **Sorting:**
    *   Verify files are strictly sorted by `mtime` descending.
    *   When `mtime` is equal, verify sorting by `name` ascending for stability.
*   **Labels (Fresh):**
    *   `now - mtime <= 5m` → `Some(Fresh)`
    *   `now - mtime > 5m` → `None`
    *   `--no-labels` フラグで常に `None`

## 2. Integration Tests (CLI)
*   **Default Behavior:**
    *   Run `ftime` in a folder with mixed file ages. Verify buckets appear.
*   **Filtering:**
    *   Create `.hidden_file`. Verify it does not appear by default.
    *   Run `ftime -H`. Verify it appears.
    *   Create files with extensions; `--ext rs,toml` で whitelisted のみ表示。大小無視。
*   **History Folding:**
    *   Create >21 old files. Verify "History" bucket shows only count/summary by default.
    *   Run `ftime -a`. Verify all history files are listed.
*   **Pipe Behavior:**
    *   Run `ftime | cat`. Verify output is plain text (no color codes) and tab-separated.
*   **JSON Output:**
    *   `ftime --json` returns JSON Lines with fields {path,bucket,mtime,relative_time,is_dir,is_symlink,symlink_target,label}
    *   `label` is `"fresh"` only when assigned; otherwise omitted.
    *   `symlink_target` is present only for resolved symlinks; otherwise omitted.
*   **Ignore Rules:**
    *   デフォルト除外: `.DS_Store` / `Thumbs.db`。
    *   グローバル ignore: `FTIME_IGNORE` で指すファイル、なければ `~/.ftimeignore`（`#`コメント・空行無視・簡易グロブ）。
    *   ローカル ignore: ルート直下の `.ftimeignore`（存在すればグローバルに後続で適用）。
    *   `--no-ignore` で全ignoreを無効化。

## 3. Edge Cases
*   **Empty Directory:** Should print `No recent files found` (no crash).
*   **Permission Denied:** Run on a folder with a locked file inside. Should skip file without crashing.
*   **Broken Symlink:** Should list the link itself without crashing.
*   **Symlink Display:** (TTY) Shows `name -> target` with the link colored and target dimmed.
*   **Icons feature off:** `--icons` is a no-op when built without icons; CLI should not error.
*   **PATH is file:** Should exit 1 before ignore/ext/label are applied.
