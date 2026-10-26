$ErrorActionPreference = 'Stop'
$sourcePaths = @(
    'C:\github\craft\Cargo.toml',
    'C:\github\craft\crates\authoring\Cargo.toml',
    'C:\github\poet\native\src\graphical\geometry_host.rs',
    'C:\github\poet\native\src\graphical\craft_host.rs',
    'C:\github\QualiaDB\crates\qualia-core-db\src\specialized_libs\computational_geometry\authoring.rs',
    'C:\github\QualiaDB\crates\qualia-core-db\src\specialized_libs\computational_geometry\authoring_surface_tests.rs'
)
$sourcePaths += Get-ChildItem -LiteralPath 'C:\github\craft\crates\authoring\src' -File -Filter '*.rs' | Select-Object -ExpandProperty FullName
$records = foreach ($sourcePath in $sourcePaths) {
    @{
        path = $sourcePath
        sha256 = (Get-FileHash -LiteralPath $sourcePath -Algorithm SHA256).Hash.ToLower()
        bytes = (Get-Item -LiteralPath $sourcePath).Length
        lines = @(Get-Content -LiteralPath $sourcePath).Count
    }
}
$originMap = Get-Content -LiteralPath 'C:\github\craft\migration\origins\cadcraft\source-map.json' -Raw | ConvertFrom-Json
$copiedChecks = foreach ($source in $originMap.files) {
    $originNow = (Get-FileHash -LiteralPath $source.origin -Algorithm SHA256).Hash.ToLower()
    $copyNow = (Get-FileHash -LiteralPath $source.destination -Algorithm SHA256).Hash.ToLower()
    if ($originNow -ne $source.origin_sha256 -or $copyNow -ne $source.copied_sha256) { throw 'Original/copy source changed; explicit migration reconciliation required' }
    @{origin=$source.origin;destination=$source.destination;sha256=$originNow;exact_copy=$true}
}
@{
    utc = [DateTime]::UtcNow.ToString('o')
    kind = 'saved Craft extraction and authoring implementation'
    cargo_run_by_worker = $false
    rustfmt_exit_code = 0
    files = @($records)
    original_and_copied_files_unchanged = @($copiedChecks)
} | ConvertTo-Json -Depth 7 | Set-Content -LiteralPath "$PSScriptRoot\source-after.json"
