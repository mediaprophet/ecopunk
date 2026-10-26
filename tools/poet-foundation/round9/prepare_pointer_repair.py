from pathlib import Path
import json,hashlib
group=Path(__file__).resolve().parent/'pointer-repair'
name='native/src/graphical/viewport_controls.rs'
raw=(Path('C:/github/poet')/name).read_bytes();text=raw.decode().replace('\r\n','\n')
assert text.count('border:1px solid #536273;')==1
text=text.replace('border:1px solid #536273;','outline:1px solid #536273;')
assert text.count('                onmouseup:end_drag,')==1
text=text.replace('                onmouseup:end_drag,','                onmouseup:end_drag,\n                onmouseleave:move |_|drag.set(None),')
before=group/'before/poet'/name;after=group/'after/poet'/name
before.parent.mkdir(parents=True,exist_ok=True);after.parent.mkdir(parents=True,exist_ok=True)
before.write_bytes(raw);after.write_text(text,encoding='utf-8',newline='\n')
rows=[dict(repository='poet',target=name,before_sha256=hashlib.sha256(raw).hexdigest(),after_sha256=hashlib.sha256(after.read_bytes()).hexdigest())]
(group/'manifest.json').write_text(json.dumps(dict(schema='poet-foundation-stage/v1',files=rows),indent=2)+'\n')
print(hashlib.sha256((group/'manifest.json').read_bytes()).hexdigest())
