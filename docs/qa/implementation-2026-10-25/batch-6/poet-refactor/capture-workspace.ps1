$ErrorActionPreference = 'Stop'
$projectionRoot = 'C:\github\poet\native\src\graphical\poet_workspace'
$newFiles = foreach ($file in (Get-ChildItem -LiteralPath $projectionRoot -File -Filter '*.rs')) {
    @{path=$file.FullName;before_status='new scoped native projection';sha256=(Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash.ToLower();lines=@(Get-Content -LiteralPath $file.FullName).Count}
}
$originalPaths = @(Get-ChildItem -LiteralPath 'C:\github\poet\wasm\src\tool_chest\core' -File -Filter '*.rs' | Select-Object -ExpandProperty FullName)
$originalPaths += 'C:\github\poet\wasm\src\tool_chest\constructs\mod.rs'
$originalPaths += 'C:\github\poet\wasm\src\tool_chest\manifolds\studio.rs'
$originals = foreach ($path in $originalPaths) {
    @{path=$path;role='original Poet source reused unchanged by path; retains original licence';sha256=(Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLower()}
}
@{utc=[DateTime]::UtcNow.ToString('o');kind='saved native projection of existing Poet model and intent contract';cargo_run_by_worker=$false;rustfmt_exit_code=0;new_files=@($newFiles);original_model_inputs=@($originals)} | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath "$PSScriptRoot\workspace-source.json"
