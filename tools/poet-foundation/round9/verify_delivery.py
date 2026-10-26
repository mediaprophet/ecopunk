"""Verify canonical saved sources, exact executed selections and retained diagnostics."""
from pathlib import Path
import json,hashlib,datetime
ROOT=Path('C:/github/ecopunk');TOOLS=Path(__file__).resolve().parent
QA=ROOT/'docs/qa/implementation-2026-10-26/round9';FIX=QA/'final/reference-output'
def sha(p):
    digest=hashlib.sha256()
    with p.open('rb') as f:
        for chunk in iter(lambda:f.read(1024*1024),b''):digest.update(chunk)
    return digest.hexdigest()
current={};applications=[]
for group in json.loads((TOOLS/'application-order.json').read_text()):
    folder=TOOLS/group;manifest=folder/'manifest.json';receipt=folder/'applied-source.json'
    rows=json.loads(receipt.read_text())
    for row in rows:
        owner=row['repository'];name=row['target'];stage=folder/'after'/owner/name
        assert sha(stage)==row['after_sha256']==row['verified_after_sha256'],str(stage)
        current[str(Path('C:/github')/owner/name)]=row['after_sha256']
    applications.append(dict(group=group,files=len(rows),manifest_sha256=sha(manifest),receipt_sha256=sha(receipt)))
for path,digest in current.items():assert sha(Path(path))==digest,path
selections={'reference':'final','authoring':'final','viewport':'final','input':'inspector-final','workbench':'final','gpu':'final','jobs':'final','workflow':'final','checkpoints':'final','carrier':'final','profile':'inspector-final','intents':'inspector-final','draft':'inspector-final','build':'inspector-complete'}
cases=set();checks=[]
for mode,attempt in selections.items():
    path=QA/attempt/(mode+'-result.json');r=json.loads(path.read_text())
    assert r['exit_code']==0 and r['test_selection_ok'] and r['source_unchanged'] and not r['source_changes'],str(path)
    assert sha(Path(r['log']))==r['log_sha256']
    assert sha(Path(r['source_snapshot']))==r['source_snapshot_sha256']
    if mode!='build':assert sum(n[0] for n in r['test_results'])==len(r['passed_case_names'])>0
    cases.update(r['passed_case_names']);checks.append(dict(mode=mode,receipt=str(path),passed=sum(n[0] for n in r['test_results']),source_snapshot_sha256=r['source_snapshot_sha256']))
baseline=json.loads((ROOT/'docs/qa/implementation-2026-10-25/batch-6/rounds/round2/status-final.json').read_text())['preserved_assets']
baseline.append(dict(path='assets/reference/community-grounds-growing-station/master.vibe',sha256='8aac87e9f98dc305ea85d45f2ebae19aba5fbd9323c165f160c83bb46ce89a63'))
for row in baseline:assert sha(ROOT/row['path'])==row['sha256'],row['path']
artifacts=[]
for path in sorted(FIX.iterdir()):
    if path.is_file():artifacts.append(dict(path=str(path),bytes=path.stat().st_size,sha256=sha(path)))
observed=json.loads((FIX/'observed.json').read_text());reopened=json.loads((FIX/'reopened.json').read_text())
assert observed['parts']==reopened['parts']==199
assert observed['source_cells']==reopened['source_cells']==5
assert observed['production_acceptance']==reopened['production_acceptance']=='unassessed-HOLD'
assert (FIX/'diagnostic-equipment.hmc').stat().st_size==observed['hmc_bytes']
source=FIX/'accepted-source.vibe';assert sha(source)==observed['history_sha256']
ledger_path=ROOT/'docs/production/community-grounds-growing-station/claims.json'
ledger=json.loads(ledger_path.read_text());assert len(ledger['claims'])==193 and len(ledger['subject_specific_claims'])==8
assert all(c['assessment']['claim_status']=='unassessed' for c in ledger['claims']+ledger['subject_specific_claims'])
# Separate fixture ledger: retain every catalogue row and all three unaccepted profiles.
ledger['subject']['dossier_id']='community-grounds-growing-station-v0.1-round9-diagnostic'
ledger['subject']['source_revision']='Unchanged master.vibe0.2 SHA256 '+observed['master_sha256']+'; R9 diagnostic five-cell history SHA256 '+observed['history_sha256']
ledger['subject']['package_content_refs'] += ['../../qa/implementation-2026-10-26/round9/final/reference-output/diagnostic-equipment.hmc','../../qa/implementation-2026-10-26/round9/final/reference-output/accepted-source.vibe']
reference_receipt=QA/'final/reference-result.json'
reference_run=json.loads(reference_receipt.read_text())
ledger['evidence_index']=[dict(id='R9-NATIVE-EQUIPMENT-ROUNDTRIP',kind='format_check',artifact_refs=[str(FIX/'diagnostic-equipment.hmc'),str(FIX/'accepted-source.vibe')],input_refs=[str(ROOT/'assets/reference/community-grounds-growing-station/master.vibe')],method_ref=str(reference_receipt),result='Executed native 199-part source-layer/edit/viewport/HMC/fresh-process fixture; exact results and limitations in the linked command receipt and progress report.',status='passed',location=str(reference_receipt),content_sha256=sha(reference_receipt),issued_by='Root software agent',at=reference_run['finished_utc'],scope='Engineering fixture only. No actual native-window/operator, artistic, engineering, rights or complete-package acceptance.')]
ledger['review_index']=[]
for claim in ledger['subject_specific_claims']:
    if claim['deliverable_id']=='CGGS-01':
        claim['assessment']['remaining_work']='Scoped native source-to-HMC/reopen engineering evidence now exists in evidence_index. Actual contributor/reviewer usability, applicability, required visual/domain/rights and all profile acceptance remain unassessed.'
fixture_ledger=ledger_path.with_name('claims-round9-diagnostic.json')
fixture_ledger.write_text(json.dumps(ledger,indent=2)+'\n')
binary=ROOT/'target/debug/poet.exe'
result=dict(captured_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),installed_distinct_files=len(current),applications=applications,all_current_source_hashes_match=True,selected_checks=checks,distinct_selected_case_names=len(cases),observed=observed,reopened=reopened,artifacts=artifacts,preserved_assets=baseline,native_binary=dict(path=str(binary),bytes=binary.stat().st_size,sha256=sha(binary)),fixture_ledger=dict(path=str(fixture_ledger),sha256=sha(fixture_ledger),catalogue_claims=193,subject_specific_claims=8,all_claims='unassessed',film='incomplete',open_world_game='incomplete',cartoon='incomplete'),boundary='Native software/GPU engineering evidence; no Computer Use, actual operator or artistic/human acceptance. Production HOLD. FND and IP-03 full gates remain open.')
(QA/'saved-evidence-verification.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(dict(installed_files=len(current),selected_test_names=len(cases),reference_parts=observed['parts'],vertices=observed['vertices'],triangles=observed['triangles'],source_cells=observed['source_cells'],native_binary_sha256=result['native_binary']['sha256'],all_current_hashes_match=True)))
