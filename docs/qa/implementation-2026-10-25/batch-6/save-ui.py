"""Save owned native UI/worker wiring with exact preimage backups."""
from pathlib import Path
import hashlib,json
root=Path(__file__).resolve().parent; base=Path('C:/github/poet/native/src/graphical')
changes={}
def read(rel):
 if rel not in changes:
  p=base/rel; b=p.read_bytes() if p.exists() else None;changes[rel]=[b,b.decode('utf-8-sig') if b is not None else '']
 return changes[rel][1]
def put(rel,text):read(rel);changes[rel][1]=text
def replace(rel,old,new):
 s=read(rel)
 if s.count(old)!=1:raise RuntimeError(f'{rel}: source anchor not unique: {old[:50]}')
 put(rel,s.replace(old,new))
def sha(b):return hashlib.sha256(b).hexdigest()
put('authoring/controls.rs',(root/'root-stage/authoring/controls.rs').read_text(encoding='utf-8'))
replace('authoring/mod.rs','pub(crate) mod package;','pub(crate) mod package;\npub(crate) mod controls;')
replace('jobs.rs','pub enum Action {','''pub enum Action {
    AuthoringInspect(SourceRef),
    AuthoringEdit { edit:super::authoring::AuthoringEdit, expected:SourceRef },
    CompileAssembly { name:String,key:String,file:qualia_core_db::poet_host::invoke::studio_store::checkpoint::ExpectedFile,expected:SourceRef },''')
replace('jobs.rs','pub struct Reply {','pub struct Reply {\n    pub authoring: Option<super::authoring::AuthoringView>,')
replace('jobs.rs','    let mut workflow = None;','    let mut workflow = None;\n    let mut authoring = None;')
replace('jobs.rs','perform(host, action, &mut inventory,&mut workflow)','perform(host, action, &mut inventory,&mut workflow,&mut authoring)')
replace('jobs.rs','        workflow: if current { workflow } else { None },','        workflow: if current { workflow } else { None },\n        authoring: if current { authoring } else { None },')
replace('jobs.rs','            Action::Submit { expected, .. } | Action::Undo(expected)','            Action::AuthoringInspect(expected) | Action::AuthoringEdit{expected,..} | Action::CompileAssembly{expected,..} | Action::Submit { expected, .. } | Action::Undo(expected)')
# Split the action router out of the worker lifecycle before adding more domain routes.
s=read('jobs.rs');start=s.index('fn perform(');end=s.index('fn execute(',start)
actions=s[start:end]
put('jobs.rs',s[:start]+'#[path="job_actions.rs"]\nmod actions;\nuse actions::perform;\n\n'+s[end:])
actions='use super::*;\n'+actions.replace('fn perform(','pub(super) fn perform(',1)
actions=actions.replace('    workflow: &mut Option<super::cooperative_workflow::WorkflowView>,','    workflow: &mut Option<crate::graphical::cooperative_workflow::WorkflowView>,\n    authoring: &mut Option<crate::graphical::authoring::AuthoringView>,')
# Moving into a child changes super from graphical to jobs for these type paths.
actions=actions.replace('super::checkpoints::PackageInventory','crate::graphical::checkpoints::PackageInventory')
actions=actions.replace('super::package::','crate::graphical::package::').replace('super::controller::EditorOutcome','crate::graphical::controller::EditorOutcome')
anchor='    let message = match action {'
new='''    let message = match action {
        Action::AuthoringInspect(expected)=>{*authoring=Some(host.authoring_view(expected)?);"Current accepted assembly inspected".into()},
        Action::AuthoringEdit{edit,expected}=>{
            let result=host.authoring_edit(edit,expected)?;
            *authoring=host.authoring_view(result.snapshot.source.clone()).ok();
            return outcome_image(result);
        },
        Action::CompileAssembly{name,key,file,expected}=>{
            let receipt=host.compile_checkpoint(&name,&key,file,expected.clone())?;
            *inventory=host.package_inventory(&name,expected).ok();
            format!("Accepted assembly compiled into {name}; SHA256 {}. Complete model/package acceptance remains separate.",crate::graphical::package::hex_digest(&receipt.digest))+&crate::graphical::package::durability_note(&receipt)
        },'''
if actions.count(anchor)!=1:raise RuntimeError('Action router anchor changed')
actions=actions.replace(anchor,new)
put('job_actions.rs',actions)
replace('jobs_tests.rs','        workflow: None,','        workflow: None,\n        authoring: None,')
replace('ui.rs','                        p { "Factor material · SDR working preview. Master quality, source rights and human review remain separate." }','''                        p { "Native assembly/material working preview. Master quality, source rights and human review remain separate." }
                        super::authoring::controls::AuthoringControls {host:host.clone(),view,draft,status,image,busy,buffer:draft_buffer.clone()}''')
replace('native_entry/create.rs','            "ComputationalGeometry.swung_surface".into(),','''            "ComputationalGeometry.swung_surface".into(),
            "Craft.profile_sweep".into(),
            "ComputationalGeometry.uv_sphere".into(),
            "ComputationalGeometry.torus".into(),''')
replace('entry.rs','                                | "ComputationalGeometry.swung_surface"','''                                | "ComputationalGeometry.swung_surface"
                                | "Craft.profile_sweep"
                                | "ComputationalGeometry.uv_sphere"
                                | "ComputationalGeometry.torus"''')
view='crate::graphical::authoring::AuthoringView'; edit='crate::graphical::authoring::AuthoringEdit'
assets='Vec<(String,qualia_core_db::render::compile_10d::CompiledAsset)>'
replace('entry.rs','impl EditorHost for DiagnosticEditor {',f'''impl EditorHost for DiagnosticEditor {{
    fn authoring_view(&self,e:SourceRef)->Result<{view},String>{{self.ready()?;self.inner.authoring_view(e)}}
    fn authoring_edit(&mut self,a:{edit},e:SourceRef)->Result<EditorOutcome,String>{{self.ready()?;let r=self.inner.authoring_edit(a,e)?;Ok(outcome(r,false,self.inner.checkpoint_supported()))}}
    fn compiled_assets(&self,k:&str,e:SourceRef)->Result<{assets},String>{{self.ready()?;self.inner.compiled_assets(k,e)}}
    fn compile_checkpoint(&mut self,n:&str,k:&str,f:ExpectedFile,e:SourceRef)->Result<SaveReceipt,String>{{self.ready()?;self.inner.compile_checkpoint(n,k,f,e)}}
''')
backup=root/'ui-before';backup.mkdir(exist_ok=True);rows=[]
for rel,(before,content)in changes.items():
 p=base/rel
 if before is None and p.exists():raise RuntimeError('New source collision '+rel)
 if before is not None and p.read_bytes()!=before:raise RuntimeError('Source changed '+rel)
 b=backup/rel
 if before is not None:
  b.parent.mkdir(parents=True,exist_ok=True)
  if b.exists():raise RuntimeError('Backup collision '+rel)
  b.write_bytes(before)
 data=content.encode();p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(data)
 assert p.read_bytes()==data
 rows.append(dict(path=str(p),before_sha256=sha(before)if before is not None else None,after_sha256=sha(data)))
(root/'ui-saved.json').write_text(json.dumps(dict(scope='Native assembly inspector/controls and same-owner worker routing',files=rows),indent=2)+'\n')
print(json.dumps(dict(saved=len(rows),receipt=str(root/'ui-saved.json'))))
