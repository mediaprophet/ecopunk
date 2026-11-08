"""Check the saved demo inventory, claim projections, source copies and resource revision.

Mesh decoding and HMC entry checks are performed by the native exporter, not
reimplemented here. This script records byte provenance for the saved outputs.
"""
import csv
import hashlib
import json
from pathlib import Path
import re
import sys
import subprocess

repo = Path(__file__).resolve().parents[1]
root = Path(sys.argv[1]).resolve()
evidence = repo / 'docs/qa/implementation-2026-10-29/demo-rebuild'

def digest(path):
    result = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            result.update(chunk)
    return result.hexdigest()

rows = list(csv.DictReader((root / 'inventory.tsv').open(encoding='utf-8-sig'), delimiter='\t'))
assert len(rows) == 41
template = json.loads((repo / 'docs/planning/tech/assets/model-production-template/claims.template.json').read_text(encoding='utf-8-sig'))
for row in rows:
    resource = Path(row['resource'])
    assert not resource.is_absolute() and '..' not in resource.parts
    assert (root / resource).is_file() and int(row['vertices']) > 0 and int(row['triangles']) > 0
    ledger = json.loads((root / 'dossier/cases' / resource.stem / 'claims.json').read_text())
    expected = dict(template)
    expected['document_kind'] = 'claim_ledger'
    expected['subject'] = ledger['subject']
    assert ledger == expected, f'Claim defaults changed: {resource}'
    assert digest(root / resource) in (root / 'dossier/cases' / resource.stem / 'README.md').read_text()
source_count = 0
for source in (root / 'sources').rglob('*'):
    if source.is_file():
        relative = source.relative_to(root / 'sources')
        assert digest(source) == digest(repo / relative), f'Packaged source differs: {relative}'
        source_count += 1
assert source_count == 6
assert (repo / 'index.html').read_bytes() == (repo / 'viewer/index.html').read_bytes()
html = (repo / 'index.html').read_text(encoding='utf-8-sig')
revision = re.search(r"const ASSET_REVISION = '([a-f0-9]+)'", html).group(1)
expected_revision = digest(repo / 'viewer/pkg/ecopunk_wasm_bg.wasm')[:16] + digest(repo / 'viewer/pkg/ecopunk_wasm.js')[:16]
assert revision == expected_revision

paths = [p for p in root.rglob('*') if p.is_file()]
paths += [repo / p for p in ['index.html', 'viewer/index.html', 'Cargo.lock', 'Cargo.toml',
    'crates/ecopunk-wasm/Cargo.toml', 'crates/ecopunk-terrain/Cargo.toml',
    'crates/ecopunk-wasm/src/lib.rs', 'crates/ecopunk-wasm/src/bin/rebuild_demo_assets.rs',
    'crates/ecopunk-terrain/src/scene_builder_10d.rs', 'crates/ecopunk-terrain/src/botanical_anatomy.rs',
    'crates/ecopunk-terrain/src/zoological_anatomy.rs', 'scripts/build-wasm.ps1',
    'scripts/build-demo-native.ps1', 'scripts/update-demo-version.ps1',
    'scripts/record-demo-claims.py', 'scripts/verify-demo-output.py',
    'viewer/pkg/ecopunk_wasm.js', 'viewer/pkg/ecopunk_wasm_bg.wasm']]
paths += [p for p in evidence.glob('*.jpg')]
core = Path('C:/github/qualiaDB')
core_changed = ['crates/qualia-core-db/src/render/anatomy/webgl2.rs',
    'crates/qualia-core-db/src/render/portal/mod.rs',
    'crates/qualia-core-db/src/render/portal/scene_animation.rs']
new_core_source = 'crates/qualia-core-db/src/render/portal/part_vertices.rs'
paths += [core / p for p in core_changed + [new_core_source,
    'crates/qualia-core-db/src/render/portal/cpu_mesh.rs',
    'crates/qualia-core-db/src/render/portal/graphics.rs',
    'crates/qualia-core-db/src/render/camera.rs',
    'crates/qualia-core-db/src/shaders/viewport/mesh.wgsl']]
patch = subprocess.run(['git', '-C', str(core), 'diff', '00d551ba66068bdfb6ce7f31abc95532c747ff21', '--', *core_changed, new_core_source],
    capture_output=True, text=True, check=True).stdout
(evidence / 'qualiadb-rendering-remediation.patch').write_text(patch, encoding='utf-8')
paths += [evidence / 'qualiadb-rendering-remediation.patch', evidence / 'cpu-replay-tests.rs']
with (evidence / 'source-pins.tsv').open('w', encoding='utf-8') as stream:
    stream.write('repository\thead\tqualification\n')
    for checkout in [repo, core, Path('C:/github/etalon')]:
        head = subprocess.run(['git', '-C', str(checkout), 'rev-parse', 'HEAD'],
            capture_output=True, text=True, check=True).stdout.strip()
        stream.write(f'{checkout.as_posix()}\t{head}\tworking tree may contain additional changes; use file digests and build lock\n')
with (evidence / 'artifact-digests.tsv').open('w', encoding='utf-8', newline='') as stream:
    writer = csv.writer(stream, delimiter='\t')
    writer.writerow(['path', 'bytes', 'sha256'])
    for path in sorted(set(paths)):
        writer.writerow([path.as_posix(), path.stat().st_size, digest(path)])
print(f'PASS: {len(rows)} inventory meshes; complete default claim projections; {source_count} current packaged sources; identical HTML routes; revision {revision}.')
print(f'HMC: {root.joinpath("ecopunk-demo.hmc").stat().st_size} bytes; SHA256 {digest(root / "ecopunk-demo.hmc")}')
print('Mesh/HMC semantic verification remains the native exporter log; scientific and human acceptance are not awarded here.')
