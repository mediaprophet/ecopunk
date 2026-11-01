//! Optional native read-only Full verification over a host-staged exact file.
use super::*;
use qualia_core_db::q42_volume::{CheckStatus, Q42VerifyReceipt, Q42Volume, VerifyLevel};
use sha2::{Digest, Sha256};
use std::path::Path;

/// Does not write, publish or trust an arbitrary supplied graph. The host must
/// first stage this generated resource under its existing guarded I/O policy.
/// Canonical Core open + Full verification is reused, after file metadata bounds
/// and exact expected bytes are checked; a changed file is rejected. The receipt
/// describes a read snapshot, not future path identity or scientific adoption.
pub fn verify_generated_at(
    resource: &ResourceData,
    path: &Path,
    limits: KnowledgeLimits,
    cancel: &AtomicBool,
) -> Result<Q42VerifyReceipt, String> {
    if !super::binding::supported_key(&resource.key)
        || resource.kind != "q42"
        || resource.bytes.len() > limits.output_bytes
        || resource
            .bytes
            .len()
            .checked_mul(4)
            .is_none_or(|n| n > limits.cold_bytes)
        || cancel.load(Ordering::Acquire)
    {
        return Err("Generated Q42 Full verification input/budget/cancellation refused".into());
    }
    ReviewResources::admit(
        vec![resource.clone()],
        ReviewResourceLimits {
            q42_quins: limits.statements,
            q42_blocks: limits.statements.div_ceil(512),
            resource_bytes: limits.output_bytes,
            total_bytes: limits.output_bytes,
            cold_work_bytes: limits.cold_bytes,
            ..Default::default()
        },
    )?;
    let metadata = std::fs::metadata(path).map_err(|e| format!("Staged Q42 metadata: {e}"))?;
    if !metadata.is_file() || metadata.len() != resource.bytes.len() as u64 {
        return Err("Staged Q42 byte length differs".into());
    }
    let volume = Q42Volume::open(path).map_err(|e| format!("Canonical Q42 open: {e}"))?;
    let expected = Sha256::digest(&resource.bytes);
    if volume.as_bytes() != resource.bytes.as_slice()
        || Sha256::digest(volume.as_bytes()) != expected
    {
        return Err("Staged Q42 differs from exact generated resource".into());
    }
    if cancel.load(Ordering::Acquire) {
        return Err("Q42 Full verification cancelled before scan".into());
    }
    let receipt = Q42VerifyReceipt::from_volume(path, &volume, VerifyLevel::Full);
    if cancel.load(Ordering::Acquire) {
        return Err("Q42 Full verification cancelled; no receipt adopted".into());
    }
    if receipt.overall != CheckStatus::Pass {
        return Err("Canonical native Q42 Full verification did not pass".into());
    }
    // The existing native verifier is synchronous and not interruptible within
    // an individual scan; this method checks cancellation at its boundaries.
    Ok(receipt)
}
