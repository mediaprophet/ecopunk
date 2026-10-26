param([switch]$Apply)
$ErrorActionPreference='Stop'
$expected=@{
 'patch-map.json'='741c51314b8aff6899247690e1c5fc02a49d0a40cfc1fb57c97b49cc896adb0d'
 'poet-patch-map.json'='61e1f173654fde85acc6b8d26b42334f200829ca70e3cf3618dbd81ca3d70e4e'
}
$changes=@()
foreach($file in $expected.Keys){
 $map=Join-Path $PSScriptRoot $file
 if((Get-FileHash -LiteralPath $map -Algorithm SHA256).Hash.ToLowerInvariant() -ne $expected[$file]){throw 'Frozen patch map changed'}
 $changes+=@(Get-Content -LiteralPath $map -Raw|ConvertFrom-Json)
}
foreach($change in $changes){
 $target=[IO.Path]::GetFullPath($change.target).Replace([char]92,[char]47)
 $canonical=$target.StartsWith('C:/github/qualiaDB/crates/vibe/',[StringComparison]::OrdinalIgnoreCase)
 $poet=$target -in @('C:/github/poet/native/src/main.rs','C:/github/poet/native/src/repl.rs')
 if(-not($canonical -or $poet)){throw "Target outside reviewed source scope: $target"}
 if($null -eq $change.original_sha256){
  if(Test-Path -LiteralPath $target){throw "New source target already exists: $target"}
 }else{
  if((Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash.ToLowerInvariant() -ne $change.original_sha256){throw "Original source changed: $target"}
  if((Get-FileHash -LiteralPath $change.original -Algorithm SHA256).Hash.ToLowerInvariant() -ne $change.original_sha256){throw 'Original snapshot mismatch'}
 }
 if((Get-FileHash -LiteralPath $change.candidate -Algorithm SHA256).Hash.ToLowerInvariant() -ne $change.candidate_sha256){throw "Frozen candidate mismatch: $target"}
}
if(-not $Apply){"Verified $($changes.Count) exact source/candidate guards; no writes.";return}
foreach($change in $changes){
 [IO.Directory]::CreateDirectory([IO.Path]::GetDirectoryName($change.target))|Out-Null
 [IO.File]::WriteAllBytes($change.target,[IO.File]::ReadAllBytes($change.candidate))
 if((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash.ToLowerInvariant() -ne $change.candidate_sha256){throw "Applied bytes mismatch: $($change.target)"}
}
"Applied $($changes.Count) guarded canonical Vibe/Poet source and fixture changes."