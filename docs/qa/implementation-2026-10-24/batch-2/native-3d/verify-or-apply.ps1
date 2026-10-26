param([switch]$Apply)
$ErrorActionPreference = 'Stop'
$spec = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'apply-manifest.json') -Raw | ConvertFrom-Json
$allowed = @('studio_store.rs', 'studio_domains.rs', 'studio_journal.rs', 'studio_journal_tests.rs')
$owner = 'C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/'
foreach ($change in $spec.mutations) {
    $resolved = [IO.Path]::GetFullPath($change.target).Replace([char]92, [char]47)
    if (-not $allowed.Contains([IO.Path]::GetFileName($resolved)) -or
        -not $resolved.StartsWith($owner, [StringComparison]::OrdinalIgnoreCase) -or
        [IO.Path]::GetDirectoryName($resolved).Replace([char]92, [char]47) -ne $owner.TrimEnd('/')) {
        throw 'Target outside allocated Studio source scope'
    }
    if ($null -eq $change.original_sha256) {
        if (Test-Path -LiteralPath $resolved) { throw "New target already exists: $resolved" }
    } elseif ((Get-FileHash -LiteralPath $resolved -Algorithm SHA256).Hash -ne $change.original_sha256) {
        throw "Canonical source changed: $resolved"
    }
    $candidate = Join-Path $PSScriptRoot $change.candidate
    if ((Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -ne $change.candidate_sha256) {
        throw "Candidate changed: $candidate"
    }
}
if (-not $Apply) { 'All Studio source/candidate guards verified; no writes.'; return }
foreach ($change in $spec.mutations) {
    $candidate = Join-Path $PSScriptRoot $change.candidate
    [IO.File]::WriteAllBytes($change.target, [IO.File]::ReadAllBytes($candidate))
    if ((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.candidate_sha256) {
        throw 'Applied Studio source bytes mismatch'
    }
}
'Applied exact Studio journal repair; full current-core qualification remains separate.'
