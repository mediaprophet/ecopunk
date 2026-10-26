$ErrorActionPreference='Stop'
$base=$PSScriptRoot
$target='C:/github/Poet/native/src/graphical'
$before=Get-Content -LiteralPath (Join-Path $base 'before.json') -Raw|ConvertFrom-Json
foreach($file in $before.files){if((Get-FileHash -LiteralPath $file.path -Algorithm SHA256).Hash.ToLowerInvariant() -ne $file.sha256){throw ('Preserved Poet input changed: '+$file.path)}}
if(Test-Path -LiteralPath ($target+'/assembly')){throw 'Assembly source directory already exists; reconcile rather than replace'}
$candidate=Join-Path $base 'candidate'
$rows=@()
New-Item -ItemType Directory -Path ($target+'/assembly') | Out-Null
foreach($file in Get-ChildItem -LiteralPath ($candidate+'/assembly') -File -Filter '*.rs'){
    $dest=$target+'/assembly/'+$file.Name
    Copy-Item -LiteralPath $file.FullName -Destination $dest
    $hash=(Get-FileHash -LiteralPath $dest -Algorithm SHA256).Hash.ToLowerInvariant()
    if($hash -ne (Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash.ToLowerInvariant()){throw 'Saved assembly source differs'}
    $rows += [ordered]@{path=$dest;before=$null;after=$hash}
}
foreach($file in $before.files){
    $name=Split-Path $file.path -Leaf
    Copy-Item -LiteralPath ($candidate+'/'+$name) -Destination $file.path
    $hash=(Get-FileHash -LiteralPath $file.path -Algorithm SHA256).Hash.ToLowerInvariant()
    if($hash -ne (Get-FileHash -LiteralPath ($candidate+'/'+$name) -Algorithm SHA256).Hash.ToLowerInvariant()){throw 'Saved bridge source differs'}
    $rows += [ordered]@{path=$file.path;before=$file.sha256;after=$hash}
}
[ordered]@{scope='Saved Poet assembly projection/render/10d compile bridges only; no Cargo/test execution';saved_utc=[DateTime]::UtcNow.ToString('o');files=$rows}|ConvertTo-Json -Depth 5|Set-Content -LiteralPath (Join-Path $base 'save-receipt.json') -Encoding utf8NoBOM
Write-Output ('Saved '+$rows.Count+' authorized Poet source files')
