//! Native checkpoint service. The cooperative EditorHost must hold its current
//! trusted project/document authority guard across every call. This service
//! checks source/path consistency; it never derives authority from a file.
use qualia_core_db::poet_host::invoke::studio_store::{
    self, checkpoint as core, StoreStamp, StudioArtifact,
};
use qualia_core_db::render::compile_10d::CompiledAsset;
use std::path::{Path, PathBuf};




pub use core::{CheckpointLimits, ExpectedFile, SaveReceipt};

/// Supplied by the current trusted native owner, never deserialized from HMC.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrustedBinding {
    pub project_id: String,
    pub document_id: String,
    pub source_id: String,
    pub domain: String,
}

pub struct CheckpointService {
    configured_root: PathBuf,
    canonical_root: PathBuf,
    binding: TrustedBinding,
    limits: CheckpointLimits,
}

pub struct PreparedReopen {
    checkpoint: core::Checkpoint,
    binding: TrustedBinding,
    filename: String,
    digest: [u8; 32],
}
impl PreparedReopen {
    pub fn artifact(&self) -> &StudioArtifact {
        self.checkpoint.artifact()
    }
    pub fn file_digest(&self) -> [u8; 32] {
        self.digest
    }
    pub fn parent_digest(&self) -> Option<[u8; 32]> {
        self.checkpoint.parent_digest()
    }
}

pub struct SavedCheckpoint {
    pub receipt: SaveReceipt,
    pub source_revision: u32,
}

impl CheckpointService {
    /// Root comes from native host configuration, not an MCP/source path.
    /// Construction checks consistency only; the caller supplies authority.
    pub fn new(
        root: &Path,
        binding: TrustedBinding,
        limits: CheckpointLimits,
    ) -> Result<Self, String> {
        if [
            &binding.project_id,
            &binding.document_id,
            &binding.source_id,
            &binding.domain,
        ]
        .iter()
        .any(|id| id.is_empty() || id.as_bytes().contains(&0))
        {
            return Err(
                "trusted checkpoint association contains invalid identifiers/domain".into(),
            );
        }
        let canonical_root = root.canonicalize().map_err(|e| e.to_string())?;
        if !canonical_root.is_dir() {
            return Err("native checkpoint root must be an existing host-owned directory".into());
        }
        Ok(Self {
            configured_root: root.to_path_buf(),
            canonical_root,
            binding,
            limits,
        })
    }

    pub fn binding(&self) -> &TrustedBinding {
        &self.binding
    }

    fn path(&self, filename: &str) -> Result<PathBuf, String> {
        if filename.len() > 240
            || !filename.to_ascii_lowercase().ends_with(".hmc")
            || filename.starts_with('.')
            || filename.len() <= 4
            || !filename
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
        {
            return Err(
                "choose one plain .hmc filename within the host checkpoint directory".into(),
            );
        }
        let stem = filename
            .split('.')
            .next()
            .unwrap_or("")
            .to_ascii_uppercase();
        if matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
            || (stem.len() == 4
                && (stem.starts_with("COM") || stem.starts_with("LPT"))
                && matches!(stem.as_bytes()[3], b'1'..=b'9'))
        {
            return Err("checkpoint filename is an OS device name".into());
        }
        // Recheck root resolution on every operation. An incoming file never
        // selects a directory. The host owns this root and its ancestors; a
        // malicious outside filesystem mutation cannot be made portable-atomic.
        if self
            .configured_root
            .canonicalize()
            .map_err(|e| e.to_string())?
            != self.canonical_root
            || self
                .canonical_root
                .canonicalize()
                .map_err(|e| e.to_string())?
                != self.canonical_root
        {
            return Err("native checkpoint root resolution changed".into());
        }
        let path = self.canonical_root.join(filename);
        if std::fs::symlink_metadata(&path)
            .is_ok_and(|m| m.file_type().is_symlink() || !m.is_file())
        {
            return Err(
                "checkpoint target must be a regular file, never a link/junction/directory".into(),
            );
        }
        Ok(path)
    }

    fn matches(&self, artifact: &StudioArtifact) -> Result<(), String> {
        if artifact.id != self.binding.source_id || artifact.domain != self.binding.domain {
            return Err("checkpoint source identifiers/domain do not match the trusted native document association".into());
        }
        Ok(())
    }

    /// Caller holds the live SaveCheckpoint owner guard. ExpectedFile guards
    /// disk conflict only. It never grants write permission. Parent digest is
    /// the host's last verified durable checkpoint, distinct from StoreStamp.
    pub fn save(
        &self,
        expected: &StoreStamp,
        filename: &str,
        expected_file: ExpectedFile,
        parent_digest: Option<[u8; 32]>,
        assets: Vec<(String, CompiledAsset)>,
    ) -> Result<SavedCheckpoint, String> {
        let path = self.path(filename)?;
        if let ExpectedFile::Sha256(expected_digest) = expected_file {
            let existing = core::load(&path, self.limits)?;
            self.matches(existing.artifact())?;
            if existing.file_digest() != Some(expected_digest) {
                return Err("checkpoint replacement digest changed before source capture".into());
            }
        }
        let mut checkpoint = core::capture(&self.binding.source_id, expected, self.limits)?;
        self.matches(checkpoint.artifact())?;
        let source_revision = checkpoint.artifact().lamport_clock;
        if let Some(parent) = parent_digest {
            checkpoint = checkpoint.with_parent_digest(parent);
        }
        let bytes = core::encode(&checkpoint, assets, self.limits)?;
        if !studio_store::artifact_stamp_is_current(&self.binding.source_id, expected) {
            return Err("native source changed before checkpoint write; no write attempted".into());
        }
        // Validate root immediately before native IO as well as before work.
        if self.path(filename)? != path {
            return Err("checkpoint path changed".into());
        }
        let receipt = core::save(&path, &bytes, expected_file, self.limits)?;
        Ok(SavedCheckpoint {
            receipt,
            source_revision,
        })
    }

    /// Caller holds the live ReopenCheckpoint owner guard. Preparation reads
    /// and validates source/history/resources only. It cannot activate or replay.
    pub fn prepare_reopen(&self, filename: &str) -> Result<PreparedReopen, String> {
        let checkpoint = core::load(&self.path(filename)?, self.limits)?;
        self.matches(checkpoint.artifact())?;
        let digest = checkpoint
            .file_digest()
            .ok_or("native checkpoint digest absent")?;
        Ok(PreparedReopen {
            checkpoint,
            binding: self.binding.clone(),
            filename: filename.into(),
            digest,
        })
    }

    /// Caller rechecks its current ReopenCheckpoint owner decision for the
    /// prepared source/revision/digest and retains that guard through activation.
    /// Existing accepted source is never replaced. A fresh interpreter opens
    /// held; source-triggered grants and arbitrary effect replay are excluded.
    pub fn activate(
        &self,
        prepared: PreparedReopen,
    ) -> Result<(StudioArtifact, StoreStamp), String> {
        if prepared.binding != self.binding {
            return Err(
                "prepared checkpoint belongs to another trusted document association".into(),
            );
        }
        self.matches(prepared.checkpoint.artifact())?;
        // A stale disk selection cannot be silently activated after review.
        let current = core::load(&self.path(&prepared.filename)?, self.limits)?;
        if current.file_digest() != Some(prepared.digest) {
            return Err("checkpoint bytes changed since preparation; no activation".into());
        }
        core::activate(prepared.checkpoint, self.limits)
    }
}

#[cfg(test)]
#[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3/hmc/poet-candidate/checkpoints/tests.rs"]
mod tests;

