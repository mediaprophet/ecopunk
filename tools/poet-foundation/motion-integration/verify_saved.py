"""Record bounded coordinator verification after the serial test/build exit."""
from pathlib import Path
from datetime import datetime, timezone
from urllib.parse import unquote
import argparse
import hashlib
import json
import re

ROOT = Path('C:/github/ecopunk')
QA = ROOT / 'docs/qa/implementation-2026-10-26/poet-motion-integration'
STAGE = ROOT / 'tools/poet-foundation/motion-integration/work/integration'
def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

parser = argparse.ArgumentParser()
parser.add_argument('--test-exit', required=True, type=int)
parser.add_argument('--build-exit', required=True, type=int)
args = parser.parse_args()
assert args.test_exit == args.build_exit == 0, 'A failed command cannot be recorded as passed'
manifest = json.loads((STAGE / 'install-manifest.json').read_text(encoding='utf-8'))
receipt = json.loads((QA / 'applied-source.json').read_text(encoding='utf-8'))
assert receipt['completed'] and len(manifest['files']) == len(receipt['files']) == 16
receipt_hashes = {Path(r['path']).as_posix().lower(): r['sha256_after'] for r in receipt['files']}
files = []
for row in manifest['files']:
    canonical = Path(row['destination'])
    actual = sha(canonical)
    assert actual == row['sha256_after'] == sha(Path(row['staged']))
    assert receipt_hashes[canonical.as_posix().lower()] == actual
    files.append({'path': str(canonical), 'sha256': actual})

specs = {
    'docs/Biosphere algorithms Specifications.md': '883fe0ecf692d6a9b6d55870ef28c729df125fc550e5f506e20343176d5c5afc',
    'docs/planning/tech/Biosphere algorithms Specifications v2.md': '986da7a1c23319a272481a78e5161b5b68d11d6aa5adb291771b92550dbf5d07',
}
for path, expected in specs.items():
    assert sha(ROOT / path) == expected, 'Reviewed specification changed'
testlog = (QA / 'check-attempt-3.log').read_text(encoding='utf-8')
buildlog = (QA / 'native-build.log').read_text(encoding='utf-8')
assert 'test result: ok. 13 passed; 0 failed' in testlog
assert 'signed_material_motion_ui_mcp_history_and_hmc_reopen ... ok' in testlog
assert 'signed_material_motion_rejects_stale_observer_revoked_and_malformed_edits ... ok' in testlog
assert re.search(r'Finished `dev` profile', buildlog)
binary = ROOT / 'target/debug/poet.exe'
assert binary.is_file()
assert not (ROOT / 'target/coordination/native-check.lock').exists(), 'Active or retained check lease'
jsons = list(QA.glob('*.json')) + [ROOT / 'docs/planning/biosphere-work-packets.json', STAGE / 'install-manifest.json']
for p in jsons:
    json.loads(p.read_text(encoding='utf-8'))
linkdocs = [QA / 'README.md', QA / 'try-it.md', ROOT / 'docs/planning/poet-authoring-swarm-phases.md']
links = []
for p in linkdocs:
    for href in re.findall(r'\]\(([^)]+)\)', p.read_text(encoding='utf-8')):
        href = unquote(href.strip('<>')).split('#')[0]
        if not href or re.match(r'https?://', href):
            continue
        target = (p.parent / href).resolve()
        if target == (QA / 'verification.json').resolve():
            continue  # This invocation creates the receipt after checking its inputs.
        assert target.exists(), str(p) + ': missing link ' + href
        links.append(str(target))
report = {
    'schema': 'ecopunk-poet-material-motion-verification/v1',
    'date_utc': datetime.now(timezone.utc).isoformat(),
    'scope': 'POET-MOTION-W1 bounded material-key software qualification only',
    'canonical_files': files,
    'specifications_preserved': specs,
    'checks': [
        {'attempt': 1, 'result': 'compile failed: two new fixture type errors; no tests executed', 'evidence': 'check-attempt-1.json'},
        {'attempt': 2, 'exit': 101, 'result': '12 passed, 1 failed; observer login intentionally retired author session before project open', 'evidence': 'check-attempt-2.log'},
        {'attempt': 3, 'session_id': 48778, 'exit': args.test_exit, 'result': '13 passed, 0 failed, 249 filtered; fresh-process HMC child case included', 'evidence': 'check-attempt-3.log'},
        {'kind': 'native binary build', 'session_id': 98760, 'exit': args.build_exit, 'evidence': 'native-build.log'},
    ],
    'commands': {
        'test': 'cargo test --manifest-path C:/github/poet/Cargo.toml -p poet-cli --features native-graphical --locked --offline --target-dir C:/github/ecopunk/target --lib material_motion -- --test-threads=1',
        'build': 'cargo build --manifest-path C:/github/poet/Cargo.toml -p poet-cli --features native-graphical --locked --offline --target-dir C:/github/ecopunk/target --bin poet',
        'coordinator': 'Both commands ran through tools/poet-foundation/biosphere-shared/serial_check.py; sources frozen during checks; no worker Cargo runs.',
    },
    'binary': {'path': str(binary), 'sha256': sha(binary), 'bytes': binary.stat().st_size, 'mtime_utc': datetime.fromtimestamp(binary.stat().st_mtime, timezone.utc).isoformat()},
    'document_checks': {'json_files_parsed': len(jsons), 'local_links_resolved': len(links)},
    'source_reuse': 'EffectCraft property/key interaction reference; existing Vibe easing math. No whole EffectCraft implementation, Boa/egui host, new Cargo dependency or HMC format introduced by the allowlisted install.',
    'preservation': 'Byte preimages and successive receipts retained. Global Poet Git status was unavailable under the sandbox; no clean-worktree claim.',
    'current_check_lease': 'absent after completed root-owned commands',
    'usage_metrics': 'unknown; per-agent tokens/currency not returned',
    'limits': [
        'No current-window/Computer Use or GPU/pixel quality verification; native binary compiled, not launched.',
        'No actual human contributor/reviewer or biological/domain acceptance.',
        'Existing NativeAuthority login replaces its active session. Sequential handoff/refusal tested; simultaneous distinct-principal cooperation is unqualified.',
        'Explicit material sample/apply only; continuous transport, compositing, plant organs and connected fauna authoring remain open in this plan.',
        'IP-03 C1-C8 overall remains open; no whole ecosystem or model completion claim.',
        'Asset production HOLD; no film/game/cartoon accepted package produced; no commits, publication or payments.',
    ],
}
(QA / 'verification.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8', newline='\n')
print('Verified 16 saved canonical files, 13 passing focused cases, native binary, specification pins and document references.')
