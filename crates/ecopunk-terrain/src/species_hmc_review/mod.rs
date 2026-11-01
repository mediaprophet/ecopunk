//! Read-only review projection over a SOURCE-OWNER-ADMITTED HMC candidate.
//! This is not an arbitrary uploaded-CBOR gate, new asset store, graph adoption,
//! inferred biological variant, or complete package/authority acceptance.
mod coverage;
mod inspect;
#[cfg(test)]
mod tests;
pub use inspect::inspect_admitted;
use qualia_core_db::{
    bundle::{BiologicalPhenotype, LodTier, StructuralState},
    entity_view::entity_ontologies::LifecycleStage,
};
use serde::Serialize;
#[derive(Serialize)]
pub struct PackageReviewIndex {
    pub schema: &'static str,
    pub package_sha256: String,
    pub package_bytes: usize,
    pub subject: u8,
    pub source_revision: String,
    pub source_sha256: String,
    pub catalogue: CatalogueReview,
    pub resources: Vec<ResourceReview>,
    pub models: Vec<ModelReview>,
    pub family: Option<FamilyReview>,
    pub coverage: Vec<CoverageReview>,
    pub scope: &'static str,
}
#[derive(Serialize)]
pub struct CatalogueReview {
    pub id: String,
    pub version: String,
    pub sha256: String,
}
#[derive(Serialize)]
pub struct ResourceReview {
    pub key: String,
    pub kind: String,
    pub bytes: usize,
    pub sha256: String,
    pub metadata_bytes: usize,
    pub metadata_sha256: Option<String>,
    pub interpretation: &'static str,
}
#[derive(Serialize)]
pub struct ModelReview {
    pub key: String,
    pub sha256: String,
    pub vertices: u32,
    pub triangles: u32,
    pub section_types: Vec<String>,
    pub clips: Vec<ClipReview>,
}
#[derive(Serialize)]
pub struct ClipReview {
    pub id: String,
    pub source_name: Option<String>,
    pub duration_s: f64,
    pub looping: bool,
    pub tracks: u32,
    pub keys: u32,
    pub scope: &'static str,
}
#[derive(Serialize)]
pub struct FamilyReview {
    pub family_id: String,
    pub common_name: String,
    pub classification: String,
    pub declared_default_variant: String,
    pub variants: Vec<VariantReview>,
    pub status: &'static str,
}
#[derive(Serialize)]
pub struct VariantReview {
    pub variant_id: String,
    pub phenotype: BiologicalPhenotype,
    pub stage: LifecycleStage,
    pub lod: LodTier,
    pub state: StructuralState,
    pub asset_key: String,
    pub entity_ontology_key: Option<String>,
    pub procedural_seed: Option<String>,
    pub status: &'static str,
}
#[derive(Serialize)]
pub struct CoverageReview {
    pub dimension: &'static str,
    pub value: &'static str,
    pub catalogue_ids: Vec<&'static str>,
    pub status: &'static str,
    pub applicability: &'static str,
    pub note: &'static str,
}
impl PackageReviewIndex {
    /// Exact declared member only. Core's intentional model fallback policy is not
    /// used as proof that a missing sex/stage/reproductive variant is represented.
    pub fn select_variant_exact(
        &self,
        expected_package_sha256: &str,
        variant_id: &str,
    ) -> Result<&VariantReview, String> {
        self.guard(expected_package_sha256)?;
        self.family.as_ref().and_then(|f|f.variants.iter().find(|v|v.variant_id==variant_id)).ok_or_else(||"Requested variant is not explicitly declared in this package; no fallback substitution".into())
    }
    pub fn select_clip_exact(
        &self,
        expected_package_sha256: &str,
        model_key: &str,
        clip_id: &str,
    ) -> Result<&ClipReview, String> {
        self.guard(expected_package_sha256)?;
        self.models
            .iter()
            .find(|m| m.key == model_key)
            .and_then(|m| m.clips.iter().find(|c| c.id == clip_id))
            .ok_or_else(|| "Requested model/clip is absent from this exact package".into())
    }
    fn guard(&self, digest: &str) -> Result<(), String> {
        if digest != self.package_sha256 {
            Err("Package selection is stale or belongs to different bytes".into())
        } else {
            Ok(())
        }
    }
}
