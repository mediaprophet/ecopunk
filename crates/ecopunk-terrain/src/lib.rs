//! # ecopunk-terrain
//! Foundational geospatial terrain, 10D manifolds, .e64 containers,
//! dynamic hardware resource governor, and distributed display architectures.

pub mod display;
pub mod e64;
pub mod fauna_animation_rig;
pub mod fauna_catalog;
pub mod flora_catalog;
pub mod geospatial;
pub mod governor;
pub mod holosonic_soundscape;
pub mod hud_studio;
pub mod physics_continuum;
pub mod scene_builder_10d;
pub mod scene_graph_10d;
pub mod specimen;
pub mod streaming_hydration;

pub use etalon;
pub use etalon_agent;
pub use etalon_nquin;
pub use etalon_render;
pub use etalon_tensor;
pub use etalon_wasm;
pub use etalon_wgsl_forge;

pub use display::{CompanionDeviceRole, CompanionSyncMessage, FrameLockPacket, FrustumWedge};
pub use e64::{E64Header, E64HoloRecord, E64PointRecord};
pub use fauna_animation_rig::{
    FaunaBiomechanicalArchetype, FaunaSkeletalRig10d, FAUNA_MAX_JOINTS,
};
pub use fauna_catalog::compile_164_fauna_catalog;
pub use flora_catalog::compile_305_flora_catalog;
pub use geospatial::*;
pub use governor::{
    GovernorConfig, GovernorMode, GpuTelemetrySample, HardwareMemoryClass, LlmWorkloadPhase,
    TerrainBudget, VramAndComputeGovernor,
};
pub use holosonic_soundscape::{emit_sonic_token, EcopunkHolosonicSoundscape};
pub use hud_studio::{EcopunkTelemetryHud, StackString};
pub use physics_continuum::{
    run_sector_physics_verification_step, ContinuumBridge3D, EcopunkSectorPhysicsWorld,
    FractureMaterialProps, RoverState, SdfCollider, SectorPhysicsReceipt, XpbdWorld3D,
};
pub use scene_builder_10d::{
    build_closeup_specimen_10d, build_procedural_textures, build_sector_10d,
    render_sector_via_portal_gpu_png, CompiledSector10d, ProceduralTextureAsset,
};
pub use scene_graph_10d::EcopunkSectorSceneGraph;
pub use specimen::{
    BotanicalDevelopmentalStage, DevelopmentalStage, GhostGumOrganSubmesh, GhostGumPhenology,
    GhostGumSpecimenLoupe,
};
pub use streaming_hydration::{EcopunkPackStreamer, LocalHostQ42Ext};


