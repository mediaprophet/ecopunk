param([string]$OutputDirectory = 'target/demo-native', [switch]$ExporterOnly)
$ErrorActionPreference = 'Stop'
$repo = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
$output = Join-Path $repo $OutputDirectory
$deps = 'C:/github/qualiaDB/target/debug/deps'
New-Item -ItemType Directory -Force -Path $output | Out-Null
function Resolve-Library([string]$name) {
    $item = Get-ChildItem -LiteralPath $deps -Filter "lib$name-*.rlib" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    if (-not $item) { $item = Get-ChildItem -LiteralPath (Join-Path $repo 'target/demo-prerequisites/debug/deps') -Filter "lib$name-*.rlib" -ErrorAction SilentlyContinue | Select-Object -First 1 }
    if (-not $item) { throw "Required compiled dependency absent: $name" }
    return $item.FullName
}
$nativeArgs = @('-L',"dependency=$deps",'-L',"dependency=$output")
if (Test-Path -LiteralPath (Join-Path $repo 'target/demo-prerequisites/debug/deps')) { $nativeArgs += @('-L',('dependency=' + (Join-Path $repo 'target/demo-prerequisites/debug/deps'))) }
foreach ($folder in (Get-ChildItem 'C:/github/qualiaDB/target/debug/build' -Directory)) {
    $native = Join-Path $folder.FullName 'out'
    if (Test-Path -LiteralPath $native) { $nativeArgs += @('-L',"native=$native") }
}
foreach ($registry in (Get-ChildItem 'C:/Users/tim_h/.cargo/registry/src' -Directory)) {
    foreach ($pattern in @('windows_x86_64_gnu-*','winapi-x86_64-pc-windows-gnu-*')) {
        foreach ($package in (Get-ChildItem -LiteralPath $registry.FullName -Directory -Filter $pattern)) {
            $native = Join-Path $package.FullName 'lib'
            if (Test-Path -LiteralPath $native) { $nativeArgs += @('-L',"native=$native") }
        }
    }
}
$mingw = 'C:/Users/tim_h/AppData/Local/Microsoft/WinGet/Packages/BrechtSanders.WinLibs.POSIX.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe/mingw64/bin'
$nativeArgs += @('-C',"linker=$mingw/gcc.exe",'-C',"dlltool=$mingw/dlltool.exe")
function Build-Crate([string]$name, [string]$source, [string]$kind, [string[]]$libraries, [hashtable]$local, [string[]]$ExtraArguments = @()) {
    $destination = if ($kind -eq 'bin') { Join-Path $output "$name.exe" } else { Join-Path $output "lib$name.rlib" }
    $argsList = @('--edition=2021','--crate-name',$name,'--crate-type',$kind,$source,'-C','debuginfo=0','-C','opt-level=0','-o',$destination) + $nativeArgs + $ExtraArguments
    foreach ($library in $libraries) {
        $path = if ($local.ContainsKey($library)) { $local[$library] } else { Resolve-Library $library }
        $argsList += @('--extern',"$library=$path")
        "$library`t$path" | Add-Content -LiteralPath (Join-Path $output 'dependencies.tsv')
    }
    $env:CARGO_MANIFEST_DIR = Split-Path (Split-Path $source -Parent) -Parent
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    & rustc @argsList 2>&1 | Tee-Object -FilePath (Join-Path $output "$name-build.log") | Out-Host
    $code = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    if ($code -ne 0) { throw "Native content build failed: $name" }
    return $destination
}
if (-not $ExporterOnly) {
$sim = if (Test-Path (Join-Path $output 'libqualia_sim.rlib')) { (Get-Item (Join-Path $output 'libqualia_sim.rlib')).FullName } else { Build-Crate 'qualia_sim' 'C:/github/qualiaDB/crates/qualia-sim/src/lib.rs' 'rlib' @('bytemuck','serde','serde_json','qualia_core_db') @{} }
$geo = if (Test-Path (Join-Path $output 'libqualia_geospatial.rlib')) { (Get-Item (Join-Path $output 'libqualia_geospatial.rlib')).FullName } else { Build-Crate 'qualia_geospatial' 'C:/github/qualiaDB/crates/qualia-geospatial/src/lib.rs' 'rlib' @('bytemuck','serde','serde_json','qualia_core_db') @{} }
$render = if (Test-Path (Join-Path $output 'libwebizen_render.rlib')) { (Get-Item (Join-Path $output 'libwebizen_render.rlib')).FullName } else { Build-Crate 'webizen_render' 'C:/github/qualiaDB/crates/webizen-render/src/lib.rs' 'rlib' @('wgpu','etalon_render','bytemuck','log','serde','qualia_core_db','vibe','image','base64','pollster') @{} @('--cfg=feature=\"qualia\"','--cfg=native') }
$terrainDeps = @('bytemuck','crc32c','serde','serde_json','qualia_geospatial','qualia_sim','vibe','webizen_render','etalon','etalon_agent','etalon_wgsl_forge','etalon_nquin','etalon_tensor','etalon_render','etalon_wasm','rio_api','rio_turtle','sha2','image','flate2','qualia_core_db')
$terrain = Build-Crate 'ecopunk_terrain' (Join-Path $repo 'crates/ecopunk-terrain/src/lib.rs') 'rlib' $terrainDeps @{qualia_sim=$sim;qualia_geospatial=$geo;webizen_render=$render}
$wasm = Build-Crate 'ecopunk_wasm' (Join-Path $repo 'crates/ecopunk-wasm/src/lib.rs') 'rlib' @('ecopunk_terrain','qualia_core_db','vibe','serde_json') @{ecopunk_terrain=$terrain}
} else {
    $terrain = (Get-Item -LiteralPath (Join-Path $output 'libecopunk_terrain.rlib')).FullName
    $wasm = (Get-Item -LiteralPath (Join-Path $output 'libecopunk_wasm.rlib')).FullName
}
$exe = Build-Crate 'rebuild_demo_assets' (Join-Path $repo 'crates/ecopunk-wasm/src/bin/rebuild_demo_assets.rs') 'bin' @('ecopunk_terrain','ecopunk_wasm','qualia_core_db') @{ecopunk_terrain=$terrain;ecopunk_wasm=$wasm}
Write-Host "Native content exporter: $exe"
