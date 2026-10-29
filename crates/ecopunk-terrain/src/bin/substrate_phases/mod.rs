//! Substrate compilation phases for EcoPunk.
//!
//! Decomposed into single-responsibility submodules:
//! - `dem_admission`: Fail-closed DEM elevation admission & .m64 / .10d heightfield synthesis.
//! - `turtle_ingest`: Turtle ontology parsing to NQuins & .q42 volume compilation.
//! - `wgsl_shader_prelude`: WGSL prelude, structs, uniforms, PBR/SGGX, vertex shaders.
//! - `wgsl_shader_fragment`: WGSL fragment shader, observer modes, line overlays.
//! - `wgsl_pipeline`: Naga shader certification & WGSL Forge manifest generation.
//! - `vibe_phase`: VibeScript certification, interpreter execution, 305-Flora/164-Fauna catalogs.
//! - `scale_conformance`: QUDT metric scale invariants & compilation receipt.
//! - `hmc_packing`: .10d sector assembly & .hmc bundle packaging.
//! - `hmc_verification`: Cryptographic digest verification, streaming tests & PortalGpu offscreen rendering.

pub mod dem_admission;
pub mod hmc_packing;
pub mod hmc_verification;
pub mod scale_conformance;
pub mod turtle_ingest;
pub mod vibe_phase;
pub mod wgsl_pipeline;
pub mod wgsl_shader_fragment;
pub mod wgsl_shader_prelude;
