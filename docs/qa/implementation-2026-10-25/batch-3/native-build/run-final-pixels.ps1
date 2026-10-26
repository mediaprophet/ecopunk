$ErrorActionPreference='Stop'
$batch=Split-Path $PSScriptRoot -Parent
$core=Get-Content (Join-Path $batch 'core-final-saved-tests.json') -Raw|ConvertFrom-Json
$native=Get-Content (Join-Path $PSScriptRoot 'tests-saved-final.json') -Raw|ConvertFrom-Json
if($core.exit_code -ne 0 -or $native.exit_code -ne 0){throw 'Final core/native suite must pass before this GPU lane'}
$raw=Get-Content (Join-Path $PSScriptRoot 'tests-saved-final.log') -Raw
$match=[regex]::Match($raw,'Running unittests src\\lib.rs \(([^\r\n]+\.exe)\)')
if(-not $match.Success){throw 'Missing exact Cargo native library test executable'}
$binary=[IO.Path]::GetFullPath((Join-Path 'C:/github/ecopunk' $match.Groups[1].Value))
if(-not $binary.StartsWith('C:\github\ecopunk\target\debug\deps\')){throw 'Unexpected test executable path'}
$digest=(Get-FileHash -LiteralPath $binary -Algorithm SHA256).Hash
$test='graphical::materials::pixels::accepted_material_edits_change_native_gpu_pixels'
$started=[DateTime]::UtcNow.ToString('o')
& $binary $test --ignored --exact --nocapture --test-threads=1 *> (Join-Path $PSScriptRoot 'material-pixels-final.log')
$code=$LASTEXITCODE
if((Get-FileHash -LiteralPath $binary -Algorithm SHA256).Hash -ne $digest){throw 'Native test executable changed during qualification'}
[ordered]@{exit_code=$code;binary=$binary;sha256=$digest;test=$test;started_utc=$started;finished_utc=[DateTime]::UtcNow.ToString('o');input_snapshot=$native.input_snapshot;core_receipt='core-final-saved-tests.json';native_receipt='native-build/tests-saved-final.json';scope='Actual saved-source native source admission, material edits and PortalGpu readback; no window or human acceptance'}|ConvertTo-Json -Depth 7|Set-Content (Join-Path $PSScriptRoot 'material-pixels-final.json')
Get-Content (Join-Path $PSScriptRoot 'material-pixels-final.log') -Tail 15
exit $code
