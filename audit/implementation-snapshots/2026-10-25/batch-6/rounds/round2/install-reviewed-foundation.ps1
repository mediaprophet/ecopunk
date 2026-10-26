$ErrorActionPreference = 'Stop'
$evidenceRoot = 'C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-6/rounds/round2'
$m64Manifest = Join-Path $evidenceRoot 'master/source-manifest.json'
$typedManifest = Join-Path $evidenceRoot 'surfaces/sdk-save-manifest.json'
if ((Get-FileHash -LiteralPath $m64Manifest -Algorithm SHA256).Hash -ne 'e3585147732bbf7089ce1b90a0e050c2dd51ab088a4181f9cbc6762ed8b0523a') { throw 'M64 review manifest changed' }
if ((Get-FileHash -LiteralPath $typedManifest -Algorithm SHA256).Hash -ne '7886467cd423a0645530fd2b0577a56b87c855845adf0ab22d2c517d161c0b58') { throw 'Typed payload review manifest changed' }
$items = @()
foreach ($entry in (Get-Content -LiteralPath $m64Manifest -Raw | ConvertFrom-Json)) {
    $isNew = $entry.before -eq 'before/new-file-empty'
    $items += [pscustomobject]@{ target=$entry.target; stage=(Join-Path "$evidenceRoot/master" $entry.after); before=$(if ($isNew) { $null } else { $entry.before_sha256 }); after=$entry.after_sha256 }
}
foreach ($entry in (Get-Content -LiteralPath $typedManifest -Raw | ConvertFrom-Json)) {
    $items += [pscustomobject]@{ target=$entry.path; stage=(Join-Path "$evidenceRoot/surfaces/after" $entry.relative); before=$entry.before_sha256; after=$entry.after_sha256 }
}
if ($items.Count -ne 17) { throw 'Unexpected reviewed target count' }
$allowedRoots = @('C:\github\qualiaDB\crates\qualia-core-db\', 'C:\github\qualiaDB\crates\qualia-geospatial\')
$seen = [Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
foreach ($item in $items) {
    $resolvedTarget = [IO.Path]::GetFullPath($item.target)
    if (-not ($allowedRoots | Where-Object { $resolvedTarget.StartsWith($_, [StringComparison]::OrdinalIgnoreCase) })) { throw "Target outside reviewed repositories: $resolvedTarget" }
    if (-not $seen.Add($resolvedTarget)) { throw "Duplicate target: $resolvedTarget" }
    if ((Get-FileHash -LiteralPath $item.stage -Algorithm SHA256).Hash -ne $item.after) { throw "Candidate changed: $($item.stage)" }
    if ($null -eq $item.before) {
        if (Test-Path -LiteralPath $item.target) { throw "New-file destination already exists: $($item.target)" }
    } elseif (-not (Test-Path -LiteralPath $item.target)) {
        throw "Original file missing: $($item.target)"
    } elseif ((Get-FileHash -LiteralPath $item.target -Algorithm SHA256).Hash -ne $item.before) {
        throw "User source changed since review: $($item.target)"
    }
}
$receipt = @()
# All seventeen preimages and candidates have passed before any live write.
foreach ($item in ($items | Sort-Object @{ Expression={ $null -ne $_.before } })) {
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $item.target) | Out-Null
    Copy-Item -LiteralPath $item.stage -Destination $item.target
    $savedHash = (Get-FileHash -LiteralPath $item.target -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($savedHash -ne $item.after) { throw "Saved bytes differ: $($item.target)" }
    $receipt += [pscustomobject]@{ path=$item.target; before_sha256=$item.before; after_sha256=$savedHash }
    $receipt | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath "$evidenceRoot/applied-source.json" -Encoding utf8
}
Write-Output "Applied and hash-verified $($receipt.Count) reviewed SDK source/test files. No build, asset, commit or publication performed."
