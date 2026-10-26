$ErrorActionPreference='Stop'
$evidence='C:\github\ecopunk\docs\qa\implementation-2026-10-25\batch-5\workflow'
$core='C:\github\QualiaDB\crates\qualia-core-db\src\poet_host\native_authority'
$store='C:\github\QualiaDB\crates\qualia-core-db\src\poet_host\invoke'
$poet='C:\github\Poet\native\src\graphical'
$utf8=[Text.UTF8Encoding]::new($false)
$receipt=@()
$coreBefore=@(Get-Content -Raw -LiteralPath "$evidence\core-save-receipt.json" | ConvertFrom-Json)
$storeBefore=@(Get-Content -Raw -LiteralPath "$evidence\store-before.json" | ConvertFrom-Json)
$poetBefore=@(Get-Content -Raw -LiteralPath "$evidence\poet-before.json" | ConvertFrom-Json)
function Save-Guarded($candidate,$target,$expected) {
    $actual=if(Test-Path -LiteralPath $target){(Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash}else{$null}
    if($actual -ne $expected){throw "Guard mismatch: $target"}
    $parent=Split-Path -Parent $target
    if(-not(Test-Path -LiteralPath $parent)){New-Item -ItemType Directory -Path $parent | Out-Null}
    $candidateHash=(Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash
    Copy-Item -LiteralPath $candidate -Destination $target
    $after=(Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash
    if($candidateHash -ne $after){throw "Saved hash mismatch: $target"}
    $script:receipt+=[pscustomobject]@{Path=$target;Before=$actual;After=$after;Kind='saved-source-only'}
}
foreach($name in @('records.rs','journal.rs','workflow.rs','workflow_tests.rs')) {
    $target=Join-Path $core $name
    $prior=$coreBefore|Where-Object {$_.Path -eq $target}
    Save-Guarded "$evidence\candidate\core\$name" $target $prior.After
}
foreach($name in @('studio_journal.rs','studio_stamped.rs','studio_store.rs')) {
    $target=Join-Path $store $name
    $prior=$storeBefore|Where-Object {$_.Path -eq $target}
    Save-Guarded "$evidence\candidate\store\$name" $target $prior.Hash
}
$target=Join-Path $poet 'editor_contract.rs'
Save-Guarded "$evidence\candidate\poet\editor_contract.rs" $target $poetBefore.Hash
foreach($name in @('mod.rs','service.rs','source.rs')) {
    Save-Guarded "$evidence\candidate\poet\cooperative_workflow\$name" "$poet\cooperative_workflow\$name" $null
}
Save-Guarded "$evidence\candidate\poet\cooperative_controls.rs" "$poet\cooperative_controls.rs" $null
foreach($name in @('mod.rs','tests.rs')) {
    $target=Join-Path $core $name
    $original=[IO.File]::ReadAllText($target)
    $actual=(Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash
    if($name -eq 'mod.rs') {
        if(-not $original.Contains('                packages: Vec::new(),')){throw 'Durable initializer anchor differs'}
        if($original.Contains('requests: Vec::new()')){throw 'Requests initializer already changed'}
        $changed=$original.Replace('                packages: Vec::new(),', "                packages: Vec::new(),`n                requests: Vec::new(),`n                enrollment: Vec::new(),")
    } else {
        if($original.Contains('workflow_tests')){throw 'Workflow test routing already changed'}
        $changed=$original+"`n#[path = `"workflow_tests.rs`"]`nmod workflow_tests;`n"
    }
    $candidate="$evidence\candidate\core\$name"
    [IO.File]::WriteAllText($candidate,$changed,$utf8)
    Save-Guarded $candidate $target $actual
}
$receipt | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath "$evidence\workflow-save-receipt.json"
$receipt
