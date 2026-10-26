"""Root-only staged integration; live siblings are read-only until pinned install."""
from pathlib import Path
import hashlib,json
HERE=Path(__file__).resolve().parent;OUT=HERE/'integration';POET=Path('C:/github/poet');P='native/src/graphical/'
manifest=OUT/'target-manifest.json'
rows=json.loads(manifest.read_text()) if manifest.exists() else []
def sha(data):return hashlib.sha256(data).hexdigest()
def get(rel):
    stage=OUT/'after'/rel
    return (stage if stage.exists() else POET/rel).read_text(encoding='utf-8')
def put(rel,text):
    assert not (HERE/"PREPARATION-FROZEN.json").exists(), "Frozen source images; use a separately reviewed repair manifest"
    target=POET/rel;stage=OUT/'after'/rel;data=target.read_bytes() if target.exists() else None
    before=OUT/'before'/rel
    if data is not None and not before.exists():before.parent.mkdir(parents=True,exist_ok=True);before.write_bytes(data)
    if before.exists():assert data==before.read_bytes(),target
    stage.parent.mkdir(parents=True,exist_ok=True);stage.write_text(text.replace('\r\n','\n'),encoding='utf-8',newline='\n')
    key=str(target).lower();old=next((r for r in rows if r['target'].lower()==key),None)
    if old:rows.remove(old)
    rows.append(dict(target=str(target),stage=str(stage.resolve()),before_sha256=sha(data) if data is not None else None,after_sha256=sha(stage.read_bytes())))
    manifest.parent.mkdir(parents=True,exist_ok=True);manifest.write_text(json.dumps(rows,indent=2),encoding='utf-8')
