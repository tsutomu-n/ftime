param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\ftime\bin"
)

$ErrorActionPreference = "Stop"

$Repo = "tsutomu-n/ftime"
$Bin = "ftime"

function Resolve-LatestTag {
    param(
        [Parameter(Mandatory = $true)][string]$Repo
    )

    $Release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
    $Tag = "$($Release.tag_name)".Trim()
    if (-not $Tag) {
        throw "failed to resolve latest release tag"
    }

    return $Tag
}

function Resolve-Download {
    param(
        [Parameter(Mandatory = $true)][string]$Version,
        [Parameter(Mandatory = $true)][string]$Repo,
        [Parameter(Mandatory = $true)][string]$Target
    )

    if ($Version -eq "latest") {
        $Tag = Resolve-LatestTag -Repo $Repo
    }
    else {
        $Tag = "v$($Version.TrimStart('v'))"
    }
    $VersionNumber = $Tag.TrimStart('v')
    return @{
        Tag = $Tag
        Url = "https://github.com/$Repo/releases/download/$Tag/$Bin-$VersionNumber-$Target.zip"
    }
}

function Split-PathEntries {
    param(
        [string]$PathValue
    )

    if (-not $PathValue) {
        return @()
    }

    return $PathValue -split ";" |
        ForEach-Object { $_.Trim().TrimEnd("\") } |
        Where-Object { $_ }
}

$Arch = $env:PROCESSOR_ARCHITECTURE
if ($Arch -ne "AMD64") {
    throw "unsupported arch: $Arch"
}

$Target = "x86_64-pc-windows-msvc"

$Download = Resolve-Download -Version $Version -Repo $Repo -Target $Target
$Tag = $Download.Tag
$Url = $Download.Url
$Asset = Split-Path $Url -Leaf

$Tmp = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString())
New-Item -ItemType Directory -Force -Path $Tmp | Out-Null
$ZipPath = Join-Path $Tmp $Asset

try {
    Invoke-WebRequest -Uri $Url -OutFile $ZipPath
}
catch {
    $StatusCode = $null
    if ($_.Exception.Response -and $_.Exception.Response.StatusCode) {
        $StatusCode = [int]$_.Exception.Response.StatusCode
    }

    if ($StatusCode -eq 404) {
        throw "No published Windows release asset was found. For unreleased main, install Rust and use cargo install --path . --force."
    }

    throw
}

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Expand-Archive -Path $ZipPath -DestinationPath $Tmp -Force

$BinPath = Join-Path $InstallDir "$Bin.exe"
Copy-Item -Path (Join-Path $Tmp "$Bin.exe") -Destination $BinPath -Force

Write-Host "$Bin installed to $InstallDir"
$InstallDirNorm = $InstallDir.TrimEnd("\")
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
$UserPathParts = Split-PathEntries -PathValue $UserPath
if ($UserPathParts -notcontains $InstallDirNorm) {
    $UpdatedUserPath = if ($UserPath) { "$UserPath;$InstallDir" } else { $InstallDir }
    [Environment]::SetEnvironmentVariable("Path", $UpdatedUserPath, "User")
    Write-Host "Added $InstallDir to your user PATH. Restart your shell if needed."
}

$ProcessPathParts = Split-PathEntries -PathValue $env:Path
if ($ProcessPathParts -notcontains $InstallDirNorm) {
    $env:Path = if ($env:Path) { "$InstallDir;$env:Path" } else { $InstallDir }
}
