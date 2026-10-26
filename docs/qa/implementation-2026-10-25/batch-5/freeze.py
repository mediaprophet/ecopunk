from pathlib import Path
from datetime import datetime,timezone
import json,hashlib
root=Path(__file__).parent
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
previous=json.loads((root.parent/'batch-4/latest-source-snapshot.json').read_text(encoding='utf-8-sig'))
prior=json.loads(Path(previous['path']).read_text(encoding='utf-8-sig'))
paths={Path(s['path']).resolve() for s in prior['sources']}
for folder in ['C:/github/poet/native/src','C:/github/qualiaDB/crates/qualia-core-db/src/identity','C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/native_authority','C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/studio_checkpoint']:
 paths.update(p.resolve() for p in Path(folder).rglob('*.rs'))
for p in ['C:/github/poet/Cargo.lock','C:/github/poet/native/Cargo.toml','C:/github/qualiaDB/Cargo.lock','C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/mod.rs','C:/github/qualiaDB/crates/qualia-core-db/src/crypto/network/ed25519.rs']:
 paths.add(Path(p).resolve())
paths.add(Path('C:/github/qualiaDB/crates/qualia-core-db/Cargo.toml').resolve())
paths.update(p.resolve() for p in Path('C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke').rglob('studio_*.rs'))
snapshot=dict(captured_utc=datetime.now(timezone.utc).isoformat(),scope='Batch5 changed-source and prior94-source reconciliation; not every dependency source',sources=[dict(path=str(p),sha256=sha(p)) for p in sorted(paths)])
data=(json.dumps(snapshot,indent=2)+'\n').encode();digest=hashlib.sha256(data).hexdigest()
folder=root/'source-snapshots';folder.mkdir(exist_ok=True);target=folder/(digest+'.json');target.write_bytes(data)
pointer=dict(path=str(target.resolve()),sha256=digest)
(root/'latest-source-snapshot.json').write_text(json.dumps(pointer,indent=2)+'\n',encoding='utf-8')
print(json.dumps(dict(sources=len(paths),snapshot=digest)))
