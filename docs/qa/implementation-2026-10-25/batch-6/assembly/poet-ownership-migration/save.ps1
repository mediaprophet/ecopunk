$ErrorActionPreference='Stop'
$before=Get-Content -LiteralPath (Join-Path $PSScriptRoot 'before.json') -Raw|ConvertFrom-Json
foreach($file in $before.files){if((Get-FileHash -LiteralPath $file.path -Algorithm SHA256).Hash.ToLowerInvariant() -ne $file.sha256){throw ('Current source changed; preserve and reconcile: '+$file.path)}}
$rows=@()
foreach($file in $before.files){
    $candidate=Join-Path (Join-Path $PSScriptRoot 'candidate') $file.name
    Copy-Item -LiteralPath $candidate -Destination $file.path
    $after=(Get-FileHash -LiteralPath $file.path -Algorithm SHA256).Hash.ToLowerInvariant()
    if($after -ne (Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash.ToLowerInvariant()){throw 'Migration save differs from candidate'}
    $rows += [ordered]@{path=$file.path;before=$file.sha256;after=$after}
}
[ordered]@{
    event='Human-authorized internal Poet ownership and capability namespace correction'
    utc=[DateTime]::UtcNow.ToString('o')
    replaces='Unreleased craft_authoring/CraftPart/Craft.profile_sweep active dependency projection'
    current='Internal poet_spatial_authoring/AssemblyPart/ComputationalGeometry.profile_sweep'
    historical_evidence='Earlier candidates, before/after pins and save scripts remain preserved as superseded evidence; do not replay their old namespace'
    source_shapes_changed=$false
    presets_changed=$false
    cargo_or_tests_run=$false
    files=$rows
}|ConvertTo-Json -Depth 6|Set-Content -LiteralPath (Join-Path $PSScriptRoot 'migration-event.json') -Encoding utf8NoBOM
Write-Output ('Saved '+$rows.Count+' approved ownership/name corrections')
