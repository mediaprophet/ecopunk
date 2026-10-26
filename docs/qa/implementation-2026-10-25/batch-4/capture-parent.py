from pathlib import Path
import hashlib,json
root=Path(__file__).parent
paths=['controller.rs','shared_host.rs','jobs.rs','ui.rs','mod.rs']
base=Path('C:/github/poet/native/src/graphical')
out=root/'parent-originals';out.mkdir(exist_ok=True)
records=[]
for name in paths:
 p=base/name; data=p.read_bytes(); backup=out/name
 assert not backup.exists(), 'Original already captured; do not replace'
 backup.write_bytes(data)
 records.append(dict(path=str(p),sha256=hashlib.sha256(data).hexdigest(),backup=str(backup)))
(root/'parent-originals.json').write_text(json.dumps(records,indent=2)+'\n',encoding='utf-8')
print('Captured parent-owned source originals before editing.')
