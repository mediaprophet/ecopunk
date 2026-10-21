$ErrorActionPreference = 'Stop'
$repo = Resolve-Path (Join-Path $PSScriptRoot '..')
$crate = Join-Path $repo 'crates\ecopunk-wasm'
$gameDemoBindgenBin = 'C:\github\game-demo\target\wasm-bindgen-cli\bin'
if (Test-Path -LiteralPath $gameDemoBindgenBin) {
    $env:PATH = "$gameDemoBindgenBin;$env:PATH"
}
$env:CARGO_PROFILE_DEV_DEBUG = '0'
$env:CARGO_INCREMENTAL = '0'

$wasmDebug = Join-Path $repo 'target\wasm32-unknown-unknown\debug'
if (Test-Path -LiteralPath $wasmDebug) {
    Remove-Item -LiteralPath $wasmDebug -Recurse -Force -ErrorAction SilentlyContinue
}

Push-Location $repo
try {
    wasm-pack build $crate --target web --out-dir ../../viewer/pkg --release --no-opt
    if ($LASTEXITCODE -ne 0) { throw 'wasm-pack build ecopunk-wasm failed' }
    $pkgIgnore = Join-Path $repo 'viewer\pkg\.gitignore'
    if (Test-Path -LiteralPath $pkgIgnore) { Remove-Item -LiteralPath $pkgIgnore }
} finally {
    Pop-Location
    $targetDir = Join-Path $repo 'target'
    if (Test-Path -LiteralPath $targetDir) {
        Remove-Item -LiteralPath $targetDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}
