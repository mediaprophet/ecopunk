//! Revision-bound compilation uses the existing core HMC codec and resource transaction.
use crate::graphical::checkpoints::{CheckpointService, ExpectedFile, SaveReceipt};
use crate::graphical::controller::EditorHost;
use crate::graphical::preview::SourceRef;
use qualia_core_db::poet_host::invoke::studio_store::checkpoint as core;

pub(crate) fn compile_checkpoint(
    inner: &dyn EditorHost,
    service: &CheckpointService,
    limits: core::CheckpointLimits,
    name: &str,
    key: &str,
    file: ExpectedFile,
    expected: SourceRef,
) -> Result<SaveReceipt, String> {
    let current = inner.snapshot()?;
    if current.source != expected
        || current.reconciliation_required
        || !current.publication_is_current()
    {
        return Err("Compilation requires current reconstructed accepted source".into());
    }
    expected.check_current()?;
    let assets = inner.compiled_assets(key, expected.clone())?;
    if matches!(file, ExpectedFile::Absent) {
        return service
            .save(&expected.stamp, name, file, None, assets)
            .map(|r| r.receipt);
    }
    // Build a detached candidate through the canonical codec. It supplies both
    // native .10d and Q42 companion resources; no application-owned wire format.
    let candidate = core::capture(&expected.artifact_id, &expected.stamp, limits)?;
    let bytes = core::encode(&candidate, assets, limits)?;
    let candidate = core::decode(&bytes, limits)?;
    let inventory = candidate.inventory();
    if inventory.is_empty() || inventory.len() > 64 {
        return Err("Compiled resource plan is empty or exceeds host entry bound".into());
    }
    let existing = match file {
        ExpectedFile::Absent => None,
        ExpectedFile::Sha256(digest) => {
            let selected = service.inventory(name)?;
            if selected.digest != digest {
                return Err("Inspected package changed before compile".into());
            }
            Some(selected)
        }
    };
    let mut edits = Vec::with_capacity(inventory.len());
    for resource in inventory {
        let data = candidate
            .resources()
            .iter()
            .find(|r| r.key == resource.key)
            .cloned()
            .ok_or("Compiled resource disappeared")?;
        if let Some(old) = existing
            .as_ref()
            .and_then(|p| p.resources.iter().find(|r| r.key == resource.key))
        {
            if old.kind != resource.kind {
                return Err("Compiled resource cannot change an existing entry kind".into());
            }
            edits.push(core::ResourceEdit::ReplaceBytes {
                key: resource.key,
                expected: old.digest,
                bytes: data.bytes,
            });
        } else {
            edits.push(core::ResourceEdit::Add(data));
        }
    }
    expected.check_current()?;
    let parent = match file {
        ExpectedFile::Absent => None,
        ExpectedFile::Sha256(d) => Some(d),
    };
    service
        .save_with_resource_edits(&expected.stamp, name, file, parent, vec![], edits)
        .map(|r| r.receipt)
}
