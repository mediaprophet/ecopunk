param([Parameter(Mandatory)][ValidateSet('Check','Core','CoreFresh','CoreCheckpoint','Native','Wasm','NativeLib')][string]$Mode,[Parameter(Mandatory)][string]$Label,[ValidateSet('workspace-sandbox','native')][string]$ExecutionBoundary='workspace-sandbox')
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
 Core {@('test','--offline','--locked','--manifest-path',$core,'-p','qualia-core-db','--lib','--no-default-features','--features','profile_target_1024,zk-culling,gpu-runtime,gpu-native-portable,gpu-native-dx12,wgsl-forge,privacy-he,sanctuary-crypto','--target-dir',$cache,'--','--test-threads=1')}
 CoreFresh {@('test','--offline','--locked','--manifest-path',$core,'-p','qualia-core-db','--lib','--no-default-features','--features','profile_target_1024,zk-culling,gpu-runtime,gpu-native-portable,gpu-native-dx12,wgsl-forge,privacy-he,sanctuary-crypto','--target-dir',$cache,'--','vibe_host::native_authority::enrollment::tests::new_process_reopens_sealed_key_signed_admission_and_requires_fresh_login','--exact','--nocapture','--test-threads=1')}
 CoreCheckpoint {@('test','--offline','--locked','--manifest-path',$core,'-p','qualia-core-db','--lib','--no-default-features','--features','profile_target_1024,zk-culling,gpu-runtime,gpu-native-portable,gpu-native-dx12,wgsl-forge,privacy-he,sanctuary-crypto','--target-dir',$cache,'--','vibe_host::invoke::studio_store::checkpoint::','--nocapture','--test-threads=1')}
 Native {@('test','--offline','--locked','--manifest-path',$poet,'-p','poet-cli','--features','native-graphical','--target-dir',$cache,'--','--test-threads=1')}
 Wasm {@('check','--offline','--locked','--manifest-path',$poet,'-p','poet','--lib','--target','wasm32-unknown-unknown','--target-dir',$cache)}
 NativeLib {@('build','--offline','--locked','--manifest-path',$poet,'-p','poet-cli','--lib','--features','native-graphical','--message-format=json','--target-dir',$cache)}
}
$snapshot=Get-Content (Join-Path $PSScriptRoot 'latest-source-snapshot.json') -Raw|ConvertFrom-Json
if((Get-FileHash $snapshot.path -Algorithm SHA256).Hash -ne $snapshot.sha256){throw 'Invalid source snapshot'}
$started=[DateTime]::UtcNow.ToString('o')
& cargo @arguments *> (Join-Path $PSScriptRoot "$Label.log")
$code=$LASTEXITCODE
if($code -eq 0 -and $Mode -in @('CoreFresh','CoreCheckpoint')){
 $log=Get-Content (Join-Path $PSScriptRoot "$Label.log") -Raw
 if($Mode -eq 'CoreFresh' -and $log -notmatch '(?m)^test result: ok\. 1 passed; 0 failed;'){$code=101}
 if($Mode -eq 'CoreCheckpoint'){
  foreach($case in @('real_deny_delete_handle_release_allows_bounded_retry','permanent_deny_delete_refusal_preserves_destination_and_cleans_staging','changed_expected_bytes_stop_retry_before_replacement')){if($log -notmatch ([regex]::Escape($case+' ... ok'))){$code=101}}
 }
 if($code -ne 0){Add-Content (Join-Path $PSScriptRoot "$Label.log") 'Qualification failed: required focused cases did not pass.'}
}
$sourceChanges=@();$sources=Get-Content $snapshot.path -Raw|ConvertFrom-Json
foreach($source in $sources.sources){if((Get-FileHash -LiteralPath $source.path -Algorithm SHA256).Hash -ne $source.sha256){$sourceChanges+=$source.path}}
[ordered]@{command='cargo';arguments=$arguments;exit_code=$code;execution_boundary=$ExecutionBoundary;input_snapshot=$snapshot;started_utc=$started;finished_utc=[DateTime]::UtcNow.ToString('o');source_changes_during_command=$sourceChanges;source_unchanged=($sourceChanges.Count -eq 0)}|ConvertTo-Json -Depth 7|Set-Content (Join-Path $PSScriptRoot "$Label.json")
if($Mode -ne 'NativeLib'){Get-Content (Join-Path $PSScriptRoot "$Label.log") -Tail 22}
else{"Native library build exit $code; see saved Cargo JSON log"}
exit $code
