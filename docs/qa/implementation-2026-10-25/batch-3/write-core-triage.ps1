$ErrorActionPreference='Stop'
$task='C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3'
$path="$task/core-library-canonical-tests.log"
$log=Get-Content -LiteralPath $path -Raw
$blocks=[regex]::Matches($log,'(?ms)^---- ([^\r\n]+) stdout ----\r?\n(.*?)(?=^---- |^failures:\r?$|\z)')
$records=@()
foreach($block in $blocks){
 $name=$block.Groups[1].Value
 $body=$block.Groups[2].Value.Trim()
 $category='unresolved'
 $note='Observed canonical failure. No matching unmodified full-suite baseline was run; unchanged test source alone does not establish preexisting failure.'
 $next='Owning subsystem should reconcile the exact failing assertion/unsupported contract under this selected native profile, with a bounded focused case and an original-source baseline where needed.'
 if($body -match '10013|kind: PermissionDenied'){
  $category='environment_permission_denial_proved'
  $note='Windows explicitly denied socket access (OS 10013). This proves the run encountered an OS access restriction; it does not identify whether sandbox, firewall or another policy supplied that restriction, or prove protocol correctness.'
  $next='Qualify the focused loopback case in an authorized environment permitting the needed socket operation; preserve the denied-run evidence.'
 }elseif($name -eq 'vibe_host::invoke::studio_store::tests::test_import_cml_and_csv'){
  $category='known_prior_cml_failure'
  $note='Exact None versus Some("#000") mismatch was already observed in hmc/first-test-failures.log. Current checkpoint changes add serde/routing/retained-state validation; importer body is preserved. This is a prior observed failure, not a claim that all 43 failures predate this work.'
  $next='Studio format owner should repair the line-oriented CML importer/test contract separately; preserve current accepted source semantics.'
 }elseif($name -eq 'render::gpu::material_pixel_tests::native_mask_hole_preserves_receiver_depth_in_ao_prepass'){
  $category='possible_changed_renderer_path'
  $note='Actual native AO readback assertion failed: hole=18, solid=15; required hole > solid+100. This executes PortalGpu, material defaults, texture masks and AO targets, so it overlaps the family containing the changed MR fallback texel/shader. Inspected diff changes only unbound MR B from 0 to 1 (and default byte 0 to 255); both test materials start from legacy_default metallic=0. That makes a direct metallic-factor explanation less apparent, but is not a before/after proof. Cannot waive or label preexisting.'
  $next='Render owner must run this single existing pixel test with original and changed MR shader/defaults on the same adapter/backend/fixture and inspect AO encoding/readback and alpha/depth coverage before accepting the renderer change.'
 }elseif($name -like '*indirect_cull_routes_mesh_lods_holographic_and_surfel_streams_zero_heap'){
  $category='unresolved_cpu_culling'
  $note='Assertion mesh_draw_count is 1 rather than 3. Inspected test calls cull_and_select_lod_indirect_zero_heap directly, a CPU routine over input/output slices with None HZB. It does not invoke the changed mesh fragment shader or material fallback textures. This source is outside the 11 changed core targets, but no original baseline proves it preexisting.'
  $next='Culling owner should inspect frustum planes, world radius, effective projected radius, policy and fixture camera setup in this focused CPU case; do not attribute the count to MR shader changes without new evidence.'
 }elseif($body -match 'call exceeds permitted effect class'){
  $category='unresolved_effect_contract'
  $note='E200 effect permission rejection is observed through legacy PoetSnapshot eval/load entry points. Inspected eval_cell_src calls vibe::eval_cell and eval_fn calls load_program/eval_function; neither routes through SourceSession::submit_validated. New callback additions are not a demonstrated cause; earlier checker compatibility and each test capability/effect declaration require reconciliation. No old baseline proves this preexisting.'
  $next='Vibe/host owner should reconcile this exact legacy test declaration and transitive effect contract; retain strict admission rather than bypass effects to make the test pass.'
 }elseif($body -match '10060'){
  $note='Socket timeout OS 10060 is proved; its cause is unresolved. Other tests report explicit access denial, but that context does not prove this timeout has the same cause.'
 }elseif($name -match '^net::|^p2p::'){
  $note='Network/session/capacity assertion failed without explicit OS permission evidence in this failure block. Shared environment restriction is plausible context, not an established cause for this individual result.'
 }elseif($name -like 'wgsl_forge::validate::*'){
  $note='Selected Naga validator rejects the named enable-extension in the exact message. This is a real selected-profile capability/validation mismatch; neither MR fallback nor checkpoint serialization directly changes these generated kernels. An original baseline and supported-feature reconciliation remain needed.'
 }
 $loc=[regex]::Match($body,'panicked at (.+?):(\d+):(\d+):')
 $records += [ordered]@{name=$name;classification=$category;log_header_line=($log.Substring(0,$block.Index).Split("`n").Length);source_location=if($loc.Success){$loc.Groups[1].Value+':'+$loc.Groups[2].Value+':'+$loc.Groups[3].Value}else{$null};exact_failure_output=$body;assessment=$note;next_action=$next}
}
if($records.Count -ne 43){throw "Expected 43 exact failure blocks, got $($records.Count)"}
$passGroups=@()
foreach($family in @('checkpoint::tests','journal_tests','stamped_tests')){
 $escaped=[regex]::Escape($family)
 $matches=[regex]::Matches($log,"(?m)^test (vibe_host::invoke::studio_store::$escaped`::[^\r\n]+) \.\.\. (ok|FAILED|ignored)[^\r\n]*")
 $results=@($matches | ForEach-Object {[ordered]@{name=$_.Groups[1].Value;result=$_.Groups[2].Value;line=$log.Substring(0,$_.Index).Split("`n").Length}})
 $passGroups += [ordered]@{family="vibe_host::invoke::studio_store::$family";observed_count=$results.Count;passed=@($results | Where-Object {$_.result -eq 'ok'}).Count;failed=@($results | Where-Object {$_.result -eq 'FAILED'}).Count;tests=$results}
}
$counts=[ordered]@{}
foreach($record in $records){if(-not $counts.Contains($record.classification)){$counts[$record.classification]=0};$counts[$record.classification]++}
$coreTargets=@((Get-Content "$task/hmc/apply-manifest.json" -Raw | ConvertFrom-Json).mutations+(Get-Content "$task/materials/renderer-fix/apply-manifest.json" -Raw | ConvertFrom-Json).mutations | ForEach-Object {$_.target})
$vibeTargets=@((Get-Content "$task/session-validation/apply-manifest.json" -Raw | ConvertFrom-Json).mutations | ForEach-Object {$_.target})
$ignored=@([regex]::Matches($log,'(?m)^test ([^\r\n]+) \.\.\. ignored[^\r\n]*') | ForEach-Object {$_.Value})
$report=[ordered]@{scope='Read-only triage of actual unfiltered selected-native-profile canonical core suite; no rerun/build/source edits';created_utc=[DateTime]::UtcNow.ToString('o');log_path=$path;log_sha256=(Get-FileHash $path -Algorithm SHA256).Hash;summary=[ordered]@{passed=9712;failed=43;ignored=3;duration_seconds=591.97;overall='FAILED'};classification_counts=$counts;changed_core_targets=$coreTargets;changed_vibe_targets=$vibeTargets;actual_store_test_families=$passGroups;ignored_tests=$ignored;limitations=@('No original full-suite baseline was executed; 43 failures cannot collectively be labelled preexisting.','No failing test was rerun during triage.','The AO pixel failure remains a renderer qualification blocker with possible changed-path relevance until bounded original/changed evidence exists.','Observed OS denial proves an access restriction for those individual failures; timeouts and missing sessions are not silently included in that classification.','This report does not establish complete native cooperative authoring or any model/package readiness.');failures=$records}
$report | ConvertTo-Json -Depth 12 | Set-Content "$task/core-failure-triage.json"
$lines=@('Canonical selected-profile core suite: FAILED — 9712 passed, 43 failed, 3 ignored; 591.97 seconds.','', 'Classification counts:')
foreach($key in $counts.Keys){$lines += "$key = $($counts[$key])"}
$lines += @('','Actual Studio evidence:')
foreach($group in $passGroups){$lines += "$($group.family): $($group.passed) passed / $($group.failed) failed / $($group.observed_count) observed"}
$lines += @('','No baseline/full rerun was performed. Do not label all 43 preexisting. AO pixel failure remains unresolved and may intersect the changed renderer path; indirect culling is a separate observed CPU routine failure.','','Every failure and exact output:')
foreach($record in $records){$lines += @('',"[$($record.classification)] $($record.name)","Log header line $($record.log_header_line)",$record.exact_failure_output,$record.assessment,"Next: $($record.next_action)")}
$lines | Set-Content "$task/core-failure-triage.txt"
$counts | ConvertTo-Json
$passGroups | ForEach-Object {"$($_.family): $($_.passed)/$($_.observed_count) passed"}
