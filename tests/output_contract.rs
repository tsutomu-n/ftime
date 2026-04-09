use assert_cmd::Command;
use filetime::{FileTime, set_file_mtime};
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::time::{Duration, SystemTime};
use tempfile::tempdir;

#[allow(deprecated)]
fn bin() -> Command {
    Command::cargo_bin("ftime").unwrap()
}

fn stdout(mut cmd: Command) -> String {
    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "command failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn line_containing<'a>(stdout: &'a str, needle: &str) -> &'a str {
    stdout
        .lines()
        .find(|line| line.contains(needle))
        .unwrap_or_else(|| panic!("missing line containing `{needle}` in:\n{stdout}"))
}

fn human_stdout(path: &Path, args: &[&str]) -> String {
    let mut cmd = bin();
    cmd.arg(path);
    for arg in args {
        cmd.arg(arg);
    }
    cmd.env("FTIME_FORCE_TTY", "1").env("NO_COLOR", "1");
    stdout(cmd)
}

#[cfg(unix)]
fn create_file_symlink(target: &Path, link: &Path) {
    std::os::unix::fs::symlink(target, link).unwrap();
}

#[cfg(windows)]
fn create_file_symlink(target: &Path, link: &Path) {
    std::os::windows::fs::symlink_file(target, link).unwrap();
}

#[test]
fn human_view_keeps_type_size_and_hint_contracts() {
    let dir = tempdir().unwrap();
    let now = SystemTime::now();

    let active = dir.path().join("alpha.txt");
    fs::write(&active, b"a").unwrap();
    set_file_mtime(
        &active,
        FileTime::from_system_time(now - Duration::from_secs(30)),
    )
    .unwrap();

    let docs = dir.path().join("docs");
    fs::create_dir(&docs).unwrap();
    let docs_child = docs.join("guide.md");
    fs::write(&docs_child, b"guide").unwrap();
    set_file_mtime(
        &docs,
        FileTime::from_system_time(now - Duration::from_secs(2 * 3600)),
    )
    .unwrap();
    set_file_mtime(
        &docs_child,
        FileTime::from_system_time(now - Duration::from_secs(45)),
    )
    .unwrap();

    let target = dir.path().join("target.txt");
    fs::write(&target, b"target").unwrap();
    let link = dir.path().join("link_to_target");
    create_file_symlink(&target, &link);

    let mut cmd = bin();
    cmd.arg(dir.path())
        .arg("--hints")
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1");
    let stdout = stdout(cmd);

    let file_line = line_containing(&stdout, "alpha.txt");
    assert!(file_line.contains("[FIL]"), "{file_line}");
    assert!(file_line.contains("1 B"), "{file_line}");

    let dir_line = line_containing(&stdout, "docs/");
    assert!(dir_line.contains("[DIR]"), "{dir_line}");
    assert!(dir_line.contains("<dir>"), "{dir_line}");
    assert!(dir_line.contains("[child: active]"), "{dir_line}");

    let link_line = line_containing(&stdout, "link_to_target");
    assert!(link_line.contains("[LNK]"), "{link_line}");
    assert!(link_line.contains("<lnk>"), "{link_line}");
    assert!(!link_line.contains("->"), "{link_line}");
}

#[test]
fn plain_and_json_keep_machine_shapes_without_human_only_fields() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("alpha.txt");
    File::create(&file_path).unwrap();

    let plain = stdout({
        let mut cmd = bin();
        cmd.arg(dir.path()).arg("--plain");
        cmd
    });
    let plain_line = plain.lines().next().unwrap();
    let cols: Vec<&str> = plain_line.split('\t').collect();
    assert_eq!(cols.len(), 3);
    assert_eq!(cols[0], "alpha.txt");
    assert_eq!(cols[1], "active");
    assert!(!plain_line.contains("[FIL]"));
    assert!(!plain_line.contains("<dir>"));

    let json = stdout({
        let mut cmd = bin();
        cmd.arg(dir.path()).arg("--json");
        cmd
    });
    let first_line = json.lines().next().unwrap();
    assert!(first_line.starts_with(
        "{\"path\":\"alpha.txt\",\"bucket\":\"active\",\"mtime\":"
    ));
    assert!(first_line.contains(
        ",\"relative_time\":"
    ));
    assert!(first_line.contains(
        ",\"is_dir\":false,\"is_symlink\":false,\"size\":0}"
    ));
    let first: Value = serde_json::from_str(first_line).unwrap();
    assert_eq!(first["path"], "alpha.txt");
    assert_eq!(first["bucket"], "active");
    assert_eq!(first["is_dir"], false);
    assert_eq!(first["is_symlink"], false);
    assert_eq!(first["size"], 0);
    assert!(first.get("label").is_none());
    assert!(first.get("child_hint").is_none());
}

#[test]
fn human_view_keeps_history_preview_and_all_history_header_contract() {
    let dir = tempdir().unwrap();
    let old_time = SystemTime::now() - Duration::from_secs(9 * 24 * 3600);

    for i in 0..7 {
        let path = dir.path().join(format!("old-{i}.txt"));
        fs::write(&path, b"x").unwrap();
        set_file_mtime(&path, FileTime::from_system_time(old_time)).unwrap();
    }

    let preview = human_stdout(dir.path(), &[]);
    assert!(preview.contains("History (5/7)"));
    assert!(!preview.contains("old-6.txt"));

    let expanded = human_stdout(dir.path(), &["--all-history"]);
    assert!(expanded.contains("History (7)"));
    assert!(expanded.contains("old-6.txt"));
}

#[test]
fn human_empty_state_only_shows_filters_when_non_default_filters_are_active() {
    let dir = tempdir().unwrap();

    let default_empty = human_stdout(dir.path(), &[]);
    assert!(default_empty.contains("No matching entries"));
    assert!(!default_empty.contains("filters:"));

    fs::create_dir(dir.path().join(".hidden_dir")).unwrap();
    let filtered_empty = human_stdout(dir.path(), &["--hide-dots"]);
    assert!(filtered_empty.contains("No matching entries"));
    assert!(filtered_empty.contains("filters:"));
}

#[test]
fn plain_absolute_keeps_tsv_shape_and_only_changes_time_column() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("alpha.txt");
    File::create(&file_path).unwrap();
    let fixed = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    set_file_mtime(&file_path, FileTime::from_system_time(fixed)).unwrap();

    let plain = stdout({
        let mut cmd = bin();
        cmd.arg(dir.path()).arg("--plain").arg("--absolute");
        cmd
    });
    let line = plain.lines().next().unwrap();
    let cols: Vec<&str> = line.split('\t').collect();
    assert_eq!(cols.len(), 3);
    assert_eq!(cols[0], "alpha.txt");
    assert_eq!(cols[1], "history");
    assert!(cols[2].starts_with("2023-"));
    assert!(cols[2].contains(" (UTC"));
}
