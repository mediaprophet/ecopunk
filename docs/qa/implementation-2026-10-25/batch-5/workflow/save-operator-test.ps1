$ErrorActionPreference='Stop'
$evidence='C:\github\ecopunk\docs\qa\implementation-2026-10-25\batch-5\workflow'
$before=@(Get-Content -Raw -LiteralPath "$evidence\operator-test-before.json" | ConvertFrom-Json)
$receipt=@()
foreach($input in $before) {
    $target=$input.Path
    $actual=(Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash
    if($actual -ne $input.Hash){throw "Operator regression guard mismatch: $target"}
    $relative=if($target.EndsWith('native_project_workflow_tests.rs')){'native_project_workflow_tests.rs'}else{'cooperative_workflow\service.rs'}
    $candidate=Join-Path "$evidence\candidate\poet" $relative
    Copy-Item -LiteralPath $candidate -Destination $target
    $after=(Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash
    if($after -ne (Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash){throw "Saved operator regression differs: $target"}
    $receipt+=[pscustomobject]@{Path=$target;Before=$actual;After=$after;Kind='saved-source-only'}
}
$receipt|ConvertTo-Json -Depth 5|Set-Content -LiteralPath "$evidence\operator-test-save-receipt.json"
$receipt
