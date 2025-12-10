use assert_cmd::Command;
use filetime::{set_file_mtime, FileTime};
use predicates::prelude::*;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, SystemTime};
use tempfile::tempdir;

#[allow(deprecated)]
fn bin() -> Command {
    Command::cargo_bin("ftime").unwrap()
}

#[test]
fn fails_when_path_is_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("a.txt");
    File::create(&file_path).unwrap();

    bin()
        .arg(&file_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("not a directory"));
}

#[test]
fn hidden_files_excluded_by_default_and_included_with_flag() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("visible")).unwrap();
    File::create(dir.path().join(".hidden")).unwrap();

    // default: hidden excluded
    bin()
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("visible"))
        .stdout(predicate::str::is_match("\\.hidden").unwrap().not());

    // with -H: hidden included
    bin()
        .current_dir(dir.path())
        .arg("-H")
        .assert()
        .success()
        .stdout(predicate::str::contains(".hidden"));
}

#[test]
fn history_bucket_collapses_and_expands() {
    let dir = tempdir().unwrap();
    // create 25 old files (>7 days)
    let old_time = SystemTime::now() - Duration::from_secs(9 * 24 * 3600);
    for i in 0..25 {
        let path = dir.path().join(format!("old-{i}"));
        File::create(&path).unwrap();
        set_file_mtime(&path, FileTime::from_system_time(old_time)).unwrap();
    }

    // default: collapsed
    bin()
        .arg(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("History (25 files hidden)"));

    // expanded with -a: shows list and summary line because >20
    bin()
        .arg(dir.path())
        .arg("-a")
        .env("FTIME_FORCE_TTY", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("💤 History"))
        .stdout(predicate::str::contains("... and 5 more items"));
}

#[test]
fn icons_flag_keeps_output_stable() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("f1");
    File::create(&file_path).unwrap();

    // add a history item to exercise the History header
    let old_path = dir.path().join("old");
    File::create(&old_path).unwrap();
    let old_time = SystemTime::now() - Duration::from_secs(9 * 24 * 3600);
    set_file_mtime(&old_path, FileTime::from_system_time(old_time)).unwrap();

    bin()
        .arg(dir.path())
        .arg("--icons")
        .env("FTIME_FORCE_TTY", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("History"));
}

#[test]
fn pipe_mode_outputs_tab_separated_without_headers() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("f1");
    File::create(&file_path).unwrap();

    let output = bin().arg(dir.path()).output().unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("f1\t"));
    assert!(!stdout.contains("Active Context"));
}

#[test]
fn pipe_mode_formats_dirs_and_symlinks_as_plain_paths() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("file");
    File::create(&file_path).unwrap();

    let subdir = dir.path().join("subdir");
    std::fs::create_dir(&subdir).unwrap();

    let link_path = dir.path().join("link_to_file");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&file_path, &link_path).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&file_path, &link_path).unwrap();

    // ensure deterministic ordering by setting mtimes
    let now = SystemTime::now();
    set_file_mtime(&file_path, FileTime::from_system_time(now)).unwrap();
    set_file_mtime(
        &subdir,
        FileTime::from_system_time(now - Duration::from_secs(1)),
    )
    .unwrap();
    set_file_mtime(
        &link_path,
        FileTime::from_system_time(now - Duration::from_secs(2)),
    )
    .unwrap();

    let output = bin().current_dir(dir.path()).output().unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("file\t")); // plain file
    assert!(stdout.contains("subdir\t")); // directory without trailing slash
    assert!(stdout.contains("link_to_file\t")); // symlink path only
    assert!(!stdout.contains("->")); // no target shown in pipe mode
}

#[test]
fn ignores_ds_store_and_thumbs_db_even_with_hidden() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("visible")).unwrap();
    File::create(dir.path().join(".DS_Store")).unwrap();
    File::create(dir.path().join("Thumbs.db")).unwrap();
    File::create(dir.path().join(".hidden")).unwrap();

    // default: .DS_Store, Thumbs.db excluded
    let out_default = bin().current_dir(dir.path()).output().unwrap();
    let stdout = String::from_utf8(out_default.stdout).unwrap();
    assert!(stdout.contains("visible"));
    assert!(!stdout.contains(".DS_Store"));
    assert!(!stdout.contains("Thumbs.db"));

    // even with --hidden they stay excluded, but hidden file is shown
    let out_hidden = bin()
        .current_dir(dir.path())
        .arg("--hidden")
        .output()
        .unwrap();
    let stdout_h = String::from_utf8(out_hidden.stdout).unwrap();
    assert!(stdout_h.contains(".hidden"));
    assert!(!stdout_h.contains(".DS_Store"));
    assert!(!stdout_h.contains("Thumbs.db"));
}

#[test]
fn json_output_contains_expected_fields() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("f1");
    File::create(&file_path).unwrap();

    let output = bin()
        .current_dir(dir.path())
        .arg("--json")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut lines = stdout.lines();
    let first = lines.next().expect("one line present");
    let v: Value = serde_json::from_str(first).unwrap();
    assert_eq!(v.get("path").unwrap(), "f1");
    assert!(v.get("bucket").is_some());
    assert!(v.get("mtime").is_some());
    assert!(v.get("relative_time").is_some());
    assert_eq!(v.get("is_dir").unwrap(), false);
    assert!(v.get("label").is_some());
}

#[test]
fn ext_filter_filters_files_case_insensitively() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("keep.rs")).unwrap();
    File::create(dir.path().join("keep.RS")).unwrap();
    File::create(dir.path().join("drop.txt")).unwrap();

    let output = bin()
        .current_dir(dir.path())
        .arg("--ext")
        .arg("rs")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("keep.rs"));
    assert!(stdout.contains("keep.RS"));
    assert!(!stdout.contains("drop.txt"));
}

#[test]
fn global_ignore_file_is_respected_and_no_ignore_overrides() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("keep.log")).unwrap();
    File::create(dir.path().join("skip.tmp")).unwrap();

    // create temporary ignore file
    let ig = tempdir().unwrap();
    let ig_path = ig.path().join("ignore");
    let mut f = File::create(&ig_path).unwrap();
    writeln!(f, "*.tmp").unwrap();
    writeln!(f, "# comment").unwrap();
    writeln!(f, "").unwrap();

    // default: skip.tmp should be hidden
    let out = bin()
        .current_dir(dir.path())
        .env("FTIME_IGNORE", &ig_path)
        .output()
        .unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("keep.log"));
    assert!(!stdout.contains("skip.tmp"));

    // with --no-ignore it should appear
    let out_no = bin()
        .current_dir(dir.path())
        .env("FTIME_IGNORE", &ig_path)
        .arg("--no-ignore")
        .output()
        .unwrap();
    let stdout_no = String::from_utf8(out_no.stdout).unwrap();
    assert!(stdout_no.contains("skip.tmp"));
}

#[test]
fn fresh_label_shows_and_can_be_disabled() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("newfile");
    File::create(&file_path).unwrap();

    // label should appear in TTY with FTIME_FORCE_TTY
    let out = bin()
        .current_dir(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("Fresh"));

    // --no-labels should remove it
    let out2 = bin()
        .current_dir(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .arg("--no-labels")
        .output()
        .unwrap();
    let stdout2 = String::from_utf8(out2.stdout).unwrap();
    assert!(!stdout2.contains("Fresh"));
}
