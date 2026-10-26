"""Stage thin native owner/worker routing; never writes live sibling source."""
from pathlib import Path
import hashlib,json
HERE=Path(__file__).resolve().parent;POET=Path('C:/github/poet')
OUT=HERE/'integration';rows=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def save(rel,text):
    target=POET/rel;data=target.read_bytes() if target.exists() else None
    before=OUT/'before'/rel;stage=OUT/'after'/rel
    if data is not None:before.parent.mkdir(parents=True,exist_ok=True);before.write_bytes(data)
    stage.parent.mkdir(parents=True,exist_ok=True);stage.write_text(text,encoding='utf-8',newline='\n')
    rows.append(dict(target=str(target),stage=str(stage),before_sha256=sha(data) if data is not None else None,after_sha256=sha(stage.read_bytes())))
def read(rel):return (POET/rel).read_text(encoding='utf-8')
P='native/src/graphical/'
signature_inspect='''    fn inspect_typed_10d(&self, request: crate::graphical::checkpoints::ResourceInspectionRequest, expected: SourceRef,
        limits: typed::Typed10dLimits) -> Result<typed::Typed10dInspection, String>'''
signature_edit='''    fn edit_typed_10d(&mut self, request: typed::Typed10dEditRequest, expected: SourceRef,
        limits: typed::Typed10dLimits, cancelled: &std::sync::atomic::AtomicBool)
        -> Result<qualia_core_db::poet_host::invoke::studio_store::checkpoint::SaveReceipt, String>'''
for rel,marker,kind in [
    ('editor_contract.rs','pub trait EditorHost: Send {','contract'),
    ('checkpoints/editor.rs','impl EditorHost for CheckpointEditorHost {','checkpoint'),
    ('cooperation/editor.rs','impl EditorHost for AuthorizedEditorHost {','authorized'),
    ('package/mod.rs','impl EditorHost for ResourceImportHost {','inner'),
    ('shared_host.rs','impl EditorHost for SharedEditorHost {','current'),
    ('session_host.rs','impl EditorHost for SessionEditorHost {','current')]:
    s=read(P+rel)
    if kind=='contract':
        body=signature_inspect+' { let _=(request,expected,limits); Err("Native typed .10d inspection unavailable".into()) }\n'+signature_edit+' { let _=(request,expected,limits,cancelled); Err("Native typed .10d editing unavailable".into()) }\n'
    elif kind=='checkpoint':
        body=signature_inspect+''' {
        if self.inner.snapshot()?.source!=expected { return Err("Typed .10d source changed; no read".into()); }
        expected.check_current()?;
        let result=self.service.inspect_typed_10d(request,&expected,limits)?;
        expected.check_current()?;
        Ok(result)
    }
'''+signature_edit+''' {
        if self.inner.snapshot()?.source!=expected { return Err("Typed .10d source changed; no write".into()); }
        expected.check_current()?;
        self.service.edit_typed_10d(request,&expected,limits,cancelled)
    }
'''
    elif kind=='authorized':
        body=signature_inspect+''' {
        request.validate_bounds()?;
        with_authorized(self.owner.as_ref(),&self.scope,Some(&expected),AuthoringAction::ReadSource,self.limits,
            |_,_| self.inner.inspect_typed_10d(request,expected.clone(),limits))
    }
'''+signature_edit+''' {
        request.validate_bounds()?;
        let name=request.resource.filename.clone();
        let file=studio_store::checkpoint::ExpectedFile::Sha256(request.resource.package_digest);
        self.write_package(&name,file,&expected,|inner| inner.edit_typed_10d(request,expected.clone(),limits,cancelled))
    }
'''
    else:
        expr='self.inner' if kind=='inner' else 'self.current()?'
        body=signature_inspect+' { '+expr+'.inspect_typed_10d(request,expected,limits) }\n'+signature_edit+' { '+expr+'.edit_typed_10d(request,expected,limits,cancelled) }\n'
    assert s.count(marker)==1
    s='use crate::graphical::checkpoints::typed_10d as typed;\n'+s if not s.startswith('//!') else s
    if 'use crate::graphical::checkpoints::typed_10d as typed;' not in s:
        # Preserve inner module documentation at the start.
        lines=s.splitlines(keepends=True);i=0
        while i<len(lines) and lines[i].startswith('//!'):i+=1
        lines.insert(i,'use crate::graphical::checkpoints::typed_10d as typed;\n');s=''.join(lines)
    s=s.replace(marker,marker+'\n'+body,1);save(P+rel,s)
s=read(P+'checkpoints/mod.rs').replace('mod georaster_edit;','pub mod typed_10d;\nmod georaster_edit;',1);save(P+'checkpoints/mod.rs',s)
s=read(P+'jobs.rs')
s=s.replace('pub enum Action {','''pub enum Action {
    InspectTyped10d { request: super::checkpoints::ResourceInspectionRequest, expected: SourceRef },
    EditTyped10d { request: super::checkpoints::typed_10d::Typed10dEditRequest, expected: SourceRef },''',1)
s=s.replace('pub struct Reply {','pub struct Reply {\n    pub typed_10d: Option<super::checkpoints::typed_10d::Typed10dInspection>,',1)
s=s.replace('let mut inspection = None;','let mut inspection = None;\n    let mut typed_10d = None;',1)
s=s.replace('Action::EditGeoRaster{expected,..} |','Action::InspectTyped10d{expected,..} | Action::EditTyped10d{expected,..} | Action::EditGeoRaster{expected,..} |',1)
s=s.replace('&mut package_workflow, cancelled)','&mut package_workflow, &mut typed_10d, cancelled)',1)
s=s.replace('    Reply {\n','    Reply {\n        typed_10d: if inspection_is_current { typed_10d } else { None },\n',1)
s=s.replace('    match &action {\n','    match &action {\n        Action::InspectTyped10d { request, .. } => request.validate_bounds()?,\n        Action::EditTyped10d { request, .. } => request.validate_bounds()?,\n',1)
save(P+'jobs.rs',s)
s=read(P+'job_actions.rs').replace('    cancelled: &std::sync::atomic::AtomicBool,','    typed_10d: &mut Option<crate::graphical::checkpoints::typed_10d::Typed10dInspection>,\n    cancelled: &std::sync::atomic::AtomicBool,',1)
s=s.replace('    let message = match action {','''    let message = match action {
        Action::InspectTyped10d { request, expected } => {
            *typed_10d=Some(host.inspect_typed_10d(request,expected,crate::graphical::checkpoints::typed_10d::Typed10dLimits::default())?);
            "Inspected selected .10d material/frame/field snapshot; structural inspection is separate from model acceptance.".into()
        }
        Action::EditTyped10d { request, expected } => {
            let name=request.resource.filename.clone();
            let receipt=host.edit_typed_10d(request,expected.clone(),crate::graphical::checkpoints::typed_10d::Typed10dLimits::default(),cancelled)?;
            *inventory=host.package_inventory(&name,expected).ok();
            format!("Saved visual .10d material derivative in {name}; SHA256 {}. Source association is not proof the source regenerates the edited resource. Refresh package review.",crate::graphical::package::hex_digest(&receipt.digest))+&crate::graphical::package::durability_note(&receipt)
        }
''',1);save(P+'job_actions.rs',s)
s=read(P+'jobs_tests.rs');s=s.replace('    let reply = Reply {','    let reply = Reply {\n        typed_10d: None,',1);save(P+'jobs_tests.rs',s)
OUT.mkdir(parents=True,exist_ok=True)
(OUT/'target-manifest.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
print(json.dumps(dict(staged=len(rows),live_writes=False)))
