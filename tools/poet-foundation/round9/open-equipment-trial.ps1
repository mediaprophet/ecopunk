[CmdletBinding()]
param([string]$ProjectRoot)
$ErrorActionPreference = 'Stop'
$repoRoot = 'C:\github\ecopunk'
$poetExe = Join-Path $repoRoot 'target\debug\poet.exe'
if (-not (Test-Path -LiteralPath $poetExe -PathType Leaf)) { throw "Built Poet is missing: $poetExe" }
$previousEquipmentProfile = $env:POET_NATIVE_EQUIPMENT_PROFILE
try {
    $env:POET_NATIVE_EQUIPMENT_PROFILE = '1'
    if ($ProjectRoot) {
        $projectDirectory = (Resolve-Path -LiteralPath $ProjectRoot).Path
        & $poetExe studio --native-project $projectDirectory
    } else {
        $fixtureRoot = Join-Path $repoRoot 'docs\qa\implementation-2026-10-26\round9\final\reference-output'
        if (-not (Test-Path -LiteralPath (Join-Path $fixtureRoot 'diagnostic-equipment.hmc') -PathType Leaf)) { throw 'The verified diagnostic HMC is missing.' }
        Write-Output 'Opening a LOCAL DIAGNOSTIC. This does not authenticate a contributor or approve a production package.'
        & $poetExe studio --diagnostic-source --checkpoint-root $fixtureRoot --reopen diagnostic-equipment.hmc --grant capability.invoke --grant ComputationalGeometry.profile_sweep --grant ComputationalGeometry.uv_sphere --grant ComputationalGeometry.torus
    }
    if ($LASTEXITCODE -ne 0) { throw "Poet exited with code $LASTEXITCODE" }
} finally {
    $env:POET_NATIVE_EQUIPMENT_PROFILE = $previousEquipmentProfile
}
