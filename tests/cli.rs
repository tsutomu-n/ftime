use assert_cmd::Command;
use filetime::{set_file_mtime, FileTime};
use predicates::prelude::*;
use std::fs::File;
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
