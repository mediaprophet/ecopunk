//! One native admission gateway for UI/MCP operations on identified source.
//! Authority is injected by a trusted owner, never restored from source/HMC.
use super::controller::{EditorHost, EditorOutcome, EditorSnapshot};
use super::preview::SourceRef;
use qualia_cooperative_core::authoring::{
    self, AuthoringAction, AuthoringPermit, AuthoringRequest, HostAuthorityView,
};
use qualia_core_db::poet_host::invoke::studio_store::{self, StudioArtifact};
use sha2::{Digest, Sha256};
use std::{io::Write, sync::Arc};

/// Host binding, not an authentication or permission claim. No Deserialize.
#[derive(Clone, Debug)]
pub struct Scope {
    pub project_id: String,
    pub document_id: String,
    pub source_id: String,
    pub domain: String,
}

#[derive(Clone, Copy)]
pub struct GatewayLimits {
    pub max_scope_text_bytes: usize,
    pub max_source_cells: usize,
    pub max_nodes: usize,
    pub max_digest_bytes: usize,
}
impl Default for GatewayLimits {
    fn default() -> Self {
        Self {
            max_scope_text_bytes: 4096,
            max_source_cells: 64,
            max_nodes: 65_536,
            max_digest_bytes: 4 * 1024 * 1024,
        }
    }
}

/// Native provider must hold its current-state lock/transaction for the entire
/// callback, including source/evidence commit. It must call exactly once and
/// update its admitted document metadata after a write before allowing another
/// action. This trait is not an authentication implementation or cross-store
/// atomicity mechanism. Failed postcommit publication must require reconciliation.
pub trait AuthorityOwner: Send + Sync {
    fn with_current(
        &self,
        operation: &mut dyn FnMut(&HostAuthorityView<'_>) -> Result<(), String>,
    ) -> Result<(), String>;
}

pub struct UnboundAuthorityOwner;
impl AuthorityOwner for UnboundAuthorityOwner {
    fn with_current(
        &self,
        _: &mut dyn FnMut(&HostAuthorityView<'_>) -> Result<(), String>,
    ) -> Result<(), String> {
        Err("No authenticated cooperative authority owner is bound; operation denied".into())
    }
}

struct DigestWriter {
    digest: Sha256,
    bytes: usize,
    limit: usize,
}
impl Write for DigestWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let next = self
            .bytes
            .checked_add(bytes.len())
            .filter(|n| *n <= self.limit)
            .ok_or_else(|| std::io::Error::other("source digest byte budget exceeded"))?;
        self.digest.update(bytes);
        self.bytes = next;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Deterministic, bounded hash projection of actual current authored state.
/// BTreeMap ordering is canonical here. Revision is separately bound to Lamport;
/// history/redo are excluded. This is not a saved authority record or file codec.
pub fn source_digest(artifact: &StudioArtifact, limits: GatewayLimits) -> Result<[u8; 32], String> {
    if artifact.nodes.len() > limits.max_nodes {
        return Err("source node count exceeds gateway bounds".into());
    }
    let mut cells = 0usize;
    for (key, node) in &artifact.nodes {
        if key.starts_with("poet.native.submission/") {
            cells = cells.checked_add(1).ok_or("source cell count overflow")?;
            if cells > limits.max_source_cells
                || key != &format!("poet.native.submission/{cells:020}")
                || !node.contains_key("text")
            {
                return Err(
                    "source cells exceed bounds or are not an identified contiguous sequence"
                        .into(),
                );
            }
        }
    }
    let mut writer = DigestWriter {
        digest: Sha256::new(),
        bytes: 0,
        limit: limits.max_digest_bytes,
    };
    serde_json::to_writer(
        &mut writer,
        &(
            "poet.source-authority.v1",
            &artifact.id,
            artifact.did_q42,
            &artifact.domain,
            &artifact.title,
            artifact.routing_lane,
            artifact.sensitivity,
            artifact.context_did_hash,
            &artifact.nodes,
            &artifact.properties,
        ),
    )
    .map_err(|error| format!("source digest projection refused: {error}"))?;
    Ok(writer.digest.finalize().into())
}

fn validate_scope(scope: &Scope, limits: GatewayLimits) -> Result<(), String> {
    if [
        &scope.project_id,
        &scope.document_id,
        &scope.source_id,
        &scope.domain,
    ]
    .into_iter()
    .any(|s| s.is_empty() || s.len() > limits.max_scope_text_bytes)
        || limits.max_source_cells == 0
        || limits.max_nodes == 0
        || limits.max_digest_bytes == 0
    {
        return Err("invalid native project/document/source scope or bounds".into());
    }
    Ok(())
}

/// The provider's current generation is authoritative; request arguments cannot
/// grant authority. Every operation receives one borrowed current permit. Saved
/// records cannot create this permit. A provider error after callback execution
/// is reported honestly: source/effects may have committed and need reconciliation.
pub fn with_authorized<T>(
    owner: &dyn AuthorityOwner,
    scope: &Scope,
    expected: Option<&SourceRef>,
    action: AuthoringAction,
    limits: GatewayLimits,
    operation: impl FnOnce(&HostAuthorityView<'_>, AuthoringPermit<'_>) -> Result<T, String>,
) -> Result<T, String> {
    validate_scope(scope, limits)?;
    let mut calls = 0usize;
    let mut duplicate = false;
    let mut operation = Some(operation);
    let mut output = None;
    let mut callback = |view: &HostAuthorityView<'_>| {
        calls = calls.saturating_add(1);
        if calls != 1 {
            duplicate = true;
            return Err(
                "authority owner invoked operation more than once; no repeated dispatch".into(),
            );
        }
        let result = (|| {
            let current = view.current_document;
            if current.project_id != scope.project_id
                || current.document_id != scope.document_id
                || current.source_id != scope.source_id
            {
                return Err("current cooperative document differs from native host scope".into());
            }
            // Authenticate before cloning/hashing actual source. The permit is
            // not used until the actual source digest/stamp is also bound.
            let permit = authoring::authorize(
                view,
                &AuthoringRequest {
                    action,
                    target: current,
                    expected_authority_generation: view.generation,
                },
            )
            .map_err(|error| error.to_string())?;
            let (artifact, stamp) = studio_store::get_artifact_stamped(&scope.source_id)
                .ok_or("native source disappeared")?;
            if artifact.id != scope.source_id || artifact.domain != scope.domain {
                return Err("native source scope/domain differs from its host binding".into());
            }
            let source = SourceRef {
                artifact_id: artifact.id.clone(),
                stamp,
            };
            if expected.is_some_and(|expected| expected != &source) {
                return Err(
                    "source stamp differs from requested native operation; no dispatch".into(),
                );
            }
            if current.revision != u64::from(artifact.lamport_clock)
                || current.source_digest != source_digest(&artifact, limits)?
            {
                return Err("current cooperative document does not bind the actual native source revision/digest".into());
            }
            source.check_current()?;
            let result =
                operation
                    .take()
                    .ok_or("native operation was already consumed")?(view, permit)?;
            if action == AuthoringAction::ReadSource {
                source.check_current()?;
            }
            Ok(result)
        })();
        let status = result.as_ref().map(|_| ()).map_err(Clone::clone);
        output = Some(result);
        status
    };
    let provider_result = owner.with_current(&mut callback);
    drop(callback);
    if duplicate {
        return Err("authority owner invoked callback repeatedly; operation was not repeated; first operation may have committed, reconciliation required".into());
    }
    if let Err(error) = provider_result {
        return Err(if calls == 0 {
            error
        } else {
            format!("authority owner failed after admission callback; operation may have committed; reconciliation required: {error}")
        });
    }
    output.ok_or("authority owner did not admit any operation")?
}

/// Wrap before passing a host to either native UI or MCP. This wrapper cannot
/// bypass the underlying canonical source stamp/session checks. No bootstrap
/// principal, CLI identity, local roster or checkpoint-derived grant is supplied.
pub struct AuthorizedEditorHost {
    inner: Box<dyn EditorHost>,
    scope: Scope,
    owner: Arc<dyn AuthorityOwner>,
    limits: GatewayLimits,
}
impl AuthorizedEditorHost {
    pub fn new(
        inner: Box<dyn EditorHost>,
        scope: Scope,
        owner: Arc<dyn AuthorityOwner>,
        limits: GatewayLimits,
    ) -> Result<Self, String> {
        validate_scope(&scope, limits)?;
        Ok(Self {
            inner,
            scope,
            owner,
            limits,
        })
    }
    fn read<T>(
        &self,
        operation: impl FnOnce(&dyn EditorHost) -> Result<T, String>,
    ) -> Result<T, String> {
        with_authorized(
            self.owner.as_ref(),
            &self.scope,
            None,
            AuthoringAction::ReadSource,
            self.limits,
            |view, _| {
                let snapshot = self.inner.snapshot()?;
                if snapshot.source.artifact_id != self.scope.source_id
                    || snapshot.artifact_id != self.scope.source_id
                    || u64::from(snapshot.source_revision) != view.current_document.revision
                {
                    return Err("foreign inner editor source".into());
                }
                snapshot.source.check_current()?;
                operation(self.inner.as_ref())
            },
        )
    }
    fn write<T>(
        &mut self,
        expected: &SourceRef,
        action: AuthoringAction,
        operation: impl FnOnce(&mut dyn EditorHost) -> Result<T, String>,
    ) -> Result<T, String> {
        let inner = &mut self.inner;
        with_authorized(
            self.owner.as_ref(),
            &self.scope,
            Some(expected),
            action,
            self.limits,
            |_, _| operation(inner.as_mut()),
        )
    }
}
impl EditorHost for AuthorizedEditorHost {
    fn snapshot(&self) -> Result<EditorSnapshot, String> {
        self.read(|inner| inner.snapshot())
    }
    fn check(&self, source: &str) -> Result<String, String> {
        self.read(|inner| inner.check(source))
    }
    fn source_cells(&self, expected: SourceRef) -> Result<Vec<String>, String> {
        with_authorized(
            self.owner.as_ref(),
            &self.scope,
            Some(&expected),
            AuthoringAction::ReadSource,
            self.limits,
            |_, _| self.inner.source_cells(expected.clone()),
        )
    }
    fn checkpoint_supported(&self) -> bool {
        self.inner.checkpoint_supported()
    }
    fn submit(&mut self, source: &str, expected: SourceRef) -> Result<EditorOutcome, String> {
        self.write(&expected, AuthoringAction::ExecuteSource, |inner| {
            inner.submit(source, expected.clone())
        })
    }
    fn material_edit(
        &mut self,
        source: &str,
        expected: SourceRef,
    ) -> Result<EditorOutcome, String> {
        self.write(&expected, AuthoringAction::MaterialEdit, |inner| {
            inner.material_edit(source, expected.clone())
        })
    }
    fn undo(&mut self, expected: SourceRef) -> Result<EditorSnapshot, String> {
        self.write(&expected, AuthoringAction::Undo, |inner| {
            inner.undo(expected.clone())
        })
    }
    fn redo(&mut self, expected: SourceRef) -> Result<EditorSnapshot, String> {
        self.write(&expected, AuthoringAction::Redo, |inner| {
            inner.redo(expected.clone())
        })
    }
    fn rebuild_execution(&mut self, expected: SourceRef) -> Result<EditorOutcome, String> {
        self.write(&expected, AuthoringAction::Rebuild, |inner| {
            inner.rebuild_execution(expected.clone())
        })
    }
    fn save_checkpoint(
        &mut self,
        name: &str,
        expected_file: studio_store::checkpoint::ExpectedFile,
        expected: SourceRef,
    ) -> Result<studio_store::checkpoint::SaveReceipt, String> {
        self.write(&expected, AuthoringAction::SaveCheckpoint, |inner| {
            inner.save_checkpoint(name, expected_file, expected.clone())
        })
    }
}

#[cfg(test)]
mod tests;
