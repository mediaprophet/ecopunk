$ErrorActionPreference = 'Stop'
$reviewRoot = $PSScriptRoot
$reviewedDirectories = @(
    'C:\github\qualiaDB\crates\qualia-core-db\src\poet_host\native_authority',
    'C:\github\qualiaDB\crates\qualia-core-db\src\poet_host\invoke\studio_checkpoint',
    'C:\github\poet\native\src\graphical\checkpoints',
    'C:\github\poet\native\src\graphical\cooperation',
    'C:\github\poet\native\src\graphical\package'
)
$reviewedFiles = @(
    'C:\github\poet\native\src\graphical\mod.rs',
    'C:\github\poet\native\src\graphical\controller.rs',
    'C:\github\poet\native\src\graphical\editor_contract.rs',
    'C:\github\poet\native\src\graphical\shared_host.rs',
    'C:\github\poet\native\src\graphical\jobs.rs',
    'C:\github\poet\native\src\graphical\ui.rs',
    'C:\github\poet\native\src\graphical\package_controls.rs',
    'C:\github\poet\native\src\graphical\package_batch_controls.rs',
    'C:\github\poet\native\src\graphical\native_project.rs',
    'C:\github\poet\native\src\graphical\native_project_tests.rs',
    'C:\github\poet\native\src\graphical\mcp\mod.rs',
    'C:\github\poet\native\src\graphical\mcp\resources.rs',
    'C:\github\qualiaDB\crates\qualia-core-db\src\crypto\network\ed25519.rs',
    'C:\github\qualiaDB\crates\qualia-cooperative-core\src\authoring\policy.rs',
    'C:\github\qualiaDB\crates\qualia-cooperative-core\src\authoring\records.rs'
)
$allInputs = @($reviewedFiles)
foreach ($directory in $reviewedDirectories) {
    $allInputs += @(Get-ChildItem -LiteralPath $directory -File -Filter '*.rs' | ForEach-Object FullName)
}
$rows = @($allInputs | Sort-Object -Unique | ForEach-Object {
    $file = Get-Item -LiteralPath $_
    [ordered]@{
        path = $file.FullName
        sha256 = (Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
        bytes = $file.Length
        lines = @(Get-Content -LiteralPath $file.FullName).Count
    }
})
$inventory = [ordered]@{
    schema = 'ecopunk.batch4.independent-source-review-inputs.v1'
    captured_at_utc = [DateTime]::UtcNow.ToString('o')
    scope = 'Saved source and test-source inspection; no execution implied'
    inputs = $rows
}
$inventory | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath (Join-Path $reviewRoot 'reviewed-inputs.json') -Encoding utf8
Write-Output ('Captured {0} source digests' -f $rows.Count)
