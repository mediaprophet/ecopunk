# Build and verify the Corymbia aparrerinja (Ghost Gum) digital twin bundle
$ErrorActionPreference = "Stop"

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "🌿 Ecopunk: Compiling Corymbia aparrerinja Specimen Bundle" -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Cyan

$cli = "C:\github\qualiaDB\target\debug\qualia-cli.exe"
if (-not (Test-Path $cli)) {
    $cli = "cargo run -p qualia-cli --"
}

$outputHmc = "$PSScriptRoot\corymbia-aparrerinja.hmc"
$dummyHero = "$PSScriptRoot\hero-geometry.bin"

# Ensure synthetic hero payload exists
if (-not (Test-Path $dummyHero)) {
    [IO.File]::WriteAllBytes($dummyHero, [byte[]](1..65536 | % { 0x01 }))
}

# Compile specimen bundle
& cargo run -p qualia-cli -- entity-compile `
    --hero $dummyHero `
    --output $outputHmc `
    --subject-id "urn:qualia:taxon:corymbia_aparrerinja" `
    --entity-type "DarwinianTaxon" `
    --issuer "did:web:ecopunk.civics.au:authoring" `
    --procedural-seed "hmc://seeds/bark/corymbia_aparrerinja_bone_white_v1" `
    --acoustic-low 180 `
    --acoustic-mid 950 `
    --acoustic-high 6200 `
    --licence-class "SocialPermissiveCommons" `
    --production-cost-cents 15000 `
    --transaction-cost-per-mib-cents 1 `
    --dump-json

Write-Host "`n🔍 Verifying Specimen Bundle against Permissive Commons Gates..." -ForegroundColor Cyan
& cargo run -p qualia-cli -- entity-verify $outputHmc

Write-Host "`n✅ Build and Verification Complete." -ForegroundColor Green
