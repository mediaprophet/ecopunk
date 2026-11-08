$ErrorActionPreference = 'Stop'
$assetRoot = Split-Path -Parent $PSScriptRoot
$deps = 'C:\github\qualiaDB\target\debug\deps'
$core = Get-ChildItem -LiteralPath $deps -Filter 'libqualia_core_db-*.rlib' | Sort-Object LastWriteTime -Descending | Select-Object -First 1
$vibe = Get-ChildItem -LiteralPath $deps -Filter 'libvibe-*.rlib' | Sort-Object LastWriteTime -Descending | Select-Object -First 1
if (-not $core -or -not $vibe) { throw 'Existing compiled native libraries required; this helper does not rebuild them.' }
$mingw = 'C:\Users\tim_h\AppData\Local\Microsoft\WinGet\Packages\BrechtSanders.WinLibs.POSIX.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe\mingw64\bin'
$compileArgs = @('--edition=2021','--crate-name','ecopunk_spray_asset',(Join-Path $PSScriptRoot 'generate.rs'),'-C','debuginfo=0','-C','opt-level=1','--extern',"qualia_core_db=$($core.FullName)",'--extern',"vibe=$($vibe.FullName)",'-L',"dependency=$deps",'-o',(Join-Path $PSScriptRoot 'generate.exe'))
if (Test-Path -LiteralPath (Join-Path $mingw 'gcc.exe')) {
    $compileArgs += @('-C',"linker=$(Join-Path $mingw 'gcc.exe')",'-C',"dlltool=$(Join-Path $mingw 'dlltool.exe')")
}
foreach ($dir in (Get-ChildItem C:/github/qualiaDB/target/debug/build -Directory)) {
    $native = Join-Path $dir.FullName 'out'
    if (Test-Path -LiteralPath $native) { $compileArgs += @('-L',"native=$native") }
}
foreach ($registry in (Get-ChildItem C:/Users/tim_h/.cargo/registry/src -Directory)) {
    foreach ($package in (Get-ChildItem -LiteralPath $registry.FullName -Directory -Filter 'winapi-x86_64-pc-windows-gnu-*')) {
        $native = Join-Path $package.FullName 'lib'
        if (Test-Path -LiteralPath $native) { $compileArgs += @('-L',"native=$native") }
    }
}
& rustc @compileArgs 2>&1 | Tee-Object -FilePath (Join-Path $PSScriptRoot 'build-helper.log')
exit $LASTEXITCODE
