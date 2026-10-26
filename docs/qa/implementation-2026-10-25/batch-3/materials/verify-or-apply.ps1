param([switch]$Apply)
$ErrorActionPreference = 'Stop'
$spec = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'apply-manifest.json') -Raw | ConvertFrom-Json
$names = @('entry.rs','material_controls.rs','material_pixels.rs','materials.rs','materials_tests.rs','mod.rs','preview.rs','ui.rs')
if ($spec.mutations.Count -ne $names.Count) { throw 'Unexpected material target count' }
$seen = @{}
foreach ($change in $spec.mutations) {
    $target = [IO.Path]::GetFullPath($change.target).Replace([char]92,[char]47)
    $name = [IO.Path]::GetFileName($target)
    if ($name -notin $names -or $target -ne "C:/github/poet/native/src/graphical/$name" -or $seen.ContainsKey($name)) { throw 'Unallocated or duplicate material target' }
    $seen[$name] = $true
    if ($null -eq $change.original_sha256) {
        if (Test-Path -LiteralPath $target) { throw "New target already exists: $target" }
    } elseif ((Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash -ne $change.original_sha256) { throw "Actual source changed: $target" }
    $candidate = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot $change.candidate))
    $expected = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot "candidate/graphical/$name"))
    if ($candidate -ne $expected) { throw 'Candidate differs from exact owned path' }
    if ((Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -ne $change.candidate_sha256) { throw "Candidate changed: $candidate" }
}
if (-not $Apply) { 'All 8 material proposal guards verified; no writes.'; return }
foreach ($change in $spec.mutations) {
    [IO.File]::WriteAllBytes($change.target,[IO.File]::ReadAllBytes((Join-Path $PSScriptRoot $change.candidate)))
    if ((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.candidate_sha256) { throw 'Applied material source mismatch' }
}
'Applied exact material proposal files; controller/source admission and coherent native tests remain required.'
