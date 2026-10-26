param([switch]$Apply)
$ErrorActionPreference='Stop'
$spec=Get-Content -LiteralPath (Join-Path $PSScriptRoot 'apply-manifest.json') -Raw|ConvertFrom-Json
$paths=@('src/render/gpu/ao.rs','src/render/gpu/ao_depth.rs','src/render/gpu/ao_math.rs','src/render/gpu/material_pixel_tests.rs','src/shaders/viewport/ao_prepass.wgsl','src/shaders/viewport/ao_generate.wgsl','src/shaders/viewport/mesh.wgsl')
if($spec.mutations.Count -ne $paths.Count){throw 'Unexpected AO precision target count'}
$seen=@{}
foreach($change in $spec.mutations){
 $relative=$change.target.Replace('C:/github/QualiaDB/crates/qualia-core-db/','')
 if($relative -notin $paths -or $change.target -ne "C:/github/QualiaDB/crates/qualia-core-db/$relative" -or $seen.ContainsKey($relative)){throw 'Unallocated or duplicate AO precision target'}
 $seen[$relative]=$true
 if($null -eq $change.original_sha256){if(Test-Path -LiteralPath $change.target){throw 'New AO precision target already exists'}}
 elseif((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.original_sha256){throw "Actual AO source changed: $relative"}
 $candidate=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot $change.candidate))
 $expected=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot "candidate/$relative"))
 if($candidate -ne $expected -or (Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -ne $change.candidate_sha256){throw 'Unexpected AO candidate/digest'}
}
if(-not $Apply){'All 7 AO precision actual/candidate guards verified; no writes.';return}
foreach($change in $spec.mutations){
 $mode=if($null -eq $change.original_sha256){[IO.FileMode]::CreateNew}else{[IO.FileMode]::Create}
 $stream=[IO.File]::Open($change.target,$mode,[IO.FileAccess]::Write)
 try{$bytes=[IO.File]::ReadAllBytes((Join-Path $PSScriptRoot $change.candidate));$stream.Write($bytes,0,$bytes.Length)}finally{$stream.Dispose()}
 if((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.candidate_sha256){throw 'Applied AO source mismatch'}
}
'Applied exact internal AO precision proposal; coherent core/Poet build and unchanged/strengthened GPU qualification remain required.'
