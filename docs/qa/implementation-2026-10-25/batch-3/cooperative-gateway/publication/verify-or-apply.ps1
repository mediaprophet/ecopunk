param([switch]$Apply)
$ErrorActionPreference='Stop'
$spec=Get-Content -LiteralPath (Join-Path $PSScriptRoot 'apply-manifest.json') -Raw|ConvertFrom-Json
$allowed=@('mod.rs','tests.rs','mcp_tests.rs')
if($spec.mutations.Count -ne 3){throw 'Unexpected publication target count'}
$seen=@{}
foreach($change in $spec.mutations){
 $name=[IO.Path]::GetFileName($change.target)
 if($name -notin $allowed -or $change.target -ne "C:/github/poet/native/src/graphical/cooperation/$name" -or $seen.ContainsKey($name)){throw 'Unallocated or duplicate publication target'}
 $seen[$name]=$true
 if($null -eq $change.original_sha256){if(Test-Path -LiteralPath $change.target){throw 'New publication MCP tests target already exists'}}
 elseif((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.original_sha256){throw "Actual publication source changed: $name"}
 $candidate=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot $change.candidate))
 $expected=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot "candidate/graphical/cooperation/$name"))
 if($candidate -ne $expected -or (Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -ne $change.candidate_sha256){throw 'Unexpected publication candidate/digest'}
}
if(-not $Apply){'All 3 publication actual/candidate guards verified; no writes.';return}
foreach($change in $spec.mutations){
 $mode=if($null -eq $change.original_sha256){[IO.FileMode]::CreateNew}else{[IO.FileMode]::Create}
 $stream=[IO.File]::Open($change.target,$mode,[IO.FileAccess]::Write)
 try{$bytes=[IO.File]::ReadAllBytes((Join-Path $PSScriptRoot $change.candidate));$stream.Write($bytes,0,$bytes.Length)}finally{$stream.Dispose()}
 if((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.candidate_sha256){throw 'Applied publication source mismatch'}
}
'Applied native publication stamp and protocol fixture follow-up; root snapshot/context/UI/MCP wiring and coherent checks remain required.'
