$ErrorActionPreference = 'Stop'
$evidence = 'C:\github\ecopunk\docs\qa\implementation-2026-10-25\batch-5\workflow'
$root = 'C:\github\QualiaDB\crates\qualia-core-db\src\poet_host\native_authority'
$before = @(Get-Content -Raw -LiteralPath "$evidence\before.json" | ConvertFrom-Json)
$receipt = @()
foreach ($name in @('records.rs','journal.rs','workflow.rs','workflow_tests.rs')) {
    $target = Join-Path $root $name
    $original = $before | Where-Object { $_.Path -eq $target }
    $actual = if (Test-Path -LiteralPath $target) { (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash } else { $null }
    if ($original) {
        if ($actual -ne $original.Hash) { throw "Changed original: $target" }
    } elseif ($actual) { throw "New target already exists: $target" }
    $candidate = Join-Path "$evidence\candidate\core" $name
    $candidateHash = (Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash
    Copy-Item -LiteralPath $candidate -Destination $target
    $after = (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash
    if ($after -ne $candidateHash) { throw "Saved digest differs: $target" }
    $receipt += [pscustomobject]@{ Path=$target; Before=$actual; After=$after; Kind='saved-source-only' }
}
$receipt | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath "$evidence\core-save-receipt.json"
$receipt
