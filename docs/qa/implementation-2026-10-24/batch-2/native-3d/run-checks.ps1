param([switch]$Baseline, [string]$Filter = 'journal_tests')
$ErrorActionPreference = 'Stop'
$taskRoot = $PSScriptRoot
$deps = 'C:/github/ecopunk/target/debug/deps'
$exeName = if ($Baseline) { 'baseline-tests.exe' } else { 'journal-tests.exe' }
$logName = if ($Baseline) { 'baseline' } else { 'candidate' }
$outputPath = Join-Path $taskRoot "harness/$exeName"
$compileLog = Join-Path $taskRoot "$logName-compile.log"
$runLog = Join-Path $taskRoot "$logName-$Filter.log"
$rustArgs = @('--test', '--edition=2021', (Join-Path $taskRoot 'harness/lib.rs'),
    '--extern', "qualia_core_db=$deps/libqualia_core_db.rlib",
    '--extern', "vibe=$deps/libvibe-09a4340642d70c17.rlib",
    '-L', "dependency=$deps",
    '-L', 'native=C:/github/ecopunk/target/debug/build/blake3-da8ba380479449f1/out',
    '-L', 'native=C:/github/ecopunk/target/debug/build/ring-b53baf53c685dea7/out',
    '-L', 'native=C:/github/ecopunk/target/debug/build/zstd-sys-ddb15c1df437b75d/out',
    '-L', 'native=C:/Users/tim_h/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/winapi-x86_64-pc-windows-gnu-0.4.0/lib',
    '-L', 'native=C:/github/qualiaDB/vendor/directml/bin/x64-win',
    '-o', $outputPath)
if ($Baseline) { $rustArgs += @('--cfg', 'feature="baseline"') }
& rustc @rustArgs *> $compileLog
$compileExit = $LASTEXITCODE
if ($compileExit -ne 0) {
    Get-Content -LiteralPath $compileLog
    exit $compileExit
}
& $outputPath $Filter --nocapture *> $runLog
$runExit = $LASTEXITCODE
Get-Content -LiteralPath $runLog
exit $runExit
