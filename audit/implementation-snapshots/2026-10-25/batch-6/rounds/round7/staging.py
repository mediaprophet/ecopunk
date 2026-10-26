"""Bounded review staging only; no live sibling mutations or executable recipes."""
from pathlib import Path
import hashlib,json
HERE=Path(__file__).resolve().parent
OWNERS=[Path('C:/github/poet'),Path('C:/github/qualiaDB')]
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def relative(target):
    p=Path(target).resolve()
    owner=next(x for x in OWNERS if p.is_relative_to(x.resolve()))
    return Path(owner.name)/p.relative_to(owner)
def stage_path(target):return HERE/'integration/after'/relative(target)
def read(target):
    p=stage_path(target)
    return (p if p.exists() else Path(target)).read_text(encoding='utf-8')
def put(target,text):
    assert not (HERE/"PREPARATION-FROZEN.json").exists(),"Historical one-shot staging is frozen"
    target=Path(target);p=stage_path(target)
    before=HERE/'integration/before'/relative(target)
    if not p.exists() and target.exists():
        before.parent.mkdir(parents=True,exist_ok=True);before.write_bytes(target.read_bytes())
    p.parent.mkdir(parents=True,exist_ok=True);p.write_text(text,encoding='utf-8')
def replace(target,old,new,count=1):
    text=read(target);assert text.count(old)==count,(str(target),old[:90],text.count(old),count)
    put(target,text.replace(old,new))
def manifest():
    rows=[]
    for p in sorted((HERE/'integration/after').rglob('*')):
        if not p.is_file():continue
        rel=p.relative_to(HERE/'integration/after');owner=next(x for x in OWNERS if x.name==rel.parts[0])
        target=owner.joinpath(*rel.parts[1:]);before=HERE/'integration/before'/rel
        rows.append(dict(target=str(target),stage=str(p),before_sha256=sha(before) if before.exists() else None,after_sha256=sha(p)))
    (HERE/'integration/target-manifest.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
    return rows
