"""Static packaging only: all writes stay inside this Ecopunk staging folder."""
from pathlib import Path
import hashlib,json,difflib
HERE=Path(__file__).resolve().parent
manifest_path=HERE/'sdk-save-manifest.json'
rows=json.loads(manifest_path.read_text())
module=HERE/'after/src/container_10d/typed_payloads/mod.rs'
text=module.read_text()
if 'pub struct TypedPayloadResolver' in text:
    resolver=text[text.index('use super::{'):]
    # Resolver moves one level down; the .10d parent becomes the grandparent.
    resolver=resolver.replace('use super::{','use super::super::{',1)
    resolver=resolver.replace('Frames(super::NormalSectionError)','Frames(super::super::NormalSectionError)')
    resolver=resolver.replace('Morph(super::MorphSectionError)','Morph(super::super::MorphSectionError)')
    resolver=resolver.replace('super::is_normal_section(payload)','super::super::is_normal_section(payload)')
    (module.parent/'resolver.rs').write_text('//! Validated borrowed field selection; codec readers remain separate.\n'+resolver)
    module.write_text('//! Typed-field SDK over existing MID2 ranges and payload codecs.\nmod readers;\nmod resolver;\npub use readers::{read_container_mesh_frames, read_container_morph_targets_into};\npub use resolver::{TypedPayload, TypedPayloadError, TypedPayloadResolver};\n')
    base=next(row for row in rows if row['relative']=='src/container_10d/typed_payloads/mod.rs')
    rows.append(dict(path=base['path'].replace('mod.rs','resolver.rs'),relative='src/container_10d/typed_payloads/resolver.rs',before_sha256=None))
test=HERE/'after/tests/typed_field_payloads.rs'
test.write_text(test.read_text().replace('self as c, ',''))
diff=[]
for row in rows:
    rel=row['relative'];after=HERE/'after'/rel
    payload=after.read_bytes()
    row['after_sha256']=hashlib.sha256(payload).hexdigest()
    row['lines']=len(payload.decode().splitlines())
    live=Path(row['path'])
    current=hashlib.sha256(live.read_bytes()).hexdigest() if live.exists() else None
    if current!=row['before_sha256']:raise RuntimeError('Live preimage changed: '+str(live))
    old=(HERE/'before'/rel).read_text().splitlines(True) if row['before_sha256'] else []
    diff.extend(difflib.unified_diff(old,after.read_text().splitlines(True),fromfile='a/'+rel if row['before_sha256'] else '/dev/null',tofile='b/'+rel))
manifest_path.write_text(json.dumps(rows,indent=2)+'\n')
(HERE/'foundation-sdk.patch').write_text(''.join(diff))
print(json.dumps({'candidates':len(rows),'live_core_unchanged':True,'new_file_lines':{r['relative']:r['lines'] for r in rows if r['before_sha256'] is None}},indent=2))
