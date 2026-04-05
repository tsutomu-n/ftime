use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Dir,
    Symlink,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub kind: EntryKind,
    pub mtime: SystemTime,
    pub size: Option<u64>,
    pub symlink_target: Option<PathBuf>,
}

impl FileEntry {
    pub fn is_dir(&self) -> bool {
        self.kind == EntryKind::Dir
    }

    pub fn is_symlink(&self) -> bool {
        self.kind == EntryKind::Symlink
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeBucket {
    Active,
    Today,
    ThisWeek,
    History,
}

impl TimeBucket {
    pub fn key(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Today => "today",
            Self::ThisWeek => "this_week",
            Self::History => "history",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Today => "Today",
            Self::ThisWeek => "This Week",
            Self::History => "History",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildActivityHint {
    Active,
    Today,
}
