"""Stage one reviewed test-only repair and retain the failed verification."""
from pathlib import Path
import hashlib,json
HERE=Path(__file__).resolve().parent
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
candidate=json.loads((HERE/'review/fixture-repair/target-manifest.json').read_text())['targets']
assert len(candidate)==1
out=HERE/'repairs/history-fixture';out.mkdir(parents=True,exist_ok=True)
rows=[]
for row in candidate:
    target=Path(row['target']);source=Path(row['after'])
    assert sha(target)==row['before_sha256'] and sha(source)==row['after_sha256']
    stage=out/'history.rs';stage.write_bytes(source.read_bytes())
    rows.append(dict(target=str(target),stage=str(stage),before_sha256=row['before_sha256'],after_sha256=row['after_sha256']))
manifest=out/'install-manifest.json';manifest.write_text(json.dumps(rows,indent=2),encoding='utf-8')
for suffix in ['.log','-source-before.json','-result.json']:
    src=HERE/('public'+suffix);dst=HERE/('public-before-fixture-repair'+suffix)
    assert not dst.exists();src.rename(dst)
receipt=HERE/'public-before-fixture-repair-result.json'
r=json.loads(receipt.read_text());r['log']=str(HERE/'public-before-fixture-repair.log');r['source_snapshot']=str(HERE/'public-before-fixture-repair-source-before.json')
receipt.write_text(json.dumps(r,indent=2),encoding='utf-8')
print(json.dumps(dict(manifest=str(manifest),manifest_sha256=sha(manifest),targets=len(rows))))
