param([switch]$Apply)
$ErrorActionPreference='Stop'
$owner='C:/github/QualiaDB/crates/qualia-core-db/src/poet_host/invoke/'
$allowed=@('studio_store.rs','studio_journal.rs','studio_stamped.rs','studio_checkpoint/mod.rs','studio_checkpoint/snapshot.rs','studio_checkpoint/codec.rs','studio_checkpoint/graph.rs','studio_checkpoint/native_io.rs','studio_checkpoint/tests.rs')
$spec=Get-Content -LiteralPath (Join-Path $PSScriptRoot 'apply-manifest.json') -Raw | ConvertFrom-Json
if($spec.mutations.Count -ne $allowed.Count){throw 'Unexpected mutation count'}
$seen=@{}
foreach($change in $spec.mutations){
 $target=[IO.Path]::GetFullPath($change.target).Replace([char]92,[char]47)
 if(-not $target.StartsWith($owner,[StringComparison]::OrdinalIgnoreCase)){throw 'Target outside allocated store owner'}
 $relative=$target.Substring($owner.Length)
 if(-not $allowed.Contains($relative) -or $seen.ContainsKey($relative)){throw 'Unallocated/duplicate target'}
 $seen[$relative]=$true
 if($null -eq $change.original_sha256){
  if(Test-Path -LiteralPath $target){throw "New target already exists: $target"}
 }elseif((Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash -ne $change.original_sha256){throw "Canonical source changed: $target"}
 $candidate=Join-Path $PSScriptRoot $change.candidate
 $expected=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot "candidate/$relative"))
 if([IO.Path]::GetFullPath($candidate) -ne $expected){throw 'Candidate outside exact owned path'}
 if((Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -ne $change.candidate_sha256){throw "Candidate changed: $candidate"}
}
if(-not $Apply){'All 9 source/candidate guards verified; no writes.';return}
foreach($change in $spec.mutations){
 [IO.Directory]::CreateDirectory([IO.Path]::GetDirectoryName($change.target)) | Out-Null
 [IO.File]::WriteAllBytes($change.target,[IO.File]::ReadAllBytes((Join-Path $PSScriptRoot $change.candidate)))
 if((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.candidate_sha256){throw 'Applied bytes mismatch'}
}
'Applied exact native checkpoint store package; actual product linkage and cooperative/native acceptance remain separate.'
