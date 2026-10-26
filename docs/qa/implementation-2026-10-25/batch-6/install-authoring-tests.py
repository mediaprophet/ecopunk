from pathlib import Path
import hashlib,json
root=Path(__file__).resolve().parent;poet=Path('C:/github/poet/native');rows=[]
changes={
 'src/graphical/authoring/mod.rs': ('pub(crate) mod controls;','pub(crate) mod controls;\n#[cfg(test)]\nmod tests;'),
 'Cargo.toml': ('[dev-dependencies]','[[example]]\nname = "growing_reference"\npath = "examples/growing_reference/main.rs"\nrequired-features = ["native-graphical"]\n\n[dev-dependencies]'),
}
for rel,(old,new)in changes.items():
 p=poet/rel;before=p.read_bytes();text=before.decode('utf-8-sig')
 if text.count(old)!=1:raise RuntimeError('Unexpected source anchor '+rel)
 backup=root/'authoring-tests-before'/rel;backup.parent.mkdir(parents=True,exist_ok=True)
 if backup.exists():raise RuntimeError('Backup exists '+rel)
 backup.write_bytes(before);after=text.replace(old,new).encode();p.write_bytes(after)
 rows.append(dict(path=str(p),before_sha256=hashlib.sha256(before).hexdigest(),after_sha256=hashlib.sha256(after).hexdigest()))
p=poet/'src/graphical/authoring/tests.rs'
if p.exists():raise RuntimeError('New test already exists')
p.write_bytes((root/'root-stage/authoring/tests.rs').read_bytes())
rows.append(dict(path=str(p),before_sha256=None,after_sha256=hashlib.sha256(p.read_bytes()).hexdigest()))
(root/'authoring-tests-saved.json').write_text(json.dumps(rows,indent=2)+'\n')
print('Saved authoring/persistence regression tests and native reference example registration')
