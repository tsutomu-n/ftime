mod support;

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[allow(deprecated)]
fn bin() -> Command {
    Command::cargo_bin("ftime").unwrap()
}

#[test]
fn plain_and_json_cannot_be_combined() {
    bin()
        .arg("--plain")
        .arg("--json")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "--plain and --json cannot be combined",
        ));
}

#[test]
fn all_and_hide_dots_cannot_be_combined() {
    bin()
        .arg("-a")
        .arg("--hide-dots")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "-a and --hide-dots cannot be combined",
        ));
}

#[test]
fn since_rejects_invalid_value() {
    bin()
        .arg("--since")
        .arg("not-a-time")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn json_rejects_human_only_flags() {
    for flag in [
        "--absolute",
        "--all-history",
        "--hints",
        "--icons",
        "--color",
    ] {
        let mut cmd = bin();
        cmd.arg("--json").arg(flag);
        if flag == "--color" {
            cmd.arg("always");
        }
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains(
                "--json cannot be combined with human-only flags",
            ));
    }
}

#[test]
fn plain_rejects_human_only_flags_but_keeps_absolute() {
    for flag in ["--all-history", "--hints", "--icons", "--color"] {
        let mut cmd = bin();
        cmd.arg("--plain").arg(flag);
        if flag == "--color" {
            cmd.arg("always");
        }
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains(
                "--plain cannot be combined with human-only flags",
            ));
    }

    let dir = tempdir().unwrap();
    fs::write(dir.path().join("f1"), b"x").unwrap();
    bin()
        .arg(dir.path())
        .arg("--plain")
        .arg("--absolute")
        .assert()
        .success();
}

#[test]
fn update_flags_reject_each_other_and_scan_inputs() {
    let dir = tempdir().unwrap();

    bin()
        .arg("--self-update")
        .arg("--check-update")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "--self-update and --check-update cannot be combined",
        ));

    bin()
        .arg("--self-update")
        .arg(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "--self-update cannot be combined with scan options or PATH",
        ));

    bin()
        .arg("--check-update")
        .arg("--hide-dots")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "--check-update cannot be combined with scan options or PATH",
        ));
}

#[cfg(unix)]
#[test]
fn self_update_warns_for_probable_cargo_bin_installs() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let cargo_bin = home.join(".cargo").join("bin");
    fs::create_dir_all(&cargo_bin).unwrap();

    let script_path = tmp.path().join("install.sh");
    fs::write(&script_path, "#!/usr/bin/env bash\nset -euo pipefail\n").unwrap();

    bin()
        .arg("--self-update")
        .env("HOME", &home)
        .env(
            "FTIME_SELF_UPDATE_URL",
            format!("file://{}", script_path.display()),
        )
        .env("FTIME_SELF_UPDATE_INSTALL_DIR", &cargo_bin)
        .assert()
        .success()
        .stdout(predicate::str::contains("self-update completed"))
        .stderr(predicate::str::contains(
            "warning: --self-update is intended for GitHub Releases installs;",
        ))
        .stderr(predicate::str::contains("cargo install"));
}
