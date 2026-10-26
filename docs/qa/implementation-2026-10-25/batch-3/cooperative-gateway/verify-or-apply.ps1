param([switch]$Apply)
$ErrorActionPreference = 'Stop'
$spec = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'apply-manifest.json') -Raw | ConvertFrom-Json
$names = @('mod.rs','tests.rs')
if ($spec.mutations.Count -ne $names.Count) { throw 'Unexpected gateway target count' }
$seen = @{}
foreach ($change in $spec.mutations) {
 $target = [IO.Path]::GetFullPath($change.target).Replace([char]92,[char]47)
 $name = [IO.Path]::GetFileName($target)
 if ($name -notin $names -or $target -ne "C:/github/poet/native/src/graphical/cooperation/$name" -or $seen.ContainsKey($name)) { throw 'Unallocated or duplicate gateway target' }
 $seen[$name] = $true
 if ($null -ne $change.original_sha256 -or (Test-Path -LiteralPath $target)) { throw "Gateway new target already exists or has unexpected original digest: $target" }
 $candidate = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot $change.candidate))
 $expected = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot "candidate/graphical/cooperation/$name"))
 if ($candidate -ne $expected) { throw 'Gateway candidate differs from exact owned path' }
 if ((Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -ne $change.candidate_sha256) { throw "Gateway candidate changed: $candidate" }
}
if (-not $Apply) { 'Both new gateway candidate guards verified; no writes.'; return }
$directory = 'C:/github/poet/native/src/graphical/cooperation'
if (-not (Test-Path -LiteralPath $directory)) { [IO.Directory]::CreateDirectory($directory) | Out-Null }
foreach ($change in $spec.mutations) {
 $stream = [IO.File]::Open($change.target, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write)
 try { $bytes = [IO.File]::ReadAllBytes((Join-Path $PSScriptRoot $change.candidate)); $stream.Write($bytes, 0, $bytes.Length) } finally { $stream.Dispose() }
 if ((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.candidate_sha256) { throw 'Applied gateway source mismatch' }
}
'Applied exact gateway proposal; native routing/dependencies/provider/coherent checks remain required.'
