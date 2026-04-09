mod support;

fn assert_contains_all(content: &str, path: &str, snippets: &[&str]) {
    for snippet in snippets {
        assert!(
            content.contains(snippet),
            "missing required snippet in {path}: {snippet}"
        );
    }
}

#[test]
fn install_ps1_uses_windows_release_defaults_and_messages() {
    let content = support::read_repo_file("scripts/install.ps1");

    assert_contains_all(
        &content,
        "scripts/install.ps1",
        &[
            "[string]$Version = \"latest\"",
            "[string]$InstallDir = \"$env:LOCALAPPDATA\\Programs\\ftime\\bin\"",
            "https://api.github.com/repos/$Repo/releases/latest",
            "$Target = \"x86_64-pc-windows-msvc\"",
            "$Arch = $env:PROCESSOR_ARCHITECTURE",
            "if ($Arch -ne \"AMD64\")",
            "throw \"unsupported arch: $Arch\"",
            "Invoke-WebRequest -Uri $Url -OutFile $ZipPath",
            "Expand-Archive -Path $ZipPath -DestinationPath $Tmp -Force",
            "Copy-Item -Path (Join-Path $Tmp \"$Bin.exe\") -Destination $BinPath -Force",
            "Write-Host \"$Bin installed to $InstallDir\"",
            "Write-Host \"PATHに $InstallDir を追加してください\"",
        ],
    );
}

#[test]
fn install_ps1_keeps_404_guidance_for_unreleased_main() {
    let content = support::read_repo_file("scripts/install.ps1");

    assert_contains_all(
        &content,
        "scripts/install.ps1",
        &[
            "if ($StatusCode -eq 404)",
            "No published Windows release asset was found. For unreleased main, install Rust and use cargo install --path . --force.",
        ],
    );
}

#[test]
fn uninstall_ps1_keeps_default_dir_binary_name_and_messages() {
    let content = support::read_repo_file("scripts/uninstall.ps1");

    assert_contains_all(
        &content,
        "scripts/uninstall.ps1",
        &[
            "[string]$InstallDir = \"$env:LOCALAPPDATA\\Programs\\ftime\\bin\"",
            "$Bin = \"ftime.exe\"",
            "$BinPath = Join-Path $InstallDir $Bin",
            "if (Test-Path $BinPath)",
            "Remove-Item -Path $BinPath -Force",
            "Write-Host \"ftime uninstalled from $InstallDir\"",
            "exit 0",
            "Write-Host \"ftime is not installed in $InstallDir\"",
        ],
    );
}
