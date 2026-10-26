$ErrorActionPreference = 'Stop'
$evidence = 'C:\github\ecopunk\docs\qa\implementation-2026-10-25\batch-4\authentication'
$core = 'C:\github\qualiaDB\crates\qualia-core-db\src\poet_host'
$poet = 'C:\github\poet\native\src\graphical\cooperation'
$receipts = @()
$priorReceipts = if (Test-Path -LiteralPath "$evidence\save-receipt.json") { @(Get-Content -Raw -LiteralPath "$evidence\save-receipt.json" | ConvertFrom-Json) } else { @() }
New-Item -ItemType Directory -Force -Path "$core\native_authority" | Out-Null
foreach ($file in Get-ChildItem -LiteralPath "$evidence\candidate\native_authority" -File) {
    $target = Join-Path "$core\native_authority" $file.Name
    $before = if (Test-Path -LiteralPath $target) { (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash } else { $null }
    Copy-Item -LiteralPath $file.FullName -Destination $target
    $after = (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash
    $receipts += [pscustomobject]@{ Path = $target; Before = $before; After = $after }
}
$target = "$poet\native_owner.rs"
$before = if (Test-Path -LiteralPath $target) { (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash } else { $null }
Copy-Item -LiteralPath "$evidence\candidate\native_owner.rs" -Destination $target
$receipts += [pscustomobject]@{ Path = $target; Before = $before; After = (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash }
if (Test-Path -LiteralPath "$evidence\candidate\cooperation") {
    foreach ($file in Get-ChildItem -LiteralPath "$evidence\candidate\cooperation" -File) {
        $target = Join-Path $poet $file.Name
        $before = if (Test-Path -LiteralPath $target) { (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash } else { $null }
        Copy-Item -LiteralPath $file.FullName -Destination $target
        $receipts += [pscustomobject]@{ Path = $target; Before = $before; After = (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash }
    }
}
$routing = "$core\mod.rs"
$before = (Get-FileHash -LiteralPath $routing -Algorithm SHA256).Hash
$content = [IO.File]::ReadAllText($routing)
if (-not $content.Contains('pub mod native_authority;')) {
    [IO.File]::WriteAllText($routing, $content.Replace('pub mod scoped_effects;', "pub mod scoped_effects;`r`n#[cfg(not(target_arch = `"wasm32`"))]`r`npub mod native_authority;"), [Text.UTF8Encoding]::new($false))
}
elseif (-not $content.Contains('#[cfg(not(target_arch = "wasm32"))]' + "`r`npub mod native_authority;")) {
    $content = $content.Replace('pub mod native_authority;', "#[cfg(not(target_arch = `"wasm32`"))]`r`npub mod native_authority;")
    [IO.File]::WriteAllText($routing, $content, [Text.UTF8Encoding]::new($false))
}
$receipts += [pscustomobject]@{ Path = $routing; Before = $before; After = (Get-FileHash -LiteralPath $routing -Algorithm SHA256).Hash }
$routing = "$poet\mod.rs"
$before = (Get-FileHash -LiteralPath $routing -Algorithm SHA256).Hash
$content = [IO.File]::ReadAllText("$evidence\candidate\cooperation_mod.rs")
if (-not $content.Contains('pub mod native_owner;')) {
    $content = "pub mod native_owner;`r`n$content"
}
[IO.File]::WriteAllText($routing, $content, [Text.UTF8Encoding]::new($false))
$receipts += [pscustomobject]@{ Path = $routing; Before = $before; After = (Get-FileHash -LiteralPath $routing -Algorithm SHA256).Hash }
if (Test-Path -LiteralPath "$evidence\candidate\native_project_tests.rs") {
    $target = 'C:\github\poet\native\src\graphical\native_project_tests.rs'
    $before = if (Test-Path -LiteralPath $target) { (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash } else { $null }
    Copy-Item -LiteralPath "$evidence\candidate\native_project_tests.rs" -Destination $target
    $receipts += [pscustomobject]@{ Path = $target; Before = $before; After = (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash }
    $target = 'C:\github\poet\native\src\graphical\native_project.rs'
    $before = (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash
    $source = [IO.File]::ReadAllText($target)
    if (-not $source.Contains('native_project_tests.rs')) {
        [IO.File]::WriteAllText($target, $source + "`r`n#[cfg(test)]`r`n#[path = `"native_project_tests.rs`"]`r`nmod tests;`r`n", [Text.UTF8Encoding]::new($false))
    }
    $receipts += [pscustomobject]@{ Path = $target; Before = $before; After = (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash }
}
@($priorReceipts) + @($receipts) | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath "$evidence\save-receipt.json"
$receipts | Format-Table -AutoSize
