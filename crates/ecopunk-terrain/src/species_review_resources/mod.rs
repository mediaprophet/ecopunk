//! Resource-preserving limited review packages using canonical HMC/Studio data.
//! Structural integrity does not grant rights, adopt graph assertions, execute
//! source, or establish a complete species. Unknown resource kinds are refused.
mod session;
mod corpus;
pub use corpus::CorpusOptions;
pub(crate) use corpus::{copy_retained_corpus_inputs, generate_corpus_resources};
pub mod query;
pub mod query_estimate;
pub mod query_assignment;
mod validation;
use crate::species_authored_review::AuthoredReviewAsset;
use qualia_core_db::bundle::{BundleReader, BundleWriter, BUNDLE_ENTRY_ALIGN};
pub use qualia_core_db::poet_host::invoke::studio_store::checkpoint::{
    ResourceData, ResourceSummary,
};
pub use session::SpeciesReviewPackageSession;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug)]
pub struct ReviewResourceLimits {
    pub entries: usize,
    pub resource_bytes: usize,
    pub total_bytes: usize,
    pub metadata_bytes: usize,
    pub package_bytes: usize,
    pub cold_work_bytes: usize,
    pub q42_blocks: usize,
    pub q42_quins: usize,
    pub m64_chunks: usize,
    pub m64_records: u64,
}
impl Default for ReviewResourceLimits {
    fn default() -> Self {
        Self {
            entries: 32,
            resource_bytes: 16 * 1024 * 1024,
            total_bytes: 32 * 1024 * 1024,
            metadata_bytes: 1024,
            package_bytes: 128 * 1024 * 1024,
            cold_work_bytes: 256 * 1024 * 1024,
            q42_blocks: 256,
            q42_quins: 200_000,
            m64_chunks: 64,
            m64_records: 262_144,
        }
    }
}
#[derive(Default)]
pub struct ReviewResources {
    entries: Vec<ResourceData>,
}
impl ReviewResources {
    /// Canonical format/record integrity only. Opaque CBOR metadata and Q42
    /// namespace/rights assertions remain byte-exact caller claims, unverified.
    pub fn admit(entries: Vec<ResourceData>, limits: ReviewResourceLimits) -> Result<Self, String> {
        validate_inventory(&entries, limits)?;
        for resource in &entries {
            validation::validate(resource, limits)?;
        }
        Ok(Self { entries })
    }
    pub fn entries(&self) -> &[ResourceData] {
        &self.entries
    }
    pub(crate) fn into_entries(self) -> Vec<ResourceData> { self.entries }
    pub fn inventory(&self) -> Vec<ResourceSummary> {
        self.entries.iter().map(ResourceData::summary).collect()
    }
    pub(crate) fn from_bundle(
        reader: &BundleReader<'_>,
        limits: ReviewResourceLimits,
    ) -> Result<Self, String> {
        // Check borrowed lengths and metadata before making resource copies.
        let mut total = 0usize;
        let mut count = 0usize;
        for entry in reader
            .entries()
            .iter()
            .filter(|e| e.key.starts_with("resources/"))
        {
            count = count.checked_add(1).ok_or("Resource count overflow")?;
            let len = usize::try_from(entry.length).map_err(|_| "Resource byte size overflow")?;
            total = total
                .checked_add(len)
                .and_then(|n| n.checked_add(entry.meta.as_ref().map_or(0, Vec::len)))
                .ok_or("Resource size overflow")?;
            if count > limits.entries
                || len > limits.resource_bytes
                || total > limits.total_bytes
                || entry.meta.as_ref().map_or(0, Vec::len) > limits.metadata_bytes
            {
                return Err("Review retained resource budget exceeded".into());
            }
        }
        if total
            .checked_mul(2)
            .is_none_or(|n| n > limits.cold_work_bytes)
        {
            return Err("Review resource copy exceeds cold-work budget".into());
        }
        let entries = reader
            .entries()
            .iter()
            .filter(|e| e.key.starts_with("resources/"))
            .map(|e| ResourceData {
                key: e.key.clone(),
                kind: e.kind.clone(),
                bytes: reader.get(&e.key).unwrap().to_vec(),
                meta: e.meta.clone(),
            })
            .collect();
        Self::admit(entries, limits)
    }
    pub(crate) fn retain_in(
        &self,
        mut asset: AuthoredReviewAsset,
        limits: ReviewResourceLimits,
    ) -> Result<AuthoredReviewAsset, String> {
        validate_inventory(&self.entries, limits)?;
        if asset.bundle.len() > limits.package_bytes
            || asset.bundle.len() > 128 * 1024 * 1024
            || asset
                .bundle
                .len()
                .checked_mul(4)
                .is_none_or(|n| n > limits.cold_work_bytes)
        {
            return Err("Generated base review package exceeds file/cold-work budget".into());
        }
        if self.entries.is_empty() {
            return Ok(asset);
        }
        let reader = BundleReader::parse(&asset.bundle)
            .map_err(|e| format!("Generated review bundle: {e:?}"))?;
        let mut projected = asset.bundle.len();
        for resource in &self.entries {
            projected = projected
                .checked_add(resource.bytes.len())
                .and_then(|n| n.checked_add(resource.meta.as_ref().map_or(0, Vec::len)))
                .and_then(|n| {
                    n.checked_add(
                        resource.key.len() + resource.kind.len() + BUNDLE_ENTRY_ALIGN + 256,
                    )
                })
                .ok_or("Review package size overflow")?;
        }
        if projected > limits.package_bytes
            || projected > 128 * 1024 * 1024
            || projected
                .checked_mul(4)
                .is_none_or(|n| n > limits.cold_work_bytes)
        {
            return Err("Review package retention exceeds file/cold-work budget".into());
        }
        let mut writer = BundleWriter::new();
        for e in reader.entries() {
            writer
                .add_file(
                    &e.key,
                    &e.kind,
                    reader.get(&e.key).unwrap().to_vec(),
                    e.meta.clone(),
                )
                .map_err(|e| format!("Retained base resource: {e:?}"))?;
        }
        for r in &self.entries {
            writer
                .add_file(&r.key, &r.kind, r.bytes.clone(), r.meta.clone())
                .map_err(|e| format!("Retained native resource: {e:?}"))?;
        }
        let bundle = writer
            .build()
            .map_err(|e| format!("Retained review bundle: {e:?}"))?;
        if bundle.len() > limits.package_bytes {
            return Err("Encoded review package exceeds file budget".into());
        }
        // Preserve the same bounded reopen profile even when trusted callers
        // request larger resource quotas. Refuse before source promotion.
        let count = u32::from_le_bytes(bundle[8..12].try_into().unwrap());
        let index = u64::from_le_bytes(bundle[20..28].try_into().unwrap());
        if count > 165 || index > 256 * 1024 {
            return Err("Encoded resources exceed bounded review index profile".into());
        }
        asset.bundle = bundle;
        Ok(asset)
    }
}
pub(crate) fn supported_key(key: &str, kind: &str) -> bool {
    key.strip_prefix("resources/").is_some_and(|tail| {
        !tail.is_empty()
            && tail.len() <= 240
            && tail.split('/').all(|part| {
                !part.is_empty()
                    && part != "."
                    && part != ".."
                    && part
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
            })
            && match kind {
                "q42" => tail.ends_with(".q42"),
                "m64" => tail.ends_with(".m64"),
                _ => false,
            }
    })
}
fn validate_inventory(
    entries: &[ResourceData],
    limits: ReviewResourceLimits,
) -> Result<(), String> {
    if entries.len() > limits.entries {
        return Err("Review resource entry budget exceeded".into());
    }
    let mut keys = BTreeSet::new();
    let mut total = 0usize;
    for r in entries {
        if !supported_key(&r.key, &r.kind) || !keys.insert(r.key.as_str()) {
            return Err("Unsupported or duplicate retained resource key/kind".into());
        }
        let meta = r.meta.as_ref().map_or(0, Vec::len);
        total = total
            .checked_add(r.bytes.len())
            .and_then(|n| n.checked_add(meta))
            .ok_or("Resource byte size overflow")?;
        if r.bytes.len() > limits.resource_bytes
            || meta > limits.metadata_bytes
            || total > limits.total_bytes
        {
            return Err("Review resource byte/metadata budget exceeded".into());
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests;
