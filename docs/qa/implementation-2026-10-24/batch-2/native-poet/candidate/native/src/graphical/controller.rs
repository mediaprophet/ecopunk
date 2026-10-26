//! Canonical session + Studio source adapter. UI drafts are not document state.
use super::geometry_host::GeometryHost;
use super::preview::{self, PreviewFrame, SourceRef};
use qualia_core_db::poet_host::invoke::studio_store::{self, StudioArtifact};
use vibe::{
    session::{SessionLimits, SourceSession},
    Budget, Value,
};
#[path = "execution.rs"]
mod execution;
pub use execution::{RebuildLimits, SessionEpoch};

const SOURCE_PREFIX: &str = "poet.native.submission/";

#[derive(Clone)]
pub struct EditorSnapshot {
    pub artifact_id: String,
    pub title: String,
    pub source_revision: u32,
    pub source: SourceRef,
    pub session_revision: u64,
    pub execution_epoch: SessionEpoch,
    pub rebuild_supported: bool,
    pub latest_source: String,
    pub reconciliation_required: bool,
}

pub struct EditorOutcome {
    pub snapshot: EditorSnapshot,
    pub message: String,
    pub frame: Option<PreviewFrame>,
}

pub trait PreviewProvider: Send {
    /// Called with actual evaluated engine output after checked source commit.
    fn preview(&mut self, source: SourceRef, value: &Value) -> Result<PreviewFrame, String>;
}

pub struct MeshPreviewProvider {
    /// Caller-supplied native material/view inputs; no inferred physical rights.
    pub material: qualia_core_db::container_10d::MaterialRecord,
    pub camera: preview::PreviewCamera,
    pub limits: preview::PreviewLimits,
}

impl PreviewProvider for MeshPreviewProvider {
    fn preview(&mut self, source: SourceRef, value: &Value) -> Result<PreviewFrame, String> {
        let mesh = preview::mesh_from_invoke(value, self.limits)?;
        preview::render_mesh(source, &mesh, self.material, self.camera, self.limits)
    }
}

pub trait EditorHost: Send {
    fn snapshot(&self) -> Result<EditorSnapshot, String>;
    fn check(&self, source: &str) -> Result<String, String>;
    fn submit(&mut self, source: &str, expected: SourceRef) -> Result<EditorOutcome, String>;
    fn undo(&mut self, expected: SourceRef) -> Result<EditorSnapshot, String>;
    fn redo(&mut self, expected: SourceRef) -> Result<EditorSnapshot, String>;
    fn rebuild_execution(&mut self, _expected: SourceRef) -> Result<EditorOutcome, String> {
        Err("explicit execution rebuild is unavailable for this host profile".into())
    }
}

pub struct CanonicalEditorHost {
    artifact_id: String,
    session: SourceSession,
    host: GeometryHost,
    preview: Box<dyn PreviewProvider>,
    reconciliation_required: bool,
    active_stamp: studio_store::StoreStamp,
    factory_limits: SessionLimits,
    rebuild_limits: RebuildLimits,
    execution_epoch: SessionEpoch,
}

fn source_cells(artifact: &StudioArtifact) -> Result<Vec<&str>, String> {
    artifact
        .nodes
        .iter()
        .filter(|(key, _)| key.starts_with(SOURCE_PREFIX))
        .enumerate()
        .map(|(index, (key, node))| {
            if key != &format!("{SOURCE_PREFIX}{:020}", index + 1) {
                return Err(
                    "stored native source cells are not a contiguous identified sequence".into(),
                );
            }
            node.get("text")
                .map(String::as_str)
                .ok_or_else(|| "stored source cell lacks text".into())
        })
        .collect()
}

impl CanonicalEditorHost {
    /// Host-owned factory grants/limits create the canonical interpreter session.
    /// Existing source opens held until explicit read-only profile reconstruction.
    pub fn open(
        artifact_id: String,
        factory_limits: SessionLimits,
        host: GeometryHost,
        preview: Box<dyn PreviewProvider>,
    ) -> Result<Self, String> {
        Self::open_with_rebuild_limits(
            artifact_id,
            factory_limits,
            host,
            preview,
            RebuildLimits::default(),
        )
    }

    pub fn open_with_rebuild_limits(
        artifact_id: String,
        factory_limits: SessionLimits,
        host: GeometryHost,
        preview: Box<dyn PreviewProvider>,
        rebuild_limits: RebuildLimits,
    ) -> Result<Self, String> {
        let (artifact, active_stamp) = studio_store::get_artifact_stamped(&artifact_id)
            .ok_or_else(|| {
                "native Poet requires an existing identified source artifact".to_string()
            })?;
        let reconciliation_required = !source_cells(&artifact)?.is_empty();
        let session = SourceSession::new(host.actual_grants().to_vec(), factory_limits.clone());
        Ok(Self {
            artifact_id,
            session,
            host,
            preview,
            reconciliation_required,
            active_stamp,
            factory_limits,
            rebuild_limits,
            execution_epoch: SessionEpoch::new(),
        })
    }

    fn expected(&self, expected: &SourceRef) -> Result<(), String> {
        if expected.artifact_id != self.artifact_id || expected.stamp != self.active_stamp {
            return Err("source/session incarnation mismatch; no evaluation attempted".into());
        }
        expected.check_current()
    }

    fn activate_rebuild(
        &mut self,
        candidate: execution::PreparedRebuild,
    ) -> Result<EditorOutcome, String> {
        // Prepare every fallible read/shape check and the complete response
        // before changing execution. No store read can fail after promotion.
        let latest_source = source_cells(&candidate.artifact)?
            .last()
            .copied()
            .unwrap_or("")
            .to_string();
        let next_epoch = SessionEpoch::new();
        let outcome = EditorOutcome {
            snapshot: EditorSnapshot {
                artifact_id: candidate.artifact.id.clone(), title: candidate.artifact.title.clone(),
                source_revision: candidate.artifact.lamport_clock,
                source: candidate.source.clone(), session_revision: candidate.session.revision(),
                execution_epoch: next_epoch.clone(), rebuild_supported: true, latest_source,
                reconciliation_required: false,
            }, frame: None,
            message: "Execution rebuilt from accepted source under the unchanged read-only geometry profile; no frame published".into(),
        };
        if candidate.source.artifact_id != self.artifact_id
            || candidate.source.check_current().is_err()
        {
            self.reconciliation_required = true;
            return Err(
                "rebuild source could not be revalidated; previous execution/epoch preserved"
                    .into(),
            );
        }
        self.execution_epoch.retire();
        self.session = candidate.session;
        self.host = candidate.host;
        self.active_stamp = candidate.source.stamp;
        self.execution_epoch = next_epoch;
        self.reconciliation_required = false;
        Ok(outcome)
    }
}

impl Drop for CanonicalEditorHost {
    fn drop(&mut self) {
        self.execution_epoch.retire();
    }
}

impl EditorHost for CanonicalEditorHost {
    fn snapshot(&self) -> Result<EditorSnapshot, String> {
        let (artifact, stamp) = studio_store::get_artifact_stamped(&self.artifact_id)
            .ok_or_else(|| "source artifact disappeared".to_string())?;
        let latest_source = source_cells(&artifact)?
            .last()
            .copied()
            .unwrap_or("")
            .to_string();
        Ok(EditorSnapshot {
            artifact_id: artifact.id.clone(),
            title: artifact.title,
            source_revision: artifact.lamport_clock,
            session_revision: self.session.revision(),
            execution_epoch: self.execution_epoch.clone(),
            rebuild_supported: true,
            source: SourceRef {
                artifact_id: artifact.id,
                stamp: stamp.clone(),
            },
            latest_source,
            reconciliation_required: self.reconciliation_required || stamp != self.active_stamp,
        })
    }

    fn check(&self, source: &str) -> Result<String, String> {
        if self.snapshot()?.reconciliation_required {
            return Err("source/session reconciliation required".into());
        }
        self.session
            .prepare(source)
            .map(|_| "Canonical Vibe check passed; no execution attempted".into())
            .map_err(|diagnostic| {
                format!(
                    "{:?} at {:?}: {}",
                    diagnostic.code, diagnostic.span, diagnostic.message
                )
            })
    }

    fn submit(&mut self, source: &str, expected: SourceRef) -> Result<EditorOutcome, String> {
        if self.reconciliation_required {
            return Err("source/session reconciliation required before further execution".into());
        }
        self.expected(&expected)?;
        if !self.execution_epoch.is_current() {
            return Err("execution epoch has retired".into());
        }
        self.host.begin_submission();
        let result = match self
            .session
            .submit(source, &mut self.host, Budget::default())
        {
            Ok(result) => result,
            Err(error) => {
                self.reconciliation_required = error.host_reconciliation_required;
                return Err(format!(
                    "{:?} at {:?}: {}; execution attempted={}; host reconciliation required={}",
                    error.diagnostic.code,
                    error.diagnostic.span,
                    error.diagnostic.message,
                    error.execution_attempted,
                    error.host_reconciliation_required
                ));
            }
        };
        // External effects and the session may already have advanced. Do not
        // retry or pretend rollback if another contributor won the store commit.
        let key = format!("{SOURCE_PREFIX}{:020}", result.revision);
        let committed =
            studio_store::mutate_artifact_stamped(&self.artifact_id, &expected.stamp, |artifact| {
                artifact
                    .nodes
                    .entry(key)
                    .or_default()
                    .insert("text".into(), source.to_string());
                Ok(())
            });
        let (artifact, stamp) = match committed {
            Ok(result) => result,
            Err(error) => {
                self.reconciliation_required = true;
                return Err(format!("{error}; session/effects already attempted; host reconciliation required, no retry"));
            }
        };
        self.active_stamp = stamp.clone();
        let binding = SourceRef {
            artifact_id: artifact.id,
            stamp,
        };
        let requested = binding.clone();
        let (frame, message) = match self.preview.preview(binding, &result.value) {
            Ok(mut frame) if frame.source == requested => {
                frame.session_revision = Some(result.revision);
                frame.execution_epoch = Some(self.execution_epoch.clone());
                (
                    Some(frame),
                    "Source accepted; native engine preview rendered (review unassessed)".into(),
                )
            }
            Ok(_) => (
                None,
                "Source accepted; provider returned mismatched source; frame withheld".into(),
            ),
            Err(error) => (
                None,
                format!("Source accepted; preview unavailable: {error}"),
            ),
        };
        Ok(EditorOutcome {
            snapshot: self.snapshot()?,
            message,
            frame,
        })
    }

    fn undo(&mut self, expected: SourceRef) -> Result<EditorSnapshot, String> {
        if expected.artifact_id != self.artifact_id {
            return Err("foreign source reference".into());
        }
        let (_, stamp) = studio_store::undo_artifact_stamped(&self.artifact_id, &expected.stamp)?;
        self.active_stamp = stamp;
        // SourceSession has no reverse-effect/runtime rollback contract. Keep
        // the restored editable source and require host reconciliation/reopen.
        self.reconciliation_required = true;
        self.snapshot()
    }

    fn redo(&mut self, expected: SourceRef) -> Result<EditorSnapshot, String> {
        if expected.artifact_id != self.artifact_id {
            return Err("foreign source reference".into());
        }
        let (_, stamp) = studio_store::redo_artifact_stamped(&self.artifact_id, &expected.stamp)?;
        self.active_stamp = stamp;
        self.reconciliation_required = true;
        self.snapshot()
    }

    fn rebuild_execution(&mut self, expected: SourceRef) -> Result<EditorOutcome, String> {
        if expected.artifact_id != self.artifact_id {
            return Err("foreign rebuild source reference".into());
        }
        if !self.execution_epoch.is_current() {
            return Err("execution epoch has retired".into());
        }
        let candidate = match execution::prepare(
            expected,
            &self.host,
            &self.factory_limits,
            &self.rebuild_limits,
        ) {
            Ok(candidate) => candidate,
            Err(error) => {
                self.reconciliation_required = true;
                return Err(error);
            }
        };
        self.activate_rebuild(candidate)
    }
}

#[cfg(test)]
#[path = "controller_tests.rs"]
mod tests;
