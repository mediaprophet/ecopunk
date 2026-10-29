param([string]$PackageDirectory = 'viewer/pkg')
$ErrorActionPreference = 'Stop'
$repo = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
$package = if ([IO.Path]::IsPathRooted($PackageDirectory)) { $PackageDirectory } else { Join-Path $repo $PackageDirectory }
$wasmHash = (Get-FileHash -LiteralPath (Join-Path $package 'ecopunk_wasm_bg.wasm') -Algorithm SHA256).Hash.ToLowerInvariant()
$jsHash = (Get-FileHash -LiteralPath (Join-Path $package 'ecopunk_wasm.js') -Algorithm SHA256).Hash.ToLowerInvariant()
$revision = $wasmHash.Substring(0,16) + $jsHash.Substring(0,16)
foreach ($entry in @('index.html','viewer/index.html')) {
    $path = Join-Path $repo $entry
    $html = [IO.File]::ReadAllText($path)
    if ($html -notmatch "const ASSET_REVISION = '[^']*';") { throw "Missing asset revision marker in $entry" }
    $html = $html -replace "const ASSET_REVISION = '[^']*';", "const ASSET_REVISION = '$revision';"
    [IO.File]::WriteAllText($path, $html, (New-Object Text.UTF8Encoding($false)))
}
Write-Output "Demo resource revision: $revision"
