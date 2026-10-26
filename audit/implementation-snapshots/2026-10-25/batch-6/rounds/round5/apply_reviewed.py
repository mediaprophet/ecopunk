"""Apply only the supplied manifest digest and verified exact before images."""
from pathlib import Path
import hashlib,json,sys,re
HERE=Path(__file__).resolve().parent
if len(sys.argv)==3:
    assert re.fullmatch(r'[a-z][a-z0-9-]*',sys.argv[2])
    HERE=HERE/'repairs'/sys.argv[2]
OWNERS=[Path('C:/github/qualiaDB').resolve(),Path('C:/github/poet').resolve()]
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
manifest=HERE/'install-manifest.json'
assert len(sys.argv) in (2,3) and sha(manifest)==sys.argv[1]
rows=json.loads(manifest.read_text())
for row in rows:
    target=Path(row['target']);stage=Path(row['stage'])
    assert any(target.resolve().is_relative_to(p) for p in OWNERS),target
    assert stage.resolve().is_relative_to(HERE),stage
    assert sha(stage)==row['after_sha256']
    assert (sha(target) if target.exists() else None)==row['before_sha256'],target
receipt=[]
for row in rows:
    target=Path(row['target']);stage=Path(row['stage'])
    assert (sha(target) if target.exists() else None)==row['before_sha256'],target
    owner=next(p for p in OWNERS if target.resolve().is_relative_to(p))
    if target.exists():
        backup=HERE/'installed-before'/owner.name/target.resolve().relative_to(owner)
        backup.parent.mkdir(parents=True,exist_ok=True);backup.write_bytes(target.read_bytes())
    target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(stage.read_bytes())
    assert sha(target)==row['after_sha256']
    receipt.append({**row,'verified_after_sha256':sha(target)})
    (HERE/'applied-source.json').write_text(json.dumps(receipt,indent=2),encoding='utf-8')
print(json.dumps(dict(applied=len(receipt),all_after_hashes_verified=True,manifest_sha256=sha(manifest))))
