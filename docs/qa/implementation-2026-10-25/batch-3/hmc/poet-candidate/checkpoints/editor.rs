//! Decorator mounted inside the native AuthorizedEditorHost. All caller intent
//! must pass that current owner gate before this service reaches filesystem IO.
use super::super::controller::{EditorHost, EditorOutcome, EditorSnapshot};
use super::super::preview::SourceRef;
use super::{CheckpointService, ExpectedFile, SaveReceipt};

pub struct CheckpointEditorHost {
    inner: Box<dyn EditorHost>,
    service: CheckpointService,
}

impl CheckpointEditorHost {
    pub fn new(inner: Box<dyn EditorHost>, service: CheckpointService) -> Result<Self, String> {
        if inner.snapshot()?.artifact_id != service.binding().source_id {
            return Err("checkpoint service does not match the selected native source host".into());
        }
        Ok(Self { inner, service })
    }
}

impl EditorHost for CheckpointEditorHost {
    fn snapshot(&self) -> Result<EditorSnapshot, String> {
        self.inner.snapshot().map(|mut snapshot| {
            snapshot.checkpoint_supported = true;
            snapshot
        })
    }
    fn check(&self, source: &str) -> Result<String, String> {
        self.inner.check(source)
    }
    fn submit(&mut self, source: &str, expected: SourceRef) -> Result<EditorOutcome, String> {
        self.inner.submit(source, expected)
    }
    fn undo(&mut self, expected: SourceRef) -> Result<EditorSnapshot, String> {
        self.inner.undo(expected)
    }
    fn redo(&mut self, expected: SourceRef) -> Result<EditorSnapshot, String> {
        self.inner.redo(expected)
    }
    fn rebuild_execution(&mut self, expected: SourceRef) -> Result<EditorOutcome, String> {
        self.inner.rebuild_execution(expected)
    }
    fn source_cells(&self, expected: SourceRef) -> Result<Vec<String>, String> {
        self.inner.source_cells(expected)
    }
    fn material_edit(
        &mut self,
        source: &str,
        expected: SourceRef,
    ) -> Result<EditorOutcome, String> {
        self.inner.material_edit(source, expected)
    }
    fn checkpoint_supported(&self) -> bool {
        true
    }
    fn save_checkpoint(
        &mut self,
        name: &str,
        expected_file: ExpectedFile,
        expected: SourceRef,
    ) -> Result<SaveReceipt, String> {
        let current = self.inner.snapshot()?;
        if current.source != expected {
            return Err("native checkpoint request source changed; no write".into());
        }
        expected.check_current()?;
        // Digest is accepted as lineage only after the service reloads and
        // validates those exact existing bytes against this source association.
        let parent = match expected_file {
            ExpectedFile::Absent => None,
            ExpectedFile::Sha256(digest) => Some(digest),
        };
        // Durable source/history save is available while execution is held.
        // This method does not evaluate source merely to manufacture resources.
        self.service
            .save(&expected.stamp, name, expected_file, parent, Vec::new())
            .map(|saved| saved.receipt)
    }
}
