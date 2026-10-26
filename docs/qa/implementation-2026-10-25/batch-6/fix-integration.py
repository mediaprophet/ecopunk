from pathlib import Path
import hashlib,json
root=Path(__file__).resolve().parent;base=Path('C:/github/poet/native/src/graphical')
replacements={
 'editor_contract.rs': [('super::assembly::','crate::graphical::assembly::')],
 'assembly/compile.rs': [('attach_material_section(&geometry, &assembly.materials, &assembly.ranges)','attach_material_section(&geometry, &assembly.materials, &assembly.ranges, u32::try_from(assembly.mesh.triangles.len().checked_mul(3).ok_or("assembly index count overflow")?).map_err(|_|"assembly index count exceeds native format")?)')],
}
rows=[]
for rel,edits in replacements.items():
 p=base/rel;before=p.read_bytes();text=before.decode('utf-8-sig')
 for old,new in edits:
  if old not in text:raise RuntimeError('Missing integration anchor '+rel)
  text=text.replace(old,new)
 backup=root/'integration-before'/rel;backup.parent.mkdir(parents=True,exist_ok=True)
 if backup.exists():raise RuntimeError('Existing backup')
 backup.write_bytes(before);after=text.encode();p.write_bytes(after)
 rows.append(dict(path=str(p),before_sha256=hashlib.sha256(before).hexdigest(),after_sha256=hashlib.sha256(after).hexdigest()))
(root/'integration-fixed.json').write_text(json.dumps(rows,indent=2)+'\n')
print('Saved narrow adapter integration fixes')
