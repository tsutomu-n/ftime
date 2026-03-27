use anyhow::{Context, Result, bail};
use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[cfg(unix)]
const UNIX_INSTALLER_URL: &str =
    "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.sh";
#[cfg(windows)]
const WINDOWS_INSTALLER_URL: &str =
    "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.ps1";

pub fn self_update() -> Result<()> {
    let current_exe = env::current_exe().context("failed to resolve current executable path")?;
    let install_dir = if let Some(dir) = env::var_os("FTIME_SELF_UPDATE_INSTALL_DIR") {
        PathBuf::from(dir)
    } else {
        let invoked_exe = resolve_invoked_executable(&current_exe);
        resolve_install_dir(&current_exe, invoked_exe.as_deref())?
    };
    let previous_version = read_binary_version(&current_exe);

    run_platform_update(&install_dir)?;
    let installed_exe = install_dir.join(
        current_exe
            .file_name()
            .context("failed to resolve installed binary name")?,
    );
    let current_version = read_binary_version(&installed_exe);
    println!(
        "{}",
        format_self_update_message(
            previous_version.as_deref(),
            current_version.as_deref(),
            &install_dir,
        )
    );
    Ok(())
}

fn installer_url() -> String {
    env::var("FTIME_SELF_UPDATE_URL").unwrap_or_else(|_| default_installer_url().to_string())
}

fn resolve_install_dir(current_exe: &Path, invoked_exe: Option<&Path>) -> Result<PathBuf> {
    if looks_like_cargo_target_dir(current_exe) {
        bail!("--self-update is not available for cargo build outputs");
    }

    invoked_exe
        .unwrap_or(current_exe)
        .parent()
        .map(Path::to_path_buf)
        .context("failed to resolve install directory")
}

fn resolve_invoked_executable(current_exe: &Path) -> Option<PathBuf> {
    let arg0 = env::args_os().next()?;
    let candidate = resolve_argv0_path(Path::new(&arg0))?;
    if canonical_paths_match(&candidate, current_exe) {
        Some(candidate)
    } else {
        None
    }
}

fn resolve_argv0_path(arg0: &Path) -> Option<PathBuf> {
    if arg0.components().count() > 1 {
        absolutize_path(arg0).ok()
    } else {
        find_executable_in_path(arg0)
    }
}

fn absolutize_path(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(env::current_dir()
            .context("failed to obtain current directory")?
            .join(path))
    }
}

fn find_executable_in_path(command: &Path) -> Option<PathBuf> {
    let path_var = env::var_os("PATH")?;
    env::split_paths(&path_var)
        .map(|dir| dir.join(command))
        .find(|candidate| candidate.exists())
}

fn canonical_paths_match(lhs: &Path, rhs: &Path) -> bool {
    match (lhs.canonicalize(), rhs.canonicalize()) {
        (Ok(lhs), Ok(rhs)) => lhs == rhs,
        _ => false,
    }
}

fn read_binary_version(executable: &Path) -> Option<String> {
    let output = Command::new(executable).arg("--version").output().ok()?;
    if !output.status.success() {
        return None;
    }

    parse_version_output(&String::from_utf8_lossy(&output.stdout))
}

fn parse_version_output(output: &str) -> Option<String> {
    let mut parts = output.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some("ftime"), Some(version)) => Some(version.to_string()),
        _ => None,
    }
}

fn format_self_update_message(
    previous_version: Option<&str>,
    current_version: Option<&str>,
    install_dir: &Path,
) -> String {
    match (previous_version, current_version) {
        (Some(previous), Some(current)) if previous == current => {
            format!(
                "ftime is already up to date at {current} in {}",
                install_dir.display()
            )
        }
        (Some(previous), Some(current)) => match compare_versions(previous, current) {
            Some(std::cmp::Ordering::Less) => {
                format!(
                    "ftime updated {previous} -> {current} in {}",
                    install_dir.display()
                )
            }
            Some(std::cmp::Ordering::Greater) => {
                format!(
                    "ftime now points to {current} (was {previous}) in {}",
                    install_dir.display()
                )
            }
            _ => format!(
                "ftime version changed {previous} -> {current} in {}",
                install_dir.display()
            ),
        },
        _ => format!("self-update completed: {}", install_dir.display()),
    }
}

fn compare_versions(lhs: &str, rhs: &str) -> Option<std::cmp::Ordering> {
    let lhs = parse_version_tuple(lhs)?;
    let rhs = parse_version_tuple(rhs)?;
    Some(lhs.cmp(&rhs))
}

fn parse_version_tuple(version: &str) -> Option<(u64, u64, u64)> {
    let mut parts = version.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

fn looks_like_cargo_target_dir(path: &Path) -> bool {
    let parts: Vec<_> = path
        .components()
        .filter_map(|component| match component {
            std::path::Component::Normal(segment) => Some(segment),
            _ => None,
        })
        .collect();

    parts
        .iter()
        .position(|segment| *segment == "target")
        .map(|target_index| &parts[target_index + 1..])
        .is_some_and(matches_cargo_output_tail)
}

fn matches_cargo_output_tail(tail: &[&std::ffi::OsStr]) -> bool {
    matches!(tail, [profile, _bin] if is_cargo_profile_dir(profile))
        || matches!(tail, [profile, subdir, _bin]
            if is_cargo_profile_dir(profile) && is_cargo_binary_subdir(subdir))
        || matches!(tail, [_triple, profile, _bin] if is_cargo_profile_dir(profile))
        || matches!(tail, [_triple, profile, subdir, _bin]
            if is_cargo_profile_dir(profile) && is_cargo_binary_subdir(subdir))
}

fn is_cargo_profile_dir(segment: &std::ffi::OsStr) -> bool {
    !segment.is_empty() && !is_known_non_profile_dir(segment)
}

fn is_cargo_binary_subdir(segment: &std::ffi::OsStr) -> bool {
    segment == "deps" || segment == "examples"
}

fn is_known_non_profile_dir(segment: &std::ffi::OsStr) -> bool {
    matches!(
        segment.to_str(),
        Some(
            ".fingerprint"
                | "bin"
                | "build"
                | "deps"
                | "doc"
                | "examples"
                | "incremental"
                | "lib"
                | "package"
                | "tmp"
        )
    )
}

#[cfg(unix)]
fn default_installer_url() -> &'static str {
    UNIX_INSTALLER_URL
}

#[cfg(windows)]
fn default_installer_url() -> &'static str {
    WINDOWS_INSTALLER_URL
}

#[cfg(unix)]
fn run_platform_update(install_dir: &Path) -> Result<()> {
    let url = installer_url();
    let installer = Command::new("curl")
        .arg("-fsSL")
        .arg(&url)
        .output()
        .with_context(|| format!("failed to download installer from {url}"))?;

    if !installer.status.success() {
        bail!("failed to download installer from {url}");
    }

    let mut bash = Command::new("bash")
        .env("INSTALL_DIR", install_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("failed to start installer shell")?;

    let stdin = bash
        .stdin
        .as_mut()
        .context("failed to open installer stdin")?;
    stdin
        .write_all(&installer.stdout)
        .context("failed to write installer script to bash")?;
    let _ = bash.stdin.take();

    let status = bash.wait().context("failed to wait for installer")?;
    if !status.success() {
        bail!("self-update installer failed");
    }
    Ok(())
}

#[cfg(windows)]
fn run_platform_update(install_dir: &Path) -> Result<()> {
    let status = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg("& ([scriptblock]::Create((Invoke-WebRequest -Uri $env:FTIME_SELF_UPDATE_URL -UseBasicParsing).Content)) -InstallDir $env:FTIME_SELF_UPDATE_INSTALL_DIR")
        .env("FTIME_SELF_UPDATE_URL", installer_url())
        .env("FTIME_SELF_UPDATE_INSTALL_DIR", install_dir)
        .status()
        .context("failed to start PowerShell installer")?;

    if !status.success() {
        bail!("self-update installer failed");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn default_url_points_to_latest_release_installer() {
        let url = default_installer_url();
        assert!(url.contains("releases/latest/download/ftime-install"));
    }

    #[test]
    fn resolve_install_dir_rejects_cargo_target_outputs() {
        let path = PathBuf::from("/tmp/work/target/debug/ftime");
        let err = resolve_install_dir(&path, None).unwrap_err().to_string();
        assert!(err.contains("--self-update is not available for cargo build outputs"));
    }

    #[test]
    fn resolve_install_dir_rejects_cross_target_outputs() {
        let path = PathBuf::from("/tmp/work/target/x86_64-unknown-linux-gnu/release/ftime");
        let err = resolve_install_dir(&path, None).unwrap_err().to_string();
        assert!(err.contains("--self-update is not available for cargo build outputs"));
    }

    #[test]
    fn resolve_install_dir_rejects_custom_profile_outputs() {
        let path = PathBuf::from("/tmp/work/target/dist/ftime");
        let err = resolve_install_dir(&path, None).unwrap_err().to_string();
        assert!(err.contains("--self-update is not available for cargo build outputs"));
    }

    #[test]
    fn resolve_install_dir_rejects_cross_target_custom_profile_outputs() {
        let path = PathBuf::from("/tmp/work/target/aarch64-apple-darwin/dist/ftime");
        let err = resolve_install_dir(&path, None).unwrap_err().to_string();
        assert!(err.contains("--self-update is not available for cargo build outputs"));
    }

    #[test]
    fn resolve_install_dir_accepts_non_cargo_target_like_paths() {
        let path = PathBuf::from("/tmp/release/tools/target/ftime");
        let install_dir = resolve_install_dir(&path, None).unwrap();
        assert_eq!(install_dir, Path::new("/tmp/release/tools/target"));
    }

    #[test]
    fn resolve_install_dir_accepts_common_bin_layout_under_target() {
        let path = PathBuf::from("/opt/target/bin/ftime");
        let install_dir = resolve_install_dir(&path, None).unwrap();
        assert_eq!(install_dir, Path::new("/opt/target/bin"));
    }

    #[test]
    fn resolve_install_dir_accepts_regular_install_locations() {
        let path = PathBuf::from("/home/tn/.local/bin/ftime");
        let install_dir = resolve_install_dir(&path, None).unwrap();
        assert_eq!(install_dir, Path::new("/home/tn/.local/bin"));
    }

    #[test]
    fn resolve_install_dir_prefers_invoked_symlink_parent() {
        let current_exe = PathBuf::from("/tmp/work/real/ftime");
        let invoked_exe = PathBuf::from("/tmp/work/link/ftime");
        let install_dir = resolve_install_dir(&current_exe, Some(&invoked_exe)).unwrap();
        assert_eq!(install_dir, Path::new("/tmp/work/link"));
    }

    #[test]
    fn parse_version_output_reads_clap_version_output() {
        let version = parse_version_output("ftime 1.0.0\n").unwrap();
        assert_eq!(version, "1.0.0");
    }

    #[test]
    fn format_self_update_message_reports_upgrade() {
        let message = format_self_update_message(
            Some("1.0.0"),
            Some("1.0.1"),
            Path::new("/home/tn/.local/bin"),
        );
        assert_eq!(
            message,
            "ftime updated 1.0.0 -> 1.0.1 in /home/tn/.local/bin"
        );
    }

    #[test]
    fn format_self_update_message_reports_same_version() {
        let message = format_self_update_message(
            Some("1.0.0"),
            Some("1.0.0"),
            Path::new("/home/tn/.local/bin"),
        );
        assert_eq!(
            message,
            "ftime is already up to date at 1.0.0 in /home/tn/.local/bin"
        );
    }

    #[test]
    fn format_self_update_message_reports_retargeted_version() {
        let message = format_self_update_message(
            Some("1.0.2"),
            Some("1.0.0"),
            Path::new("/home/tn/.local/bin"),
        );
        assert_eq!(
            message,
            "ftime now points to 1.0.0 (was 1.0.2) in /home/tn/.local/bin"
        );
    }
}
