"""Read-only source, executed-case and retained-fixture reconciliation."""
from pathlib import Path
import hashlib,json,re
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[5]
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def read(p):return json.loads(Path(p).read_text(encoding='utf-8-sig'))
def passed_names(logtext):
    names=[];pending=None
    for line in logtext.splitlines():
        match=re.match(r'^test ([^\s]+) \.\.\. (.*)$',line)
        if match:
            pending=match[1] if not match[2].strip() else None
            if match[2].strip()=='ok':names.append(match[1])
        elif pending and line.strip()=='ok':
            names.append(pending);pending=None
    return names

expected={};overrides=[]
receipts=[HERE/'applied-source.json']
repair_list=HERE/'repair-order.json'
if repair_list.exists():receipts.extend(HERE/p for p in read(repair_list))
for receipt in receipts:
    for row in read(receipt):
        key=str(Path(row['target'])).lower()
        if key in expected:
            assert row['before_sha256']==expected[key]['after_sha256'],key
            overrides.append(dict(target=row['target'],receipt=str(receipt),before=row['before_sha256'],after=row['after_sha256']))
        expected[key]=row
assert len(expected)==37
for row in expected.values():assert sha(row['target'])==row['after_sha256'],row['target']
counts={'authoring':14,'typed':8,'fiel':3,'workflow':1,'intents':5,'jobs':13,'inspection':6}
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
        names=passed_names(log)
        assert sum(n for n,_,_ in parsed)==counts[mode] and all(f==0 and i==0 for _,f,i in parsed),mode
        assert names==row['passed_case_names'] and len(names)==counts[mode],mode
        all_cases.extend(names)
    commands[mode]=dict(receipt=str(HERE/(mode+'-result.json')),receipt_sha256=sha(HERE/(mode+'-result.json')),tests=counts.get(mode,0),exit_code=0)
assert len(all_cases)==50 and len(set(all_cases))==49
assert 'graphical::native_project::tests::spatial_workflow_tests::spatial_authoring_native_intents_compile_typed_inspect_and_separate_package_review_reopen' in all_cases
override_paths={str(Path(r['target'])).lower() for r in overrides};final_pin=pins[-1]
for pin in pins:
    assert pin.keys()==final_pin.keys()
    delta={str(Path(p)).lower() for p in pin if pin[p]!=final_pin[p]};assert delta<=override_paths,delta
for path,digest in final_pin.items():assert sha(path)==digest,path
historical=[]
for mode in ['authoring-before-runtime-repair','workflow-before-selection-repair']:
    row=read(HERE/(mode+'-result.json'))
    assert row['exit_code']==101 and row['source_unchanged']
    assert sha(row['log'])==row['log_sha256']
    assert sha(row['source_snapshot'])==row['source_snapshot_sha256']
    historical.append(str(HERE/(mode+'-result.json')))
assets=read(HERE.parent/'round2/status-final.json')['preserved_assets']
for row in assets:
    path=ROOT/row['path'];assert sha(path)==row['sha256'] and path.stat().st_size==row['bytes']
master=ROOT/'assets/reference/community-grounds-growing-station/master.vibe'
assert sha(master)=='8aac87e9f98dc305ea85d45f2ebae19aba5fbd9323c165f160c83bb46ce89a63'
binary=ROOT/'target/debug/poet.exe'
with binary.open('rb') as handle:assert handle.read(2)==b'MZ'
result=dict(status='passed',kind='read_only_final_reconciliation',distinct_installed_targets=37,
    all_latest_installed_hashes_match=True,recorded_overrides=overrides,commands=commands,
    selected_native_case_executions=50,unique_selected_native_tests=49,duplicated_inspection_job_case_count=1,
    previous_rounds_not_added=True,final_local_source_pin_files=len(final_pin),latest_local_source_hashes_match=True,
    failed_runs_retained=historical,
    native_binary=dict(path=str(binary),bytes=binary.stat().st_size,sha256=sha(binary),launched=False),
    sdk_wasm='actual Core/geospatial/Portal compilation; runtime interoperability untested',
    preserved_regression_artifacts=assets,reference_master_unchanged=True,
    foundation_gates='FND-01–06 open',IP03='C1–C8 open',asset_production='on_hold',gui_verification='not_run',human_acceptance=False,
    model_claims='193 catalogue plus eight subject-specific claims remain unassessed; all three target packages incomplete',
    builds_or_tests_executed_by_this_verifier=False)
print(json.dumps(result,indent=2,ensure_ascii=True))
