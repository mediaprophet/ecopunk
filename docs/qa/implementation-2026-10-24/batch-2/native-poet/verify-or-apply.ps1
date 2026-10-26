param([switch]$Apply)
$ErrorActionPreference = 'Stop'
$spec = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'apply-manifest.json') -Raw | ConvertFrom-Json
$allowed = @('Cargo.toml','Cargo.lock','native/Cargo.toml','native/src/main.rs','wasm/Cargo.toml',
 'native/src/graphical/controller.rs','native/src/graphical/controller_tests.rs','native/src/graphical/execution.rs',
 'native/src/graphical/entry.rs','native/src/graphical/geometry_host.rs','native/src/graphical/preview.rs',
 'native/src/graphical/mod.rs','native/src/graphical/jobs.rs','native/src/graphical/jobs_tests.rs','native/src/graphical/ui.rs')
if ($spec.mutations.Count -ne $allowed.Count) { throw 'Unexpected mutation count' }
$seen = @{}
foreach ($change in $spec.mutations) {
 $target = [IO.Path]::GetFullPath($change.target).Replace([char]92,[char]47)
 $owner = 'C:/github/poet/'
 if (-not $target.StartsWith($owner,[StringComparison]::OrdinalIgnoreCase)) { throw 'Target outside Poet' }
 $relative = $target.Substring($owner.Length)
 if (-not $allowed.Contains($relative) -or $seen.ContainsKey($relative)) { throw 'Unallocated or duplicate target' }
 $seen[$relative] = $true
 if ($null -eq $change.original_sha256) {
  if (Test-Path -LiteralPath $target) { throw "New target already exists: $target" }
 } elseif ((Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash -ne $change.original_sha256) {
  throw "Canonical source changed: $target"
 }
 $candidate = Join-Path $PSScriptRoot $change.candidate
 $expected = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot "candidate/$relative"))
 if ([IO.Path]::GetFullPath($candidate) -ne $expected) { throw 'Candidate outside exact owned source path' }
 if ((Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -ne $change.candidate_sha256) { throw "Candidate changed: $candidate" }
}
if (-not $Apply) { 'All 15 Poet source/candidate guards verified; no writes.'; return }
foreach ($change in $spec.mutations) {
 [IO.Directory]::CreateDirectory([IO.Path]::GetDirectoryName($change.target)) | Out-Null
 [IO.File]::WriteAllBytes($change.target,[IO.File]::ReadAllBytes((Join-Path $PSScriptRoot $change.candidate)))
 if ((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.candidate_sha256) { throw 'Applied source bytes mismatch' }
}
'Applied exact optional native Poet source package; build/window qualification remains pending.'
