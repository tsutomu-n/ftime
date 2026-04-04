mod support;

use assert_cmd::Command;
use filetime::{FileTime, set_file_mtime};
use predicates::prelude::*;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
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
fn hidden_files_included_by_default_and_excluded_with_flag() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("visible")).unwrap();
    File::create(dir.path().join(".hidden")).unwrap();

    bin()
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("visible"))
        .stdout(predicate::str::contains(".hidden"));

    bin()
        .current_dir(dir.path())
        .arg("--exclude-dots")
        .assert()
        .success()
        .stdout(predicate::str::contains("visible"))
        .stdout(predicate::str::is_match("\\.hidden").unwrap().not());
}

#[test]
fn removed_hidden_flag_is_rejected() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join(".hidden")).unwrap();

    bin()
        .current_dir(dir.path())
        .arg("-H")
        .assert()
        .failure()
        .stderr(predicate::str::contains("-H"));
}

#[test]
fn version_reports_current_package_version() {
    bin()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "ftime {}",
            support::package_version()
        )));
}

#[test]
fn help_mentions_file_time_concept_and_reinstall_note() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "files by time: a read-only File Time CLI",
        ))
        .stdout(predicate::str::contains("--check-update"))
        .stdout(predicate::str::contains(
            "If your installed binary predates --self-update",
        ));
}

#[test]
fn help_lists_check_update_before_self_update() {
    let output = bin().arg("--help").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let check = stdout.find("--check-update").unwrap();
    let update = stdout.find("--self-update").unwrap();

    assert!(check < update);
}

#[cfg(unix)]
#[test]
fn self_update_runs_installer_for_current_binary_dir() {
    let dir = tempdir().unwrap();
    let script_path = dir.path().join("install.sh");
    let marker_path = dir.path().join("marker.txt");

    fs::write(
        &script_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$INSTALL_DIR\" > \"$FTIME_SELF_UPDATE_MARKER\"\n",
    )
    .unwrap();

    let output = bin()
        .arg("--self-update")
        .env(
            "FTIME_SELF_UPDATE_URL",
            format!("file://{}", script_path.display()),
        )
        .env(
            "FTIME_SELF_UPDATE_INSTALL_DIR",
            dir.path().display().to_string(),
        )
        .env("FTIME_SELF_UPDATE_MARKER", &marker_path)
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("self-update completed"));

    let recorded = fs::read_to_string(marker_path).unwrap();
    assert_eq!(recorded.trim(), dir.path().display().to_string());
}

#[cfg(unix)]
#[test]
fn self_update_prefers_invoked_symlink_directory() {
    let dir = tempdir().unwrap();
    let real_dir = dir.path().join("real");
    let link_dir = dir.path().join("link");
    fs::create_dir_all(&real_dir).unwrap();
    fs::create_dir_all(&link_dir).unwrap();

    let source_bin = assert_cmd::cargo::cargo_bin!("ftime");
    let real_bin = real_dir.join("ftime");
    fs::copy(source_bin, &real_bin).unwrap();

    let link_bin = link_dir.join("ftime");
    std::os::unix::fs::symlink(&real_bin, &link_bin).unwrap();

    let script_path = dir.path().join("install.sh");
    let marker_path = dir.path().join("marker.txt");
    fs::write(
        &script_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$INSTALL_DIR\" > \"$FTIME_SELF_UPDATE_MARKER\"\n",
    )
    .unwrap();

    let output = Command::new(&link_bin)
        .arg("--self-update")
        .env(
            "FTIME_SELF_UPDATE_URL",
            format!("file://{}", script_path.display()),
        )
        .env("FTIME_SELF_UPDATE_MARKER", &marker_path)
        .output()
        .unwrap();

    assert!(output.status.success());
    let recorded = fs::read_to_string(marker_path).unwrap();
    assert_eq!(recorded.trim(), link_dir.display().to_string());
}

#[test]
fn self_update_rejects_scan_arguments() {
    let dir = tempdir().unwrap();

    bin()
        .arg("--self-update")
        .arg(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "--self-update cannot be combined with scan options or PATH",
        ));
}

#[test]
fn check_update_rejects_scan_arguments() {
    let dir = tempdir().unwrap();

    bin()
        .arg("--check-update")
        .arg(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "--check-update cannot be combined with scan options or PATH",
        ));
}

#[test]
fn check_update_reports_when_already_current() {
    bin()
        .arg("--check-update")
        .env(
            "FTIME_SELF_UPDATE_LATEST_VERSION",
            support::package_version(),
        )
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "ftime is already up to date at {}",
            support::package_version()
        )));
}

#[test]
fn check_update_reports_when_update_is_available() {
    bin()
        .arg("--check-update")
        .env("FTIME_SELF_UPDATE_LATEST_VERSION", "1.0.6")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "update available: {} -> 1.0.6",
            support::package_version()
        )));
}

#[test]
fn check_update_reports_when_latest_is_renumbered_lower() {
    bin()
        .arg("--check-update")
        .env(
            "FTIME_SELF_UPDATE_CURRENT_VERSION",
            support::package_version(),
        )
        .env("FTIME_SELF_UPDATE_LATEST_VERSION", "1.0.0")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "latest published release is 1.0.0 (current binary reports {})",
            support::package_version()
        )));
}

#[test]
fn self_update_rejects_cargo_build_outputs() {
    bin()
        .arg("--self-update")
        .env(
            "FTIME_SELF_UPDATE_URL",
            "file:///definitely-not-used-because-should-fail-first",
        )
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "--self-update is not available for cargo build outputs",
        ));
}

#[test]
fn history_bucket_collapses_and_expands() {
    let dir = tempdir().unwrap();
    let old_time = SystemTime::now() - Duration::from_secs(9 * 24 * 3600);
    for i in 0..25 {
        let path = dir.path().join(format!("old-{i}"));
        File::create(&path).unwrap();
        set_file_mtime(&path, FileTime::from_system_time(old_time)).unwrap();
    }

    bin()
        .arg(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("History (25 files hidden)"));

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
fn pipe_mode_outputs_two_tsv_columns_without_headers() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("f1");
    File::create(&file_path).unwrap();

    let output = bin().arg(dir.path()).output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let line = stdout.lines().next().unwrap();

    assert_eq!(line.split('\t').count(), 2);
    assert!(stdout.contains("f1\t"));
    assert!(!stdout.contains("Active Context"));
}

#[test]
fn absolute_time_flag_changes_pipe_output_format() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("f1");
    File::create(&file_path).unwrap();
    let fixed = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    set_file_mtime(&file_path, FileTime::from_system_time(fixed)).unwrap();

    let output = bin().arg(dir.path()).arg("--absolute").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let line = stdout.lines().next().unwrap();
    let cols: Vec<&str> = line.split('\t').collect();

    assert_eq!(cols.len(), 2);
    assert!(
        predicate::str::is_match(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} \(UTC[+-]\d{2}:\d{2}\)$")
            .unwrap()
            .eval(cols[1])
    );
}

#[test]
fn tty_output_shows_size_column_and_absolute_time() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("f1");
    File::create(&file_path).unwrap();
    let fixed = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    set_file_mtime(&file_path, FileTime::from_system_time(fixed)).unwrap();

    let output = bin()
        .arg(dir.path())
        .arg("--absolute")
        .env("FTIME_FORCE_TTY", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("0 B"));
    assert!(stdout.contains("|"));
    assert!(
        predicate::str::is_match(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} \(UTC[+-]\d{2}:\d{2}\)")
            .unwrap()
            .eval(&stdout)
    );
}

#[test]
fn tty_output_shows_skew_warning_and_timezone_footer() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("future-file");
    File::create(&file_path).unwrap();
    let future = SystemTime::now() + Duration::from_secs(5 * 60);
    set_file_mtime(&file_path, FileTime::from_system_time(future)).unwrap();

    let output = bin()
        .arg(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("[Skew]"));
    assert!(
        predicate::str::is_match(r"Current Timezone: UTC[+-]\d{2}:\d{2}")
            .unwrap()
            .eval(&stdout)
    );
}

#[test]
fn tty_output_with_no_color_keeps_text_contract_without_ansi() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("future-file");
    File::create(&file_path).unwrap();
    let future = SystemTime::now() + Duration::from_secs(5 * 60);
    set_file_mtime(&file_path, FileTime::from_system_time(future)).unwrap();

    let output = bin()
        .arg(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("[Skew]"));
    assert!(stdout.contains("Current Timezone: "));
    assert!(!stdout.contains("\u{1b}["));
}

#[test]
fn tty_output_shows_child_activity_hints_only_for_directories() {
    let dir = tempdir().unwrap();
    let docs_dir = dir.path().join("docs");
    fs::create_dir(&docs_dir).unwrap();
    let docs_child = docs_dir.join("guide.md");
    File::create(&docs_child).unwrap();

    let target_dir = dir.path().join("target");
    fs::create_dir(&target_dir).unwrap();
    let target_child = target_dir.join("artifact.bin");
    File::create(&target_child).unwrap();

    let readme = dir.path().join("README.md");
    File::create(&readme).unwrap();

    let now = SystemTime::now();
    set_file_mtime(
        &docs_child,
        FileTime::from_system_time(now - Duration::from_secs(2 * 3600)),
    )
    .unwrap();
    set_file_mtime(
        &target_child,
        FileTime::from_system_time(now - Duration::from_secs(30)),
    )
    .unwrap();
    set_file_mtime(
        &docs_dir,
        FileTime::from_system_time(now - Duration::from_secs(3 * 24 * 3600)),
    )
    .unwrap();
    set_file_mtime(
        &target_dir,
        FileTime::from_system_time(now - Duration::from_secs(14 * 24 * 3600)),
    )
    .unwrap();
    set_file_mtime(
        &readme,
        FileTime::from_system_time(now - Duration::from_secs(2 * 3600)),
    )
    .unwrap();

    let output = bin()
        .arg(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("docs/ | - |"));
    assert!(stdout.contains("[child: today]"));
    assert!(stdout.contains("target/ | - |"));
    assert!(stdout.contains("[child: active]"));
    assert!(!stdout.contains("README.md | 0 B | 2 hours ago [child:"));
}

#[test]
fn pipe_and_json_output_do_not_include_child_activity_hints() {
    let dir = tempdir().unwrap();
    let docs_dir = dir.path().join("docs");
    fs::create_dir(&docs_dir).unwrap();
    let docs_child = docs_dir.join("guide.md");
    File::create(&docs_child).unwrap();
    let now = SystemTime::now();
    set_file_mtime(
        &docs_child,
        FileTime::from_system_time(now - Duration::from_secs(30)),
    )
    .unwrap();
    set_file_mtime(
        &docs_dir,
        FileTime::from_system_time(now - Duration::from_secs(3 * 24 * 3600)),
    )
    .unwrap();

    let pipe_output = bin().arg(dir.path()).output().unwrap();
    let pipe_stdout = String::from_utf8(pipe_output.stdout).unwrap();
    assert!(!pipe_stdout.contains("[child:"));

    let json_output = bin().arg(dir.path()).arg("--json").output().unwrap();
    let json_stdout = String::from_utf8(json_output.stdout).unwrap();
    let first = json_stdout.lines().next().expect("one line present");
    let value: Value = serde_json::from_str(first).unwrap();
    assert!(value.get("child_activity").is_none());
}

#[test]
fn tty_child_activity_hint_respects_exclude_dots_and_child_local_ignore() {
    let dir = tempdir().unwrap();
    let hidden_dir = dir.path().join("hidden-only");
    fs::create_dir(&hidden_dir).unwrap();
    let hidden_child = hidden_dir.join(".recent.log");
    File::create(&hidden_child).unwrap();

    let ignored_dir = dir.path().join("ignored-only");
    fs::create_dir(&ignored_dir).unwrap();
    fs::write(ignored_dir.join(".ftimeignore"), "recent.log\n").unwrap();
    let ignored_child = ignored_dir.join("recent.log");
    File::create(&ignored_child).unwrap();

    let now = SystemTime::now();
    set_file_mtime(
        &hidden_child,
        FileTime::from_system_time(now - Duration::from_secs(30)),
    )
    .unwrap();
    set_file_mtime(
        &ignored_child,
        FileTime::from_system_time(now - Duration::from_secs(30)),
    )
    .unwrap();
    set_file_mtime(
        &hidden_dir,
        FileTime::from_system_time(now - Duration::from_secs(8 * 24 * 3600)),
    )
    .unwrap();
    set_file_mtime(
        &ignored_dir,
        FileTime::from_system_time(now - Duration::from_secs(8 * 24 * 3600)),
    )
    .unwrap();

    let output = bin()
        .arg(dir.path())
        .arg("--exclude-dots")
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    let hidden_line = stdout
        .lines()
        .find(|line| line.contains("hidden-only/ | - |"))
        .expect("hidden-only directory row should exist");
    let ignored_line = stdout
        .lines()
        .find(|line| line.contains("ignored-only/ | - |"))
        .expect("ignored-only directory row should exist");

    assert!(!hidden_line.contains("[child:"));
    assert!(!ignored_line.contains("[child:"));
}

#[test]
fn tty_child_activity_hint_is_not_shown_for_symlink_entries() {
    let dir = tempdir().unwrap();
    let real_dir = dir.path().join("docs");
    fs::create_dir(&real_dir).unwrap();
    let recent_child = real_dir.join("guide.md");
    File::create(&recent_child).unwrap();

    let link_dir = dir.path().join("docs-link");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&real_dir, &link_dir).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(&real_dir, &link_dir).unwrap();

    let now = SystemTime::now();
    set_file_mtime(
        &recent_child,
        FileTime::from_system_time(now - Duration::from_secs(30)),
    )
    .unwrap();
    set_file_mtime(
        &real_dir,
        FileTime::from_system_time(now - Duration::from_secs(8 * 24 * 3600)),
    )
    .unwrap();

    let output = bin()
        .arg(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    let symlink_line = stdout
        .lines()
        .find(|line| line.contains("docs-link"))
        .expect("symlink row should exist");

    assert!(!symlink_line.contains("[child:"));
}

#[cfg(unix)]
#[test]
fn tty_child_activity_hint_is_not_shown_for_unreadable_directories() {
    let dir = tempdir().unwrap();
    let locked_dir = dir.path().join("locked");
    fs::create_dir(&locked_dir).unwrap();
    let recent_child = locked_dir.join("recent.log");
    File::create(&recent_child).unwrap();

    let now = SystemTime::now();
    set_file_mtime(
        &recent_child,
        FileTime::from_system_time(now - Duration::from_secs(30)),
    )
    .unwrap();
    set_file_mtime(
        &locked_dir,
        FileTime::from_system_time(now - Duration::from_secs(8 * 24 * 3600)),
    )
    .unwrap();

    fs::set_permissions(&locked_dir, std::fs::Permissions::from_mode(0o000)).unwrap();

    let output = bin()
        .arg(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();

    fs::set_permissions(&locked_dir, std::fs::Permissions::from_mode(0o755)).unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let locked_line = stdout
        .lines()
        .find(|line| line.contains("locked/ | - |"))
        .expect("locked directory row should exist");

    assert!(output.status.success());
    assert!(!locked_line.contains("[child:"));
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

    assert!(stdout.contains("file\t"));
    assert!(stdout.contains("subdir\t"));
    assert!(stdout.contains("link_to_file\t"));
    assert!(!stdout.contains("->"));
}

#[test]
fn ignores_ds_store_and_thumbs_db_even_with_hidden_default() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("visible")).unwrap();
    File::create(dir.path().join(".DS_Store")).unwrap();
    File::create(dir.path().join("Thumbs.db")).unwrap();
    File::create(dir.path().join(".hidden")).unwrap();

    let out_default = bin().current_dir(dir.path()).output().unwrap();
    let stdout = String::from_utf8(out_default.stdout).unwrap();
    assert!(stdout.contains("visible"));
    assert!(stdout.contains(".hidden"));
    assert!(!stdout.contains(".DS_Store"));
    assert!(!stdout.contains("Thumbs.db"));
}

#[test]
fn json_output_contains_expected_fields_including_size_for_files() {
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
    assert_eq!(v.get("size").and_then(Value::as_u64), Some(0));
}

#[test]
fn json_output_omits_size_for_directories() {
    let dir = tempdir().unwrap();
    std::fs::create_dir(dir.path().join("subdir")).unwrap();

    let output = bin()
        .current_dir(dir.path())
        .arg("--json")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut lines = stdout.lines();
    let first = lines.next().expect("one line present");
    let v: Value = serde_json::from_str(first).unwrap();
    assert_eq!(v.get("is_dir").unwrap(), true);
    assert!(v.get("size").is_none());
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

    let ig = tempdir().unwrap();
    let ig_path = ig.path().join("ignore");
    let mut f = File::create(&ig_path).unwrap();
    writeln!(f, "*.tmp").unwrap();
    writeln!(f, "# comment").unwrap();
    writeln!(f).unwrap();

    let out = bin()
        .current_dir(dir.path())
        .env("FTIME_IGNORE", &ig_path)
        .output()
        .unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("keep.log"));
    assert!(!stdout.contains("skip.tmp"));

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

    let out = bin()
        .current_dir(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("Fresh"));

    let out2 = bin()
        .current_dir(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .arg("--no-labels")
        .output()
        .unwrap();
    let stdout2 = String::from_utf8(out2.stdout).unwrap();
    assert!(!stdout2.contains("Fresh"));
}
