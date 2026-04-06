mod support;

use std::path::Path;

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

    for path in ["docs/CLI.md", "docs/RELEASE-NOTES-v2.0.md"] {
        let content = support::read_repo_file(path);
        assert!(
            content.contains(&format!("v{}", support::package_version())),
            "{path} must describe v{}",
            support::package_version()
        );
    }
}

#[test]
fn readme_surfaces_v2_contract() {
    let root = support::read_repo_file("README.md");
    assert_contains_all(
        &root,
        "README.md",
        &[
            "What changed in this folder recently?",
            "Human-first bucket view by default",
            "hidden files stay visible by default while hidden directories stay hidden",
            "## Command quick reference",
            "| `ftime --no-ignore` | Show ignored entries too |",
            "| `ftime --check-update` | Check for a newer published release |",
            "| `ftime --version` | Print the installed version |",
            "ftime --plain",
            "ftime --json | jq -r '.path'",
            "`-a, --all`: show hidden files and hidden directories",
            "`--all-history`: expand the History bucket",
            "`--hide-dots`: hide all hidden entries",
            "`--files-only`: only show regular files",
            "`--no-hints`: disable `[child: ...]` hints",
            "`--color <auto|always|never>`",
        ],
    );
    assert_contains_none(
        &root,
        "README.md",
        &[
            "Current Timezone: UTC±HH:MM",
            "Fresh",
            "--exclude-dots",
            "--no-labels",
            "Directories are excluded.",
        ],
    );
}

#[test]
fn cli_contract_documents_v2_shapes() {
    let cli = support::read_repo_file("docs/CLI.md");
    assert_contains_all(
        &cli,
        "docs/CLI.md",
        &[
            &format!("# ftime v{} CLI Contract", support::package_version()),
            "`-a, --all`",
            "`--all-history`",
            "`--hide-dots`",
            "`--plain`",
            "`--json`",
            "`--files-only`",
            "`--no-hints`",
            "`--color <auto|always|never>`",
            "Default output is always the human view.",
            "path<TAB>bucket<TAB>time",
            "JSON Lines",
            "child hint",
            "No matching entries",
            "Skipped N unreadable entries",
        ],
    );
    assert_contains_none(
        &cli,
        "docs/CLI.md",
        &["--exclude-dots", "--no-labels", "Current Timezone:"],
    );
}

#[test]
fn japanese_docs_track_v2_roles() {
    let readme = support::read_repo_file("docs/README-ja.md");
    assert_contains_all(
        &readme,
        "docs/README-ja.md",
        &[
            "このフォルダで最近何が変わった？",
            "デフォルトは人間向け bucket view",
            "## コマンド早見表",
            "`ftime --check-update`",
            "`ftime --version`",
            "`--all-history`",
            "`--hide-dots`",
            "`--plain`",
            "`--no-hints`",
        ],
    );
    assert_contains_none(
        &readme,
        "docs/README-ja.md",
        &["--exclude-dots", "--no-labels", "Fresh"],
    );

    let cli = support::read_repo_file("docs/CLI-ja.md");
    assert_contains_all(
        &cli,
        "docs/CLI-ja.md",
        &[
            "## コマンド署名",
            "`--all-history`",
            "`--hide-dots`",
            "`--files-only`",
            "`--plain`",
            "`--check-update`",
            "`--self-update`",
            "`--no-hints`",
            "`--color <auto|always|never>`",
            "Unicode 表示幅",
            "No matching entries",
        ],
    );

    let guide = support::read_repo_file("docs/USER-GUIDE-ja.md");
    assert_contains_all(
        &guide,
        "docs/USER-GUIDE-ja.md",
        &[
            "## 使い分け",
            "`ftime -a`",
            "`ftime --hide-dots`",
            "`ftime --check-update`",
        ],
    );

    let overview = support::read_repo_file("docs/ftime-overview-ja.md");
    assert_contains_all(
        &overview,
        "docs/ftime-overview-ja.md",
        &["README-ja.md", "CLI-ja.md", "USER-GUIDE-ja.md"],
    );
}

#[test]
fn chinese_readme_tracks_v2_core_flags() {
    let zh = support::read_repo_file("docs/README-zh.md");
    assert_contains_all(
        &zh,
        "docs/README-zh.md",
        &[
            "这个文件夹最近有什么变化？",
            "## 命令速查",
            "`ftime --check-update`",
            "`ftime --version`",
            "`--all-history`",
            "`--hide-dots`",
            "`--plain`",
            "`--files-only`",
            "`--no-hints`",
        ],
    );
    assert_contains_none(
        &zh,
        "docs/README-zh.md",
        &["--exclude-dots", "--no-labels", "Fresh"],
    );
}

#[test]
fn demo_assets_and_release_notes_reference_v2_commands() {
    let demo = support::read_repo_file("demo/README.md");
    assert_contains_all(
        &demo,
        "demo/README.md",
        &[
            "ftime",
            "ftime -a",
            "ftime --all-history",
            "ftime --plain",
            "ftime --json | jq -r '.path'",
        ],
    );

    let tape = support::read_repo_file("demo/tapes/demo_ftime.tape");
    assert_contains_all(
        &tape,
        "demo/tapes/demo_ftime.tape",
        &[
            "Type \"ftime\"",
            "Type \"ftime --all-history\"",
            "Type \"ftime --plain\"",
            "Type \"ftime --json | jq -r '.path'\"",
        ],
    );

    let notes = support::read_repo_file("docs/RELEASE-NOTES-v2.0.md");
    assert_contains_all(
        &notes,
        "docs/RELEASE-NOTES-v2.0.md",
        &[
            &format!("# ftime v{} Release Notes", support::package_version()),
            "`-a` now means hidden entries",
            "default output is now the human view",
            "`--plain` adds the TSV contract",
            "`Fresh` was removed",
        ],
    );
}

#[test]
fn public_doc_set_remains_canonical() {
    for path in [
        "README.md",
        "docs/CLI.md",
        "docs/CLI-ja.md",
        "docs/README-ja.md",
        "docs/README-zh.md",
        "docs/USER-GUIDE-ja.md",
        "docs/ftime-overview-ja.md",
        "docs/RELEASE-NOTES-v2.0.md",
        "demo/README.md",
        "demo/tapes/demo_ftime.tape",
    ] {
        assert!(
            Path::new(env!("CARGO_MANIFEST_DIR")).join(path).exists(),
            "{path} must exist"
        );
    }
}
