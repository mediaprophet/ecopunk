$ErrorActionPreference = 'Stop'
$entryRoot = 'C:\github\poet\native\src\graphical\native_entry'
$ownedFiles = @('runtime.rs', 'files.rs', 'actions.rs', 'create.rs')
$records = foreach ($ownedFile in $ownedFiles) {
    $sourcePath = Join-Path $entryRoot $ownedFile
    $file = Get-Item -LiteralPath $sourcePath
    @{
        path = $file.FullName
        before_sha256 = $null
        before_status = 'new canonical file, absent before this worker created it'
        after_sha256 = (Get-FileHash -LiteralPath $sourcePath -Algorithm SHA256).Hash.ToLower()
        bytes = $file.Length
        lines = @(Get-Content -LiteralPath $sourcePath).Count
    }
}
@{
    kind = 'saved-native-entry-backend'
    utc = [DateTime]::UtcNow.ToString('o')
    rustfmt_exit_code = 0
    cargo_run_by_worker = $false
    os_credential_store_accessed_by_worker = $false
    gui_or_computer_use_by_worker = $false
    files = @($records)
} | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath "$PSScriptRoot\after.json"
