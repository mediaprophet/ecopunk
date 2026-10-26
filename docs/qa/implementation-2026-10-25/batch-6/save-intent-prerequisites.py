from pathlib import Path
import hashlib,json
root=Path(__file__).resolve().parent; poet=Path('C:/github/poet'); rows=[]
changes={
 'native/src/graphical/jobs.rs':[
  ('pub struct Reply {','pub struct Reply {\n    /// Actual backend result, independent of human-readable diagnostics.\n    pub operation_succeeded: bool,'),
  ('    let (mut message, bound_image) = match result {','    let operation_succeeded = result.is_ok();\n    let (mut message, bound_image) = match result {'),
  ('    Reply {\n        origin,','    Reply {\n        operation_succeeded,\n        origin,'),
 ],
 'native/src/graphical/jobs_tests.rs': [('    let reply = Reply {','    let reply = Reply {\n        operation_succeeded: true,')],
 'native/Cargo.toml': [('[dev-dependencies]','[dev-dependencies]\nciborium.workspace = true')],
}
for rel,edits in changes.items():
 p=poet/rel;before=p.read_bytes();text=before.decode('utf-8-sig')
 for old,new in edits:
  if text.count(old)!=1:raise RuntimeError('Unexpected source anchor '+rel)
  text=text.replace(old,new)
 b=root/'intent-prerequisites-before'/rel;b.parent.mkdir(parents=True,exist_ok=True)
 if b.exists():raise RuntimeError('Existing preimage '+rel)
 b.write_bytes(before);after=text.encode();p.write_bytes(after)
 rows.append(dict(path=str(p),before_sha256=hashlib.sha256(before).hexdigest(),after_sha256=hashlib.sha256(after).hexdigest()))
(root/'intent-prerequisites-saved.json').write_text(json.dumps(rows,indent=2)+'\n')
print('Saved truthful native intent result and portable Poet model test dependency')
