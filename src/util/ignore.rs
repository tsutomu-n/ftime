use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn load_ignore_patterns() -> Vec<String> {
    if let Some(path) = env::var_os("FTIME_IGNORE") {
        return read_ignore_file(PathBuf::from(path));
    }
    if let Some(home) = env::var_os("HOME") {
        let default = PathBuf::from(home).join(".ftimeignore");
        return read_ignore_file(default);
    }
    Vec::new()
}

pub fn load_local_ignore(root: &Path) -> Vec<String> {
    let candidate = root.join(".ftimeignore");
    read_ignore_file(candidate)
}

fn read_ignore_file(path: PathBuf) -> Vec<String> {
    let Ok(contents) = fs::read_to_string(path) else {
        return Vec::new();
    };
    contents
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .collect()
}
