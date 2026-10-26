$ErrorActionPreference='Stop'
$evidence='C:\github\ecopunk\docs\qa\implementation-2026-10-25\batch-5\workflow'
$root='C:\github\Poet\native\src\graphical'
$target="$root\native_project_workflow_tests.rs"
if(Test-Path -LiteralPath $target){throw 'New native workflow test already exists'}
$candidate="$evidence\candidate\poet\native_project_workflow_tests.rs"
Copy-Item -LiteralPath $candidate -Destination $target
$after=(Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash
if($after -ne (Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash){throw 'Saved test differs'}
$receipt=@([pscustomobject]@{Path=$target;Before=$null;After=$after;Kind='saved-source-only'})
$target="$root\native_project_tests.rs"
$before=(Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash
$text=[IO.File]::ReadAllText($target)
if($text.Contains('native_project_workflow_tests')){throw 'Native workflow test already routed'}
$text+="`n#[path = `"native_project_workflow_tests.rs`"]`nmod workflow_tests;`n"
[IO.File]::WriteAllText($target,$text,[Text.UTF8Encoding]::new($false))
$receipt+=[pscustomobject]@{Path=$target;Before=$before;After=(Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash;Kind='saved-source-only'}
$receipt | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath "$evidence\native-test-save-receipt.json"
$receipt
