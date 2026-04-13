param(
    [string]$Version = '0.1.0'
)

$ErrorActionPreference = 'Stop'
$Root = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$DistDir = Join-Path $Root 'dist\windows'
$PackageDir = Join-Path $DistDir 'cliptube-ai-windows-x86_64'
$ZipPath = Join-Path $DistDir 'cliptube-ai-windows-x86_64.zip'

if (Test-Path $PackageDir) { Remove-Item $PackageDir -Recurse -Force }
if (Test-Path $ZipPath) { Remove-Item $ZipPath -Force }

New-Item -ItemType Directory -Path $PackageDir | Out-Null
Copy-Item (Join-Path $Root 'target\release\cliptube-ai.exe') (Join-Path $PackageDir 'cliptube-ai.exe')
Copy-Item (Join-Path $Root 'assets\icon.ico') (Join-Path $PackageDir 'icon.ico')
Copy-Item (Join-Path $Root 'README.md') (Join-Path $PackageDir 'README.md')

@"
ClipTube AI $Version

Contents:
- cliptube-ai.exe
- icon.ico
- README.md

Tip:
Create a shortcut to cliptube-ai.exe and assign icon.ico if Windows does not pick the executable icon automatically.
"@ | Set-Content -Path (Join-Path $PackageDir 'README-Windows.txt') -Encoding UTF8

Compress-Archive -Path (Join-Path $PackageDir '*') -DestinationPath $ZipPath
Write-Host "Created: $ZipPath"
