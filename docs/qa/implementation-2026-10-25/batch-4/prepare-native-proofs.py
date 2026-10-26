from pathlib import Path
root=Path(__file__).parent
prior=root.parent/'batch-3'
gpu=(prior/'native-build/run-final-pixels.ps1').read_text(encoding='utf-8-sig')
gpu=gpu.replace("$batch=Split-Path $PSScriptRoot -Parent","$batch=$PSScriptRoot")
gpu=gpu.replace('core-final-saved-tests.json','core-final.json').replace('tests-saved-final.json','native-final.json')
gpu=gpu.replace('tests-saved-final.log','native-final.log').replace('material-pixels-final','gpu-final')
gpu=gpu.replace("'native-build/native-final.json'","'native-final.json'")
gpu=gpu.replace("input_snapshot=$native.input_snapshot;","input_snapshot=$native.input_snapshot;source_unchanged=$true;")
# GPU qualification is on an immutable compiled executable. Also bind every
# saved source byte to the current native run rather than asserting stability.
guard="""
$sourceSnapshot=Get-Content $native.input_snapshot.path -Raw|ConvertFrom-Json
foreach($source in $sourceSnapshot.sources){if((Get-FileHash -LiteralPath $source.path -Algorithm SHA256).Hash -ne $source.sha256){throw 'Native source pin changed before GPU proof'}}
"""
gpu=gpu.replace("$raw=Get-Content",guard+"\n$raw=Get-Content")
gpu=gpu.replace("[ordered]@{exit_code=",guard+"\n[ordered]@{exit_code=")
(root/'run-final-pixels.ps1').write_text(gpu,encoding='utf-8')
folder=root/'direct-cli';folder.mkdir(exist_ok=True)
link=(prior/'native-build/direct-cli/link_and_check.py').read_text(encoding='utf-8-sig')
link=link.replace('poet-batch3','poet-batch4')
(folder/'link_and_check.py').write_text(link,encoding='utf-8')
print('Prepared source-bound final GPU and actual-source CLI checks; neither was run.')
