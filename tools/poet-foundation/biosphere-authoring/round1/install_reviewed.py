"""Apply only reviewed, preimage-pinned biosphere round-one candidates."""
from pathlib import Path
from datetime import datetime, timezone
import argparse, hashlib, json, os, tempfile

ROOT = Path('C:/github/ecopunk').resolve()
STAGE = ROOT / 'tools/poet-foundation/biosphere-authoring/round1/work'
TARGETS = {'qualiadb': Path('C:/github/qualiaDB').resolve(), 'poet': Path('C:/github/poet').resolve()}
parser = argparse.ArgumentParser()
parser.add_argument('--manifest', required=True)
parser.add_argument('--receipt', required=True)
args = parser.parse_args()
manifest_path = (STAGE / args.manifest).resolve()
receipt = (ROOT / 'docs/qa/implementation-2026-10-26/biosphere-authoring-round1' / args.receipt).resolve()
assert manifest_path.is_relative_to(STAGE)
assert receipt.is_relative_to(ROOT / 'docs/qa/implementation-2026-10-26/biosphere-authoring-round1')
manifest = json.loads(manifest_path.read_text(encoding='utf-8'))
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
prepared = []
for row in manifest['files']:
    target = TARGETS[row['repo']]
    dst = (target / row['path']).resolve()
    src = Path(row['staged']).resolve()
    assert dst.is_relative_to(target) and '.git' not in dst.parts and dst.name != 'AGENTS.md'
    assert src.is_relative_to(STAGE) and sha(src) == row['after']
    actual = sha(dst) if dst.is_file() else None
    assert actual in (row['before'], row['after']), 'Concurrent source change: ' + str(dst)
    prepared.append((row, src, dst, actual))
report = {'date_utc': datetime.now(timezone.utc).isoformat(), 'scope':manifest['scope'], 'completed':False, 'files':[]}
receipt.parent.mkdir(parents=True, exist_ok=True)
for row, src, dst, actual in prepared:
    if actual != row['after']:
        if actual is not None:
            backup = STAGE / 'preimages' / row['repo'] / actual / row['path']
            backup.parent.mkdir(parents=True, exist_ok=True)
            assert not backup.exists() or sha(backup) == actual
            backup.write_bytes(dst.read_bytes())
        dst.parent.mkdir(parents=True, exist_ok=True)
        with tempfile.NamedTemporaryFile(dir=dst.parent, prefix='.biosphere-round1-', delete=False) as stream:
            stream.write(src.read_bytes()); stream.flush(); os.fsync(stream.fileno()); pending=Path(stream.name)
        assert (sha(dst) if dst.is_file() else None) == actual, 'Concurrent source change; staged temporary retained'
        os.replace(pending, dst)
    report['files'].append({'path':str(dst),'before':actual,'after':sha(dst)})
    receipt.write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8',newline='\n')
report['completed']=True
receipt.write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8',newline='\n')
print('Saved/verified', len(report['files']), 'reviewed files;', receipt)
