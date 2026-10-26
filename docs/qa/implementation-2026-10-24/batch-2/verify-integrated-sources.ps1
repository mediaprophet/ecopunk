param([string[]]$AdditionalMaps = @())
$ErrorActionPreference = 'Stop'
$maps = @(
    'parser/apply-manifest.json',
    'runtime/patch-map.json',
    'runtime/poet-patch-map.json',
    'native-3d/apply-manifest.json',
    'native-poet/store-addendum/apply-manifest.json',
    'native-poet/store-publication/apply-manifest.json'
) + $AdditionalMaps
$latest = [Collections.Generic.Dictionary[string,object]]::new([StringComparer]::OrdinalIgnoreCase)
$mapPins = @()
$transitions = 0
foreach ($relative in $maps) {
    $mapPath = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot $relative))
    $inside = $PSScriptRoot.TrimEnd([char]92,[char]47) + [IO.Path]::DirectorySeparatorChar
    if (-not $mapPath.StartsWith($inside,[StringComparison]::OrdinalIgnoreCase)) { throw 'Map outside owned evidence directory' }
    $mapPins += @{path=$relative; sha256=(Get-FileHash -LiteralPath $mapPath -Algorithm SHA256).Hash.ToLowerInvariant()}
    $map = Get-Content -LiteralPath $mapPath -Raw | ConvertFrom-Json
    $rows = if ($map -is [Array]) { $map } elseif (-not [string]::IsNullOrWhiteSpace($map.target)) { @($map) } else { $map.mutations }
    foreach ($row in $rows) {
        if ([string]::IsNullOrWhiteSpace($row.target)) { throw "Missing target in source map: $relative" }
        $target = [IO.Path]::GetFullPath($row.target)
        if ($latest.ContainsKey($target) -and $latest[$target].sha256 -ne $row.original_sha256.ToLowerInvariant()) {
            throw "Broken reviewed source transition: $target"
        }
        $latest[$target] = @{path=$target; sha256=$row.candidate_sha256.ToLowerInvariant(); final_map=$relative}
        $transitions++
    }
}
foreach ($source in $latest.Values) {
    if ((Get-FileHash -LiteralPath $source.path -Algorithm SHA256).Hash.ToLowerInvariant() -ne $source.sha256) {
        throw "Integrated source differs from reviewed maps: $($source.path)"
    }
}
$receipt = [ordered]@{
    date='2026-10-24'; status='Integrated source bytes and sequential reviewed transitions verified'
    unique_sources=$latest.Count; source_transitions=$transitions; maps=$mapPins
    sources=@($latest.Values | Sort-Object path)
    qualification='Source-byte verification only; command/build/window receipts and product/model acceptance are separate.'
    human_acceptance_claimed=$false
}
$currentPath = Join-Path $PSScriptRoot 'integrated-sources.json'
[IO.File]::WriteAllText($currentPath,($receipt|ConvertTo-Json -Depth 10)+[Environment]::NewLine,[Text.UTF8Encoding]::new($false))
$digest = (Get-FileHash -LiteralPath $currentPath -Algorithm SHA256).Hash.ToLowerInvariant()
$snapshotDirectory = Join-Path $PSScriptRoot 'source-snapshots'
[IO.Directory]::CreateDirectory($snapshotDirectory) | Out-Null
$snapshotPath = Join-Path $snapshotDirectory ($digest + '.json')
if (Test-Path -LiteralPath $snapshotPath) {
    if ((Get-FileHash -LiteralPath $snapshotPath -Algorithm SHA256).Hash.ToLowerInvariant() -ne $digest) { throw 'Source snapshot digest conflict' }
} else {
    [IO.File]::WriteAllBytes($snapshotPath,[IO.File]::ReadAllBytes($currentPath))
}
Write-Output "Verified $($latest.Count) current source files across $transitions reviewed transitions."
