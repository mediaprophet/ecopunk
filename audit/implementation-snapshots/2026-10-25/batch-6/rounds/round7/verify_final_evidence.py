"""Read-only current source, actual execution and preserved-artifact reconciliation."""
from pathlib import Path
import hashlib,json,re
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[5]
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def read(p):return json.loads(Path(p).read_text(encoding='utf-8-sig'))
def passed_names(log):
    names=[];pending=None
    for line in log.splitlines():
        m=re.match(r'^test ([^\s]+) \.\.\. (.*)$',line)
        if m:
            pending=m[1] if m[2].strip() not in ('ok','FAILED','ignored') and not m[2].strip().startswith('ignored,') else None
            if m[2].strip()=='ok':names.append(m[1])
        elif pending and line.strip()=='ok':names.append(pending);pending=None
    return names
minimum={'authoring':30,'surfaces':6,'wire':1,'detail':6,'atlas':3,'integration':4,'uvkernel':10,'image':4,'workflow':1,'legacyworkflow':1,'jobs':15,'intents':5,'inspection':8,'hmctextures':6,'gpu':1}
receipts=[HERE/'applied-source.json']
if (HERE/'repair-order.json').exists():receipts.extend(HERE/p for p in read(HERE/'repair-order.json'))
expected={};repair_paths=set();new_paths=set();overrides=[]
for i,p in enumerate(receipts):
    for row in read(p):
        k=str(Path(row['target'])).lower()
        if k in expected:
            assert expected[k]['after_sha256']==row['before_sha256'],k
            overrides.append(dict(target=row['target'],receipt=str(p),before=row['before_sha256'],after=row['after_sha256']))
        if i:repair_paths.add(k)
        if i and row['before_sha256'] is None:new_paths.add(k)
        expected[k]=row
assert len(read(HERE/'applied-source.json'))==58
for row in expected.values():assert sha(row['target'])==row['after_sha256'],row['target']
commands={};pins=[];cases=[]
for mode in ['check',*minimum,'build','wasm']:
    p=HERE/(mode+'-result.json');r=read(p)
    assert r['exit_code']==0 and r['source_unchanged'] and not r['source_changes'] and r['test_selection_ok'],mode
    assert sha(r['log'])==r['log_sha256'] and sha(r['source_snapshot'])==r['source_snapshot_sha256'],mode
    pins.append(read(r['source_snapshot']));n=0
    if mode in minimum:
        log=Path(r['log']).read_text(encoding='utf-8');results=[tuple(map(int,x)) for x in re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored',log)]
        names=passed_names(log);n=sum(v for v,_,_ in results)
        assert n>=minimum[mode] and all(f==0 and i==0 for _,f,i in results),mode
        assert names==r['passed_case_names'] and len(names)==n,mode
        cases.extend(names)
    commands[mode]=dict(receipt=str(p),receipt_sha256=sha(p),passing_case_executions=n,exit_code=0)
assert any(n.endswith('uv_atlas_paint_native_intents_resource_closure_review_and_reopen') for n in cases)
assert any(n.endswith('authored_atlas_paint_and_procedural_detail_change_actual_native_pixels') for n in cases)
final=pins[-1]
for pin in pins:
    assert pin.keys()<=final.keys()
    missing={str(Path(p)).lower() for p in final.keys()-pin.keys()};assert missing<=new_paths,missing
    changed={str(Path(p)).lower() for p in pin if pin[p]!=final[p]};assert changed<=repair_paths,changed
for p,h in final.items():assert sha(p)==h,p
failed=[]
for p in sorted(HERE.glob('*-before-*-result.json')):
    r=read(p);assert r['exit_code']!=0 and r['source_unchanged'];assert sha(r['log'])==r['log_sha256'] and sha(r['source_snapshot'])==r['source_snapshot_sha256']
    failed.append(dict(receipt=str(p),sha256=sha(p),exit_code=r['exit_code']))
images=[]
for p in sorted((HERE/'gpu-calibration').glob('*.png')):
    b=p.read_bytes();assert b[:8]==bytes([137,80,78,71,13,10,26,10]);assert int.from_bytes(b[16:20],'big')==64 and int.from_bytes(b[20:24],'big')==64
    assert len(b)<=32768;images.append(dict(path=str(p),bytes=len(b),sha256=sha(p)))
assert len(images)==3 and sum(r['bytes'] for r in images)<=512*1024
gpu=Path(read(HERE/'gpu-result.json')['log']).read_text();measures=re.findall(r'native_atlas_(paint|procedural) changed_pixels=(\d+)',gpu)
assert len(measures)==2 and all(int(n)>=4 for _,n in measures)
assets=read(HERE.parent/'round2/status-final.json')['preserved_assets']
for r in assets:p=ROOT/r['path'];assert sha(p)==r['sha256'] and p.stat().st_size==r['bytes']
master=ROOT/'assets/reference/community-grounds-growing-station/master.vibe';assert sha(master)=='8aac87e9f98dc305ea85d45f2ebae19aba5fbd9323c165f160c83bb46ce89a63'
binary=ROOT/'target/debug/poet.exe'
with binary.open('rb') as f:assert f.read(2)==b'MZ'
result=dict(status='passed',kind='read_only_final_reconciliation',distinct_installed_targets=len(expected),all_latest_installed_hashes_match=True,recorded_overrides=overrides,
    commands=commands,successful_command_count=len(commands),selected_native_case_executions=len(cases),unique_selected_native_tests=len(set(cases)),duplicate_executions=len(cases)-len(set(cases)),previous_rounds_not_added=True,
    final_local_source_pin_files=len(final),latest_local_source_hashes_match=True,failed_runs_retained=failed,gpu_calibration=images,native_pixel_changes=dict(measures),
    native_binary=dict(path=str(binary),bytes=binary.stat().st_size,sha256=sha(binary),launched=False),sdk_wasm='actual Core/geospatial/Portal compilation; runtime untested',
    preserved_regression_artifacts=assets,reference_master_unchanged=True,foundation_gates='FND-01–06 open',IP03='C1–C8 open',asset_production='on_hold',gui_verification='not_run',human_acceptance=False,
    model_claims='193 catalogue plus eight subject-specific claims remain unassessed; film/game/cartoon packages incomplete',builds_or_tests_executed_by_this_verifier=False)
print(json.dumps(result,indent=2,ensure_ascii=True))
