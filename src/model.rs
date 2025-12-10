use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub mtime: SystemTime,
    pub is_symlink: bool,
    #[allow(dead_code)]
    pub symlink_target: Option<PathBuf>,
    pub label: Option<Label>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeBucket {
    Active,
    Today,
    ThisWeek,
    History,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Label {
    Fresh,
}
