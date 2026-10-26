from pathlib import Path
import hashlib,json
HERE=Path(__file__).resolve().parent
GROUP=HERE/'integration'
def sha(b):return hashlib.sha256(b).hexdigest()
def read(name):
    path=GROUP/'after/poet'/name
    return (path if path.exists() else Path('C:/github/poet')/name).read_text(encoding='utf-8').replace('\r\n','\n')
def stage(name,text):
    before=GROUP/'before/poet'/name;after=GROUP/'after/poet'/name
    live=Path('C:/github/poet')/name
    if live.exists() and not before.exists():before.parent.mkdir(parents=True,exist_ok=True);before.write_bytes(live.read_bytes())
    after.parent.mkdir(parents=True,exist_ok=True);after.write_text(text,encoding='utf-8',newline='\n')
def change(text,old,new):
    assert text.count(old)==1,(old,text.count(old))
    return text.replace(old,new)
def manifest():
    rows=[]
    for path in sorted((GROUP/'after/poet').rglob('*')):
        if not path.is_file():continue
        name=path.relative_to(GROUP/'after/poet').as_posix();before=GROUP/'before/poet'/name
        rows.append(dict(repository='poet',target=name,before_sha256=sha(before.read_bytes()) if before.exists() else None,after_sha256=sha(path.read_bytes())))
    (GROUP/'manifest.json').write_text(json.dumps(dict(schema='poet-foundation-stage/v1',files=rows),indent=2)+'\n')
