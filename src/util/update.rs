use anyhow::{Context, Result, bail};
use std::env;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

#[cfg(unix)]
const UNIX_INSTALLER_URL: &str =
    "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.sh";
#[cfg(windows)]
const WINDOWS_INSTALLER_URL: &str =
    "https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.ps1";

pub fn self_update() -> Result<()> {
    let current_exe = env::current_exe().context("failed to resolve current executable path")?;
    let install_dir = current_exe
        .parent()
        .context("failed to resolve install directory")?;

    run_platform_update(install_dir)?;
    println!("self-update completed: {}", install_dir.display());
    Ok(())
}

fn installer_url() -> String {
    env::var("FTIME_SELF_UPDATE_URL").unwrap_or_else(|_| default_installer_url().to_string())
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

    #[test]
    fn default_url_points_to_latest_release_installer() {
        let url = default_installer_url();
        assert!(url.contains("releases/latest/download/ftime-install"));
    }
}
