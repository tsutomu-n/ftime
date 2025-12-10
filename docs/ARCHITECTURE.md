# ftime Architecture Proposal

## 1. Module Structure
We recommend the following separation of concerns to support future Git integration.

```text
src/
├── main.rs          # Entry point, CLI parsing (clap), Mode selection (TTY check)
├── engine.rs        # Core logic: Scanning (depth=1), Sorting, Filtering (hidden)
├── model.rs         # Data structures (FileEntry, TimeBucket)
├── view/
│   ├── mod.rs       # View trait or switch
│   ├── tty.rs       # Colored/Rich output logic
│   ├── text.rs      # Plain/Pipe output logic
│   └── json.rs      # JSON Lines output (`--json`, feature json)
└── util/
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
```

## 3. Non-Functional Requirements
*   **Zero Panic:** Use `Result` propagation. Handle `std::io::Error` gracefully.
*   **Performance:**
    *   Use `std::fs::read_dir` (sync I/O is fine for v0.1).
    *   Avoid recursion to prevent stack overflow or massive delays.
*   **Dependencies:**
    *   `clap` (derive feature)
    *   `colored` (simple coloring)
    *   `chrono` (time math)
*   `std::io::IsTerminal` (TTY detection; `is-terminal` crate kept for compatibility)

## 4. Responsibility Boundaries
*   `engine`: `scan_dir` で depth=1 のみを列挙し、`FileEntry` を `mtime` DESC（tie-break: `name` ASC）でソート後、`bucketize` で `TimeBucket` に振り分ける。
*   `util::time`: `classify_bucket`/`relative_time` など時間境界を集約し、境界テストをここに集中させる。
*   `view::tty` / `view::text` / `view::json`: 出力レイアウトのみを担当し、エンジンのソート順・バケット順を崩さない。
*   `view::icon`（v0.1以降追加）: アイコン提供を抽象化。デフォルトは絵文字、`icons` feature + `--icons` 指定時に Nerd Font グリフへ差し替え。フォント未導入でもフォールバック可能であることを保証。
