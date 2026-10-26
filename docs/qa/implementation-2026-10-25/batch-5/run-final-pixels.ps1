$ErrorActionPreference='Stop'
$batch=$PSScriptRoot
$core=Get-Content (Join-Path $batch 'core-final.json') -Raw|ConvertFrom-Json
$native=Get-Content (Join-Path $PSScriptRoot 'native-final.json') -Raw|ConvertFrom-Json
$latest=Get-Content (Join-Path $batch 'latest-source-snapshot.json') -Raw|ConvertFrom-Json
foreach($receipt in @($core,$native)){
    if($receipt.exit_code -ne 0 -or $receipt.source_unchanged -ne $true -or $receipt.input_snapshot.sha256 -ne $latest.sha256){throw 'Current unchanged-source core/native suites must pass before this GPU lane'}
}

$sourceSnapshot=Get-Content $native.input_snapshot.path -Raw|ConvertFrom-Json
foreach($source in $sourceSnapshot.sources){if((Get-FileHash -LiteralPath $source.path -Algorithm SHA256).Hash -ne $source.sha256){throw 'Native source pin changed before GPU proof'}}

$raw=Get-Content (Join-Path $PSScriptRoot 'native-final.log') -Raw
$match=[regex]::Match($raw,'Running unittests src\\lib.rs \(([^\r\n]+\.exe)\)')
if(-not $match.Success){throw 'Missing exact Cargo native library test executable'}
$binary=[IO.Path]::GetFullPath((Join-Path 'C:/github/ecopunk' $match.Groups[1].Value))
if(-not $binary.StartsWith('C:\github\ecopunk\target\debug\deps\')){throw 'Unexpected test executable path'}
$digest=(Get-FileHash -LiteralPath $binary -Algorithm SHA256).Hash
$test='graphical::materials::pixels::accepted_material_edits_change_native_gpu_pixels'
$started=[DateTime]::UtcNow.ToString('o')
& $binary $test --ignored --exact --nocapture --test-threads=1 *> (Join-Path $PSScriptRoot 'gpu-final.log')
$code=$LASTEXITCODE
$output=Get-Content (Join-Path $PSScriptRoot 'gpu-final.log') -Raw
if($code -eq 0 -and ($output -notmatch '(?m)^running 1 test\r?$' -or $output -notmatch '(?m)^test result: ok\. 1 passed; 0 failed;' -or $output -notmatch ([regex]::Escape("test $test ...")))){$code=101;Add-Content (Join-Path $PSScriptRoot 'gpu-final.log') 'Qualification failed: the exact required GPU test did not pass once.'}
if((Get-FileHash -LiteralPath $binary -Algorithm SHA256).Hash -ne $digest){throw 'Native test executable changed during qualification'}

$sourceSnapshot=Get-Content $native.input_snapshot.path -Raw|ConvertFrom-Json
foreach($source in $sourceSnapshot.sources){if((Get-FileHash -LiteralPath $source.path -Algorithm SHA256).Hash -ne $source.sha256){throw 'Native source pin changed before GPU proof'}}

[ordered]@{exit_code=$code;binary=$binary;sha256=$digest;test=$test;started_utc=$started;finished_utc=[DateTime]::UtcNow.ToString('o');input_snapshot=$native.input_snapshot;source_unchanged=$true;core_receipt='core-final.json';native_receipt='native-final.json';scope='Actual saved-source native source admission, material edits and PortalGpu readback; no window or human acceptance'}|ConvertTo-Json -Depth 7|Set-Content (Join-Path $PSScriptRoot 'gpu-final.json')
Get-Content (Join-Path $PSScriptRoot 'gpu-final.log') -Tail 15
exit $code
