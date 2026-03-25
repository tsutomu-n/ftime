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

fn read_repo_file(path: &str) -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap()
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

#[test]
fn uninstall_docs_use_bash_side_install_dir_for_custom_unix_paths() {
    for path in ["README.md", "docs/README-ja.md"] {
        let content = read_repo_file(path);

        assert!(
            !content.contains(
                "INSTALL_DIR=/custom/bin curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/uninstall.sh | bash"
            ),
            "broken custom uninstall example remains in {path}"
        );
        assert!(
            content.contains(
                "curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/uninstall.sh | env INSTALL_DIR=/custom/bin bash"
            ),
            "missing fixed custom uninstall example in {path}"
        );
    }
}

#[test]
fn uninstall_docs_show_custom_windows_install_dir_example() {
    for path in ["README.md", "docs/README-ja.md"] {
        let content = read_repo_file(path);

        assert!(
            content.contains(
                "powershell -ExecutionPolicy Bypass -Command \"& ([scriptblock]::Create((iwr https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/uninstall.ps1 -UseBasicParsing).Content)) -InstallDir 'C:\\custom\\bin'\""
            ),
            "missing custom Windows uninstall example in {path}"
        );
    }
}
