"""Verify saved-source, command receipts, preserved assets and code relocation."""
from pathlib import Path
import hashlib,json,re,datetime
ROOT=Path('C:/github/ecopunk');TOOLS=Path(__file__).resolve().parent
QA=ROOT/'docs/qa/implementation-2026-10-26/round8'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
order=json.loads((TOOLS/'application-order.json').read_text())
current={};receipts=[]
for group in order:
    folder=TOOLS/group
    manifest=folder/'manifest.json';applied=folder/'applied-source.json'
    rows=json.loads(applied.read_text())
    for row in rows:
        owner=row.get('repository',row.get('repo'));name=row.get('target',row.get('path'))
        stage=folder/'after'/owner/name
        assert sha(stage)==row['after_sha256'],str(stage)
        assert row['verified_after_sha256']==row['after_sha256']
        current[str(Path('C:/github')/owner/name)]=row['after_sha256']
    receipts.append(dict(group=group,manifest_sha256=sha(manifest),receipt_sha256=sha(applied),writes=len(rows)))
for path,digest in current.items():assert sha(Path(path))==digest,path
assets=json.loads((ROOT/'docs/qa/implementation-2026-10-25/batch-6/rounds/round2/status-final.json').read_text())['preserved_assets']
assets.append(dict(path='assets/reference/community-grounds-growing-station/master.vibe',sha256='8aac87e9f98dc305ea85d45f2ebae19aba5fbd9323c165f160c83bb46ce89a63'))
for asset in assets:assert sha(ROOT/asset['path'])==asset['sha256'],asset['path']
commands=[];names=set();attempts=[]
for path in sorted(QA.glob('*/*-result.json')):
    r=json.loads(path.read_text());attempts.append(dict(path=str(path),exit_code=r['exit_code']))
    assert sha(Path(r['log']))==r['log_sha256']
    source=Path(r['source_snapshot']);assert sha(source)==r['source_snapshot_sha256']
    assert r['source_unchanged'] and not r['source_changes']
    if r['exit_code']==0:
        assert r['test_selection_ok']
        commands.append(dict(path=str(path),mode=r['mode'],passed=sum(c[0] for c in r['test_results']),cases=len(r['passed_case_names'])))
        names.update(r['passed_case_names'])
index=ROOT/'audit/implementation-snapshots/2026-10-25/batch-6/migration-index.json'
archive=json.loads(index.read_text(encoding='utf-8-sig'))
assert archive['file_count']==733
for row in archive['files']:
    assert not (ROOT/row['original_path']).exists(),row['original_path']
    path=ROOT/row['archived_path'];assert path.stat().st_size==row['bytes'] and sha(path)==row['sha256']
assert archive['reference_diagnosis_before']['reference_sha256']==archive['reference_diagnosis_after']['reference_sha256']
links=[]
for path in sorted(QA.glob('*.md')):
    for target in re.findall(r'\[[^\]]*\]\(([^)]+)\)',path.read_text(encoding='utf-8')):
        target=target.strip('<>').split('#')[0]
        if not target or '://' in target:continue
        resolved=(path.parent/target).resolve()
        # This report's own result is created below after all other checks.
        assert resolved.exists() or resolved==QA/'saved-evidence-verification.json',(str(path),target,str(resolved))
        links.append(dict(report=path.name,target=target))
result=dict(captured_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),installed_distinct_targets=len(current),application_receipts=receipts,all_current_installed_hashes_match=True,preserved_assets=assets,successful_commands=commands,attempts=attempts,successful_case_executions=sum(c['passed'] for c in commands),distinct_case_names=len(names),archived_files_verified=733,archive_index_sha256=sha(index),historical_reference_diagnosis_unchanged=True,report_links_checked=len(links),foundation_gates='FND-01–06 and IP-03 C1–C8 remain open; asset production HOLD',boundary='No GUI/human acceptance, production asset or universal format-quality claim.')
binary=ROOT/'target/debug/poet.exe'
result['native_binary']=dict(path=str(binary),bytes=binary.stat().st_size,sha256=sha(binary))
(QA/'saved-evidence-verification.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
