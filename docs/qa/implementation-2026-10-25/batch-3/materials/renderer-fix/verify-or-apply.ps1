param([switch]$Apply)
$ErrorActionPreference = 'Stop'
$spec = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'apply-manifest.json') -Raw | ConvertFrom-Json
$allowed = @{
 'C:/github/QualiaDB/crates/qualia-core-db/src/shaders/viewport/mesh.wgsl' = 'candidate/src/shaders/viewport/mesh.wgsl'
 'C:/github/QualiaDB/crates/qualia-core-db/src/render/gpu/material_textures.rs' = 'candidate/src/render/gpu/material_textures.rs'
}
if ($spec.mutations.Count -ne $allowed.Count) { throw 'Unexpected renderer target count' }
$seen = @{}
foreach ($change in $spec.mutations) {
 $target = [IO.Path]::GetFullPath($change.target).Replace([char]92,[char]47)
 if (-not $allowed.ContainsKey($target) -or $seen.ContainsKey($target)) { throw 'Unallocated or duplicate renderer target' }
 $seen[$target] = $true
 if ((Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash -ne $change.original_sha256) { throw "Actual renderer source changed: $target" }
 $candidate = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot $change.candidate))
 $expected = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot $allowed[$target]))
 if ($candidate -ne $expected) { throw 'Candidate differs from exact renderer owned path' }
 if ((Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -ne $change.candidate_sha256) { throw "Renderer candidate changed: $candidate" }
}
if (-not $Apply) { 'Both actual/candidate renderer guards verified; no writes.'; return }
foreach ($change in $spec.mutations) {
 [IO.File]::WriteAllBytes($change.target,[IO.File]::ReadAllBytes((Join-Path $PSScriptRoot $change.candidate)))
 if ((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.candidate_sha256) { throw 'Applied renderer source mismatch' }
}
'Applied exact unbound metallic-roughness correction; actual core rebuild and GPU regression remain required.'
