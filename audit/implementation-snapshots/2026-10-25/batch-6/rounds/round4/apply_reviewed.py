"""Apply only the pinned reviewed Core/Poet manifest, preserving exact preimages."""
from pathlib import Path
import hashlib,json,sys
HERE=Path(__file__).resolve().parent
if len(sys.argv)==3:
    choices={'ui-key-repair':'repairs/ui-keys','history-fixture-repair':'repairs/history-fixture','ui-text-repair':'repairs/ui-text'}
    assert sys.argv[2] in choices
    HERE=HERE/choices[sys.argv[2]]
OWNERS=[Path('C:/github/qualiaDB').resolve(),Path('C:/github/poet').resolve()]
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
manifest=HERE/'install-manifest.json'
assert len(sys.argv) in (2,3) and sha(manifest)==sys.argv[1]
rows=json.loads(manifest.read_text())
for row in rows:
    target,stage=Path(row['target']),Path(row['stage'])
    assert any(target.resolve().is_relative_to(owner) for owner in OWNERS)
    assert stage.resolve().is_relative_to(HERE)
    assert sha(stage)==row['after_sha256']
    assert (sha(target) if target.exists() else None)==row['before_sha256'],str(target)
receipt=[]
for row in rows:
    target,stage=Path(row['target']),Path(row['stage'])
    assert (sha(target) if target.exists() else None)==row['before_sha256'],str(target)
    owner=next(owner for owner in OWNERS if target.resolve().is_relative_to(owner))
    backup=HERE/'installed-before'/owner.name/target.resolve().relative_to(owner)
    if target.exists():backup.parent.mkdir(parents=True,exist_ok=True);backup.write_bytes(target.read_bytes())
    target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(stage.read_bytes())
    assert sha(target)==row['after_sha256']
    receipt.append({**row,'verified_after_sha256':sha(target)})
    (HERE/'applied-source.json').write_text(json.dumps(receipt,indent=2),encoding='utf-8')
print(json.dumps(dict(applied=len(receipt),all_after_hashes_verified=True,manifest_sha256=sha(manifest))))
