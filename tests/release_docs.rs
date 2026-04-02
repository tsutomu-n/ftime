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

fn assert_in_order(content: &str, path: &str, snippets: &[&str]) {
    let mut cursor = 0usize;
    for snippet in snippets {
        let next = content[cursor..]
            .find(snippet)
            .unwrap_or_else(|| panic!("missing ordered snippet in {path}: {snippet}"));
        cursor += next + snippet.len();
    }
}

#[test]
fn current_release_docs_match_current_package_version() {
    let cargo_toml = support::read_repo_file("Cargo.toml");
    assert!(cargo_toml.contains(&format!("version = \"{}\"", support::package_version())));
    assert_contains_all(
        &cargo_toml,
        "Cargo.toml",
        &[
            "description = \"Read-only CLI for browsing recently changed files in a folder\"",
            "keywords = [\"cli\", \"filesystem\", \"mtime\", \"jsonl\", \"productivity\"]",
            "categories = [\"command-line-utilities\"]",
        ],
    );

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
fn japanese_extended_docs_track_current_bucket_names_and_version() {
    let spec_ja = support::read_repo_file("docs/SPEC-ja.md");
    assert_contains_all(
        &spec_ja,
        "docs/SPEC-ja.md",
        &[
            &format!("# ftime v{} 仕様書", support::package_version()),
            "Active | `now - mtime < 1時間`",
            "Today | `mtime >= 今日 00:00:00`",
            "例 `🔥 Active`。",
            "行形式: `name | size | time`。",
            "全件をタブ区切り1行1エントリで出力: `<path>\\t<time>`。",
        ],
    );
    assert_contains_none(
        &spec_ja,
        "docs/SPEC-ja.md",
        &["Active Context", "Today's Session", "v1.0.0 仕様書"],
    );

    let testplan_ja = support::read_repo_file("docs/TESTPLAN-ja.md");
    assert_contains_all(
        &testplan_ja,
        "docs/TESTPLAN-ja.md",
        &[
            &format!("# ftime v{} テスト計画", support::package_version()),
            "```",
            "🔥 Active",
            "  • visible | 0 B | just now",
            "💤 History (25 files hidden)",
        ],
    );
    assert_contains_none(
        &testplan_ja,
        "docs/TESTPLAN-ja.md",
        &["Active Context (< 1h)", "v1.0.0 テスト計画"],
    );
}

#[test]
fn cli_contract_stays_english_only_for_canonical_doc() {
    let cli = support::read_repo_file("docs/CLI.md");
    assert_contains_all(
        &cli,
        "docs/CLI.md",
        &[
            "Directories are excluded.",
            "Per-entry I/O errors are skipped and scanning continues.",
        ],
    );
    assert_contains_none(
        &cli,
        "docs/CLI.md",
        &[
            "Directoriesは除外される。",
            "Per-entry I/O エラーはスキップし処理継続する。",
        ],
    );
}

#[test]
fn release_notes_separate_patch_changes_from_current_contract_snapshot() {
    let notes = support::read_repo_file("docs/RELEASE-NOTES-v1.0.md");
    assert_contains_all(
        &notes,
        "docs/RELEASE-NOTES-v1.0.md",
        &[
            &format!("# ftime v{} Release Notes", support::package_version()),
            "## Changes in v",
            "## Current v1 Contract Snapshot",
            "Self-update now resolves the latest tag before downloading the installer asset.",
            "Unix and PowerShell installers now resolve the latest tag before downloading versioned platform assets.",
        ],
    );
    assert_contains_none(&notes, "docs/RELEASE-NOTES-v1.0.md", &["## New Behavior"]);
}

#[test]
fn current_release_notes_capture_windows_installer_follow_up() {
    let notes = support::read_repo_file("docs/RELEASE-NOTES-v1.0.md");
    assert_contains_all(
        &notes,
        "docs/RELEASE-NOTES-v1.0.md",
        &[
            "Windows PowerShell installer no longer defaults to `.cargo\\bin`.",
            "GitHub Releases installer now documents that Rust is not required.",
            "%LOCALAPPDATA%\\Programs\\ftime\\bin",
        ],
    );
}

#[test]
fn installers_resolve_latest_tag_before_downloading_platform_binaries() {
    let install_sh = support::read_repo_file("scripts/install.sh");
    assert_contains_all(
        &install_sh,
        "scripts/install.sh",
        &[
            "https://api.github.com/repos/${REPO}/releases/latest",
            "asset=\"${BIN}-${version#v}-${target}.tar.gz\"",
            "url=\"https://github.com/${REPO}/releases/download/${version}/${asset}\"",
        ],
    );
    assert_contains_none(
        &install_sh,
        "scripts/install.sh",
        &["url=\"https://github.com/${REPO}/releases/latest/download/${asset}\""],
    );

    let install_ps1 = support::read_repo_file("scripts/install.ps1");
    assert_contains_all(
        &install_ps1,
        "scripts/install.ps1",
        &[
            "https://api.github.com/repos/$Repo/releases/latest",
            "Url = \"https://github.com/$Repo/releases/download/$Tag/$Bin-$VersionNumber-$Target.zip\"",
        ],
    );
    assert_contains_none(
        &install_ps1,
        "scripts/install.ps1",
        &["https://github.com/$Repo/releases/latest/download/$Bin-$Target.zip"],
    );
}

#[test]
fn social_preview_asset_exists_as_png() {
    let bytes = support::read_repo_bytes("assets/social-preview.png");
    assert!(
        bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]),
        "assets/social-preview.png must be a PNG asset"
    );
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
            "What changed in this folder recently?",
            "The name stands for `files by time`.",
            "It scans only the first level of a directory",
            "Depth-1 only: see the current folder, not the whole tree",
            "Human-readable sizes in TTY output; plain text and JSON Lines available for scripts",
            "## Why `ftime`?",
            "## Common examples",
            "## Tool fit",
            "clean up `~/Downloads`",
            "`--json` emits one JSON object per line",
            "## Example output",
            "• Cargo.toml | 2.1 KiB | 12s ago",
            "• README.md | 8.4 KiB | 2h ago",
            "• docs/ | - | 3d ago",
            "• target/ | - | 2w ago",
            "Directories show `-` in the size column.",
            "## Install",
            "Rust is not required for the GitHub Releases installer.",
            "### crates.io",
            "cargo install ftime --locked",
            "Default Windows install dir: `%LOCALAPPDATA%\\Programs\\ftime\\bin`.",
            "Windows installer currently targets x86_64 / AMD64.",
            "### From source (for unreleased main)",
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
            "このフォルダで最近何が変わった？",
            "指定ディレクトリ直下だけを走査し",
            "深さ1固定: 今見ているフォルダだけを対象",
            "TTY では人間可読サイズを表示し、非 TTY ではプレーンテキスト、`--json` では JSON Lines を使えます",
            "## どんな時に使うか",
            "## 例",
            "## 出力例",
            "## 他のツールとの違い",
            "`--json` は 1 行 1 JSON で出る",
            "## インストール",
            "GitHub Releases installer には Rust は不要です。",
            "### crates.io",
            "cargo install ftime --locked",
            "Windows の既定 install 先は `%LOCALAPPDATA%\\Programs\\ftime\\bin` です。",
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
            "这个文件夹最近有什么变化？",
            "它只扫描目录的第一层",
            "固定深度 1：只看当前文件夹，不递归整个目录树",
            "TTY 中显示人类可读的大小，非 TTY 可用纯文本，`--json` 可输出 JSON Lines",
            "## 适合什么场景",
            "## 示例",
            "## 输出示例",
            "## 和其他工具的区别",
            "`--json` 会按每行一个 JSON 对象输出",
            "## 安装",
            "GitHub Releases installer 不需要 Rust。",
            "### crates.io",
            "cargo install ftime --locked",
            "Windows 默认安装目录是 `%LOCALAPPDATA%\\Programs\\ftime\\bin`。",
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
                "## Quick Usage",
            ],
        );
    }
}

#[test]
fn readme_flags_keep_update_commands_after_scan_focused_flags() {
    let root = support::read_repo_file("README.md");
    assert_in_order(
        &root,
        "README.md",
        &[
            "- `-a, --all`",
            "- `-A, --absolute`",
            "- `--exclude-dots`",
            "- `--json`",
            "- `--check-update`",
            "- `--self-update`",
        ],
    );

    let ja = support::read_repo_file("docs/README-ja.md");
    assert_in_order(
        &ja,
        "docs/README-ja.md",
        &[
            "- `-a, --all`",
            "- `-A, --absolute`",
            "- `--exclude-dots`",
            "- `--json`",
            "- `--check-update`",
            "- `--self-update`",
        ],
    );

    let zh = support::read_repo_file("docs/README-zh.md");
    assert_in_order(
        &zh,
        "docs/README-zh.md",
        &[
            "- `-a, --all`",
            "- `-A, --absolute`",
            "- `--exclude-dots`",
            "- `--json`",
            "- `--check-update`",
            "- `--self-update`",
        ],
    );
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
fn crate_package_excludes_local_only_assets_and_maintainer_docs() {
    let cargo_toml = support::read_repo_file("Cargo.toml");
    assert_contains_all(
        &cargo_toml,
        "Cargo.toml",
        &[
            "\"assets/demo_ftime.gif\"",
            "\"assets/demo_ftime.mp4\"",
            "\"assets/social-preview.png\"",
            "\"docs/PUBLISH-CHECKLIST-ja.md\"",
            "\"docs/PLATFORM-VERIFY-v1.0.md\"",
            "\"docs/ARCHITECTURE.md\"",
            "\"docs/ARCHITECTURE-ja.md\"",
            "\"docs/12-10_ROADMAP.md\"",
            "\"docs/SPEC-*\"",
            "\"docs/TESTPLAN-*\"",
            "\"docs/RELEASE-NOTES-*\"",
            "\"scripts/build-release-fast.sh\"",
            "\"AGENTS.md\"",
        ],
    );
}

#[test]
fn powershell_install_scripts_use_non_cargo_default_dir_and_help_missing_release_asset() {
    let install = support::read_repo_file("scripts/install.ps1");
    assert_contains_all(
        &install,
        "scripts/install.ps1",
        &[
            "$env:LOCALAPPDATA\\Programs\\ftime\\bin",
            "No published Windows release asset was found.",
            "For unreleased main, install Rust and use cargo install --path . --force.",
        ],
    );
    assert_contains_none(
        &install,
        "scripts/install.ps1",
        &["$env:USERPROFILE\\.cargo\\bin"],
    );

    let uninstall = support::read_repo_file("scripts/uninstall.ps1");
    assert_contains_all(
        &uninstall,
        "scripts/uninstall.ps1",
        &["$env:LOCALAPPDATA\\Programs\\ftime\\bin"],
    );
    assert_contains_none(
        &uninstall,
        "scripts/uninstall.ps1",
        &["$env:USERPROFILE\\.cargo\\bin"],
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
            "PUBLISH-CHECKLIST-ja.md",
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
