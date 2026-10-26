$ErrorActionPreference = 'Stop'
$candidateRoot = Join-Path $PSScriptRoot 'candidate'
$destinationRoot = 'C:/github/Poet/native/examples/growing_reference'
if (Test-Path -LiteralPath $destinationRoot) { throw 'New example destination already exists; inspect before saving' }
$inputs = @(Get-ChildItem -LiteralPath $candidateRoot -Filter '*.rs' -File | Sort-Object Name | ForEach-Object {
    [pscustomobject]@{ name=$_.Name; source=$_.FullName; sha256=(Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant() }
})
if ($inputs.Count -ne 6) { throw 'Expected six reviewed source modules' }
New-Item -ItemType Directory -Path $destinationRoot | Out-Null
foreach ($inputFile in $inputs) {
    if ((Get-FileHash -LiteralPath $inputFile.source -Algorithm SHA256).Hash.ToLowerInvariant() -ne $inputFile.sha256) { throw 'Candidate changed while saving' }
    $destination = Join-Path $destinationRoot $inputFile.name
    $bytes = [IO.File]::ReadAllBytes($inputFile.source)
    $stream = [IO.File]::Open($destination,[IO.FileMode]::CreateNew,[IO.FileAccess]::Write,[IO.FileShare]::None)
    try { $stream.Write($bytes,0,$bytes.Length); $stream.Flush($true) } finally { $stream.Dispose() }
    if ((Get-FileHash -LiteralPath $destination -Algorithm SHA256).Hash.ToLowerInvariant() -ne $inputFile.sha256) { throw 'Saved source digest mismatch' }
}
[pscustomobject]@{ schema='poet-reference-producer-source-save/1'; created_utc=[DateTime]::UtcNow.ToString('o'); destination=$destinationRoot; before='absent'; files=$inputs; cargo_run=$false; gpu_run=$false; os_credentials_accessed=$false; human_acceptance=$false } | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $PSScriptRoot 'save-receipt.json') -Encoding utf8
$inputs | Select-Object name,sha256 | ConvertTo-Json
