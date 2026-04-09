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
