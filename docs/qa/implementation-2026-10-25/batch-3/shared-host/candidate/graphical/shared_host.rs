//! Share one canonical editor/session between native UI and MCP projections.
//!
//! Mount raw CanonicalEditorHost (optionally decorated by CheckpointEditorHost)
//! INSIDE this adapter. Mount a separate AuthorizedEditorHost OUTSIDE each clone
//! with its own current native owner/scope. Never wrap an AuthorizedEditorHost
//! here: nested owner acquisition could deadlock and would confuse caller scope.
//! This adapter owns no project, authority, source cache or second session.
use super::controller::{EditorHost, EditorOutcome, EditorSnapshot};
use super::preview::SourceRef;
use qualia_core_db::poet_host::invoke::studio_store::checkpoint::{ExpectedFile, SaveReceipt};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};

#[derive(Clone)]
pub struct SharedEditorHost {
    inner: Arc<Mutex<Box<dyn EditorHost>>>,
    // Native service availability is fixed at composition, independently of
    // current user authority or momentary lock contention. No auth is cached.
    checkpoint_supported: bool,
}

impl SharedEditorHost {
    /// Supply only the raw canonical/checkpoint editor, before authority wraps
    /// each frontend. Existing host grants/state remain in that one editor.
    pub fn new(inner: Box<dyn EditorHost>) -> Self {
        let checkpoint_supported = inner.checkpoint_supported();
        Self {
            inner: Arc::new(Mutex::new(inner)),
            checkpoint_supported,
        }
    }

    fn current(&self) -> Result<MutexGuard<'_, Box<dyn EditorHost>>, String> {
        match self.inner.try_lock() {
            Ok(editor) => Ok(editor),
            Err(TryLockError::WouldBlock) => Err(
                "Shared native editor is busy; operation was not dispatched; no automatic retry"
                    .into(),
            ),
            Err(TryLockError::Poisoned(_)) => Err(
                "Shared native editor is poisoned; host reconciliation is required before dispatch"
                    .into(),
            ),
        }
    }
}

impl EditorHost for SharedEditorHost {
    fn snapshot(&self) -> Result<EditorSnapshot, String> {
        self.current()?.snapshot()
    }
    fn check(&self, source: &str) -> Result<String, String> {
        self.current()?.check(source)
    }
    fn source_cells(&self, expected: SourceRef) -> Result<Vec<String>, String> {
        self.current()?.source_cells(expected)
    }
    fn checkpoint_supported(&self) -> bool {
        self.checkpoint_supported
    }
    fn submit(&mut self, source: &str, expected: SourceRef) -> Result<EditorOutcome, String> {
        self.current()?.submit(source, expected)
    }
    fn material_edit(
        &mut self,
        source: &str,
        expected: SourceRef,
    ) -> Result<EditorOutcome, String> {
        self.current()?.material_edit(source, expected)
    }
    fn undo(&mut self, expected: SourceRef) -> Result<EditorSnapshot, String> {
        self.current()?.undo(expected)
    }
    fn redo(&mut self, expected: SourceRef) -> Result<EditorSnapshot, String> {
        self.current()?.redo(expected)
    }
    fn rebuild_execution(&mut self, expected: SourceRef) -> Result<EditorOutcome, String> {
        self.current()?.rebuild_execution(expected)
    }
    fn save_checkpoint(
        &mut self,
        name: &str,
        expected_file: ExpectedFile,
        expected: SourceRef,
    ) -> Result<SaveReceipt, String> {
        self.current()?
            .save_checkpoint(name, expected_file, expected)
    }
}

#[cfg(test)]
#[path = "shared_host_tests.rs"]
mod tests;
