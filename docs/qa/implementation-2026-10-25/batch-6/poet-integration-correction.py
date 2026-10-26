"""Integrate functional Poet libraries; source origins remain historical evidence."""
from pathlib import Path
import hashlib,json
root=Path(__file__).resolve().parent; poet=Path('C:/github/poet'); changes={}
def read(rel):
 if rel not in changes:
  p=poet/rel;b=p.read_bytes();changes[rel]=[b,b.decode('utf-8-sig')]
 return changes[rel][1]
def replace(rel,old,new,count=1):
 text=read(rel)
 if text.count(old)!=count:raise RuntimeError(f'Unexpected source anchor in {rel}: {old!r}')
 changes[rel][1]=text.replace(old,new)
replace('Cargo.toml','    "wasm",','    "wasm",\n    "crates/geometry",\n    "crates/spatial-authoring",')
replace('native/Cargo.toml','craft-authoring = { path = "C:/github/craft/crates/authoring", optional = true }','poet-spatial-authoring = { path = "../crates/spatial-authoring", optional = true }')
replace('native/Cargo.toml','"dep:craft-authoring"','"dep:poet-spatial-authoring"')
for rel in ('native/src/graphical/entry.rs','native/src/graphical/native_entry/create.rs'):
 replace(rel,'Craft.profile_sweep','ComputationalGeometry.profile_sweep')
replace('native/src/graphical/entry.rs','CheckpointEditorHost, CheckpointLimits, CheckpointService, TrustedBinding,','CheckpointEditorHost, CheckpointLimits, CheckpointService, TrustedBinding, ExpectedFile, SaveReceipt,')
rel='native/src/graphical/authoring/controls.rs'
text=read(rel)
if text.count('"input{')!=5:raise RuntimeError('Unexpected input node source')
changes[rel][1]=text.replace('"input{','" input{')
replace('native/src/graphical/job_actions.rs','fn outcome_image(','pub(super) fn outcome_image(')
replace('native/src/graphical/jobs.rs','use actions::perform;','use actions::perform;\n#[cfg(test)]\nuse actions::outcome_image;')
rows=[];backup=root/'poet-correction-before'
for rel,(before,text) in changes.items():
 p=poet/rel
 if p.read_bytes()!=before:raise RuntimeError('Concurrent source change '+rel)
 b=backup/rel;b.parent.mkdir(parents=True,exist_ok=True)
 if b.exists():raise RuntimeError('Backup exists '+rel)
 b.write_bytes(before);after=text.encode('utf-8');p.write_bytes(after)
 rows.append(dict(path=str(p),before_sha256=hashlib.sha256(before).hexdigest(),after_sha256=hashlib.sha256(after).hexdigest()))
(root/'poet-correction-saved.json').write_text(json.dumps(dict(scope='Poet baseline/functional owner correction and concrete compile repairs',files=rows),indent=2)+'\n')
print(json.dumps(dict(saved=len(rows),workspace='Poet',license_scope='Imported libraries MIT; existing Poet terms unchanged')))
