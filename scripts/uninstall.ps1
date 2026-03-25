param(
    [string]$InstallDir = "$env:USERPROFILE\.cargo\bin"
)

$ErrorActionPreference = "Stop"

$Bin = "ftime.exe"
$BinPath = Join-Path $InstallDir $Bin

if (Test-Path $BinPath) {
    Remove-Item -Path $BinPath -Force
    Write-Host "ftime uninstalled from $InstallDir"
    exit 0
}

Write-Host "ftime is not installed in $InstallDir"
