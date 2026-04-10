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
fn maintaining_doc_exists_and_captures_sync_workflow() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/MAINTAINING.md");
    assert!(path.exists(), "docs/MAINTAINING.md must exist");

    let content = support::read_repo_file("docs/MAINTAINING.md");
    assert_contains_all(
        &content,
        "docs/MAINTAINING.md",
        &[
            "# ftime Maintainer Guide",
            "tests -> code/help -> docs/CLI.md -> README/translated docs -> demo -> release notes -> cargo check/test",
            "## Which tests to update",
            "Help text / option description changes -> tests/help_contract.rs",
            "CLI validation failure changes -> tests/cli_validation_contract.rs",
            "Human / plain / JSON output contract changes -> tests/output_contract.rs",
            "PowerShell installer / uninstaller contract changes -> tests/powershell_contract.rs",
            "Public docs / README / release-notes / demo text changes -> tests/release_docs.rs",
            "Maintainer workflow / sync-order changes -> tests/maintaining_docs.rs",
            "README.md",
            "docs/COMMANDS.md",
            "docs/INSTALL.md",
            "docs/CLI.md",
            "docs/CLI-ja.md",
            "docs/README-ja.md",
            "docs/README-zh.md",
            "docs/USER-GUIDE-ja.md",
            "docs/ftime-overview-ja.md",
            "docs/RELEASE-NOTES-v2.0.md",
            "demo/README.md",
            "demo/tapes/demo_ftime.tape",
            "demo/render-assets.sh",
            "human-visible output changes",
            "Cargo.toml",
            "cargo check",
            "cargo test --quiet",
        ],
    );
}

#[test]
fn release_workflow_uses_gh_cli_instead_of_node_based_release_action() {
    let content = support::read_repo_file(".github/workflows/release.yml");

    assert_contains_all(
        &content,
        ".github/workflows/release.yml",
        &[
            "create_release:",
            "needs: create_release",
            "GH_TOKEN: ${{ github.token }}",
            "gh release view \"$TAG\"",
            "gh release edit \"$TAG\" --title \"$TAG\" --notes-file docs/RELEASE-NOTES-v2.0.md",
            "gh release create \"$TAG\" --title \"$TAG\" --notes-file docs/RELEASE-NOTES-v2.0.md --verify-tag",
            "gh release upload \"$TAG\" dist/* --clobber",
            "gh release upload $env:TAG $files --clobber",
        ],
    );
    assert_contains_none(
        &content,
        ".github/workflows/release.yml",
        &[
            "softprops/action-gh-release",
            "FORCE_JAVASCRIPT_ACTIONS_TO_NODE24",
        ],
    );
}
