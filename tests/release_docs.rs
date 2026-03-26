use std::fs;
use std::path::Path;

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
