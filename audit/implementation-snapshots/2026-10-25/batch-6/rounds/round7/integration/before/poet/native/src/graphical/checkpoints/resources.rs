//! Native working-copy persistence; no resource type is silently discarded.
use super::*;
use qualia_core_db::render::compile_10d::CompiledAsset;

/// Selected by trusted host configuration, never inferred from incoming HMC.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PackageWritePolicy {
    WorkingCopy,
    ImmutableRelease { filename: String, digest: [u8; 32] },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageInventory {
    pub digest: [u8; 32],
    pub resources: Vec<ResourceSummary>,
}

impl CheckpointService {
    fn check_writable_target(
        &self,
        filename: &str,
        expected_file: ExpectedFile,
    ) -> Result<(), String> {
        if let PackageWritePolicy::ImmutableRelease {
            filename: origin,
            digest,
        } = &self.policy
        {
            if origin.eq_ignore_ascii_case(filename)
                || matches!(expected_file, ExpectedFile::Sha256(expected) if expected == *digest)
            {
                return Err("immutable publication/release cannot be overwritten; choose a new working-copy filename".into());
            }
        }
        Ok(())
    }

    /// Current native owner calls this before reading a selected import file.
    /// Final save repeats every guard; this does not reserve or mutate a file.
    pub fn preflight_resource_edit(
        &self,
        filename: &str,
        expected_file: ExpectedFile,
    ) -> Result<(), String> {
        let ExpectedFile::Sha256(expected) = expected_file else {
            return Err("resource edits require the expected existing package digest".into());
        };
        self.path(filename)?;
        self.check_writable_target(filename, expected_file)?;
        let inventory = self.inventory(filename)?;
        if inventory.digest != expected {
            return Err("resource edit package digest changed; no import read".into());
        }
        if self
            .retained
            .lock()
            .map_err(|_| "checkpoint resource carrier poisoned")?
            .as_ref()
            .is_some_and(|base| base.file_digest() != Some(expected))
        {
            return Err("resource edit differs from retained package base; no import read".into());
        }
        Ok(())
    }

    pub(super) fn check_release_input(
        &self,
        filename: &str,
        digest: [u8; 32],
    ) -> Result<(), String> {
        if let PackageWritePolicy::ImmutableRelease {
            filename: origin,
            digest: pinned,
        } = &self.policy
        {
            if origin.eq_ignore_ascii_case(filename) && *pinned != digest {
                return Err(
                    "immutable release bytes differ from the native host's pinned digest".into(),
                );
            }
        }
        Ok(())
    }

    /// Caller holds a current ReadSource owner guard for the bound document.
    /// Listing neither activates source nor decodes arbitrary resources in UI.
    pub fn inventory(&self, filename: &str) -> Result<PackageInventory, String> {
        let checkpoint = core::load(&self.path(filename)?, self.limits)?;
        self.matches(checkpoint.artifact())?;
        let digest = checkpoint.file_digest().ok_or("checkpoint digest absent")?;
        self.check_release_input(filename, digest)?;
        Ok(PackageInventory {
            digest,
            resources: checkpoint.inventory(),
        })
    }

    pub fn save(
        &self,
        expected: &StoreStamp,
        filename: &str,
        expected_file: ExpectedFile,
        parent_digest: Option<[u8; 32]>,
        assets: Vec<(String, CompiledAsset)>,
    ) -> Result<SavedCheckpoint, String> {
        self.save_with_resource_edits(
            expected,
            filename,
            expected_file,
            parent_digest,
            assets,
            vec![],
        )
    }

    /// Current native owner must retain its SaveCheckpoint/EditSource guard
    /// through this entire call. Source and resources install as one HMC rename;
    /// errors before install leave disk and the retained carrier unchanged.
    /// Core IO documents the portable external-writer and power-loss limits.
    pub fn save_with_resource_edits(
        &self,
        expected: &StoreStamp,
        filename: &str,
        expected_file: ExpectedFile,
        parent_digest: Option<[u8; 32]>,
        assets: Vec<(String, CompiledAsset)>,
        edits: Vec<ResourceEdit>,
    ) -> Result<SavedCheckpoint, String> {
        let path = self.path(filename)?;
        if !edits.is_empty() && !matches!(expected_file, ExpectedFile::Sha256(_)) {
            return Err("resource edits require the expected existing package digest".into());
        }
        self.check_writable_target(filename, expected_file)?;
        let mut retained = self
            .retained
            .lock()
            .map_err(|_| "checkpoint resource carrier poisoned")?;
        let disk_base = match expected_file {
            ExpectedFile::Absent => None,
            ExpectedFile::Sha256(digest) => {
                let existing = core::load(&path, self.limits)?;
                self.matches(existing.artifact())?;
                if existing.file_digest() != Some(digest) {
                    return Err(
                        "checkpoint replacement digest changed before source capture".into(),
                    );
                }
                if retained
                    .as_ref()
                    .is_some_and(|base| base.file_digest() != Some(digest))
                {
                    return Err("selected package differs from retained resource base; reopen the intended base explicitly".into());
                }
                Some(existing)
            }
        };
        let release_base = if disk_base.is_none() && retained.is_none() {
            match &self.policy {
                PackageWritePolicy::ImmutableRelease {
                    filename: origin,
                    digest,
                } => {
                    let input = core::load(&self.path(origin)?, self.limits)?;
                    self.matches(input.artifact())?;
                    if input.file_digest() != Some(*digest) {
                        return Err(
                            "immutable release base differs from native pinned digest".into()
                        );
                    }
                    Some(input)
                }
                PackageWritePolicy::WorkingCopy => None,
            }
        } else {
            None
        };
        let base = disk_base
            .as_ref()
            .or(retained.as_ref())
            .or(release_base.as_ref());
        let mut checkpoint = core::capture(&self.binding.source_id, expected, self.limits)?;
        self.matches(checkpoint.artifact())?;
        let source_revision = checkpoint.artifact().lamport_clock;
        if let Some(base) = base {
            let origin = base.file_digest().ok_or("resource base digest absent")?;
            if parent_digest.is_some_and(|parent| parent != origin) {
                return Err("checkpoint parent differs from exact retained resource base".into());
            }
            checkpoint = checkpoint
                .with_resources_from(base, self.limits)?
                .with_parent_digest(origin);
        } else if parent_digest.is_some() {
            return Err("checkpoint lineage requires a verified retained package base".into());
        }
        let checkpoint = checkpoint.apply_resource_edits(edits, self.limits)?;
        // Core writer rejects collisions with retained entries. Replacement is
        // available only through an explicit guarded ResourceEdit, never by an
        // incidental compiler output sharing an old entry's key.
        let bytes = core::encode(&checkpoint, assets, self.limits)?;
        let next_carrier = core::decode(&bytes, self.limits)?;
        if !studio_store::artifact_stamp_is_current(&self.binding.source_id, expected) {
            return Err("native source changed before checkpoint write; no write attempted".into());
        }
        if self.path(filename)? != path {
            return Err("checkpoint path changed".into());
        }
        let receipt = core::save(&path, &bytes, expected_file, self.limits)?;
        *retained = Some(next_carrier);
        Ok(SavedCheckpoint {
            receipt,
            source_revision,
        })
    }
}
