//! Reconnect each native operation to the current admitted session.
use super::{
    checkpoints::PackageInventory,
    controller::{EditorHost, EditorOutcome, EditorSnapshot},
    cooperation::AuthorizedEditorHost,
    cooperative_workflow::{WorkflowAction, WorkflowService, WorkflowView},
    native_project::{NativeProject, SessionSlot},
    package::{ImportRequest, LibraryEdit},
    preview::SourceRef,
};
use qualia_core_db::poet_host::invoke::studio_store::checkpoint::{
    ExpectedFile, ResourceEdit, SaveReceipt,
};
use qualia_core_db::poet_host::native_authority::SessionHandle;
use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, MutexGuard},
};

struct CurrentHost<'a> {
    guard: MutexGuard<'a, Option<(SessionHandle, AuthorizedEditorHost)>>,
}
impl Deref for CurrentHost<'_> {
    type Target = AuthorizedEditorHost;
    fn deref(&self) -> &Self::Target {
        &self.guard.as_ref().expect("bound native frontend").1
    }
}
impl DerefMut for CurrentHost<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard.as_mut().expect("bound native frontend").1
    }
}

pub struct SessionEditorHost {
    project: Arc<NativeProject>,
    sessions: SessionSlot,
    workflow: WorkflowService,
    connection: Mutex<Option<(SessionHandle, AuthorizedEditorHost)>>,
}
impl SessionEditorHost {
    pub fn new(project: Arc<NativeProject>, sessions: SessionSlot) -> Self {
        let workflow = project.workflow_service(sessions.clone());
        Self {
            project,
            sessions,
            workflow,
            connection: Mutex::new(None),
        }
    }
    fn current(&self) -> Result<CurrentHost<'_>, String> {
        let session = self
            .sessions
            .try_lock()
            .map_err(|_| "Native session selection is busy or unavailable")?
            .clone()
            .ok_or(
                "Unlock and authenticate an admitted contributor or reviewer before authoring",
            )?;
        let mut guard = self
            .connection
            .try_lock()
            .map_err(|_| "Native frontend binding is busy or poisoned")?;
        if guard
            .as_ref()
            .is_none_or(|(current, _)| current != &session)
        {
            let host = self.project.connect(session.clone())?;
            *guard = Some((session, host));
        }
        // Keep the derived frontend owner stable while the same opaque session
        // is selected. Its native authorization is still checked on every call;
        // rebuilding the owner on every read would invalidate publication stamps.
        Ok(CurrentHost { guard })
    }
}
impl EditorHost for SessionEditorHost {
    fn workflow(&mut self, action: WorkflowAction) -> Result<WorkflowView, String> {
        self.workflow.run(action)
    }
    fn snapshot(&self) -> Result<EditorSnapshot, String> {
        self.current()?.snapshot()
    }
    fn check(&self, s: &str) -> Result<String, String> {
        self.current()?.check(s)
    }
    fn submit(&mut self, s: &str, e: SourceRef) -> Result<EditorOutcome, String> {
        self.current()?.submit(s, e)
    }
    fn undo(&mut self, e: SourceRef) -> Result<EditorSnapshot, String> {
        self.current()?.undo(e)
    }
    fn redo(&mut self, e: SourceRef) -> Result<EditorSnapshot, String> {
        self.current()?.redo(e)
    }
    fn rebuild_execution(&mut self, e: SourceRef) -> Result<EditorOutcome, String> {
        self.current()?.rebuild_execution(e)
    }
    fn material_edit(&mut self, s: &str, e: SourceRef) -> Result<EditorOutcome, String> {
        self.current()?.material_edit(s, e)
    }
    fn source_cells(&self, e: SourceRef) -> Result<Vec<String>, String> {
        self.current()?.source_cells(e)
    }
    fn checkpoint_supported(&self) -> bool {
        true
    }
    fn resource_import_supported(&self) -> bool {
        self.current()
            .is_ok_and(|host| host.resource_import_supported())
    }
    fn save_checkpoint(
        &mut self,
        n: &str,
        f: ExpectedFile,
        e: SourceRef,
    ) -> Result<SaveReceipt, String> {
        self.current()?.save_checkpoint(n, f, e)
    }
    fn package_inventory(&self, n: &str, e: SourceRef) -> Result<PackageInventory, String> {
        self.current()?.package_inventory(n, e)
    }
    fn resource_edit_preflight(
        &self,
        n: &str,
        f: ExpectedFile,
        e: SourceRef,
    ) -> Result<(), String> {
        self.current()?.resource_edit_preflight(n, f, e)
    }
    fn edit_package_resources(
        &mut self,
        n: &str,
        f: ExpectedFile,
        r: Vec<ResourceEdit>,
        e: SourceRef,
    ) -> Result<SaveReceipt, String> {
        self.current()?.edit_package_resources(n, f, r, e)
    }
    fn import_package_resource(
        &mut self,
        n: &str,
        f: ExpectedFile,
        r: ImportRequest,
        e: SourceRef,
    ) -> Result<SaveReceipt, String> {
        self.current()?.import_package_resource(n, f, r, e)
    }
    fn apply_resource_plan(
        &mut self,
        n: &str,
        f: ExpectedFile,
        r: Vec<LibraryEdit>,
        e: SourceRef,
    ) -> Result<SaveReceipt, String> {
        self.current()?.apply_resource_plan(n, f, r, e)
    }
}
