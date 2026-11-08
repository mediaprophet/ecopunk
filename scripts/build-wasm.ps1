param([string]$TargetDirectory, [string]$OutputDirectory = 'viewer/pkg', [int]$Jobs = 2)
$ErrorActionPreference = 'Stop'
$repo = Resolve-Path (Join-Path $PSScriptRoot '..')
$crate = Join-Path $repo 'crates\ecopunk-wasm'
$gameDemoBindgenBin = 'C:\github\game-demo\target\wasm-bindgen-cli\bin'
if (Test-Path -LiteralPath $gameDemoBindgenBin) {
    $env:PATH = "$gameDemoBindgenBin;$env:PATH"
}
$env:CARGO_PROFILE_DEV_DEBUG = '0'
$env:CARGO_INCREMENTAL = '0'
if ($Jobs -lt 1) { throw 'Jobs must be positive.' }
$env:CARGO_BUILD_JOBS = [string]$Jobs

if (-not $TargetDirectory) { $TargetDirectory = Join-Path $repo 'target\demo-wasm' }
$env:CARGO_TARGET_DIR = [System.IO.Path]::GetFullPath($TargetDirectory)
$output = [System.IO.Path]::GetFullPath((Join-Path $repo $OutputDirectory))

Push-Location $repo
try {
    wasm-pack build $crate --target web --out-dir $output --release --no-opt
    if ($LASTEXITCODE -ne 0) { throw 'wasm-pack build ecopunk-wasm failed' }
    & (Join-Path $PSScriptRoot 'update-demo-version.ps1') -PackageDirectory $output
    $pkgIgnore = Join-Path $output '.gitignore'
    if (Test-Path -LiteralPath $pkgIgnore) { Remove-Item -LiteralPath $pkgIgnore }
} finally {
    Pop-Location
}
