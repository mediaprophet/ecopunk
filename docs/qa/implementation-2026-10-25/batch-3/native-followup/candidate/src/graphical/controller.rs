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
    pub material_edit_id: Option<u64>,
    pub checkpoint_supported: bool,
}

pub struct EditorOutcome {
    pub snapshot: EditorSnapshot,
    pub message: String,
    pub frame: Option<PreviewFrame>,
}

pub trait PreviewProvider: Send {
    /// Native output admission before local session/source promotion; no GPU.
    fn validate(&self, _value: &Value) -> Result<(), String> { Ok(()) }
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
    fn validate(&self, value: &Value) -> Result<(), String> {
        if let Value::Record(record)=value {
            if record.contains_key("geometry") || record.contains_key("material") {
                return super::materials::authored_scene(value,self.material,self.limits).map(|_| ());
            }
            if record.contains_key("base_color") && record.contains_key("metallic") && record.contains_key("roughness") {
                return super::materials::material_from_value(value).map(|_| ());
            }
        }
        // Ordinary helper/function/scalar cells remain valid persistent REPL
        // submissions. A non-scene result simply cannot produce a mesh frame.
        Ok(())
    }
    fn preview(&mut self, source: SourceRef, value: &Value) -> Result<PreviewFrame, String> {
        let (mesh, material) = super::materials::authored_scene(value, self.material, self.limits)?;
        preview::render_mesh(source, &mesh, material, self.camera, self.limits)
    }
}

pub trait EditorHost: Send {
    fn source_cells(&self, _expected: SourceRef) -> Result<Vec<String>, String> {
        Err("identified source-cell read is unavailable for this host".into())
    }
    fn material_edit(&mut self, source: &str, expected: SourceRef) -> Result<EditorOutcome, String> {
        let _ = (source, expected);
        Err("material-only source admission is unavailable for this host".into())
    }
    fn checkpoint_supported(&self) -> bool { false }
    fn save_checkpoint(&mut self, _name: &str,
        _expected_file: studio_store::checkpoint::ExpectedFile, _expected: SourceRef,
    ) -> Result<studio_store::checkpoint::SaveReceipt, String> {
        Err("native checkpoint destination/admission is unavailable for this host".into())
    }
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

fn material_edit_id(session: &SourceSession, held: bool) -> Option<u64> {
    if held || !session.is_mutable("poet_geometry") || !session.is_mutable("poet_material") {
        return None;
    }
    let material = super::materials::material_from_value(session.value("poet_material")?).ok()?;
    if !matches!(session.value("poet_geometry")?, Value::Record(_)) { return None; }
    Some(material.id)
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
                material_edit_id: material_edit_id(&candidate.session, false),
                checkpoint_supported: false,
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
    fn material_edit(&mut self, source: &str, expected: SourceRef) -> Result<EditorOutcome,String> {
        let snapshot=self.snapshot()?;
        let id=snapshot.material_edit_id.ok_or("coherent mutable material/geometry bindings are required")?;
        let next=self.session.revision().checked_add(1).ok_or("material revision exhausted")?;
        super::material_edit::validate(source,next,id)?;
        self.submit(source,expected)
    }
    fn source_cells(&self, expected: SourceRef) -> Result<Vec<String>, String> {
        if expected.artifact_id != self.artifact_id { return Err("foreign source read".into()); }
        expected.check_current()?;
        let (artifact, stamp) = studio_store::get_artifact_stamped(&self.artifact_id)
            .ok_or("source artifact disappeared")?;
        if stamp != expected.stamp { return Err("source changed before read".into()); }
        let cells = source_cells(&artifact)?.into_iter().map(str::to_string).collect();
        expected.check_current()?;
        Ok(cells)
    }
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
            material_edit_id: material_edit_id(&self.session, self.reconciliation_required || stamp != self.active_stamp),
            checkpoint_supported: false,
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
        let provider = &self.preview;
        let result = match self.session.submit_validated(source, &mut self.host, Budget::default(), |value| {
            provider.validate(value).map_err(|message| vibe::Diagnostic::new(
                vibe::DiagCode::E701, vibe::Span::point(0), message))?;
            expected.check_current().map_err(|message| vibe::Diagnostic::new(
                vibe::DiagCode::E701, vibe::Span::point(0), message))
        })
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
            self.preview.as_ref(),
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

#[cfg(test)]
#[path="material_admission_tests.rs"]
mod material_admission_tests;
