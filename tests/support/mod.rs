use std::fs;
use std::path::Path;
use std::sync::OnceLock;

static PACKAGE_VERSION: OnceLock<String> = OnceLock::new();

pub fn read_repo_file(path: &str) -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap()
}

pub fn package_version() -> &'static str {
    PACKAGE_VERSION
        .get_or_init(|| {
            let cargo_toml = read_repo_file("Cargo.toml");
            let mut in_package = false;

            for line in cargo_toml.lines() {
                let trimmed = line.trim();

                if trimmed.starts_with('[') {
                    in_package = trimmed == "[package]";
                    continue;
                }

                if !in_package {
                    continue;
                }

                if let Some(version) = trimmed
                    .strip_prefix("version = \"")
                    .and_then(|rest| rest.strip_suffix('"'))
                {
                    return version.to_string();
                }
            }

            panic!("failed to find package version in Cargo.toml");
        })
        .as_str()
}

#[allow(dead_code)]
pub fn release_tag() -> String {
    format!("v{}", package_version())
}
