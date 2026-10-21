//! # ecopunk-terrain
//! Foundational geospatial terrain, 10D manifolds, .e64 containers,
//! dynamic hardware resource governor, and distributed display architectures.

pub mod botanical_anatomy;
pub mod botanical_ontology_q42;
pub mod display;
pub mod e64;
pub mod exploration_controller;
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
pub mod world_continuum;
pub mod zoological_anatomy;

pub use etalon;
pub use etalon_agent;
pub use etalon_nquin;
pub use etalon_render;
pub use etalon_tensor;
pub use etalon_wasm;
pub use etalon_wgsl_forge;

pub use botanical_anatomy::{
    build_scale1_whole_organism_10d, build_scale2_stem_bark_wedge_10d,
    build_scale3_floral_and_gumnut_dissection_10d, build_scale4_leaf_venation_and_histology_10d,
    BotanicalEducationScale, CompiledBotanicalOrgan10d, HistologicalTissueLayer,
    TissueSubmeshSlice,
};
#[cfg(not(target_arch = "wasm32"))]
pub use botanical_anatomy::write_corymbia_educational_assets_to_disk;
pub use botanical_ontology_q42::{
    query_closest_anatomical_pin_zero_heap, AnatomicalCalloutPin, AnatomicalOntologyStandard,
    CANONICAL_ANATOMICAL_PINS,
};
#[cfg(not(target_arch = "wasm32"))]
pub use botanical_ontology_q42::emit_anatomical_ontology_nquins_to_writer;
pub use display::{CompanionDeviceRole, CompanionSyncMessage, FrameLockPacket, FrustumWedge};
pub use e64::{E64Header, E64HoloRecord, E64PointRecord};
pub use exploration_controller::{
    ContinuumExplorationSession, ExplorationCameraMode, ExplorationMode,
    ExplorationTelemetrySnapshot, ExplorationTickInput, LoupeDissectionState,
};
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
    build_closeup_specimen_10d, build_procedural_textures, build_sector_10d, CompiledSector10d,
    ProceduralTextureAsset,
};
#[cfg(not(target_arch = "wasm32"))]
pub use scene_builder_10d::render_sector_via_portal_gpu_png;
pub use scene_graph_10d::EcopunkSectorSceneGraph;
pub use specimen::{
    BotanicalDevelopmentalStage, DevelopmentalStage, GhostGumOrganSubmesh, GhostGumPhenology,
    GhostGumSpecimenLoupe,
};
pub use streaming_hydration::{EcopunkPackStreamer, LocalHostQ42Ext};
pub use world_continuum::{
    RegionalCorridorWorld, SoilProfile, StreamingChunkSlot, StudySiteDescriptor, StudySiteId,
    SurfaceMaterialKind, TerrainLod,
};
pub use zoological_anatomy::{
    build_scale5_vertebrate_anatomy_10d, evaluate_anatomical_layer_peel_zero_heap,
    CompiledVertebrateAnatomy10d, VertebrateMyologyGroup, VertebrateOsteologyRegion,
    ZoologicalAnatomicalLayer,
};


