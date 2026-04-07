mod support;

use assert_cmd::Command;
use chrono::{DateTime, Local};
use filetime::{FileTime, set_file_mtime};
use predicates::prelude::*;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, SystemTime};
use tempfile::tempdir;

#[allow(deprecated)]
fn bin() -> Command {
    Command::cargo_bin("ftime").unwrap()
}

fn local_history_date(ts: SystemTime) -> String {
    let dt: DateTime<Local> = ts.into();
    dt.format("%Y-%m-%d").to_string()
}

fn next_patch_version() -> String {
    let mut parts: Vec<u64> = support::package_version()
        .split('.')
        .map(|part| part.parse().unwrap())
        .collect();
    let last = parts.len() - 1;
    parts[last] += 1;
    format!("{}.{}.{}", parts[0], parts[1], parts[2])
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
fn help_mentions_v2_flags() {
    let output = bin().arg("--help").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    for flag in [
        "--plain",
        "--json",
        "--hide-dots",
        "--all-history",
        "--files-only",
        "--no-hints",
        "--color",
    ] {
        assert!(stdout.contains(flag), "help missing {flag}");
    }
}

#[test]
fn default_output_stays_human_even_when_piped() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("f1")).unwrap();

    let output = bin().arg(dir.path()).output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("Active (1)"));
    assert!(stdout.contains("f1"));
    assert!(!stdout.contains('\t'));
}

#[test]
fn plain_output_is_three_column_tsv() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("f1")).unwrap();

    let output = bin().arg(dir.path()).arg("--plain").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let line = stdout.lines().next().unwrap();
    let cols: Vec<&str> = line.split('\t').collect();

    assert_eq!(cols.len(), 3);
    assert_eq!(cols[0], "f1");
    assert_eq!(cols[1], "active");
}

#[test]
fn absolute_time_flag_changes_plain_output_format() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("f1");
    File::create(&file_path).unwrap();
    let fixed = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    set_file_mtime(&file_path, FileTime::from_system_time(fixed)).unwrap();

    let output = bin()
        .arg(dir.path())
        .arg("--plain")
        .arg("--absolute")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let line = stdout.lines().next().unwrap();
    let cols: Vec<&str> = line.split('\t').collect();

    assert_eq!(cols.len(), 3);
    assert!(
        predicate::str::is_match(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} \(UTC[+-]\d{2}:\d{2}\)$")
            .unwrap()
            .eval(cols[2])
    );
}

#[test]
fn default_dot_policy_shows_hidden_files_but_not_hidden_directories() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("visible")).unwrap();
    File::create(dir.path().join(".hidden")).unwrap();
    fs::create_dir(dir.path().join(".hidden_dir")).unwrap();

    let output = bin()
        .arg(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("visible"));
    assert!(stdout.contains(".hidden"));
    assert!(!stdout.contains(".hidden_dir/"));
}

#[test]
fn all_flag_shows_hidden_files_and_directories() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join(".hidden")).unwrap();
    fs::create_dir(dir.path().join(".hidden_dir")).unwrap();

    bin()
        .arg(dir.path())
        .arg("-a")
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains(".hidden"))
        .stdout(predicate::str::contains(".hidden_dir/"));
}

#[test]
fn hide_dots_hides_hidden_files_and_directories() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join(".hidden")).unwrap();
    fs::create_dir(dir.path().join(".hidden_dir")).unwrap();
    File::create(dir.path().join("visible")).unwrap();

    bin()
        .arg(dir.path())
        .arg("--hide-dots")
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("visible"))
        .stdout(predicate::str::contains(".hidden").not())
        .stdout(predicate::str::contains(".hidden_dir/").not());
}

#[test]
fn removed_hidden_flag_is_rejected() {
    bin()
        .arg("--exclude-dots")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--exclude-dots"));
}

#[test]
fn removed_labels_flag_is_rejected() {
    bin()
        .arg("--no-labels")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--no-labels"));
}

#[test]
fn history_bucket_previews_by_default_and_expands_with_all_history() {
    let dir = tempdir().unwrap();
    let old_time = SystemTime::now() - Duration::from_secs(9 * 24 * 3600);
    for i in 0..7 {
        let path = dir.path().join(format!("old-{i}"));
        File::create(&path).unwrap();
        set_file_mtime(&path, FileTime::from_system_time(old_time)).unwrap();
    }

    let output = bin()
        .arg(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("History (5/7)"));
    assert!(!stdout.contains("old-6"));

    let output = bin()
        .arg(dir.path())
        .arg("--all-history")
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("History (7)"));
    assert!(stdout.contains("old-6"));
}

#[test]
fn ext_filter_keeps_directories_and_symlinks() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("keep.rs")).unwrap();
    File::create(dir.path().join("drop.txt")).unwrap();
    fs::create_dir(dir.path().join("docs")).unwrap();

    let link_path = dir.path().join("link_to_keep");
    #[cfg(unix)]
    std::os::unix::fs::symlink(dir.path().join("keep.rs"), &link_path).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(dir.path().join("keep.rs"), &link_path).unwrap();

    let output = bin()
        .arg(dir.path())
        .arg("--plain")
        .arg("--ext")
        .arg("rs")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("keep.rs\t"));
    assert!(stdout.contains("docs\t"));
    assert!(stdout.contains("link_to_keep\t"));
    assert!(!stdout.contains("drop.txt\t"));
}

#[test]
fn files_only_excludes_directories_and_symlinks() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("keep.rs")).unwrap();
    fs::create_dir(dir.path().join("docs")).unwrap();

    let link_path = dir.path().join("link_to_keep");
    #[cfg(unix)]
    std::os::unix::fs::symlink(dir.path().join("keep.rs"), &link_path).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(dir.path().join("keep.rs"), &link_path).unwrap();

    let output = bin()
        .arg(dir.path())
        .arg("--plain")
        .arg("--files-only")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("keep.rs\t"));
    assert!(!stdout.contains("docs\t"));
    assert!(!stdout.contains("link_to_keep\t"));
}

#[test]
fn human_output_shows_compact_skew_without_footer_or_fresh_label() {
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

    assert!(stdout.contains("[skew]"));
    assert!(!stdout.contains("Current Timezone:"));
    assert!(!stdout.contains("Fresh"));
}

#[test]
fn human_output_supports_no_hints() {
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
    assert!(stdout.contains("[child: active]"));

    let output = bin()
        .arg(dir.path())
        .arg("--no-hints")
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains("[child:"));
}

#[test]
fn plain_and_json_do_not_include_child_hints() {
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
        FileTime::from_system_time(now - Duration::from_secs(8 * 24 * 3600)),
    )
    .unwrap();

    let plain_output = bin().arg(dir.path()).arg("--plain").output().unwrap();
    let plain_stdout = String::from_utf8(plain_output.stdout).unwrap();
    assert!(!plain_stdout.contains("[child:"));

    let json_output = bin().arg(dir.path()).arg("--json").output().unwrap();
    let json_stdout = String::from_utf8(json_output.stdout).unwrap();
    let first = json_stdout.lines().next().expect("one line present");
    let value: Value = serde_json::from_str(first).unwrap();
    assert!(value.get("child_activity").is_none());
}

#[test]
fn plain_and_json_conflicts_are_rejected() {
    bin().arg("--plain").arg("--json").assert().failure();
}

#[test]
fn all_and_hide_dots_conflict_is_rejected() {
    bin().arg("-a").arg("--hide-dots").assert().failure();
}

#[test]
fn json_rejects_human_only_flags() {
    for flag in [
        "--absolute",
        "--all-history",
        "--no-hints",
        "--icons",
        "--color",
    ] {
        let mut cmd = bin();
        cmd.arg("--json").arg(flag);
        if flag == "--color" {
            cmd.arg("always");
        }
        cmd.assert().failure();
    }
}

#[test]
fn plain_rejects_human_only_flags_except_absolute() {
    for flag in ["--all-history", "--no-hints", "--icons", "--color"] {
        let mut cmd = bin();
        cmd.arg("--plain").arg(flag);
        if flag == "--color" {
            cmd.arg("always");
        }
        cmd.assert().failure();
    }
}

#[test]
fn plain_output_formats_dirs_and_symlinks_as_undecorated_paths() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("file");
    File::create(&file_path).unwrap();
    let subdir = dir.path().join("subdir");
    fs::create_dir(&subdir).unwrap();
    let link_path = dir.path().join("link_to_file");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&file_path, &link_path).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&file_path, &link_path).unwrap();

    let output = bin()
        .current_dir(dir.path())
        .arg("--plain")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("file\t"));
    assert!(stdout.contains("subdir\t"));
    assert!(stdout.contains("link_to_file\t"));
    assert!(!stdout.contains("->"));
    assert!(!stdout.contains("subdir/"));
}

#[test]
fn ignores_ds_store_and_thumbs_db_even_with_hidden_default() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("visible")).unwrap();
    File::create(dir.path().join(".DS_Store")).unwrap();
    File::create(dir.path().join("Thumbs.db")).unwrap();
    File::create(dir.path().join(".hidden")).unwrap();

    let out_default = bin()
        .current_dir(dir.path())
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(out_default.stdout).unwrap();
    assert!(stdout.contains("visible"));
    assert!(stdout.contains(".hidden"));
    assert!(!stdout.contains(".DS_Store"));
    assert!(!stdout.contains("Thumbs.db"));
}

#[test]
fn json_output_contains_expected_fields_without_label() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("f1");
    File::create(&file_path).unwrap();

    let output = bin()
        .current_dir(dir.path())
        .arg("--json")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let first = stdout.lines().next().expect("one line present");
    assert!(first.starts_with("{\"path\":\"f1\",\"bucket\":\"active\",\"mtime\":"));
    assert!(!first.contains("\"label\""));

    let v: Value = serde_json::from_str(first).unwrap();
    assert_eq!(v.get("path").unwrap(), "f1");
    assert_eq!(v.get("bucket").unwrap(), "active");
    assert_eq!(v.get("is_dir").unwrap(), false);
    assert_eq!(v.get("size").and_then(Value::as_u64), Some(0));
}

#[test]
fn json_output_omits_size_for_directories() {
    let dir = tempdir().unwrap();
    fs::create_dir(dir.path().join("subdir")).unwrap();

    let output = bin()
        .current_dir(dir.path())
        .arg("--json")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let first = stdout.lines().next().expect("one line present");
    let v: Value = serde_json::from_str(first).unwrap();
    assert_eq!(v.get("is_dir").unwrap(), true);
    assert!(v.get("size").is_none());
}

#[test]
fn no_matching_entries_message_mentions_filters() {
    let dir = tempdir().unwrap();
    fs::create_dir(dir.path().join(".hidden_dir")).unwrap();

    let output = bin()
        .arg(dir.path())
        .arg("--hide-dots")
        .env("FTIME_FORCE_TTY", "1")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("No matching entries"));
    assert!(stdout.contains("filters:"));
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
        .arg("--plain")
        .output()
        .unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("keep.log"));
    assert!(!stdout.contains("skip.tmp"));

    let out_no = bin()
        .current_dir(dir.path())
        .env("FTIME_IGNORE", &ig_path)
        .arg("--plain")
        .arg("--no-ignore")
        .output()
        .unwrap();
    let stdout_no = String::from_utf8(out_no.stdout).unwrap();
    assert!(stdout_no.contains("skip.tmp"));
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
    let latest = next_patch_version();

    bin()
        .arg("--check-update")
        .env("FTIME_SELF_UPDATE_LATEST_VERSION", &latest)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "update available: {} -> {}",
            support::package_version(),
            latest
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
        .env("FTIME_SELF_UPDATE_LATEST_VERSION", "1.9.9")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "latest published release is 1.9.9 (current binary reports {})",
            support::package_version()
        )));
}

#[test]
fn human_output_globally_aligns_columns_and_moves_symlink_targets_to_suffix() {
    let dir = tempdir().unwrap();
    let now = SystemTime::now();

    let hidden = dir.path().join(".hidden");
    fs::write(&hidden, b"hidden-bytes!").unwrap();
    set_file_mtime(
        &hidden,
        FileTime::from_system_time(now - Duration::from_secs(2 * 3600)),
    )
    .unwrap();

    let readme = dir.path().join("README.md");
    fs::write(&readme, vec![b'x'; 1200]).unwrap();
    set_file_mtime(
        &readme,
        FileTime::from_system_time(now - Duration::from_secs(2 * 3600)),
    )
    .unwrap();

    let docs = dir.path().join("docs");
    fs::create_dir(&docs).unwrap();
    let docs_child = docs.join("guide.md");
    fs::write(&docs_child, b"guide").unwrap();
    set_file_mtime(
        &docs_child,
        FileTime::from_system_time(now - Duration::from_secs(2 * 3600)),
    )
    .unwrap();
    let docs_mtime = now - Duration::from_secs(9 * 24 * 3600);
    set_file_mtime(&docs, FileTime::from_system_time(docs_mtime)).unwrap();

    let license = dir.path().join("LICENSE");
    fs::write(&license, vec![b'l'; 2048]).unwrap();
    let license_mtime = now - Duration::from_secs(10 * 24 * 3600);
    set_file_mtime(&license, FileTime::from_system_time(license_mtime)).unwrap();

    let output = bin()
        .arg(dir.path())
        .arg("--color")
        .arg("never")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        concat!(
            "Today (2)\n",
            "  {:<9}  {:>7}  {:>10}\n",
            "  {:<9}  {:>7}  {:>10}\n",
            "\n",
            "History (2)\n",
            "  {:<9}  {:>7}  {:>10}{}\n",
            "  {:<9}  {:>7}  {:>10}\n",
            "\n"
        ),
        ".hidden",
        "13 B",
        "2h",
        "README.md",
        "1.2 KiB",
        "2h",
        "docs/",
        "<dir>",
        local_history_date(docs_mtime),
        " [child: today]",
        "LICENSE",
        "2.0 KiB",
        local_history_date(license_mtime),
    );

    assert_eq!(stdout, expected);
}

#[test]
fn human_output_aligns_columns_using_unicode_display_width() {
    let dir = tempdir().unwrap();
    let now = SystemTime::now();

    let wide = dir.path().join("日本語.txt");
    fs::write(&wide, b"w").unwrap();
    set_file_mtime(
        &wide,
        FileTime::from_system_time(now - Duration::from_secs(2 * 3600)),
    )
    .unwrap();

    let ascii = dir.path().join("a.txt");
    fs::write(&ascii, b"a").unwrap();
    set_file_mtime(
        &ascii,
        FileTime::from_system_time(now - Duration::from_secs(3 * 3600)),
    )
    .unwrap();

    let output = bin()
        .arg(dir.path())
        .arg("--color")
        .arg("never")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        concat!(
            "Today (2)\n",
            "  日本語.txt  1 B  2h\n",
            "  a.txt{}  1 B  3h\n",
            "\n"
        ),
        " ".repeat(5),
    );

    assert_eq!(stdout, expected);
}

#[test]
fn human_output_truncates_long_unicode_file_names_but_plain_and_json_keep_them() {
    let dir = tempdir().unwrap();
    let now = SystemTime::now();
    let long_name = "あいうえおかきくけこさしすせそ.pdf";
    let truncated = "あいうえおかきくけこさ~.pdf";
    let path = dir.path().join(long_name);

    fs::write(&path, b"x").unwrap();
    set_file_mtime(
        &path,
        FileTime::from_system_time(now - Duration::from_secs(2 * 3600)),
    )
    .unwrap();

    let human = bin()
        .arg(dir.path())
        .arg("--color")
        .arg("never")
        .output()
        .unwrap();
    let human_stdout = String::from_utf8(human.stdout).unwrap();

    assert!(human_stdout.contains(truncated), "{human_stdout}");
    assert!(!human_stdout.contains(long_name), "{human_stdout}");

    let plain = bin().arg(dir.path()).arg("--plain").output().unwrap();
    let plain_stdout = String::from_utf8(plain.stdout).unwrap();
    assert!(plain_stdout.contains(long_name), "{plain_stdout}");

    let json = bin().arg(dir.path()).arg("--json").output().unwrap();
    let json_stdout = String::from_utf8(json.stdout).unwrap();
    assert!(json_stdout.contains(long_name), "{json_stdout}");
}

#[test]
fn human_output_truncates_long_unicode_directory_names_and_keeps_the_slash() {
    let dir = tempdir().unwrap();
    let now = SystemTime::now();
    let long_name = "あいうえおかきくけこさしすせそ";
    let truncated = "あいうえおかきくけこさしす~/";
    let path = dir.path().join(long_name);

    fs::create_dir(&path).unwrap();
    set_file_mtime(
        &path,
        FileTime::from_system_time(now - Duration::from_secs(2 * 3600)),
    )
    .unwrap();

    let output = bin()
        .arg(dir.path())
        .arg("--no-hints")
        .arg("--color")
        .arg("never")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains(truncated), "{stdout}");
    assert!(!stdout.contains(&format!("{long_name}/")), "{stdout}");
}

#[test]
fn human_output_places_symlink_target_after_the_time_column() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("README.md");
    fs::write(&target, b"target").unwrap();

    let link = dir.path().join("link_to_readme");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&target, &link).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&target, &link).unwrap();

    let link_mtime = fs::symlink_metadata(&link).unwrap().modified().unwrap();
    let dt: DateTime<Local> = link_mtime.into();
    let absolute = format!(
        "{} ({})",
        dt.format("%Y-%m-%d %H:%M:%S"),
        dt.format("UTC%:z")
    );

    let output = bin()
        .arg(dir.path())
        .arg("--absolute")
        .arg("--color")
        .arg("never")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    let expected_line = format!(
        "  {:<14}  {:>5}  {} -> README.md",
        "link_to_readme", "<lnk>", absolute,
    );

    assert!(stdout.contains(&expected_line), "{stdout}");
    assert!(
        !stdout.contains("link_to_readme -> README.md  —"),
        "{stdout}"
    );
}

#[test]
fn color_always_colors_only_the_symlink_name_not_placeholder_or_target() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("README.md");
    fs::write(&target, b"target").unwrap();

    let link = dir.path().join("link_to_readme");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&target, &link).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&target, &link).unwrap();

    let output = bin()
        .arg(dir.path())
        .arg("--absolute")
        .arg("--color")
        .arg("always")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let line = stdout
        .lines()
        .find(|line| line.contains("link_to_readme"))
        .expect("symlink row present");

    assert!(line.contains("\u{1b}["), "{line}");
    assert!(line.contains("link_to_readme"), "{line}");
        assert!(line.contains("\u{1b}[0m") && line.matches("<lnk>").count() == 1 && !line.contains("<lnk>\u{1b}["), "{line}");
    assert!(line.contains(" -> README.md"), "{line}");
    assert!(!line.contains("<lnk>\u{1b}["), "{line}");
    assert!(!line.contains("README.md\u{1b}["), "{line}");
}

#[test]
fn color_always_uses_semantic_bucket_colors() {
    let dir = tempdir().unwrap();
    let now = SystemTime::now();

    let active = dir.path().join("active.txt");
    fs::write(&active, b"a").unwrap();
    set_file_mtime(
        &active,
        FileTime::from_system_time(now - Duration::from_secs(5 * 60)),
    )
    .unwrap();

    let weekly = dir.path().join("weekly.txt");
    fs::write(&weekly, b"w").unwrap();
    set_file_mtime(
        &weekly,
        FileTime::from_system_time(now - Duration::from_secs(2 * 24 * 3600)),
    )
    .unwrap();

    let history = dir.path().join("history.txt");
    fs::write(&history, b"h").unwrap();
    set_file_mtime(
        &history,
        FileTime::from_system_time(now - Duration::from_secs(10 * 24 * 3600)),
    )
    .unwrap();

    let output = bin()
        .arg(dir.path())
        .arg("--color")
        .arg("always")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("\u{1b}[1;32mActive (1)\u{1b}[0m"));
    assert!(stdout.contains("\u{1b}[1;32m"));
    assert!(stdout.contains("5m\u{1b}[0m"));
    assert!(stdout.contains("\u{1b}[1;36mThis Week (1)\u{1b}[0m"));
    assert!(stdout.contains("\u{1b}[36m"));
    assert!(stdout.contains("2d\u{1b}[0m"));
    assert!(stdout.contains("History (1)"));
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
