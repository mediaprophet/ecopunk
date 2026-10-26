[CmdletBinding()]
param([switch]$Apply)

$ErrorActionPreference = 'Stop'
$expectedCount = 733
$expectedMapSha256 = 'd193fa59715d41b741cf0118095c3b0749d5e901de01f4b46e24f15418a4901a'
if ($PSVersionTable.PSVersion.Major -ge 7) { $PSNativeCommandUseErrorActionPreference = $false }
$repoRoot = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..\..\..\..'))
if (-not $repoRoot.Equals('C:\github\ecopunk', [StringComparison]::OrdinalIgnoreCase)) {
    throw "Refusing: expected C:\github\ecopunk, got $repoRoot"
}
$mapPath = Join-Path $PSScriptRoot 'relocation-dry-run.json'
$sourceRoot = Join-Path $repoRoot 'docs\qa\implementation-2026-10-25\batch-6\rounds'
$indexPath = Join-Path $repoRoot 'audit\implementation-snapshots\2026-10-25\batch-6\migration-index.json'

function Resolve-ContainedPath([string]$Root, [string]$RelativePath) {
    $rootFull = [System.IO.Path]::GetFullPath($Root).TrimEnd('\') + '\'
    $candidate = [System.IO.Path]::GetFullPath((Join-Path $Root $RelativePath))
    if (-not $candidate.StartsWith($rootFull, [StringComparison]::OrdinalIgnoreCase)) {
        throw "Path escapes root: $RelativePath"
    }
    return $candidate
}

function Assert-NoReparsePath([string]$Path, [string]$LimitRoot) {
    $limit = [System.IO.Path]::GetFullPath($LimitRoot).TrimEnd('\')
    $current = [System.IO.Path]::GetFullPath($Path)
    while ($current.StartsWith($limit, [StringComparison]::OrdinalIgnoreCase)) {
        if (Test-Path -LiteralPath $current) {
            $item = Get-Item -LiteralPath $current -Force
            if (($item.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0) {
                throw "Refusing reparse point in path: $current"
            }
        }
        if ($current.Equals($limit, [StringComparison]::OrdinalIgnoreCase)) { break }
        $current = Split-Path -Parent $current
    }
}

function Get-Sha256([string]$Path) {
    return (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
}

function Get-ReferenceDiagnosis($Entries, [string]$Root) {
    $patterns = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
    foreach ($entry in $Entries) {
        foreach ($relative in @([string]$entry.original_path, [string]$entry.proposed_destination)) {
            $slash = $relative.Replace('\', '/')
            foreach ($term in @($slash, $slash.Replace('/', '\'))) { if ($term) { $patterns.Add($term) | Out-Null } }
            $parent = Split-Path $slash -Parent
            while ($parent -and $parent -notmatch '^(round[1-7]|audit)$') {
                foreach ($term in @($parent, $parent.Replace('/', '\'))) { $patterns.Add($term) | Out-Null }
                $parent = Split-Path $parent -Parent
            }
            $repoRelative = if ($relative -like 'audit/*') { $relative } else { 'docs/qa/implementation-2026-10-25/batch-6/rounds/' + $relative }
            foreach ($prefix in @('C:/github/ecopunk/', 'C:\github\ecopunk\')) {
                $absolute = $prefix + $repoRelative.Replace('\', '/')
                $patterns.Add($absolute) | Out-Null
                $patterns.Add($absolute.Replace('/', '\')) | Out-Null
            }
        }
    }

    $tempPatterns = New-TemporaryFile
    try {
        $patterns | Set-Content -LiteralPath $tempPatterns.FullName -Encoding utf8
        Push-Location $Root
        try {
            $raw = @(& rg -n -F --hidden --no-ignore -f $tempPatterns.FullName --glob '!.git/**' --glob '!**/target/**' --glob '!tools/poet-foundation/round8/hygiene/**' . 2>&1)
            $rgExit = $LASTEXITCODE
        } finally { Pop-Location }
        if ($rgExit -gt 1) { throw "ripgrep failed ($rgExit): $($raw -join [Environment]::NewLine)" }

        $mapped = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
        foreach ($entry in $Entries) {
            $mapped.Add(('docs/qa/implementation-2026-10-25/batch-6/rounds/' + $entry.original_path).Replace('/', '\')) | Out-Null
            $mapped.Add(([string]$entry.proposed_destination).Replace('/', '\')) | Out-Null
        }
        $references = @()
        foreach ($line in $raw) {
            $parts = ([string]$line).Split(':', 3)
            if ($parts.Count -lt 3) { continue }
            $file = $parts[0].TrimStart([char[]]@('.', '\', '/')).Replace('/', '\')
            if ($mapped.Contains($file)) { continue }
            $references += [pscustomobject]@{ path = $file; line = [int]$parts[1]; text = $parts[2] }
        }
        $serialized = ConvertTo-Json -InputObject $references -Depth 4 -Compress
        $sha = [System.Security.Cryptography.SHA256]::Create()
        try { $digest = ([BitConverter]::ToString($sha.ComputeHash([System.Text.Encoding]::UTF8.GetBytes($serialized)))).Replace('-', '').ToLowerInvariant() }
        finally { $sha.Dispose() }
        return [pscustomobject]@{ matching_references = $references.Count; reference_sha256 = $digest; files = @($references.path | Sort-Object -Unique); references = $references }
    } finally {
        Remove-Item -LiteralPath $tempPatterns.FullName -Force -ErrorAction SilentlyContinue
    }
}

if ((Get-Sha256 $mapPath) -ne $expectedMapSha256) { throw 'Relocation dry-run map digest changed; review and pin a new map before proceeding.' }
$entries = @(Get-Content -LiteralPath $mapPath -Raw | ConvertFrom-Json)
if ($entries.Count -ne $expectedCount) { throw "Expected $expectedCount map entries, found $($entries.Count)" }
if (Test-Path -LiteralPath $indexPath) { throw "Migration index already exists: $indexPath" }

$seenSources = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
$seenDestinations = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
$plan = @()
foreach ($entry in $entries) {
    $sourceRelative = [string]$entry.original_path
    $destinationRelative = [string]$entry.proposed_destination
    if ($sourceRelative -notmatch '^round[1-7]/') { throw "Invalid source map path: $sourceRelative" }
    if (-not $destinationRelative.StartsWith('audit/implementation-snapshots/2026-10-25/batch-6/rounds/', [StringComparison]::OrdinalIgnoreCase)) { throw "Invalid destination map path: $destinationRelative" }
    if (-not $seenSources.Add($sourceRelative)) { throw "Duplicate source path: $sourceRelative" }
    if (-not $seenDestinations.Add($destinationRelative)) { throw "Duplicate destination path: $destinationRelative" }
    $source = Resolve-ContainedPath $sourceRoot $sourceRelative
    $destination = Resolve-ContainedPath $repoRoot $destinationRelative
    Assert-NoReparsePath $source $repoRoot
    Assert-NoReparsePath $destination $repoRoot
    if (-not (Test-Path -LiteralPath $source -PathType Leaf)) { throw "Source is missing: $source" }
    if (Test-Path -LiteralPath $destination) { throw "Destination already exists: $destination" }
    if ((Get-Item -LiteralPath $source).Length -ne [long]$entry.bytes) { throw "Source byte count changed: $source" }
    if ((Get-Sha256 $source) -ne [string]$entry.sha256) { throw "Source digest changed: $source" }
    $plan += [pscustomobject]@{ entry = $entry; source = $source; destination = $destination }
}

$beforeReferences = Get-ReferenceDiagnosis $entries $repoRoot
Write-Output "Preflight passed: $($plan.Count) exact source paths/hashes; every mapped destination is absent."
Write-Output "Before-move path-reference hits: $($beforeReferences.matching_references) in $($beforeReferences.files.Count) files; digest $($beforeReferences.reference_sha256)."
if (-not $Apply) {
    Write-Output 'Dry run only. Pass -Apply after independent review to copy, verify, then unlink exact mapped files.'
    return
}

$createdDestinations = [System.Collections.Generic.List[object]]::new()
$temporaryFiles = [System.Collections.Generic.List[string]]::new()
$removedSources = [System.Collections.Generic.List[object]]::new()
$indexTemp = $null
try {
    foreach ($item in $plan) {
        $parent = Split-Path -Parent $item.destination
        if (-not (Test-Path -LiteralPath $parent)) { New-Item -ItemType Directory -Path $parent -Force | Out-Null }
        Assert-NoReparsePath $parent $repoRoot
        $copyTemp = Join-Path $parent ('.' + [System.IO.Path]::GetFileName($item.destination) + '.' + [guid]::NewGuid().ToString('N') + '.tmp')
        $temporaryFiles.Add($copyTemp)
        Copy-Item -LiteralPath $item.source -Destination $copyTemp
        if ((Get-Item -LiteralPath $copyTemp).Length -ne [long]$item.entry.bytes -or (Get-Sha256 $copyTemp) -ne [string]$item.entry.sha256) { throw "Temporary copy verification failed: $copyTemp" }
        Move-Item -LiteralPath $copyTemp -Destination $item.destination
        $createdDestinations.Add($item)
    }

    foreach ($item in $plan) {
        if ((Get-Item -LiteralPath $item.destination).Length -ne [long]$item.entry.bytes -or (Get-Sha256 $item.destination) -ne [string]$item.entry.sha256) {
            throw "Copied destination verification failed: $($item.destination)"
        }
    }
    $afterCopyReferences = Get-ReferenceDiagnosis $entries $repoRoot
    if ($afterCopyReferences.reference_sha256 -ne $beforeReferences.reference_sha256) { throw 'Path-reference diagnosis changed during copy; refusing source removal.' }

    foreach ($item in $plan) {
        if ((Get-Sha256 $item.source) -ne [string]$item.entry.sha256) { throw "Source changed after copy verification: $($item.source)" }
        if ((Get-Sha256 $item.destination) -ne [string]$item.entry.sha256) { throw "Destination changed before source removal: $($item.destination)" }
        Remove-Item -LiteralPath $item.source -Force
        $removedSources.Add($item)
    }
    $afterReferences = Get-ReferenceDiagnosis $entries $repoRoot
    if ($afterReferences.reference_sha256 -ne $beforeReferences.reference_sha256) { throw 'Path-reference diagnosis changed after source removal; rolling back.' }

    $index = [pscustomobject]@{
        schema = 'ecopunk-implementation-snapshot-migration/1'
        status = 'copied-hash-verified-original-files-unlinked'
        source_root = 'docs/qa/implementation-2026-10-25/batch-6/rounds'
        destination_root = 'audit/implementation-snapshots/2026-10-25/batch-6/rounds'
        file_count = $plan.Count
        reference_diagnosis_before = $beforeReferences
        reference_diagnosis_after = $afterReferences
        files = @($plan | ForEach-Object { [pscustomobject]@{ original_path = ('docs/qa/implementation-2026-10-25/batch-6/rounds/' + $_.entry.original_path); archived_path = $_.entry.proposed_destination; bytes = $_.entry.bytes; sha256 = $_.entry.sha256 } })
    }
    $indexTemp = Join-Path (Split-Path -Parent $indexPath) ('.migration-index-' + [guid]::NewGuid().ToString('N') + '.tmp')
    $index | ConvertTo-Json -Depth 7 | Set-Content -LiteralPath $indexTemp -Encoding utf8
    Move-Item -LiteralPath $indexTemp -Destination $indexPath
    $indexTemp = $null
    Write-Output "Relocation complete: $($plan.Count) exact files copied, hash-verified, and unlinked. Resolver index: $indexPath"
}
catch {
    $failureMessage = $_.Exception.Message
    $rollbackErrors = [System.Collections.Generic.List[string]]::new()
    foreach ($item in $removedSources) {
        try {
            if (-not (Test-Path -LiteralPath $item.source)) {
                $parent = Split-Path -Parent $item.source
                if (-not (Test-Path -LiteralPath $parent)) { New-Item -ItemType Directory -Path $parent -Force | Out-Null }
                Copy-Item -LiteralPath $item.destination -Destination $item.source
            }
            if ((Get-Item -LiteralPath $item.source).Length -ne [long]$item.entry.bytes -or (Get-Sha256 $item.source) -ne [string]$item.entry.sha256) { throw "Restored source failed hash verification: $($item.source)" }
        } catch { $rollbackErrors.Add($_.Exception.Message) }
    }
    if ($rollbackErrors.Count -eq 0) {
        foreach ($item in $createdDestinations) {
            try {
                if (Test-Path -LiteralPath $item.destination) {
                    if ((Get-Sha256 $item.destination) -ne [string]$item.entry.sha256) { throw "Refusing rollback removal of changed destination: $($item.destination)" }
                    Remove-Item -LiteralPath $item.destination -Force
                }
            } catch { $rollbackErrors.Add($_.Exception.Message) }
        }
    }
    foreach ($tempPath in $temporaryFiles) { if (Test-Path -LiteralPath $tempPath) { Remove-Item -LiteralPath $tempPath -Force -ErrorAction SilentlyContinue } }
    if ($indexTemp -and (Test-Path -LiteralPath $indexTemp)) { Remove-Item -LiteralPath $indexTemp -Force -ErrorAction SilentlyContinue }
    if ($rollbackErrors.Count) { throw "Relocation failed: $failureMessage; rollback needs manual recovery; source/destination recovery details: $($rollbackErrors -join '; ')" }
    throw "Relocation failed; sources are preserved/restored and verified staged copies removed: $failureMessage"
}

