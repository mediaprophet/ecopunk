$ErrorActionPreference = 'Stop'
$base = 'C:\github\ecopunk\tools\poet-foundation\full-scope'
$poet = 'C:\github\poet\native\src\graphical'
$receipt = Join-Path $base 'junction-installed'
if (Test-Path -LiteralPath $receipt) { throw 'Installation receipt already exists; reconcile before rerunning' }
if (Get-CimInstance Win32_Process -Filter "name='cargo.exe' OR name='rustc.exe'") { throw 'Rust compilation is active; source installation deferred' }
$mapping = @(
    @('fork-adopted-adapter\fork_adopted_adapter.rs', 'fork_adopted_adapter.rs'),
    @('fork-adopted-adapter\junction_generate.rs', 'junction_generate.rs'),
    @('junction-budget-enforcement\junction_budget.rs', 'junction_budget.rs'),
    @('junction-budget-enforcement\geometry_host.rs', 'geometry_host.rs'),
    @('junction-budget-enforcement\execution.rs', 'execution.rs'),
    @('junction-budget-enforcement\native_authoring_profile.rs', 'native_authoring_profile.rs'),
    @('junction-budget-enforcement\native_create.rs', 'native_entry\create.rs'),
    @('junction-budget-enforcement\native_runtime.rs', 'native_entry\runtime.rs'),
    @('junction-budget-enforcement\entry.rs', 'entry.rs'),
    @('junction-controller-integration\controller.rs', 'controller.rs'),
    @('fork-authoring-integration\junction_source.rs', 'authoring\botanical\junction_source.rs'),
    @('fork-authoring-integration\junction_edit.rs', 'authoring\botanical\junction_edit.rs'),
    @('fork-authoring-integration\junction_tests.rs', 'authoring\botanical\junction_tests.rs'),
    @('fork-authoring-integration\botanical_mod.rs', 'authoring\botanical\mod.rs'),
    @('fork-authoring-integration\source_edit.rs', 'authoring\botanical\source_edit.rs'),
    @('fork-authoring-integration\typed_controls.rs', 'authoring\botanical\typed_controls.rs'),
    @('fork-authoring-integration\junction_session.rs', 'authoring\junction_session.rs'),
    @('fork-authoring-integration\junction_controls.rs', 'authoring\junction_controls.rs'),
    @('fork-authoring-integration\authoring_mod.rs', 'authoring\mod.rs'),
    @('fork-authoring-integration\authoring_source.rs', 'authoring\source.rs'),
    @('fork-authoring-integration\authoring_controller.rs', 'authoring\controller.rs'),
    @('fork-authoring-integration\controls.rs', 'authoring\controls.rs')
)
foreach ($pair in $mapping) {
    if (-not (Test-Path -LiteralPath (Join-Path $base $pair[0]))) { throw "Missing candidate $($pair[0])" }
}
$controllerBefore = [IO.File]::ReadAllText((Join-Path $base 'junction-controller-integration\controller.before.rs')).Replace("`r`n", "`n")
if ([IO.File]::ReadAllText((Join-Path $poet 'controller.rs')).Replace("`r`n", "`n") -cne $controllerBefore) { throw 'Canonical controller changed since staging' }
New-Item -ItemType Directory -Path (Join-Path $receipt 'before') -Force | Out-Null
$rows = [Collections.Generic.List[string]]::new()
$rows.Add("target`tbefore_sha256`tafter_sha256`tcandidate")
foreach ($pair in $mapping) {
    $source = Join-Path $base $pair[0]
    $target = Join-Path $poet $pair[1]
    $hashBefore = 'NEW'
    if (Test-Path -LiteralPath $target) {
        $backup = Join-Path (Join-Path $receipt 'before') $pair[1]
        New-Item -ItemType Directory -Path (Split-Path -Parent $backup) -Force | Out-Null
        Copy-Item -LiteralPath $target -Destination $backup
        $hashBefore = (Get-FileHash -LiteralPath $backup -Algorithm SHA256).Hash
    }
    Copy-Item -LiteralPath $source -Destination $target
    $hashAfter = (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash
    if ($hashAfter -cne (Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash) { throw "Copy verification failed: $target" }
    $rows.Add("$target`t$hashBefore`t$hashAfter`t$source")
}
$mod = Join-Path $poet 'mod.rs'
Copy-Item -LiteralPath $mod -Destination (Join-Path $receipt 'before\mod.rs')
$text = [IO.File]::ReadAllText($mod)
foreach ($name in @('fork_adopted_adapter', 'junction_generate', 'junction_budget')) {
    if ($text -match "(?m)^.*mod $name;") { throw "Module already registered: $name" }
    $text += "`npub(crate) mod $name;`n"
}
[IO.File]::WriteAllText($mod, $text, [Text.UTF8Encoding]::new($false))
$rows.Add("$mod`t$((Get-FileHash -LiteralPath (Join-Path $receipt 'before\mod.rs') -Algorithm SHA256).Hash)`t$((Get-FileHash -LiteralPath $mod -Algorithm SHA256).Hash)`tmodule-registration")
[IO.File]::WriteAllLines((Join-Path $receipt 'source-manifest.tsv'), $rows, [Text.UTF8Encoding]::new($false))
Write-Output "Saved and hash-verified $($mapping.Count + 1) Poet source files; tests and build deferred."
