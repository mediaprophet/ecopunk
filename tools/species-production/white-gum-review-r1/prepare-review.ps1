param([ValidateSet('whole-tree','adult-detail')][string]$View = 'whole-tree', [Parameter(Mandatory=$true)][string]$Revision)
$ErrorActionPreference = 'Stop'
if ($Revision -notmatch '^[A-Za-z0-9][A-Za-z0-9_-]{0,60}$') { throw 'Revision must be a short filename component.' }
$source = 'C:\github\ecopunk\assets\production-studies\white-gum-review-r1'
$destination = Join-Path $source ('revisions\' + $Revision + '-' + $View)
if (Test-Path -LiteralPath $destination) { throw 'Revision already exists; choose a new revision rather than overwrite evidence.' }
New-Item -ItemType Directory -Path (Join-Path $destination 'authoring'),(Join-Path $destination 'dossier') -Force | Out-Null
foreach ($name in @('model.vibe','adult-detail.vibe','generate.rs','build-helper.ps1')) {
    Copy-Item -LiteralPath (Join-Path $source ('authoring\' + $name)) -Destination (Join-Path $destination ('authoring\' + $name))
}
if ($View -eq 'adult-detail') { Copy-Item -LiteralPath (Join-Path $source 'authoring\adult-detail.vibe') -Destination (Join-Path $destination 'authoring\model.vibe') -Force }
Copy-Item -LiteralPath (Join-Path $source 'README.md') -Destination (Join-Path $destination 'README.md')
foreach ($name in @('README.md','claims.json','inventories.json','references.json')) { Copy-Item -LiteralPath (Join-Path $source ('dossier\' + $name)) -Destination (Join-Path $destination ('dossier\' + $name)) }
$pins = Get-ChildItem -LiteralPath $destination -Recurse -File | ForEach-Object { [pscustomobject]@{path=$_.FullName.Substring($destination.Length+1);sha256=(Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant()} }
[pscustomobject]@{kind='staged-inputs-only';view=$View;revision=$Revision;source_evaluation_file='authoring/model.vibe';files=$pins;execution='not_executed';coordinator_review='pending';timothy_review='pending'} | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $destination 'staged-inputs.json')
Write-Output $destination
