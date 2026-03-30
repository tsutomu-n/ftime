param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\ftime\bin"
)

$ErrorActionPreference = "Stop"

$Repo = "tsutomu-n/ftime"
$Bin = "ftime"

function Resolve-Download {
    param(
        [Parameter(Mandatory = $true)][string]$Version,
        [Parameter(Mandatory = $true)][string]$Repo,
        [Parameter(Mandatory = $true)][string]$Target
    )

    if ($Version -eq "latest") {
        return @{
            Tag = "latest"
            Url = "https://github.com/$Repo/releases/latest/download/$Bin-$Target.zip"
        }
    }

    $Tag = "v$($Version.TrimStart('v'))"
    return @{
        Tag = $Tag
        Url = "https://github.com/$Repo/releases/download/$Tag/$Bin-$($Tag.TrimStart('v'))-$Target.zip"
    }
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
$PathParts = $env:Path -split ";" | ForEach-Object { $_.TrimEnd("\") }
$InstallDirNorm = $InstallDir.TrimEnd("\")
if ($PathParts -notcontains $InstallDirNorm) {
    Write-Host "PATHに $InstallDir を追加してください"
}
