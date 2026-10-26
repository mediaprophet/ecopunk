param([switch]$Apply)
$ErrorActionPreference='Stop'
$spec=Get-Content -LiteralPath (Join-Path $PSScriptRoot 'apply-manifest.json') -Raw|ConvertFrom-Json
$paths=@('src/render/assets/glb_skins_anims.rs','src/render/assets/glb_rigid_skin.rs','src/render/assets/glb_rigid_animation.rs','src/render/assets/glb_rigid_tests.rs','src/render/texture_transcode.rs','src/render/texture_bc7_conformance_tests.rs')
if($spec.mutations.Count -ne $paths.Count){throw 'Unexpected GLB/BC7 target count'}
$seen=@{}
foreach($change in $spec.mutations){
 $relative=$change.target.Replace('C:/github/QualiaDB/crates/qualia-core-db/','')
 if($relative -notin $paths -or $change.target -ne "C:/github/QualiaDB/crates/qualia-core-db/$relative" -or $seen.ContainsKey($relative)){throw 'Unallocated or duplicate GLB/BC7 target'}
 $seen[$relative]=$true
 if($null -eq $change.original_sha256){if(Test-Path -LiteralPath $change.target){throw 'New GLB/BC7 target already exists'}}
 elseif((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.original_sha256){throw "Actual GLB/BC7 source changed: $relative"}
 $candidate=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot $change.candidate))
 $expected=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot "candidate/$relative"))
 if($candidate -ne $expected -or (Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -ne $change.candidate_sha256){throw 'Unexpected GLB/BC7 candidate/digest'}
}
if(-not $Apply){'All 6 GLB/BC7 actual/candidate guards verified; no writes.';return}
foreach($change in $spec.mutations){
 $mode=if($null -eq $change.original_sha256){[IO.FileMode]::CreateNew}else{[IO.FileMode]::Create}
 $stream=[IO.File]::Open($change.target,$mode,[IO.FileAccess]::Write)
 try{$bytes=[IO.File]::ReadAllBytes((Join-Path $PSScriptRoot $change.candidate));$stream.Write($bytes,0,$bytes.Length)}finally{$stream.Dispose()}
 if((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.candidate_sha256){throw 'Applied GLB/BC7 source mismatch'}
}
'Applied exact GLB production repair and BC7 precision regressions; coherent core/native tests remain required.'
