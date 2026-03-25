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

fn assert_contains_all(content: &str, path: &str, snippets: &[&str]) {
    for snippet in snippets {
        assert!(
            content.contains(snippet),
            "missing required snippet in {path}: {snippet}"
        );
    }
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
fn uninstall_docs_cover_release_and_cargo_paths() {
    for path in ["README.md", "docs/README-ja.md", "docs/README-zh.md"] {
        let content = read_repo_file(path);

        assert!(
            !content.contains(
                "INSTALL_DIR=/custom/bin curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/uninstall.sh | bash"
            ),
            "broken custom uninstall example remains in {path}"
        );
        assert_contains_all(
            &content,
            path,
            &[
                "curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/uninstall.sh | bash",
                "curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/uninstall.sh | env INSTALL_DIR=/custom/bin bash",
                "https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/uninstall.ps1",
                "-InstallDir 'C:\\custom\\bin'",
                "cargo uninstall ftime",
            ],
        );
    }
}

#[test]
fn japanese_uninstall_docs_explain_unix_and_windows_custom_args() {
    let content = read_repo_file("docs/README-ja.md");

    assert!(
        !content.contains("同じ場所を `INSTALL_DIR` で渡します。"),
        "outdated Japanese wording remains"
    );
    assert_contains_all(
        &content,
        "docs/README-ja.md",
        &["`INSTALL_DIR`", "`-InstallDir`"],
    );
}

#[test]
fn chinese_readme_has_uninstall_section() {
    let content = read_repo_file("docs/README-zh.md");

    assert_contains_all(
        &content,
        "docs/README-zh.md",
        &[
            "## 卸载",
            "### GitHub Releases 安装",
            "### `cargo install` / `cargo install --path .` 安装",
        ],
    );
}
