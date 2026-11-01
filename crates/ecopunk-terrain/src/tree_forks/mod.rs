//! Complete, deterministic adoption over the existing authored botanical source.
//! Geometry is delegated to the Core-backed collar owner; no hard-union fallback.
mod adoption;
mod cuts;
mod pieces;
mod plan;
mod topology;
pub(super) use adoption::apply;
pub(super) use pieces::ChainSpan;
#[cfg(test)]
mod tests;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AllForkSpec {
    pub schema: String,
    pub cut_fraction: f64,
    pub guard_fraction: f64,
    pub transition_fraction: f64,
    pub collar: crate::smooth_collars::CollarControls,
    /// Only these explicitly named connected groups may blend across multiple
    /// junctions. Every unlisted required junction remains an individual job.
    #[serde(default)]
    pub coupled_junctions: Vec<Vec<String>>,
}

#[derive(Clone, Copy, Debug)]
pub struct AllForkLimits {
    /// Aggregate envelope, not a fresh budget per junction.
    pub bytes: usize,
    pub work: usize,
    pub output_vertices: usize,
    pub output_indices: usize,
    pub collar: crate::smooth_collars::CollarLimits,
}
impl Default for AllForkLimits {
    fn default() -> Self {
        Self {
            bytes: 1024 * 1024 * 1024,
            work: 2_000_000_000,
            output_vertices: 400_000,
            output_indices: 1_800_000,
            collar: crate::smooth_collars::CollarLimits::default(),
        }
    }
}

pub(super) fn validate_spec(spec: &AllForkSpec) -> Result<(), String> {
    if spec.schema != "ecopunk.all-fork-collars/v1"
        || !spec.cut_fraction.is_finite()
        || spec.cut_fraction <= 0.0
        || spec.cut_fraction >= 0.5
        || !spec.guard_fraction.is_finite()
        || spec.guard_fraction <= 0.0
        || spec.guard_fraction >= 1.0
        || !spec.transition_fraction.is_finite()
        || spec.transition_fraction <= 0.0
        || spec.transition_fraction > 1.0
        || spec.coupled_junctions.len() > 256
        || spec
            .coupled_junctions
            .iter()
            .any(|g| g.is_empty() || g.len() > 256)
    {
        return Err("Invalid complete all-fork source profile".into());
    }
    crate::smooth_collars::validate_controls(spec.collar)
}

/// Reuses the existing canonical .10d readback owner for the exact contiguous
/// changed streams. This is source/projection admission, not visual acceptance.
pub(crate) fn projection_witness(
    wood: &crate::scene_builder_10d::SubmeshBuilder,
    foliage: &crate::scene_builder_10d::SubmeshBuilder,
    metadata: &serde_json::Value,
) -> Result<crate::tree_junction::ProjectionWitness, String> {
    crate::tree_junction::projection_witness(
        wood,
        foliage,
        &serde_json::json!({"chain_union":metadata["all_forks"]["readback"]}),
    )
}
pub(crate) use crate::tree_junction::qualify_compiled_projection;
