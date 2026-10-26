$ErrorActionPreference = 'Stop'
$evidenceRoot = 'C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-6/rounds/round2/master'
$coreRoot = 'C:/github/qualiaDB/crates/qualia-core-db'
$geoRoot = 'C:/github/qualiaDB/crates/qualia-geospatial'
$existingFiles = @(
    @{Before="$evidenceRoot/before/m64-mod.rs"; Stage="$evidenceRoot/sdk-staging/m64-mod.rs"; Target="$coreRoot/src/q42/m64_matter/mod.rs"},
    @{Before="$evidenceRoot/before/geotiff.rs"; Stage="$evidenceRoot/sdk-staging/geotiff.rs"; Target="$geoRoot/src/geotiff.rs"}
)
$newFiles = @(
    @{Stage="$evidenceRoot/sdk-staging/checked_reader/mod.rs"; Target="$coreRoot/src/q42/m64_matter/checked_reader/mod.rs"},
    @{Stage="$evidenceRoot/sdk-staging/checked_reader/intervals.rs"; Target="$coreRoot/src/q42/m64_matter/checked_reader/intervals.rs"},
    @{Stage="$evidenceRoot/sdk-staging/checked_reader/validation.rs"; Target="$coreRoot/src/q42/m64_matter/checked_reader/validation.rs"},
    @{Stage="$evidenceRoot/sdk-staging/m64_container_sdk.rs"; Target="$coreRoot/tests/m64_container_sdk.rs"}
)
foreach ($item in $existingFiles) {
    if ((Get-FileHash -LiteralPath $item.Before).Hash -ne (Get-FileHash -LiteralPath $item.Target).Hash) {
        throw "Source changed since snapshot: $($item.Target)"
    }
}
foreach ($item in $newFiles) {
    if (Test-Path -LiteralPath $item.Target) { throw "Refusing to replace existing new-file destination: $($item.Target)" }
}
foreach ($item in $newFiles) {
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $item.Target) | Out-Null
    Copy-Item -LiteralPath $item.Stage -Destination $item.Target
}
foreach ($item in $existingFiles) { Copy-Item -LiteralPath $item.Stage -Destination $item.Target }
Write-Output 'Installed six scoped M64 SDK source/test files. No builds or assets produced.'
