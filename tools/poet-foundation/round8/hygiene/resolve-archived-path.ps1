[CmdletBinding()]
param([Parameter(Mandatory = $true)][string]$OriginalPath)

$ErrorActionPreference = 'Stop'
$repoRoot = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..\..\..\..'))
if (-not $repoRoot.Equals('C:\github\ecopunk', [StringComparison]::OrdinalIgnoreCase)) { throw "Unexpected repo root: $repoRoot" }
$indexPath = Join-Path $repoRoot 'audit\implementation-snapshots\2026-10-25\batch-6\migration-index.json'
$mapPath = Join-Path $PSScriptRoot 'relocation-dry-run.json'
$expectedMapSha256 = 'd193fa59715d41b741cf0118095c3b0749d5e901de01f4b46e24f15418a4901a'
if (-not (Test-Path -LiteralPath $indexPath -PathType Leaf)) { throw "Migration index not found: $indexPath" }
if ((Get-FileHash -LiteralPath $mapPath -Algorithm SHA256).Hash.ToLowerInvariant() -ne $expectedMapSha256) { throw 'Pinned dry-run map digest mismatch.' }
$index = Get-Content -LiteralPath $indexPath -Raw | ConvertFrom-Json
$map = @(Get-Content -LiteralPath $mapPath -Raw | ConvertFrom-Json)
$normalized = $OriginalPath.Replace('\', '/')
$prefix = 'docs/qa/implementation-2026-10-25/batch-6/rounds/'
if ($normalized -match '^round[1-7]/') { $normalized = $prefix + $normalized }
$row = $index.files | Where-Object { $_.original_path -ceq $normalized } | Select-Object -First 1
if (-not $row) { throw "Original path is not in the migration index: $normalized" }
$mapRelative = $normalized.Substring($prefix.Length)
$planned = $map | Where-Object { $_.original_path -ceq $mapRelative } | Select-Object -First 1
if (-not $planned -or $row.archived_path -cne $planned.proposed_destination -or [long]$row.bytes -ne [long]$planned.bytes -or [string]$row.sha256 -cne [string]$planned.sha256) {
    throw 'Migration index differs from the pinned source map.'
}
$repoPrefix = $repoRoot.TrimEnd('\') + '\'
$archivePath = [System.IO.Path]::GetFullPath((Join-Path $repoRoot ([string]$row.archived_path)))
if (-not $archivePath.StartsWith($repoPrefix, [StringComparison]::OrdinalIgnoreCase)) { throw 'Archived path escapes the Ecopunk repository.' }
$current = $archivePath
while ($current.StartsWith($repoRoot, [StringComparison]::OrdinalIgnoreCase)) {
    if (Test-Path -LiteralPath $current) {
        $item = Get-Item -LiteralPath $current -Force
        if (($item.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0) { throw "Refusing reparse point: $current" }
    }
    if ($current.Equals($repoRoot, [StringComparison]::OrdinalIgnoreCase)) { break }
    $current = Split-Path -Parent $current
}
if (-not (Test-Path -LiteralPath $archivePath -PathType Leaf)) { throw "Archived bytes are missing: $archivePath" }
$bytes = (Get-Item -LiteralPath $archivePath).Length
$digest = (Get-FileHash -LiteralPath $archivePath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($bytes -ne [long]$row.bytes -or $digest -ne [string]$row.sha256) { throw "Archived bytes fail migration-index verification: $archivePath" }
[pscustomobject]@{
    original_path = $row.original_path
    archived_path = $row.archived_path
    bytes = $bytes
    sha256 = $digest
    content_verified = $true
} | ConvertTo-Json -Depth 3
