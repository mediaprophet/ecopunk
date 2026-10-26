"""Root source/lock integration with preimage guards; no sibling writes."""
from pathlib import Path
import hashlib,json,tomllib

HERE=Path(__file__).resolve().parent
POET=Path('C:/github/poet');QUALIA=Path('C:/github/qualiaDB')
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def read(p):return json.loads(Path(p).read_text(encoding='utf-8-sig'))
rows=[]
def add(target,stage,before,job):
    target,stage=Path(target),Path(stage)
    assert (sha(target) if target.exists() else None)==before,str(target)
    rows.append(dict(target=str(target.resolve()),stage=str(stage.resolve()),before_sha256=before,after_sha256=sha(stage),job=job))

for row in read(HERE/'review/target-manifest.json')['targets']:
    assert sha(row['after'])==row['after_sha256']
    add(row['target'],row['after'],row['before_sha256'],'R4 canonical package review')
producer=read(HERE/'producer/target-manifest.json')['targets']
before_by_target={str(Path(row['target'])).lower():row['before_sha256'] for row in producer}
prior=read(HERE.parent/'round3/poet-build-source-before.json')
prior={str(Path(k)).lower():v for k,v in prior.items()}
for stage in (HERE/'integration/after').rglob('*'):
    if not stage.is_file():continue
    target=POET/stage.relative_to(HERE/'integration/after')
    before=before_by_target.get(str(target).lower(),prior.get(str(target).lower()))
    add(target,stage,before,'R4 integrated native producer/review')

# Precisely add the local geospatial path package; preserve all registry/git
# versions, checksums and existing dependency entries. Cargo --locked verifies it.
lock=POET/'Cargo.lock';old=lock.read_text();old_data=tomllib.loads(old)
pkg={row['name']:row for row in old_data['package'] if 'source' not in row}
text=old
start=text.index('[[package]]\nname = "poet-cli"\n')
end=text.find('[[package]]',start+12);end=len(text) if end<0 else end
block=text[start:end]
if ' "qualia-geospatial",' not in block:
    assert ' "qualia-core-db",' in block
    block=block.replace(' "qualia-core-db",',' "qualia-core-db",\n "qualia-geospatial",',1)
    text=text[:start]+block+text[end:]
if 'qualia-geospatial' not in pkg:
    extra='''[[package]]
name = "qualia-geospatial"
version = "0.0.40"
dependencies = [
 "bytemuck",
 "qualia-core-db",
 "serde",
 "serde_json",
]

'''
    anchor=text.index('[[package]]\nname = "qualia-mcp"')
    text=text[:anchor]+extra+text[anchor:]
new_data=tomllib.loads(text)
def key(row):return row['name'],row['version'],row.get('source')
old_pkgs={key(row):row for row in old_data['package']};new_pkgs={key(row):row for row in new_data['package']}
for k,old_row in old_pkgs.items():
    actual=new_pkgs[k]
    if old_row['name']=='poet-cli' and 'source' not in old_row:
        expected=dict(old_row);expected['dependencies']=sorted(set(old_row['dependencies'])|{'qualia-geospatial'})
        assert actual==expected
    else:assert actual==old_row,k
assert len(new_pkgs)-len(old_pkgs) in (0,1)
stage=HERE/'integration/Cargo.lock';stage.write_text(text,encoding='utf-8')
add(lock,stage,sha(lock),'R4 exact offline local-path dependency lock')
(HERE/'integration/lock-delta.json').write_text(json.dumps(dict(old_packages=len(old_pkgs),new_packages=len(new_pkgs),registry_git_entries_unchanged=True,local_geospatial_added='qualia-geospatial' not in pkg),indent=2),encoding='utf-8')

doc=HERE/'integration/native-poet-package-review.md'
doc.write_text('''# Native Poet exact-package review and producer contract

Developmental implementation; no human, GUI, full-package or model acceptance.
Native source review and package review use separate grants and records.

PackageRevision binds a project-scoped declared release identifier, filename,
actual native saved digest/byte count, source revision, recorded producer and
prior destination digest. It does not establish universal referent equivalence
or full SaveAs/resource derivation ancestry. Known retained predecessor writers
and same-scope recorded source contributors remain self-review exclusions.

Explicit package grants are signed by the existing admitted native steward
provider, bound to exact target/reviewer/validity/evidence and invalidated by
revocation. Empty grant fields preserve old signed-admission bytes. Saved
records do not grant sessions or convert software-agent decisions to human acts.

Poet uses the same source/session/authority owner and canonical checkpoint
resource writer. GRM3 creation requires explicit grid data and source declaration.
Synthetic source metadata is retained with original payload digest and unverified
qualification. Selected opaque/provided/unknown/observational metadata refuses
semantic editing until a derivation contract exists; unrelated bytes remain exact.

Native record lock order is authority -> raw editor -> bounded checkpoint/file
check. The raw guard is retained through journal commit. A nonserialized package
publication stamp additionally rejects same-source native/MCP package successors.
Uncooperative external file changes are checked on refresh/consume, not claimed
to be continuously detected or erased from pixels.

New native controls create/edit resources, record contribution/work status,
request review and record a decision for the exact saved HMC. Steward grant/revoke
reuses the protected signer route and locks old sessions. These scoped decisions
do not pass model catalogue, scientific, rights or source-agreement gates.

Cold resource/workspace and checkpoint budgets are separate. Cancellation is
checked before the existing noninterruptible save commit. The bounded UI profile
is not an asset/master-quality ceiling. Graph mappings, complete derivation,
whole-workload performance, GUI/human acceptance and full target interoperability
remain qualification work. Asset production stays on hold.

Commands/results/source hashes are in the Ecopunk Round 4 evidence directory.
Authored or formatted tests are not executed passes; consult actual receipts.
''',encoding='utf-8')
target=QUALIA/'docs/manuals/standards/native-poet-package-review.md'
add(target,doc,sha(target) if target.exists() else None,'R4 canonical implementation contract')
assert len({row['target'].lower() for row in rows})==len(rows)
manifest=HERE/'install-manifest.json';manifest.write_text(json.dumps(rows,indent=2),encoding='utf-8')
print(json.dumps(dict(targets=len(rows),manifest_sha256=sha(manifest),live_files_written=False)))
