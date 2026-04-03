# ftime Architecture (v1.0)

## 1. Module Structure
The current codebase separates concerns to keep FS mode stable while allowing future extensions (e.g., Git).

```text
src/
├── main.rs          # Entry point, CLI parsing (clap), Mode selection (TTY check)
├── engine.rs        # Core logic: Scanning (depth=1), Sorting, Filtering (exclude_dots / ignore)
├── model.rs         # Data structures (FileEntry, TimeBucket, ChildActivityHint)
├── view/
│   ├── mod.rs       # View trait or switch
│   ├── tty.rs       # Colored/Rich output logic
│   ├── text.rs      # Plain/Pipe output logic
│   └── json.rs      # JSON Lines output (`--json`, feature json)
└── util/
    ├── ignore.rs    # Ignore file loading (`~/.ftimeignore`, local `.ftimeignore`)
    └── time.rs      # Relative time calculation logic
```

## 2. Core Data Structures

```rust
// Unified entry point for both FS and Git modes (future)
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub mtime: SystemTime,
    pub is_symlink: bool,
    pub symlink_target: Option<PathBuf>,
    // v0.2: pub btime: Option<SystemTime>,
    // v1.1: pub git_status: Option<GitStatus>,
}

pub enum TimeBucket {
    Active,   // < 1h
    Today,    // Today
    ThisWeek, // < 7d
    History,  // Older
}

pub enum ChildActivityHint {
    Active,
    Today,
}
```

## 3. Non-Functional Requirements
*   **Zero Panic:** Use `Result` propagation. Handle `std::io::Error` gracefully.
*   **Performance:**
    *   Use `std::fs::read_dir` (sync I/O is fine for v1.0).
    *   Avoid recursion to prevent stack overflow or massive delays.
*   **Dependencies:**
    *   `clap` (derive feature)
    *   `colored` (simple coloring)
    *   `chrono` (time math)
*   `std::io::IsTerminal` (TTY detection; `is-terminal` crate kept for compatibility)
*   **Toolchain:** Rust edition 2024 (MSRV 1.92).

## 4. Responsibility Boundaries
*   `engine`: `scan_dir` で depth=1 のみを列挙し、`FileEntry` を `mtime` DESC（tie-break: `name` ASC）でソート後、`bucketize` で `TimeBucket` に振り分ける。`ScanOptions` は `exclude_dots` / `ext_filter` / ignore（デフォルト + グローバル + ローカル `.ftimeignore`）/ label無効化 を受け付ける。`dir_child_activity_hint` is also owned here so child activity uses the same inclusion semantics as the root scan.
*   `util::time`: `classify_bucket`/`relative_time`/`absolute_time`/`current_timezone_offset` など時間境界と整形を集約し、境界テストをここに集中させる。
*   `view::tty` / `view::text` / `view::json`: 出力レイアウトのみを担当し、エンジンのソート順・バケット順を崩さない。TTY は `name | size | time` と timezone footer を扱う. TTY may append a child activity suffix on directory rows; text and JSON stay unchanged.
*   `view::icon`（現行機能）: アイコン提供を抽象化。デフォルトは絵文字、`icons` feature + `--icons` 指定時に Nerd Font グリフへ差し替え。フォント未導入でもフォールバック可能であることを保証。
*   `engine::ScanOptions` は `exclude_dots`, `ext_filter`（拡張子ホワイトリスト）を受け取り、ファイル拡張子によるフィルタはスキャン段階で適用する（ディレクトリは除外）。
