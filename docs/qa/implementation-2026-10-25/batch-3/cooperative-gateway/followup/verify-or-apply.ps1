param([switch]$Apply)
$ErrorActionPreference = 'Stop'
$spec = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'apply-manifest.json') -Raw | ConvertFrom-Json
$allowed = @('tests.rs','mcp_tests.rs')
if ($spec.mutations.Count -ne 2) { throw 'Unexpected follow-up target count' }
if ((Get-FileHash -LiteralPath 'C:/github/poet/native/src/graphical/cooperation/mod.rs' -Algorithm SHA256).Hash -ne $spec.unchanged_mod_sha256) { throw 'Gateway implementation changed during follow-up' }
$seen = @{}
foreach ($change in $spec.mutations) {
 $name = [IO.Path]::GetFileName($change.target)
 if ($name -notin $allowed -or $change.target -ne "C:/github/poet/native/src/graphical/cooperation/$name" -or $seen.ContainsKey($name)) { throw 'Unallocated or duplicate follow-up target' }
 $seen[$name] = $true
 if ($null -eq $change.original_sha256) { if (Test-Path -LiteralPath $change.target) { throw 'New MCP integration test target already exists' } }
 elseif ((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.original_sha256) { throw 'Actual gateway tests changed' }
 $candidate = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot $change.candidate))
 $expected = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot "candidate/graphical/cooperation/$name"))
 if ($candidate -ne $expected -or (Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -ne $change.candidate_sha256) { throw 'Unexpected follow-up candidate or digest' }
}
if (-not $Apply) { 'Both follow-up test guards and unchanged gateway implementation verified; no writes.'; return }
foreach ($change in $spec.mutations) {
 $mode = if ($null -eq $change.original_sha256) { [IO.FileMode]::CreateNew } else { [IO.FileMode]::Create }
 $stream = [IO.File]::Open($change.target,$mode,[IO.FileAccess]::Write)
 try { $bytes=[IO.File]::ReadAllBytes((Join-Path $PSScriptRoot $change.candidate)); $stream.Write($bytes,0,$bytes.Length) } finally { $stream.Dispose() }
 if ((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.candidate_sha256) { throw 'Applied follow-up test mismatch' }
}
'Applied exact gateway test follow-up; coherent native tests remain required.'
