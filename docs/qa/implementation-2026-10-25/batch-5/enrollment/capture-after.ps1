$ErrorActionPreference = 'Stop'
$nativeRoot = 'C:\github\QualiaDB\crates\qualia-core-db\src\poet_host\native_authority'
$sourcePaths = @(
    'C:\github\QualiaDB\crates\qualia-core-db\Cargo.toml',
    'C:\github\QualiaDB\Cargo.lock',
    'C:\github\poet\Cargo.lock',
    'C:\github\QualiaDB\crates\qualia-core-db\src\identity\key_vault.rs',
    "$nativeRoot\mod.rs",
    "$nativeRoot\admission.rs",
    "$nativeRoot\journal.rs"
)
$sourcePaths += Get-ChildItem -LiteralPath "$nativeRoot\enrollment" -File -Filter '*.rs' | Select-Object -ExpandProperty FullName
$sourcePaths += Get-ChildItem -LiteralPath 'C:\github\QualiaDB\crates\qualia-core-db\src\identity\key_vault' -File -Filter '*.rs' | Select-Object -ExpandProperty FullName
$records = foreach ($sourcePath in $sourcePaths) {
    $sourceFile = Get-Item -LiteralPath $sourcePath
    @{
        path = $sourceFile.FullName
        sha256 = (Get-FileHash -LiteralPath $sourcePath -Algorithm SHA256).Hash.ToLower()
        bytes = $sourceFile.Length
        lines = @(Get-Content -LiteralPath $sourcePath).Count
    }
}
@{
    kind = 'saved-source-inventory'
    utc = [DateTime]::UtcNow.ToString('o')
    cargo_run_by_worker = $false
    rustfmt_exit_code = 0
    os_credential_store_accessed_by_worker = $false
    files = @($records)
} | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath "$PSScriptRoot\after.json"
