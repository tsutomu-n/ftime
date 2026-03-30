mod support;

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
fn current_release_docs_match_current_package_version() {
    let cargo_toml = support::read_repo_file("Cargo.toml");
    assert!(cargo_toml.contains(&format!("version = \"{}\"", support::package_version())));

    for path in [
        "docs/CLI.md",
        "docs/SPEC-v1.0.md",
        "docs/TESTPLAN-v1.0.md",
        "docs/RELEASE-NOTES-v1.0.md",
    ] {
        let content = support::read_repo_file(path);
        assert!(
            content.contains(&format!("v{}", support::package_version())),
            "{path} must describe v{}",
            support::package_version()
        );
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
        let content = support::read_repo_file(path);
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
    let root = support::read_repo_file("README.md");
    assert_contains_all(
        &root,
        "README.md",
        &[
            "# `ftime` = files by time",
            "read-only File Time CLI for browsing files by recency",
            "The name stands for `files by time`",
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
            "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.sh",
            "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.ps1",
            "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh",
            "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1",
        ],
    );

    let ja = support::read_repo_file("docs/README-ja.md");
    assert_contains_all(
        &ja,
        "docs/README-ja.md",
        &[
            "files by time",
            "更新の新しさでファイルを見るための、読み取り専用の File Time CLI",
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
            "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.sh",
            "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.ps1",
            "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh",
            "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1",
        ],
    );

    let zh = support::read_repo_file("docs/README-zh.md");
    assert_contains_all(
        &zh,
        "docs/README-zh.md",
        &[
            "files by time",
            "按新旧顺序浏览文件的只读 File Time CLI",
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
            "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.sh",
            "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.ps1",
            "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh",
            "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1",
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
                "https://raw.githubusercontent.com/tsutomu-n/ftime/v1.0.0/scripts/install.sh",
                "https://raw.githubusercontent.com/tsutomu-n/ftime/v1.0.0/scripts/install.ps1",
                "https://raw.githubusercontent.com/tsutomu-n/ftime/v1.0.0/scripts/uninstall.sh",
                "https://raw.githubusercontent.com/tsutomu-n/ftime/v1.0.0/scripts/uninstall.ps1",
                "crates.io",
                "cargo install ftime",
            ],
        );
    }
}

#[test]
fn release_workflow_publishes_latest_bootstrap_assets() {
    let workflow = support::read_repo_file(".github/workflows/release.yml");

    assert_contains_all(
        &workflow,
        ".github/workflows/release.yml",
        &[
            "ftime-install.sh",
            "ftime-install.ps1",
            "ftime-uninstall.sh",
            "ftime-uninstall.ps1",
            "ftime-${{ matrix.target }}.tar.gz",
            "ftime-${{ matrix.target }}.zip",
        ],
    );
}

#[test]
fn japanese_docs_have_separated_roles() {
    let guide = support::read_repo_file("docs/USER-GUIDE-ja.md");
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

    let cli = support::read_repo_file("docs/CLI-ja.md");
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

    let overview = support::read_repo_file("docs/ftime-overview-ja.md");
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
