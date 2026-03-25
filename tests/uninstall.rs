use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn run_uninstall(home: &Path, install_dir: &Path) -> std::process::Output {
    Command::new("bash")
        .arg("scripts/uninstall.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("HOME", home)
        .env("INSTALL_DIR", install_dir)
        .output()
        .unwrap()
}

#[test]
fn uninstall_removes_binary_from_install_dir() {
    let home = tempdir().unwrap();
    let install_dir = tempdir().unwrap();
    let bin_path = install_dir.path().join("ftime");
    File::create(&bin_path).unwrap();

    let output = run_uninstall(home.path(), install_dir.path());

    assert!(output.status.success());
    assert!(!bin_path.exists());
}

#[test]
fn uninstall_falls_back_to_home_local_bin() {
    let home = tempdir().unwrap();
    let install_dir = tempdir().unwrap();
    let local_bin = home.path().join(".local/bin");
    fs::create_dir_all(&local_bin).unwrap();
    let bin_path = local_bin.join("ftime");
    File::create(&bin_path).unwrap();

    let output = run_uninstall(home.path(), install_dir.path());

    assert!(output.status.success());
    assert!(!bin_path.exists());
}

#[test]
fn uninstall_succeeds_when_binary_is_missing() {
    let home = tempdir().unwrap();
    let install_dir = tempdir().unwrap();

    let output = run_uninstall(home.path(), install_dir.path());
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(stdout.contains("not installed"));
}
