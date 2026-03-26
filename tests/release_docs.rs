use std::fs;
use std::path::Path;

fn read_repo_file(path: &str) -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap()
}

fn current_release_tag() -> String {
    "v1.0.0".to_string()
}

fn assert_contains_all(content: &str, path: &str, snippets: &[&str]) {
    for snippet in snippets {
        assert!(
            content.contains(snippet),
            "missing required snippet in {path}: {snippet}"
        );
    }
}

fn assert_contains_none(content: &str, path: &str, snippets: &[&str]) {
    for snippet in snippets {
        assert!(
            !content.contains(snippet),
            "unexpected snippet in {path}: {snippet}"
        );
    }
}

#[test]
fn current_release_docs_are_v1_0_0() {
    let cargo_toml = read_repo_file("Cargo.toml");
    assert!(cargo_toml.contains("version = \"1.0.0\""));

    for path in [
        "docs/CLI.md",
        "docs/SPEC-v1.0.md",
        "docs/TESTPLAN-v1.0.md",
        "docs/RELEASE-NOTES-v1.0.md",
    ] {
        let content = read_repo_file(path);
        assert!(content.contains("v1.0.0"), "{path} must describe v1.0.0");
        assert!(
            !content.contains("v2.0.0"),
            "{path} should not describe v2.0.0 as current"
        );
    }
}

#[test]
fn v2_docs_are_archived_after_renumbering() {
    for path in [
        "docs/SPEC-v2.0.md",
        "docs/TESTPLAN-v2.0.md",
        "docs/RELEASE-NOTES-v2.0.md",
    ] {
        let content = read_repo_file(path);
        assert_contains_all(
            &content,
            path,
            &[
                "Archived",
                "Current canonical references:",
                "docs/SPEC-v1.0.md",
                "docs/TESTPLAN-v1.0.md",
                "docs/RELEASE-NOTES-v1.0.md",
            ],
        );
    }
}

#[test]
fn readme_surfaces_link_only_to_current_primary_docs() {
    let tag = current_release_tag();
    let root = read_repo_file("README.md");
    assert_contains_all(
        &root,
        "README.md",
        &[
            "## Install",
            "Windows installer currently targets x86_64 / AMD64.",
            "## Quick Usage",
            "Uninstall steps are documented in `## Uninstall`, including custom install directories.",
            "## Learn More",
            "[CLI contract](docs/CLI.md)",
            "[日本語](docs/README-ja.md)",
            "[中文](docs/README-zh.md)",
        ],
    );
    assert_contains_all(
        &root,
        "README.md",
        &[
            &format!("https://raw.githubusercontent.com/tsutomu-n/ftime/{tag}/scripts/install.sh"),
            &format!("https://raw.githubusercontent.com/tsutomu-n/ftime/{tag}/scripts/install.ps1"),
            &format!(
                "https://raw.githubusercontent.com/tsutomu-n/ftime/{tag}/scripts/uninstall.sh"
            ),
            &format!(
                "https://raw.githubusercontent.com/tsutomu-n/ftime/{tag}/scripts/uninstall.ps1"
            ),
        ],
    );

    let ja = read_repo_file("docs/README-ja.md");
    assert_contains_all(
        &ja,
        "docs/README-ja.md",
        &[
            "## インストール",
            "Windows installer は現状 x86_64 / AMD64 を対象にしています。",
            "## クイックスタート",
            "アンインストール手順は下の `## アンインストール` にまとめています。",
            "## 詳細ドキュメント",
            "[使い方ガイド](USER-GUIDE-ja.md)",
            "[CLI リファレンス](CLI-ja.md)",
            "[読み分け案内](ftime-overview-ja.md)",
        ],
    );
    assert_contains_all(
        &ja,
        "docs/README-ja.md",
        &[
            &format!("https://raw.githubusercontent.com/tsutomu-n/ftime/{tag}/scripts/install.sh"),
            &format!("https://raw.githubusercontent.com/tsutomu-n/ftime/{tag}/scripts/install.ps1"),
            &format!(
                "https://raw.githubusercontent.com/tsutomu-n/ftime/{tag}/scripts/uninstall.sh"
            ),
            &format!(
                "https://raw.githubusercontent.com/tsutomu-n/ftime/{tag}/scripts/uninstall.ps1"
            ),
        ],
    );

    let zh = read_repo_file("docs/README-zh.md");
    assert_contains_all(
        &zh,
        "docs/README-zh.md",
        &[
            "## 安装",
            "Windows installer 目前仅覆盖 x86_64 / AMD64。",
            "## 快速开始",
            "卸载步骤写在下方的 `## 卸载`，也包含自定义安装目录的情况。",
            "## 详细文档",
            "[CLI contract](CLI.md)",
        ],
    );
    assert_contains_all(
        &zh,
        "docs/README-zh.md",
        &[
            &format!("https://raw.githubusercontent.com/tsutomu-n/ftime/{tag}/scripts/install.sh"),
            &format!("https://raw.githubusercontent.com/tsutomu-n/ftime/{tag}/scripts/install.ps1"),
            &format!(
                "https://raw.githubusercontent.com/tsutomu-n/ftime/{tag}/scripts/uninstall.sh"
            ),
            &format!(
                "https://raw.githubusercontent.com/tsutomu-n/ftime/{tag}/scripts/uninstall.ps1"
            ),
        ],
    );

    for (path, content) in [
        ("README.md", &root),
        ("docs/README-ja.md", &ja),
        ("docs/README-zh.md", &zh),
    ] {
        assert_contains_none(
            content,
            path,
            &[
                "SPEC-v2.0.md",
                "TESTPLAN-v2.0.md",
                "RELEASE-NOTES-v2.0.md",
                "12-10_ROADMAP.md",
                "https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/install.sh",
                "https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/install.ps1",
                "https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/uninstall.sh",
                "https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/uninstall.ps1",
                "crates.io",
                "cargo install ftime",
            ],
        );
    }
}

#[test]
fn japanese_docs_have_separated_roles() {
    let guide = read_repo_file("docs/USER-GUIDE-ja.md");
    assert_contains_all(
        &guide,
        "docs/USER-GUIDE-ja.md",
        &[
            "## 典型的な使い方",
            "## 出力の読み方",
            "## フィルタの使い分け",
            "README-ja.md",
            "CLI-ja.md",
        ],
    );
    assert_contains_none(
        &guide,
        "docs/USER-GUIDE-ja.md",
        &["## インストール", "## 環境変数", "## 終了コード"],
    );

    let cli = read_repo_file("docs/CLI-ja.md");
    assert_contains_all(
        &cli,
        "docs/CLI-ja.md",
        &[
            "## コマンド署名",
            "## オプション一覧",
            "## 環境変数",
            "## 終了コード",
            "## 出力契約",
        ],
    );
    assert_contains_none(
        &cli,
        "docs/CLI-ja.md",
        &[
            "## シナリオ別ガイド",
            "## コマンド利用チェックリスト",
            "## コミュニケーションテンプレ",
            "## 将来のCLI拡張アイデア",
        ],
    );

    let overview = read_repo_file("docs/ftime-overview-ja.md");
    assert_contains_all(
        &overview,
        "docs/ftime-overview-ja.md",
        &[
            "README-ja.md",
            "USER-GUIDE-ja.md",
            "CLI-ja.md",
            "どのドキュメントを読むべきか",
        ],
    );
    assert_contains_none(
        &overview,
        "docs/ftime-overview-ja.md",
        &["Appendix", "ロードマップ", "実装準拠"],
    );

    let overview_line_count = overview.lines().count();
    assert!(
        overview_line_count <= 80,
        "docs/ftime-overview-ja.md should stay short, found {overview_line_count} lines"
    );
}
