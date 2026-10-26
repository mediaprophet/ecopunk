"""Reconcile saved targets, executed cases and held assets; no builds or tests."""
from pathlib import Path
import hashlib,json,re
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[5]
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def read(p):return json.loads(Path(p).read_text(encoding='utf-8-sig'))
expected={};overrides=[]
receipts=['applied-source.json','repairs/ui-keys/applied-source.json',
          'repairs/history-fixture/applied-source.json','repairs/ui-text/applied-source.json']
for name in receipts:
    for row in read(HERE/name):
        key=str(Path(row['target'])).lower()
        if key in expected:
            assert row['before_sha256']==expected[key]['after_sha256'],key
            overrides.append(dict(target=row['target'],receipt=name,before=row['before_sha256'],after=row['after_sha256']))
        expected[key]=row
assert len(expected)==70
for row in expected.values():assert sha(row['target'])==row['after_sha256'],row['target']
counts={'public':19,'producer':9,'workflow':1,'source-regression':1,'inspect-regression':6}
commands={};all_cases=[];pins=[]
for mode in ['check',*counts,'build','wasm']:
    row=read(HERE/(mode+'-result.json'))
    assert row['exit_code']==0 and row['source_unchanged'] and not row['source_changes'],mode
    assert row['test_selection_ok'],mode
    assert sha(row['log'])==row['log_sha256'],mode
    assert sha(row['source_snapshot'])==row['source_snapshot_sha256'],mode
    pins.append(read(row['source_snapshot']))
    if mode in counts:
        log=Path(row['log']).read_text(encoding='utf-8')
        parsed=[tuple(map(int,r)) for r in re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored',log)]
        names=re.findall(r'^test ([^\s]+) \.\.\. ok$',log,re.M)
        assert sum(n for n,_,_ in parsed)==counts[mode] and all(f==0 and i==0 for _,f,i in parsed),mode
        assert names==row['passed_case_names'] and len(names)==counts[mode],mode
        all_cases.extend(names)
    commands[mode]=dict(receipt=str(HERE/(mode+'-result.json')),receipt_sha256=sha(HERE/(mode+'-result.json')),tests=counts.get(mode,0),exit_code=0)
assert len(all_cases)==len(set(all_cases))==36
# Later changes to earlier tested pins must be exactly the recorded reviewed overrides.
override_paths={str(Path(r['target'])).lower() for r in overrides}
final_pin=pins[-1]
for pin in pins:
    assert pin.keys()==final_pin.keys()
    delta={str(Path(p)).lower() for p in pin if pin[p]!=final_pin[p]}
    assert delta<=override_paths,delta
for path,digest in final_pin.items():assert sha(path)==digest,path
historical=read(HERE/'public-before-fixture-repair-result.json')
assert historical['exit_code']==101
assert sha(historical['log'])==historical['log_sha256']
assert sha(historical['source_snapshot'])==historical['source_snapshot_sha256']
assets=read(HERE.parent/'round2/status-final.json')['preserved_assets']
for row in assets:
    path=ROOT/row['path']
    assert sha(path)==row['sha256'] and path.stat().st_size==row['bytes']
master=ROOT/'assets/reference/community-grounds-growing-station/master.vibe'
assert sha(master)=='8aac87e9f98dc305ea85d45f2ebae19aba5fbd9323c165f160c83bb46ce89a63'
binary=ROOT/'target/debug/poet.exe'
with binary.open('rb') as handle:assert handle.read(2)==b'MZ'
result=dict(status='passed',kind='read_only_final_reconciliation',distinct_installed_targets=70,
    all_latest_installed_hashes_match=True,recorded_overrides=overrides,commands=commands,
    unique_selected_native_tests=36,new_scope_cases=29,retained_regression_cases=7,
    earlier_63_not_added=True,initial_public_failure_retained=True,
    source_changes_since_earlier_checks='recorded test fixture and UI text overrides only; UI key repair predates qualifying commands',
    final_local_source_pin_files=len(final_pin),latest_local_source_hashes_match=True,
    native_binary=dict(path=str(binary),bytes=binary.stat().st_size,sha256=sha(binary),launched=False),
    sdk_wasm='actual Core/geospatial/Portal compiled; runtime interoperability untested',
    preserved_regression_artifacts=assets,reference_master_unchanged=True,
    private_journal_tests_authored_unrun=3,foundation_gates='FND-01–06 open',IP03='C1–C8 open',
    asset_production='on_hold',gui_verification='not_run',human_acceptance=False,
    model_claims='193 catalogue plus 8 subject-specific claims remain unassessed; all three target packages incomplete',
    builds_or_tests_executed_by_this_verifier=False)
print(json.dumps(result,indent=2,ensure_ascii=False))
