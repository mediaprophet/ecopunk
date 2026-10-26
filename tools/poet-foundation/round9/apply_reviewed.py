"""Apply one hash-pinned reviewed group, checking every preimage before saving."""
from pathlib import Path
import hashlib,json,sys
HERE=Path(__file__).resolve().parent
OWNERS={name:Path('C:/github')/name for name in ('poet','qualiaDB')}
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
group=Path(sys.argv[1]).resolve()
assert group.is_relative_to(HERE) and group.is_dir()
manifest=group/'manifest.json'
assert sha(manifest)==sys.argv[2]
data=json.loads(manifest.read_text())
rows=data['files']
default_owner='poet' if data.get('repository')=='C:/github/poet' else None
rows=[{**r,'repository':r.get('repository',r.get('repo',default_owner)),
       'target':r.get('target',r.get('path')),
       'before_sha256':r.get('before_sha256',r.get('beforeSha256')),
       'after_sha256':r.get('after_sha256',r.get('afterSha256'))} for r in rows]
targets=set()
for row in rows:
    owner=OWNERS[row['repository']].resolve()
    target=(owner/row['target']).resolve()
    stage=(group/'after'/row['repository']/row['target']).resolve()
    assert target.is_relative_to(owner) and stage.is_relative_to(group/'after')
    assert target not in targets;targets.add(target)
    assert sha(stage)==row['after_sha256']
    assert (sha(target) if target.exists() else None)==row['before_sha256'],str(target)
receipt=[]
for row in rows:
    target=OWNERS[row['repository']]/row['target']
    stage=group/'after'/row['repository']/row['target']
    assert (sha(target) if target.exists() else None)==row['before_sha256']
    if target.exists():
        backup=group/'installed-before'/row['repository']/row['target']
        backup.parent.mkdir(parents=True,exist_ok=True);backup.write_bytes(target.read_bytes())
    target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(stage.read_bytes())
    assert sha(target)==row['after_sha256']
    receipt.append({**row,'verified_after_sha256':sha(target)})
    (group/'applied-source.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(dict(applied=len(receipt),manifest_sha256=sha(manifest),all_after_hashes_verified=True)))
