$ErrorActionPreference = 'Stop'
$root = 'C:/github/ecopunk'
$asset = Join-Path $root 'assets/production-studies/swamp-wallaby-review-r1'
$out = Join-Path $asset 'dossier/claims.json'
if (Test-Path -LiteralPath $out) { throw 'Ledger already exists; preserve review history and edit explicitly.' }
$templatePath = Join-Path $root 'docs/planning/tech/assets/model-production-template/claims.template.json'
$cataloguePath = Join-Path $root 'docs/planning/tech/assets/model-production-template/deliverables.json'
$ledger = Get-Content -Raw -LiteralPath $templatePath | ConvertFrom-Json -AsHashtable
$catalogue = Get-Content -Raw -LiteralPath $cataloguePath | ConvertFrom-Json -AsHashtable
$ledger.document_kind = 'claim_ledger'
$ledger.subject.entity_id = 'urn:ecopunk:study:swamp-wallaby-review-r1'
$ledger.subject.dossier_id = 'urn:ecopunk:dossier:swamp-wallaby-review-r1'
$ledger.subject.families = @('fauna')
$ledger.subject.source_revision = 'tools/species-production/swamp-wallaby-review-r1/swamp_wallaby_review_append.rs; see source-pins.json'
$ledger.subject.corpus_revision = '2026-10-29 two-source qualitative research snapshot; references.md'
$ledger.subject.requirements_baseline_ref = 'docs/planning/tech/assets/model-completion-claims.md; docs/qa/implementation-2026-10-26/biological-reference/wallaby-cases.md'
$ledger.subject.identifiers = @(@{value=$ledger.subject.entity_id;scheme='project-local URN';issuer_or_authority_ref='Ecopunk authored study';scope='Synthetic candidate source, not individual observation or taxon authority';source_ref='README.md';association_status='asserted';valid_from='2026-10-29';valid_until='';mapping_ref='dossier/references.md'})
function Inv($id,$description,$source='authoring/wallaby.vibe') { @{id=$id;description=$description;source_ref=$source;compatibility_ref='dossier/inventory.json'} }
$components = @('pelvis','abdomen','chest','neck','head','muzzle','jaw','left-haunch','right-haunch','left-hindlimb','right-hindlimb','left-foot','right-foot','left-forelimb','right-forelimb','left-hand','right-hand','left-ear','right-ear','left-eye','right-eye','nose','tail')
$ledger.inventories.component = @($components | ForEach-Object {Inv $_ 'Candidate exterior region/part; current rich builder plus staged species-specific adaptation. Internal anatomy and full semantic selection remain open.' 'tools/species-production/swamp-wallaby-review-r1/swamp_wallaby_review_append.rs'})
$ledger.inventories.profile = @('film','open_world_game','cartoon') | ForEach-Object {Inv $_ 'Required complete-package target, not delivered by the single review specimen.' 'dossier/review.md'}
$ledger.inventories.representation = @(Inv 'wasm-static-review' 'Staged whole-body surface candidate for current browser renderer. Not executed or accepted.' 'tools/species-production/swamp-wallaby-review-r1/swamp_wallaby_review_append.rs')
$ledger.inventories.variant = @(Inv 'synthetic-unspecified-sex-stage' 'Synthetic source geometry; sex/stage/individual not inferred. Does not discharge researched biological variant obligations.' 'dossier/references.md')
$ledger.inventories.surface = @('coat-study','eye','nose-claws','ear-inner') | ForEach-Object {Inv $_ 'Editable authored material/shading values; no measured spectra, fur groom or physical material claim.' 'tools/species-production/swamp-wallaby-review-r1/swamp_wallaby_review_append.rs'}
$ledger.inventories.qualification_case = @('three-quarter','side','front','face-detail','feet-tail-detail') | ForEach-Object {Inv $_ 'Required actual browser review view, not yet captured.' 'dossier/review.md'}
$ledger.inventories.source = @(Inv 'npws-species-page' 'Qualitative species/locality reference; photo rights not acquired.' 'https://www.nationalparks.nsw.gov.au/plants-and-animals/swamp-wallaby'; Inv 'am-species-page' 'Independent museum description; measurements not calibrated to model.' 'https://australian.museum/learn/animals/mammals/swamp-wallaby/')
$ledger.inventories.subject_identifier = @(Inv 'local-study-urn' 'Local synthetic study identifier; taxon concept authority unresolved.' 'README.md')
$ledger.inventories.animal_behaviour = @('foraging','solitary-activity','evening-feeding-aggregation') | ForEach-Object {Inv $_ 'Qualitative source-discovered behaviour; no authored operational model. Repertoire research remains incomplete.' 'dossier/references.md'}
$ledger.inventories.animal_state = @(Inv 'authored-static-pose' 'Static authored rest/inspection pose only; locomotion, development and social/reproductive states remain missing.' 'tools/species-production/swamp-wallaby-review-r1/swamp_wallaby_review_append.rs')
$ledger.inventories.contact_case = @('left-foot-ground','right-foot-ground','tail-ground') | ForEach-Object {Inv $_ 'Required visual/physical contact review; no dynamics or validated load-bearing claim.' 'dossier/review.md'}
foreach ($profile in $ledger.profiles) {
 $profile.style = if ($profile.id -eq 'cartoon') {'To be authored; retains diagnostic identifiers and anatomy'} else {'Reference-led naturalistic; not achieved'}
 $profile.delivery_target = if ($profile.id -eq 'film') {'Film master and rendered sequences'} elseif ($profile.id -eq 'cartoon') {'Realtime and film target decisions remain open'} else {'Current WASM consumer first, native authoring/host remains separately required'}
 $profile.criteria_ref = 'dossier/review.md'
}
$catalogueMap = @{}
foreach ($group in $catalogue.groups) {foreach ($item in $group.items) {$catalogueMap[$item.id]=@{item=$item;family=$group.family}}}
$now = [DateTime]::UtcNow.ToString('o')
$sourceRef = 'tools/species-production/swamp-wallaby-review-r1/swamp_wallaby_review_append.rs'
foreach ($claim in $ledger.claims) {
 $meta = $catalogueMap[$claim.deliverable_id]
 $a = $claim.assessment
 $a.applicability = if ($meta.family -in @('all','fauna')) {'required'} else {'unassessed'}
 $a.applicability_rationale = if ($a.applicability -eq 'required') {'Complete fauna package contract retains this obligation.'} else {'Other-family applicability remains explicitly unassessed; no waiver is asserted.'}
 $a.claim_status = 'unassessed'
 $a.claim = 'No satisfied claim. Staged source does not establish native/WASM execution, biological validation or acceptance.'
 $a.claimant = '/root/wallaby_production'
 $a.evaluated_at = $now
 $a.remaining_work = $meta.item.acceptance
 if ($claim.deliverable_id -in @('ID-01','ID-03','ID-04','ID-08','KN-01','GEO-02','GEO-03','MAT-01','MAT-02')) {
   $a.claim_status='partial'; $a.artifact_refs=@($sourceRef,'dossier/inventory.json'); $a.claim='Initial explicit source/inventory authored; exact row acceptance remains open and output unexecuted.'
 }
 $cases = @(@{dimensions=@{};assessment=$null})
 $expand = @($meta.item.repeat_by).Count -gt 0
 foreach ($dim in $meta.item.repeat_by) {
   if (@($ledger.inventories[$dim]).Count -eq 0) {$expand=$false;break}
   $next = @()
   foreach ($case in $cases) {foreach ($entry in $ledger.inventories[$dim]) {
    $dims=@{};foreach($key in $case.dimensions.Keys){$dims[$key]=$case.dimensions[$key]};$dims[$dim]=$entry.id
    $assessment=($a|ConvertTo-Json -Depth 20|ConvertFrom-Json -AsHashtable);$assessment.claim_status='unassessed';$assessment.claim='Required candidate case; no execution or review pass.'
    $next+=@{dimensions=$dims;assessment=$assessment}
   }}
   $cases=$next
 }
 if ($expand) {$claim.cases=$cases}
}
$assessmentTemplate=$ledger.claims[0].assessment|ConvertTo-Json -Depth 20
$ledger.subject_specific_claims = @(1..16 | ForEach-Object {
 $a=$assessmentTemplate|ConvertFrom-Json -AsHashtable;$a.applicability='required';$a.claim_status='unassessed';$a.claim='Species case retained. No case completion or acceptance.';$a.artifact_refs=@();$a.remaining_work='Fulfil exact WALLABY case from original wallaby-cases.md and bind output/domain/human evidence.'
 @{deliverable_id=('WALLABY-{0:d2}' -f $_);assessment=$a;cases=@()}
})
$ledger | ConvertTo-Json -Depth 50 | Set-Content -LiteralPath $out -Encoding utf8
@{revision='2026-10-29-r1';status='source-only; current rich builder adaptation, WASM output pending';inventories=$ledger.inventories;coverage_limit='All empty dimensions remain unresolved, never vacuous pass. Current finite candidate inventory does not exhaust known species anatomy, behaviour or lifecycle.'} | ConvertTo-Json -Depth 40 | Set-Content -LiteralPath (Join-Path $asset 'dossier/inventory.json') -Encoding utf8
$pinPaths = @('crates/ecopunk-terrain/src/zoological_loft_builder.rs','crates/ecopunk-terrain/src/scene_builder_10d.rs','tools/species-production/swamp-wallaby-review-r1/swamp_wallaby_review_append.rs','docs/qa/implementation-2026-10-26/biological-reference/wallaby-cases.md','docs/planning/tech/assets/model-production-template/deliverables.json')
$pins = @($pinPaths | ForEach-Object { $path=Join-Path $root $_; @{path=$_;sha256=(Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant();scope=if($_ -like '*zoological*'){'Lines 1-895 and function/caller inventory; detailed SDF/loft/ear/macropod branch inspection, not full module semantic audit'} elseif($_ -like '*scene_builder*'){'SubmeshBuilder definition and call-site search; not full module semantic audit'} else {'Full authored source or documented case/template record'} } })
@{observed_at=$now;kind='source-inspection-pins';qualification='No build, asset export or execution performed by producer';pins=$pins} | ConvertTo-Json -Depth 10 | Set-Content -LiteralPath (Join-Path $asset 'dossier/source-pins.json') -Encoding utf8
Write-Output ('Created full ledger with '+$ledger.claims.Count+' base rows and '+$ledger.subject_specific_claims.Count+' wallaby cases.')
