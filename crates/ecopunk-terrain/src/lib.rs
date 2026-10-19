//! # ecopunk-terrain
//! Foundational geospatial terrain, 10D manifolds, .e64 containers,
//! dynamic hardware resource governor, and distributed display architectures.

pub mod display;
pub mod e64;
pub mod fauna_catalog;
pub mod flora_catalog;
pub mod geospatial;
pub mod governor;
pub mod scene_builder_10d;
pub mod specimen;

pub use display::{CompanionDeviceRole, CompanionSyncMessage, FrameLockPacket, FrustumWedge};
pub use e64::{E64Header, E64HoloRecord, E64PointRecord};
pub use fauna_catalog::compile_164_fauna_catalog;
pub use flora_catalog::compile_305_flora_catalog;
pub use geospatial::*;
pub use governor::{
    GovernorConfig, GovernorMode, GpuTelemetrySample, HardwareMemoryClass, LlmWorkloadPhase,
    TerrainBudget, VramAndComputeGovernor,
};
pub use scene_builder_10d::{
    build_closeup_specimen_10d, build_procedural_textures, build_sector_10d,
    render_sector_via_portal_gpu_png, CompiledSector10d, ProceduralTextureAsset,
};
pub use specimen::{GhostGumPhenology, GhostGumSpecimenLoupe};


