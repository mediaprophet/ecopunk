param([switch]$Apply)
$ErrorActionPreference = 'Stop'
$spec = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'apply-manifest.json') -Raw | ConvertFrom-Json
$allowed = @{
 'C:/github/poet/native/Cargo.toml' = 'candidate/native/Cargo.toml'
 'C:/github/poet/Cargo.lock' = 'candidate/Cargo.lock'
 'C:/github/poet/native/src/graphical/ui.rs' = 'candidate/native/src/graphical/ui.rs'
 'C:/github/rust-app-framework/packages/native-dom/src/mutation_writer.rs' = 'candidate/aether/packages/native-dom/src/mutation_writer.rs'
 'C:/github/rust-app-framework/packages/native-dom/src/textarea_value.rs' = 'candidate/aether/packages/native-dom/src/textarea_value.rs'
}
if ($spec.mutations.Count -ne $allowed.Count) { throw 'Unexpected UI5 target count' }
$seen = @{}
foreach ($change in $spec.mutations) {
 $target = [IO.Path]::GetFullPath($change.target).Replace([char]92,[char]47)
 if (-not $allowed.ContainsKey($target) -or $seen.ContainsKey($target)) { throw 'Unallocated or duplicate UI5 target' }
 $seen[$target] = $true
 if ($null -eq $change.original_sha256) {
  if (Test-Path -LiteralPath $target) { throw "New target already exists: $target" }
 } elseif ((Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash -ne $change.original_sha256) {
  throw "Actual source changed: $target"
 }
 $candidate = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot $change.candidate))
 $expected = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot $allowed[$target]))
 if ($candidate -ne $expected) { throw 'Candidate differs from exact owned path' }
 if ((Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -ne $change.candidate_sha256) { throw "Candidate changed: $candidate" }
}
if (-not $Apply) { 'All 5 UI5 actual/candidate guards verified; no writes.'; return }
foreach ($change in $spec.mutations) {
 [IO.File]::WriteAllBytes($change.target,[IO.File]::ReadAllBytes((Join-Path $PSScriptRoot $change.candidate)))
 if ((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.candidate_sha256) { throw 'Applied source mismatch' }
}
'Applied exact 5-target UI5 profile/layout/Aether proposal; actual tests/window checks remain required.'
