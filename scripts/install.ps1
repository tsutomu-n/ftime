param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:USERPROFILE\.cargo\bin"
)

$ErrorActionPreference = "Stop"

$Repo = "tsutomu-n/ftime"
$Bin = "ftime"

function Resolve-Tag {
    param(
        [Parameter(Mandatory = $true)][string]$Version,
        [Parameter(Mandatory = $true)][string]$Repo
    )

    if ($Version -eq "latest") {
        $Release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
        if (-not $Release.tag_name) { throw "failed to resolve latest release tag" }
        return $Release.tag_name
    }

    return "v$($Version.TrimStart('v'))"
}

$Arch = $env:PROCESSOR_ARCHITECTURE
if ($Arch -ne "AMD64") {
    throw "unsupported arch: $Arch"
}

$Target = "x86_64-pc-windows-msvc"

$Tag = Resolve-Tag -Version $Version -Repo $Repo

$Asset = "$Bin-$($Tag.TrimStart('v'))-$Target.zip"
$Url = "https://github.com/$Repo/releases/download/$Tag/$Asset"

$Tmp = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString())
New-Item -ItemType Directory -Force -Path $Tmp | Out-Null
$ZipPath = Join-Path $Tmp $Asset

Invoke-WebRequest -Uri $Url -OutFile $ZipPath
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Expand-Archive -Path $ZipPath -DestinationPath $Tmp -Force

$BinPath = Join-Path $InstallDir "$Bin.exe"
Copy-Item -Path (Join-Path $Tmp "$Bin.exe") -Destination $BinPath -Force

Write-Host "$Bin installed to $InstallDir"
$PathParts = $env:Path -split ";" | ForEach-Object { $_.TrimEnd("\") }
$InstallDirNorm = $InstallDir.TrimEnd("\")
if ($PathParts -notcontains $InstallDirNorm) {
    Write-Host "PATHに $InstallDir を追加してください"
}
