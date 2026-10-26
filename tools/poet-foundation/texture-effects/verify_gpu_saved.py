"""Record actual selected W2 component/GPU checks and current source pins."""
from pathlib import Path
from datetime import datetime, timezone
import argparse, hashlib, json, re, tomllib

ROOT = Path('C:/github/ecopunk')
BASE = ROOT / 'tools/poet-foundation/texture-effects'
QA = ROOT / 'docs/qa/implementation-2026-10-26/poet-texture-effects/gpu-qualification'
def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()
parser = argparse.ArgumentParser()
parser.add_argument('--library-exit', type=int, required=True)
parser.add_argument('--build-exit', type=int, required=True)
args = parser.parse_args()
assert args.library_exit == args.build_exit == 0
sources = {}
for manifest_path in [BASE / 'work/integration/install-manifest.json', BASE / 'work/gpu-qualification/integration/install-manifest.json']:
    for row in json.loads(manifest_path.read_text(encoding='utf-8'))['files']:
        sources[row['path']] = row
assert len(sources) == 16
for row in sources.values():
    assert sha(Path(row['destination'])) == row['sha256_after'] == sha(Path(row['staged']))
receipt = json.loads((QA / 'applied-source.json').read_text(encoding='utf-8'))
assert receipt['completed'] and len(receipt['files']) == 4
results = []
for filename, passed, session in [
    ('baseline-gpu.log', 1, 58623), ('component-attempt-1.log', 3, 28052),
    ('native-gpu-regressions.log', 4, 87057), ('maps-uv-gpu.log', 1, 20869),
    ('retained-gpu-attempt-2.log', 1, 92095), ('library-regressions.log', 271, 91552),
]:
    log = (QA / filename).read_text(encoding='utf-8')
    match = re.search(r'test result: ok\. (\d+) passed; 0 failed; (\d+) ignored; (\d+) measured; (\d+) filtered out', log)
    assert match and int(match.group(1)) == passed, filename
    results.append({'log': filename, 'session_id': session, 'exit': 0, 'summary': match.group(0)})
gpu_log = (QA / 'retained-gpu-attempt-2.log').read_text(encoding='utf-8')
assert 'texture_effect_gpu_parent_and_child_passed' in gpu_log
assert gpu_log.count('changed_pixels=300') == 5
assert 'Finished `dev` profile' in (QA / 'native-build.log').read_text(encoding='utf-8')
binary = ROOT / 'target/debug/poet.exe'
assert binary.stat().st_mtime >= max(Path(r['destination']).stat().st_mtime for r in sources.values())
assert not (ROOT / 'target/coordination/native-check.lock').exists()
witness = json.loads((QA / 'attempt-2/gpu-witness.json').read_text(encoding='utf-8'))
artifacts = []
for key, name in [('baseline','parent-baseline-gpu.png'), ('grayscale','parent-grayscale-gpu.png'),
                  ('invert','parent-invert-gpu.png'), ('sepia','parent-sepia-gpu.png'),
                  ('final','parent-final-gpu.png'), ('child','child-reopened-gpu.png')]:
    p = QA / 'attempt-2' / name
    assert sha(p) == witness['gpu_png_sha256'][key]
    assert p.read_bytes().startswith(b'\x89PNG\r\n\x1a\n')
    assert int.from_bytes(p.read_bytes()[16:20], 'big') == 64
    assert int.from_bytes(p.read_bytes()[20:24], 'big') == 64
    artifacts.append({'path': str(p), 'sha256': sha(p)})
assert witness['gpu_png_sha256']['final'] == witness['gpu_png_sha256']['child']
lock = Path('C:/github/poet/Cargo.lock')
resolved = [dict(name=p['name'], version=p['version'], source=p.get('source','local/path'))
            for p in tomllib.loads(lock.read_text(encoding='utf-8'))['package'] if p['name'] in ('etalon','wgpu','naga')]
deferred = json.loads((BASE / 'work/deferred-functionality.json').read_text(encoding='utf-8'))
assert len(deferred['items']) == 15 and deferred['status'] == 'deferred_not_dispatched'
report = {
    'schema':'ecopunk-poet-w2-gpu-qualification/v1', 'date_utc':datetime.now(timezone.utc).isoformat(),
    'scope':'Selected Wave2 software profile; actual component events, native GPU and fresh-process source/resource recovery',
    'canonical_sources':[{'path':r['destination'],'sha256':r['sha256_after']} for r in sources.values()],
    'passed_checks':results,
    'prior_failure':{'log':'retained-gpu-attempt-1.log','session_id':16570,'exit':101,'cause':'Parent test retained admission/session snapshots while child acquired the authority journal. Parent lifetime scoped correctly; journal guard and all GPU oracles unchanged.'},
    'native_build':{'exit':args.build_exit,'log':'native-build.log','path':str(binary),'sha256':sha(binary),'bytes':binary.stat().st_size},
    'artifacts':artifacts, 'witness':witness,
    'backend':{'effect_and_dependency_runs':'strict QUALIA_RENDER_WGPU_BACKEND=dx12; inherited by fresh child','baseline':'automatic native device selection','adapter_name_and_device_class':'unavailable; not inferred','proof_sources':[{'path':str(p),'sha256':sha(p)} for p in [Path('C:/github/qualiaDB/crates/qualia-core-db/src/render/gpu/portal_gpu/constructors.rs'),Path('C:/github/qualiaDB/crates/qualia-core-db/src/render/gpu/native_device.rs')]]},
    'dependency_resolution':{'lock_path':str(lock),'lock_sha256':sha(lock),'selected_entries':resolved,'caveat':'Resolved Etalon Git source, not a claim that the current local Etalon worktree was compiled.'},
    'default_ignored':'10 default-ignored GPU tests; 7 explicitly executed successfully across the recorded runs. Biological branching and reference parent/child diagnostics remain unrun in this qualification.',
    'deferred_cases':{'count':15,'status':'deferred_not_dispatched','inventory':str(BASE/'work/deferred-functionality.json')},
    'remaining':['Actual native-window layout/accessibility and human acceptance','Separate human/reviewer/domain approval; whole IP-03 C1-C8 open','Broader compositor/RAW/ICC/linear-light profiles and the separately tracked image cases'],
    'usage_metrics':'unknown; no per-agent token/currency totals available',
    'asset_production':'HOLD; synthetic engineering inputs/readbacks only; no commits, publication or payment',
}
(QA / 'verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
print('Verified 16 current canonical sources, 7 explicit GPU passes, component/library checks, native binary and exact parent/child GPU PNG equality.')
