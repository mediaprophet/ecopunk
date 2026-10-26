"""Root-owned review integration over frozen producer candidates; staging only."""
from pathlib import Path
import re

HERE=Path(__file__).resolve().parent
AFTER=HERE/'after'
OWNER=Path('C:/github/poet')

def get(rel):
    p=AFTER/rel
    return p.read_text(encoding='utf-8') if p.exists() else (OWNER/rel).read_text(encoding='utf-8')
def put(rel,text):
    p=AFTER/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(text,encoding='utf-8')
def edit(rel,old,new):
    s=get(rel);assert old in s,(rel,old[:70]);put(rel,s.replace(old,new,1))

G='native/src/graphical/'
edit(G+'mod.rs','pub mod package;','pub mod package;\npub mod package_workflow;\nmod package_review_controls;')
edit(G+'shared_host.rs','    fn current(&self)', '''    /// Trusted package workflow only: acquire after authority admission and
    /// retain through native ledger commit. Never wrap an authorized host here.
    pub(crate) fn package_review_guard(&self) -> Result<MutexGuard<'_, Box<dyn EditorHost>>, String> {
        self.current()
    }
    fn current(&self)''')
edit(G+'editor_contract.rs','pub trait EditorHost', 'pub trait EditorHost')
# Insert the default refusal into the actual trait body, preserving bounds.
s=get(G+'editor_contract.rs');m=re.search(r'pub trait EditorHost[^\{]*\{',s);assert m
s=s[:m.end()]+'''
    fn package_workflow(&mut self, _action: crate::graphical::package_workflow::PackageWorkflowAction,
        _expected: SourceRef) -> Result<crate::graphical::package_workflow::PackageWorkflowView,String> {
        Err("Native exact-package workflow unavailable for this host".into())
    }
'''+s[m.end():];put(G+'editor_contract.rs',s)
edit(G+'session_host.rs','impl EditorHost for SessionEditorHost {','''impl EditorHost for SessionEditorHost {
    fn package_workflow(&mut self, action: super::package_workflow::PackageWorkflowAction,
        expected: SourceRef) -> Result<super::package_workflow::PackageWorkflowView,String> {
        self.project.package_workflow_service(self.sessions.clone()).run(action,expected)
    }
''')
edit(G+'native_project.rs','impl NativeProject {','''impl NativeProject {
    pub(crate) fn package_workflow_service(&self,sessions:SessionSlot)->super::package_workflow::PackageWorkflowService {
        super::package_workflow::PackageWorkflowService::new(self.authority.clone(),sessions,self.shared.clone(),self.scope.clone(),self.limits)
    }
    pub(crate) fn verify_package_target(&self,target:&qualia_core_db::poet_host::native_authority::package_review::PackageRevision)->Result<(),String> {
        let guard=self.shared.package_review_guard()?;
        let source=guard.snapshot()?.source;
        super::package_workflow::verify_package_guard(guard.as_ref(),&self.scope,self.limits,target,&source)
    }
''')
edit(G+'jobs.rs','pub enum Action {','''pub enum Action {
    PackageWorkflow { action: super::package_workflow::PackageWorkflowAction, expected: SourceRef },''')
edit(G+'jobs.rs','pub struct Reply {','''pub struct Reply {
    pub package_workflow: Option<super::package_workflow::PackageWorkflowView>,''')
edit(G+'jobs.rs','    let mut inspection = None;','    let mut inspection = None;\n    let mut package_workflow = None;')
edit(G+'jobs.rs','Action::InspectResource{expected,..}', 'Action::PackageWorkflow{expected,..} | Action::InspectResource{expected,..}')
edit(G+'jobs.rs','&mut authoring,&mut inspection, cancelled)', '&mut authoring,&mut inspection,&mut package_workflow, cancelled)')
edit(G+'jobs.rs','    Reply {\n', '''    if package_workflow.is_some() && !inspection_is_current {
        package_workflow=None;
        message.push_str("; package workflow context changed; inspect before retrying");
    }
    Reply {
        package_workflow,
''')
edit(G+'jobs.rs','pub async fn run(host: HostHandle, action: Action, origin: Context) -> Result<Reply, String> {','''pub async fn run(host: HostHandle, action: Action, origin: Context) -> Result<Reply, String> {
    if let Action::PackageWorkflow{action,..}=&action { action.validate()?; }
''')
edit(G+'job_actions.rs','    cancelled: &std::sync::atomic::AtomicBool,','''    package_workflow: &mut Option<crate::graphical::package_workflow::PackageWorkflowView>,
    cancelled: &std::sync::atomic::AtomicBool,''')
edit(G+'job_actions.rs','    let message = match action {','''    let message = match action {
        Action::PackageWorkflow {action,expected} => {
            if cancelled.load(std::sync::atomic::Ordering::Acquire) {return Err("Worker closed before package workflow admission".into());}
            *package_workflow=Some(host.package_workflow(action,expected)?);
            "Saved-package workflow inspected/recorded for the exact digest; refresh after any package or authority change.".into()
        }
''')
edit(G+'jobs_tests.rs','    let reply = Reply {','    let reply = Reply {\n        package_workflow: None,')
edit(G+'package_controls.rs','            if let Some(package)=entries {','''            if let Some(package)=entries {
                crate::graphical::package_review_controls::PackageReviewControls{
                    host:host.clone(),current:current.clone(),filename:name(),digest:package.digest,busy,status
                }
''')

T=G+'native_entry/types.rs'
edit(T,'pub enum LaunchAction {','''pub enum LaunchAction {
    GrantPackageReview(PackageGrantIntent),
    RevokePackageReview {grant_id:String,evidence:String},''')
edit(T,'    let values: Vec<&str> = match action {','''    let values: Vec<&str> = match action {
        LaunchAction::GrantPackageReview(intent)=>return intent.validate(),
        LaunchAction::RevokePackageReview{grant_id,evidence}=>vec![grant_id,evidence],''')
put(T,get(T)+'''
#[derive(Clone,Debug)]
pub struct PackageGrantIntent {
    pub filename:String,pub package_id:String,pub package_digest:String,
    pub subject:String,pub grant_id:String,pub evidence:String,pub seconds:u32,
}
impl PackageGrantIntent {
    pub fn validate(&self)->Result<(),String> {
        crate::graphical::package::parse_digest(&self.package_digest)?;
        if self.seconds==0 || self.seconds>86400 || self.filename.len()>128
            || [&self.filename,&self.package_id,&self.subject,&self.grant_id,&self.evidence]
                .iter().any(|v|v.trim().is_empty()||v.len()>4096||v.contains('\\0')) {
            return Err("Explicit bounded package grant fields and duration 1..86400 seconds required".into());
        }
        Ok(())
    }
}
''')
edit(G+'native_entry/mod.rs','mod types;','mod types;\nmod package_admin;\nmod package_grant_controls;')
edit(G+'native_entry/actions.rs','        let result = match action {','''        let result = match action {
            LaunchAction::GrantPackageReview(intent)=>self.grant_package(intent),
            LaunchAction::RevokePackageReview{grant_id,evidence}=>self.revoke_package(&grant_id,&evidence),''')
edit(G+'native_entry/actions.rs','    fn authority_signer(&self)', '    pub(in crate::graphical::native_entry) fn authority_signer(&self)')
edit(G+'native_entry/ui.rs','fn dispatch(', 'pub(super) fn dispatch(')
edit(G+'native_entry/ui.rs','            if let Some(lease)=current.studio{','''            if current.current_subject.is_some(){
                super::package_grant_controls::PackageGrantControls{handle:handle.clone(),view,status,busy}
            }
            if let Some(lease)=current.studio{''')

# Empty new scope preserves old signed-admission serialization. Constructors
# must choose no package grants explicitly, never inherit source grants.
for rel in [G+'native_entry/create.rs',G+'native_project_tests.rs',G+'poet_workspace/tests.rs',
            'native/examples/growing_reference/authority.rs']:
    s=get(rel);assert 'AdmissionSnapshot {' in s,rel
    put(rel,s.replace('AdmissionSnapshot {','AdmissionSnapshot {\n        package_reviews: Vec::new(),'))
# Option source authorship is not a displayable string or a fabricated subject.
rel=G+'package_review_controls.rs'
s=get(rel).replace('{view.evidence.target.source.author_subject}.','{view.evidence.target.source.author_subject:?}.')
s=s.replace('status.set("Effort requires whole nonnegative minutes".into());','{let mut msg=status;msg.set("Effort requires whole nonnegative minutes".into());}')
put(rel,s)
print('Root review routing staged; no live source written')
