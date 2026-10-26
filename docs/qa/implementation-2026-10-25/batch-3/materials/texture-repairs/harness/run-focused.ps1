$ErrorActionPreference='Stop'
$repoRoot=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../../../../../../..'))
$deps=Join-Path $repoRoot 'target/debug/deps'
$native='C:/Users/tim_h/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/winapi-x86_64-pc-windows-gnu-0.4.0/lib'
$binary=Join-Path $PSScriptRoot 'candidate-tests.exe'
& rustc --edition=2021 --test (Join-Path $PSScriptRoot 'candidate.rs') --extern "qualia_core_db=$deps/libqualia_core_db.rlib" --extern "serde_json=$deps/libserde_json-0ae97aa50cc02612.rlib" --extern "zstd=$deps/libzstd-717830b2a841f364.rlib" -L "dependency=$deps" -L "native=$native" -o $binary 2>&1 | Tee-Object (Join-Path $PSScriptRoot 'candidate-compile.log')
$compileExit=$LASTEXITCODE
if($compileExit -ne 0){throw "Focused source harness compile failed: $compileExit"}
& $binary --nocapture --test-threads=1 2>&1 | Tee-Object (Join-Path $PSScriptRoot 'candidate-tests.log')
$testExit=$LASTEXITCODE
[ordered]@{compile_exit=$compileExit;test_exit=$testExit;binary_sha256=(Get-FileHash -LiteralPath $binary -Algorithm SHA256).Hash;cached_core_rlib_sha256=(Get-FileHash -LiteralPath (Join-Path $deps 'libqualia_core_db.rlib') -Algorithm SHA256).Hash}|ConvertTo-Json|Set-Content -LiteralPath (Join-Path $PSScriptRoot 'run-receipt.json')
if($testExit -ne 0){throw "Focused source harness tests failed: $testExit"}
