from pathlib import Path
import json,hashlib
group=Path(__file__).resolve().parent/'root-repair'
rows=[]
for name in ['native/src/graphical/controller_support.rs','native/src/graphical/mod.rs']:
    raw=(Path('C:/github/poet')/name).read_bytes();text=raw.decode().replace('\r\n','\n')
    if name.endswith('controller_support.rs'):
        assert text.count('super::materials::')==1
        text=text.replace('super::materials::','crate::graphical::materials::')
    else:text+='\n#[cfg(test)]\nmod reference_authoring_tests;\n'
    before=group/'before/poet'/name;after=group/'after/poet'/name
    before.parent.mkdir(parents=True,exist_ok=True);after.parent.mkdir(parents=True,exist_ok=True)
    before.write_bytes(raw);after.write_text(text,encoding='utf-8',newline='\n')
    rows.append(dict(repository='poet',target=name,before_sha256=hashlib.sha256(raw).hexdigest(),after_sha256=hashlib.sha256(after.read_bytes()).hexdigest()))
(group/'manifest.json').write_text(json.dumps(dict(schema='poet-foundation-stage/v1',files=rows),indent=2)+'\n')
print(hashlib.sha256((group/'manifest.json').read_bytes()).hexdigest())
