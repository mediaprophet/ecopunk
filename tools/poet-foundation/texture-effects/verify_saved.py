"""Verify saved W2 source, actual serial results and retained diagnostics."""
from pathlib import Path
from datetime import datetime, timezone
from urllib.parse import unquote
import argparse
import hashlib
import json
import re

ROOT = Path('C:/github/ecopunk')
QA = ROOT / 'docs/qa/implementation-2026-10-26/poet-texture-effects'
WORK = ROOT / 'tools/poet-foundation/texture-effects/work'
def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

parser = argparse.ArgumentParser()
parser.add_argument('--test-exit', required=True, type=int)
parser.add_argument('--build-exit', required=True, type=int)
parser.add_argument('--test-log', default='check-attempt-2.log')
args = parser.parse_args()
assert args.test_exit == args.build_exit == 0
manifest = json.loads((WORK / 'integration/install-manifest.json').read_text(encoding='utf-8'))
receipt = json.loads((QA / 'applied-source.json').read_text(encoding='utf-8'))
assert receipt['completed'] and len(manifest['files']) == len(receipt['files']) == 14
receipt_hashes = {Path(r['path']).as_posix().lower(): r['sha256_after'] for r in receipt['files']}
saved = []
for row in manifest['files']:
    canonical = Path(row['destination'])
    actual = sha(canonical)
    assert actual == row['sha256_after'] == sha(Path(row['staged']))
    assert receipt_hashes[canonical.as_posix().lower()] == actual
    saved.append({'path': str(canonical), 'sha256': actual})
testlog = (QA / args.test_log).read_text(encoding='utf-8')
summary = re.search(r'test result: ok\. (\d+) passed; 0 failed; (\d+) ignored; (\d+) measured; (\d+) filtered out', testlog)
assert summary, 'Serial library test command did not report a pass'
effect_cases = re.findall(r'^test (.*texture_effect.*) \.\.\. ok$', testlog, flags=re.M)
assert len(effect_cases) >= 15
buildlog = (QA / 'native-build.log').read_text(encoding='utf-8')
assert re.search(r'Finished `dev` profile', buildlog)
binary = ROOT / 'target/debug/poet.exe'
assert binary.is_file()
assert binary.stat().st_mtime >= max(Path(row['destination']).stat().st_mtime for row in manifest['files'])
assert not (ROOT / 'target/coordination/native-check.lock').exists()
witness = json.loads((QA / 'diagnostic/witness.json').read_text(encoding='utf-8'))
diagnostics = []
for name, key in [('original.png', 'base_sha256'), ('effects.png', 'output_sha256'), ('reopened.png', 'output_sha256')]:
    p = QA / 'diagnostic' / name
    assert p.read_bytes().startswith(b'\x89PNG\r\n\x1a\n')
    assert sha(p) == witness[key]
    diagnostics.append({'path': str(p), 'sha256': sha(p), 'bytes': p.stat().st_size})
deferred = json.loads((WORK / 'deferred-functionality.json').read_text(encoding='utf-8'))
assert len(deferred['items']) == len({r['id'] for r in deferred['items']}) == 15
assert sha(Path(deferred['owner'])) == deferred['sha256'].lower()
register = json.loads((ROOT / 'docs/planning/biosphere-work-packets.json').read_text(encoding='utf-8'))
wave = next(w for w in register['active_implementation_waves'] if w['id'] == 'POET-TEXTURE-W2')
assert wave['asset_production'] == 'HOLD'
json_paths = list(QA.glob('*.json')) + list(WORK.glob('*.json')) + [WORK / 'integration/install-manifest.json']
for p in json_paths:
    json.loads(p.read_text(encoding='utf-8'))
links = 0
for p in [QA / 'README.md', QA / 'try-it.md', ROOT / 'docs/planning/poet-authoring-swarm-phases.md']:
    for href in re.findall(r'\]\(([^)]+)\)', p.read_text(encoding='utf-8')):
        href = unquote(href.strip('<>')).split('#')[0]
        if not href or re.match(r'https?://', href):
            continue
        target = (p.parent / href).resolve()
        if target == (QA / 'verification.json').resolve():
            continue
        assert target.exists(), str(p) + ': ' + href
        links += 1
report = {
    'schema': 'ecopunk-poet-texture-effects-verification/v1',
    'date_utc': datetime.now(timezone.utc).isoformat(),
    'scope': 'POET-TEXTURE-W2 selected software cases, not whole-product/asset acceptance',
    'canonical_files': saved,
    'tests': {'exit': args.test_exit, 'log': args.test_log, 'summary': summary.group(0), 'selected_effect_cases': effect_cases},
    'prior_attempt': {'session_id': 68375, 'exit': 101, 'log': 'check-attempt-1.log', 'result': '266 passed, 2 failed, 9 ignored; unit-enum decoder accepted extra fields. Strict decoding repair saved; original negative tests retained.'},
    'execution_sessions': {'final_test': 30492, 'native_build': 12171},
    'commands': {
        'test': 'cargo test --manifest-path C:/github/poet/Cargo.toml -p poet-cli --features native-graphical --locked --offline --target-dir C:/github/ecopunk/target --lib -- --test-threads=1',
        'build': 'cargo build --manifest-path C:/github/poet/Cargo.toml -p poet-cli --features native-graphical --locked --offline --target-dir C:/github/ecopunk/target --bin poet',
        'execution': 'Root-only serial_check.py lease; canonical sources frozen during checks; tests opted into an existing diagnostic evidence directory.',
    },
    'native_build': {'exit': args.build_exit, 'log': 'native-build.log', 'path': str(binary), 'sha256': sha(binary), 'bytes': binary.stat().st_size},
    'diagnostics': diagnostics,
    'diagnostic_scope': witness['qualification'],
    'deferred_functionality': {'inventory': str(WORK / 'deferred-functionality.json'), 'cases': 15, 'status': 'deferred_not_dispatched', 'core_source_pin_unchanged': True},
    'checks': {'local_links_resolved': links, 'json_files_parsed': len(json_paths), 'check_lease': 'absent after completed root-owned commands'},
    'source_reuse': 'Canonical Qualia image operations, source-over, codecs, resource/source/session/history owners reused; no external shell/kernel copied, no new JS/Boa/state store/dependency or HMC wire format.',
    'preservation': 'Preimage-pinned installs and backups; prior W1 receipts retained as historical evidence; no clean global sibling Git-state claim.',
    'usage_metrics': 'unknown; collaboration tools provide no per-agent token/currency totals',
    'remaining': ['Human/current-window and actual GPU scene-quality qualification', 'Full image compositor/RAW/ICC/linear-light workflows', '15 recorded source stubs/ignored fields plus other unqualified operations', 'Simultaneous distinct-principal cooperation unqualified', 'Whole IP-03 C1-C8 and model/film/game/cartoon acceptance'],
    'asset_production': 'HOLD; only synthetic engineering PNG evidence retained; no production assets, commits, publication or payments',
}
(QA / 'verification.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8', newline='\n')
print(summary.group(0))
print('Verified 14 saved canonical files, selected effect cases, native binary, actual diagnostic PNG digests and 15 deferred cases.')
