param([switch]$Apply)
$ErrorActionPreference='Stop'
$spec=Get-Content -LiteralPath (Join-Path $PSScriptRoot 'apply-manifest.json') -Raw|ConvertFrom-Json
foreach($change in $spec.mutations){
 $resolved=[IO.Path]::GetFullPath($change.target)
 if(-not $resolved.Replace([char]92,[char]47).StartsWith('C:/github/qualiaDB/crates/vibe/',[StringComparison]::OrdinalIgnoreCase)){throw 'Target outside Vibe scope'}
 if($null -eq $change.original_sha256){
  if(Test-Path -LiteralPath $change.target){throw "New target already exists: $($change.target)"}
 } elseif((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.original_sha256){throw "Source changed: $($change.target)"}
 $candidate=Join-Path $PSScriptRoot $change.candidate
 if((Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -ne $change.candidate_sha256){throw 'Candidate changed'}
}
if(-not $Apply){'All parser source/candidate guards verified; no writes.';return}
foreach($change in $spec.mutations){
 [IO.File]::WriteAllBytes($change.target,[IO.File]::ReadAllBytes((Join-Path $PSScriptRoot $change.candidate)))
 if((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.candidate_sha256){throw 'Applied parser bytes mismatch'}
}
'Applied exact canonical parser repair and focused regressions.'
