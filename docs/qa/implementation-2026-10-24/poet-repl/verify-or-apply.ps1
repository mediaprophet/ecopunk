param([switch]$Apply)

$ErrorActionPreference = 'Stop'
$run03Manifest = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'source-guards.json') -Raw | ConvertFrom-Json
foreach ($run03Guard in $run03Manifest.source_guards) {
    $run03Actual = (Get-FileHash -LiteralPath $run03Guard.path -Algorithm SHA256).Hash
    if ($run03Actual -ne $run03Guard.sha256) {
        throw "Source changed; review/rebase required: $($run03Guard.path)"
    }
}
$run03Candidate = Join-Path $PSScriptRoot 'candidate/native/src/repl.rs'
if ((Get-FileHash -LiteralPath $run03Candidate -Algorithm SHA256).Hash -ne $run03Manifest.candidate_sha256) {
    throw 'Candidate bytes changed; review and regenerate guards before applying.'
}
if ($Apply) {
    # This deliberate opt-in still requires real filesystem authority for Poet.
    # The original bytes are preserved at repl.original.rs beside this script.
    [IO.File]::WriteAllBytes($run03Manifest.target, [IO.File]::ReadAllBytes($run03Candidate))
    if ((Get-FileHash -LiteralPath $run03Manifest.target -Algorithm SHA256).Hash -ne $run03Manifest.candidate_sha256) {
        throw 'Applied file failed its byte-integrity check.'
    }
    Write-Output "Applied exact guarded candidate: $($run03Manifest.target)"
} else {
    Write-Output 'All recorded source and candidate guards match. Candidate remains staged, not applied.'
}
