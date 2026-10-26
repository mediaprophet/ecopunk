# Historical RUN-05/06 initial-apply verifier. The later RUN-07 stamped-store
# addendum intentionally supersedes two hashes. Do not overwrite this baseline
# receipt to conceal that source transition; use the addendum's separate receipt.
$ErrorActionPreference = 'Stop'
$groups = @(
    @{ id='RUN-05A'; map='parser/apply-manifest.json'; wrapped=$true },
    @{ id='RUN-05B'; map='runtime/patch-map.json'; wrapped=$false },
    @{ id='RUN-05B-Poet'; map='runtime/poet-patch-map.json'; wrapped=$false },
    @{ id='RUN-06B'; map='native-3d/apply-manifest.json'; wrapped=$true }
)
$pins = @()
$sources = @()
foreach ($group in $groups) {
    $mapPath = Join-Path $PSScriptRoot $group.map
    $pins += @{path=$group.map; sha256=(Get-FileHash -LiteralPath $mapPath -Algorithm SHA256).Hash.ToLowerInvariant()}
    $map = Get-Content -LiteralPath $mapPath -Raw | ConvertFrom-Json
    $rows = if ($group.wrapped) { $map.mutations } else { $map }
    foreach ($row in $rows) {
        $actual = (Get-FileHash -LiteralPath $row.target -Algorithm SHA256).Hash.ToLowerInvariant()
        if ($actual -ne $row.candidate_sha256.ToLowerInvariant()) {
            throw "Applied source drift: $($row.target)"
        }
        $sources += @{job=$group.id; path=$row.target; sha256=$actual}
    }
}
$receipt = [ordered]@{
    format='ecopunk-applied-source-verification'
    date='2026-10-24'
    status='30 exact applied source files match reviewed candidate maps'
    source_count=$sources.Count
    source_maps=$pins
    sources=$sources
    qualification='Source-byte verification only; see separate command receipts for target checks and actual product acceptance.'
    human_acceptance_claimed=$false
}
if ($sources.Count -ne 30) { throw 'Unexpected applied source count' }
$outPath = Join-Path $PSScriptRoot 'applied-sources.json'
[IO.File]::WriteAllText($outPath, ($receipt | ConvertTo-Json -Depth 10) + [Environment]::NewLine, [Text.UTF8Encoding]::new($false))
Write-Output "Verified $($sources.Count) exact applied source files."
