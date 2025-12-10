# ftime Architecture Proposal

## 1. Module Structure
We recommend the following separation of concerns to support future Git integration.

```text
src/
├── main.rs          # Entry point, CLI parsing (clap), Mode selection (TTY check)
├── engine.rs        # Core logic: Scanning, Sorting, Filtering
├── model.rs         # Data structures (FileEntry, TimeBucket)
├── view/
│   ├── mod.rs       # View trait or switch
│   ├── tty.rs       # Colored/Rich output logic
│   └── text.rs      # Plain/Pipe output logic
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
