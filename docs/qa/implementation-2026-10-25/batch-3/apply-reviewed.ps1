param([Parameter(Mandatory)][string[]]$Manifests)
$ErrorActionPreference='Stop'
$base=[IO.Path]::GetFullPath($PSScriptRoot)
$all=@()
$seen=@{}
foreach($manifest in $Manifests){
 $manifestPath=[IO.Path]::GetFullPath((Join-Path $base $manifest))
 if(-not $manifestPath.StartsWith($base+[IO.Path]::DirectorySeparatorChar)){throw 'Manifest outside batch'}
 $folder=[IO.Path]::GetDirectoryName($manifestPath)
 $spec=Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
 foreach($change in $spec.mutations){
  $target=[IO.Path]::GetFullPath($change.target)
  if($target -notmatch '^C:\\github\\(qualiaDB\\crates\\(vibe|qualia-core-db|qualia-cooperative-core|qualia-mcp)\\|poet\\native\\)'){throw "Unallocated target: $target"}
  if($seen.ContainsKey($target)){throw "Duplicate target: $target"};$seen[$target]=$true
  if($null -eq $change.original_sha256){if(Test-Path -LiteralPath $target){throw "New target exists: $target"}}
  elseif((Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash -ne $change.original_sha256){throw "Source changed: $target"}
  $candidate=[IO.Path]::GetFullPath((Join-Path $folder $change.candidate))
  if(-not $candidate.StartsWith($folder+[IO.Path]::DirectorySeparatorChar)){throw 'Candidate outside scope'}
  if((Get-FileHash -LiteralPath $candidate -Algorithm SHA256).Hash -ne $change.candidate_sha256){throw "Candidate changed: $candidate"}
  $all+=@{target=$target;candidate=$candidate;sha256=$change.candidate_sha256;original_sha256=$change.original_sha256}
 }
}
foreach($change in $all){
 [IO.Directory]::CreateDirectory([IO.Path]::GetDirectoryName($change.target))|Out-Null
 [IO.File]::WriteAllBytes($change.target,[IO.File]::ReadAllBytes($change.candidate))
 if((Get-FileHash -LiteralPath $change.target -Algorithm SHA256).Hash -ne $change.sha256){throw 'Applied bytes mismatch'}
}
$receipt=[ordered]@{applied_utc=[DateTime]::UtcNow.ToString('o');manifests=$Manifests;mutations=$all}
$out=Join-Path $base ('applied-'+[DateTime]::UtcNow.ToString('yyyyMMdd-HHmmss')+'.json')
$receipt|ConvertTo-Json -Depth 7|Set-Content -LiteralPath $out -Encoding utf8
"Applied and verified $($all.Count) reviewed source files; receipt $out"
