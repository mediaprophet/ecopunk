param([Parameter(Mandatory)][ValidateSet('sandbox','native')][string]$Boundary)
$ErrorActionPreference='Stop'
$snapshot=Get-Content (Join-Path $PSScriptRoot 'latest-source-snapshot.json') -Raw|ConvertFrom-Json
$sources=Get-Content $snapshot.path -Raw|ConvertFrom-Json
function Assert-Sources {
 if((Get-FileHash $snapshot.path -Algorithm SHA256).Hash -ne $snapshot.sha256){throw 'Snapshot changed'}
 foreach($source in $sources.sources){if((Get-FileHash $source.path -Algorithm SHA256).Hash -ne $source.sha256){throw "Source changed: $($source.path)"}}
}
Assert-Sources
$binary='C:/github/ecopunk/target/debug/deps/qualia_core_db-965ec002a98bf526.exe'
$digest=(Get-FileHash $binary -Algorithm SHA256).Hash
$arguments=@('p2p::social_webnet::tests::two_meshes_exchange_an_application_datagram','--exact','--nocapture','--test-threads=1')
$log=Join-Path $PSScriptRoot "network-$Boundary-proof.log"
$started=[DateTime]::UtcNow.ToString('o')
& $binary @arguments *> $log
$code=$LASTEXITCODE
Assert-Sources
if((Get-FileHash $binary -Algorithm SHA256).Hash -ne $digest){throw 'Test executable changed'}
[ordered]@{binary=$binary;sha256=$digest;arguments=$arguments;boundary=$Boundary;exit_code=$code;started_utc=$started;finished_utc=[DateTime]::UtcNow.ToString('o');input_snapshot=$snapshot;source_unchanged=$true;log_sha256=(Get-FileHash $log -Algorithm SHA256).Hash;scope='Exact local loopback test, unchanged binary/sources; boundary selected by supervising tool invocation, not inferred from success'}|ConvertTo-Json -Depth 7|Set-Content (Join-Path $PSScriptRoot "network-$Boundary-proof.json")
Get-Content $log
exit $code
