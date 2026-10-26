param([Parameter(Mandatory)][ValidateSet('Check','Core','Native','Wasm','NativeLib')][string]$Mode,[Parameter(Mandatory)][string]$Label)
$ErrorActionPreference='Stop'
if($Label -notmatch '^[a-zA-Z0-9-]+$'){throw 'Invalid evidence label'}
$env:PYTHON3='C:/Users/tim_h/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe'
$env:PYTHONDONTWRITEBYTECODE='1'
$env:CARGO_PROFILE_DEV_DEBUG='0';$env:CARGO_PROFILE_DEV_INCREMENTAL='false'
$env:CARGO_PROFILE_TEST_DEBUG='0';$env:CARGO_PROFILE_TEST_INCREMENTAL='false'
$env:CARGO_BUILD_JOBS='2'
$env:TMP='C:/github/ecopunk/target/qa-tmp';$env:TEMP=$env:TMP
$env:POET_NATIVE_CHECKPOINT_TEST_ROOT='C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3/hmc/harness'
$core='C:/github/qualiaDB/Cargo.toml';$poet='C:/github/poet/Cargo.toml';$cache='C:/github/ecopunk/target'
$arguments=switch($Mode){
 Check {@('check','--offline','--locked','--manifest-path',$poet,'-p','poet-cli','--tests','--features','native-graphical','--target-dir',$cache)}
 Core {@('test','--offline','--locked','--manifest-path',$core,'-p','qualia-core-db','--lib','--no-default-features','--features','profile_target_1024,zk-culling,gpu-runtime,gpu-native-portable,gpu-native-dx12,wgsl-forge,privacy-he','--target-dir',$cache,'--','--test-threads=1')}
 Native {@('test','--offline','--locked','--manifest-path',$poet,'-p','poet-cli','--features','native-graphical','--target-dir',$cache,'--','--test-threads=1')}
 Wasm {@('check','--offline','--locked','--manifest-path',$poet,'-p','poet','--lib','--target','wasm32-unknown-unknown','--target-dir',$cache)}
 NativeLib {@('build','--offline','--locked','--manifest-path',$poet,'-p','poet-cli','--lib','--features','native-graphical','--message-format=json','--target-dir',$cache)}
}
$snapshot=Get-Content (Join-Path $PSScriptRoot 'latest-source-snapshot.json') -Raw|ConvertFrom-Json
if((Get-FileHash $snapshot.path -Algorithm SHA256).Hash -ne $snapshot.sha256){throw 'Invalid source snapshot'}
$started=[DateTime]::UtcNow.ToString('o')
& cargo @arguments *> (Join-Path $PSScriptRoot "$Label.log")
$code=$LASTEXITCODE
$sourceChanges=@();$sources=Get-Content $snapshot.path -Raw|ConvertFrom-Json
foreach($source in $sources.sources){if((Get-FileHash -LiteralPath $source.path -Algorithm SHA256).Hash -ne $source.sha256){$sourceChanges+=$source.path}}
[ordered]@{command='cargo';arguments=$arguments;exit_code=$code;input_snapshot=$snapshot;started_utc=$started;finished_utc=[DateTime]::UtcNow.ToString('o');source_changes_during_command=$sourceChanges;source_unchanged=($sourceChanges.Count -eq 0)}|ConvertTo-Json -Depth 7|Set-Content (Join-Path $PSScriptRoot "$Label.json")
if($Mode -ne 'NativeLib'){Get-Content (Join-Path $PSScriptRoot "$Label.log") -Tail 22}
else{"Native library build exit $code; see saved Cargo JSON log"}
exit $code
