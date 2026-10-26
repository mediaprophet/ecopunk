param(
 [Parameter(Mandatory)][ValidateSet('Core','Native','Wasm','Mcp','NativeLib')][string]$Mode,
 [Parameter(Mandatory)][string]$Label
)
$ErrorActionPreference='Stop'
if($Label -notmatch '^[a-zA-Z0-9/-]+$' -or $Label.Contains('..')){throw 'Invalid evidence label'}
$env:PYTHON3='C:/Users/tim_h/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe'
$env:PYTHONDONTWRITEBYTECODE='1'
$env:CARGO_PROFILE_DEV_DEBUG='0'
$env:CARGO_PROFILE_DEV_INCREMENTAL='false'
$env:CARGO_PROFILE_TEST_DEBUG='0'
$env:CARGO_PROFILE_TEST_INCREMENTAL='false'
$env:CARGO_BUILD_JOBS='2'
$env:TMP='C:/github/ecopunk/target/qa-tmp'
$env:TEMP=$env:TMP
$env:POET_NATIVE_CHECKPOINT_TEST_ROOT=Join-Path $PSScriptRoot 'hmc/harness'
$core='C:/github/qualiaDB/Cargo.toml'
$poet='C:/github/poet/Cargo.toml'
$cache='C:/github/ecopunk/target'
$arguments=switch($Mode){
 Core {@('test','--offline','--locked','--manifest-path',$core,'-p','qualia-core-db','--lib','--no-default-features','--features','profile_target_1024,zk-culling,gpu-runtime,gpu-native-portable,gpu-native-dx12,wgsl-forge,privacy-he','--target-dir',$cache,'--','--test-threads=1')}
 Native {@('test','--offline','--locked','--manifest-path',$poet,'-p','poet-cli','--features','native-graphical','--target-dir',$cache,'--','--test-threads=1')}
 Wasm {@('check','--offline','--locked','--manifest-path',$poet,'-p','poet','--lib','--target','wasm32-unknown-unknown','--target-dir',$cache)}
 Mcp {@('test','--offline','--locked','--manifest-path',$core,'-p','qualia-mcp','--lib','--no-default-features','--features','authoring','--target-dir',$cache,'--','--test-threads=1')}
 NativeLib {@('build','--offline','--locked','--manifest-path',$poet,'-p','poet-cli','--lib','--features','native-graphical','--message-format=json','--target-dir',$cache)}
}
$prefix=Join-Path $PSScriptRoot $Label
$started=[DateTime]::UtcNow
$snapshot=Get-Content (Join-Path $PSScriptRoot 'latest-source-snapshot.json') -Raw | ConvertFrom-Json
if((Get-FileHash $snapshot.path -Algorithm SHA256).Hash -ne $snapshot.sha256){throw 'Source pin changed'}
& cargo @arguments *> "$prefix.log"
$code=$LASTEXITCODE
[ordered]@{command='cargo';arguments=$arguments;exit_code=$code;started_utc=$started.ToString('o');finished_utc=[DateTime]::UtcNow.ToString('o');input_snapshot=$snapshot}|ConvertTo-Json -Depth 6|Set-Content "$prefix.json"
Get-Content "$prefix.log" -Tail 35
exit $code
