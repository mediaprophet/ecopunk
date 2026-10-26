"""Save owned Poet source changes with preimage backups and exact receipts."""
from pathlib import Path
import hashlib,json
root=Path(__file__).resolve().parent
poet=Path('C:/github/poet/native')
changes={}
def sha(b):return hashlib.sha256(b).hexdigest()
def load(rel):
 if rel not in changes:
  p=poet/rel
  changes[rel]=[p.read_bytes() if p.exists() else None,None]
  changes[rel][1]=changes[rel][0].decode('utf-8-sig') if changes[rel][0] is not None else ''
 return changes[rel][1]
def replace(rel,old,new):
 s=load(rel)
 if s.count(old)!=1:raise RuntimeError(f'{rel}: expected one exact source anchor {old[:60]!r}')
 changes[rel][1]=s.replace(old,new)
for p in (root/'root-stage/authoring').glob('*.rs'):
 rel='src/graphical/authoring/'+p.name
 if (poet/rel).exists():raise RuntimeError('New source already exists: '+rel)
 changes[rel]=[None,p.read_text(encoding='utf-8')]
replace('src/graphical/mod.rs','pub mod controller;','pub mod controller;\npub mod authoring;\npub mod assembly;')
replace('Cargo.toml','webizen-render = { path = "C:/github/qualiaDB/crates/webizen-render", optional = true }','webizen-render = { path = "C:/github/qualiaDB/crates/webizen-render", optional = true }\ncraft-authoring = { path = "C:/github/craft/crates/authoring", optional = true }')
replace('Cargo.toml','"dep:sha2"]','"dep:sha2", "dep:craft-authoring"]')
view='crate::graphical::authoring::AuthoringView'
edit='crate::graphical::authoring::AuthoringEdit'
assets='Vec<(String,qualia_core_db::render::compile_10d::CompiledAsset)>'
sr='SourceRef'
receipt='qualia_core_db::poet_host::invoke::studio_store::checkpoint::SaveReceipt'
file='qualia_core_db::poet_host::invoke::studio_store::checkpoint::ExpectedFile'
def delegates(receiver):return f'''
    fn authoring_view(&self,e:{sr})->Result<{view},String>{{{receiver}.authoring_view(e)}}
    fn authoring_edit(&mut self,a:{edit},e:{sr})->Result<EditorOutcome,String>{{{receiver}.authoring_edit(a,e)}}
    fn compiled_assets(&self,k:&str,e:{sr})->Result<{assets},String>{{{receiver}.compiled_assets(k,e)}}
    fn compile_checkpoint(&mut self,n:&str,k:&str,f:{file},e:{sr})->Result<{receipt},String>{{{receiver}.compile_checkpoint(n,k,f,e)}}
'''
for rel,cls,recv in [
 ('src/graphical/shared_host.rs','SharedEditorHost','self.current()?'),
 ('src/graphical/session_host.rs','SessionEditorHost','self.current()?'),
 ('src/graphical/package/mod.rs','ResourceImportHost','self.inner'),
 ('src/graphical/entry.rs','DiagnosticEditor','self.inner'),
]:
 # Diagnostic composition has a different field name; inspect instead of guessing.
 if cls=='DiagnosticEditor':continue
 anchor=f'impl EditorHost for {cls} {{'
 replace(rel,anchor,anchor+delegates(recv))
canon=f'''
    fn authoring_view(&self,e:{sr})->Result<{view},String>{{self.inspect_authoring(e)}}
    fn authoring_edit(&mut self,a:{edit},e:{sr})->Result<EditorOutcome,String>{{self.edit_assembly(a,e)}}
    fn compiled_assets(&self,k:&str,e:{sr})->Result<{assets},String>{{self.compile_authored(k,e)}}
'''
replace('src/graphical/controller.rs','impl EditorHost for CanonicalEditorHost {','impl EditorHost for CanonicalEditorHost {'+canon)
replace('src/graphical/controller.rs','pub struct CanonicalEditorHost {','#[path = "authoring/controller.rs"]\nmod authoring_implementation;\n\npub struct CanonicalEditorHost {')
cp=f'''
    fn authoring_view(&self,e:{sr})->Result<{view},String>{{self.inner.authoring_view(e)}}
    fn authoring_edit(&mut self,a:{edit},e:{sr})->Result<EditorOutcome,String>{{self.inner.authoring_edit(a,e)}}
    fn compiled_assets(&self,k:&str,e:{sr})->Result<{assets},String>{{self.inner.compiled_assets(k,e)}}
    fn compile_checkpoint(&mut self,n:&str,k:&str,f:{file},e:{sr})->Result<{receipt},String>{{crate::graphical::authoring::package::compile_checkpoint(self.inner.as_ref(),&self.service,self.service.limits,n,k,f,e)}}
'''
replace('src/graphical/checkpoints/editor.rs','impl EditorHost for CheckpointEditorHost {','impl EditorHost for CheckpointEditorHost {'+cp)
auth=f'''
    fn authoring_view(&self,e:{sr})->Result<{view},String>{{with_authorized(self.owner.as_ref(),&self.scope,Some(&e),AuthoringAction::ReadSource,self.limits,|_,_|self.inner.authoring_view(e.clone()))}}
    fn authoring_edit(&mut self,a:{edit},e:{sr})->Result<EditorOutcome,String>{{self.write(&e,AuthoringAction::ExecuteSource,|inner|inner.authoring_edit(a,e.clone()))}}
    fn compiled_assets(&self,k:&str,e:{sr})->Result<{assets},String>{{with_authorized(self.owner.as_ref(),&self.scope,Some(&e),AuthoringAction::ReadSource,self.limits,|_,_|self.inner.compiled_assets(k,e.clone()))}}
    fn compile_checkpoint(&mut self,n:&str,k:&str,f:{file},e:{sr})->Result<{receipt},String>{{self.write_package(n,f,&e,|inner|inner.compile_checkpoint(n,k,f,e.clone()))}}
'''
replace('src/graphical/cooperation/editor.rs','impl EditorHost for AuthorizedEditorHost {','impl EditorHost for AuthorizedEditorHost {'+auth)
# Nullable root is the explicitly agreed assembly source contract.
rel='src/graphical/assembly/admission.rs'
replace(rel,'if let Some(parent) = part.get("parent") {\n            key(parent)?;\n        }','if let Some(parent) = part.get("parent").filter(|p| !matches!(p, Value::Null)) {\n            key(parent)?;\n        }')
replace(rel,'parent: p.get("parent").map(key).transpose()?.map(str::to_owned),','parent: p.get("parent").filter(|p| !matches!(p, Value::Null)).map(key).transpose()?.map(str::to_owned),')
receipt_rows=[]
backup=root/'root-before';backup.mkdir(exist_ok=True)
for rel,(before,content) in changes.items():
 path=poet/rel
 if before is not None and path.read_bytes()!=before:raise RuntimeError('Concurrent source change: '+rel)
 if before is None and path.exists():raise RuntimeError('New source collision: '+rel)
 if before is not None:
  b=backup/rel;b.parent.mkdir(parents=True,exist_ok=True)
  if b.exists():raise RuntimeError('Preimage backup already exists: '+str(b))
  b.write_bytes(before)
 path.parent.mkdir(parents=True,exist_ok=True)
 data=content.encode('utf-8');path.write_bytes(data)
 if path.read_bytes()!=data:raise RuntimeError('Saved bytes differ: '+rel)
 receipt_rows.append(dict(path=str(path),before_sha256=sha(before) if before is not None else None,after_sha256=sha(data)))
(root/'root-saved.json').write_text(json.dumps(dict(scope='Native part editing and revision-bound compiled HMC operations',files=receipt_rows),indent=2)+'\n',encoding='utf-8')
print(json.dumps(dict(saved=len(receipt_rows),receipt=str(root/'root-saved.json'))))
