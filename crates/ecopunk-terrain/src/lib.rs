//! # ecopunk-terrain
//! Foundational geospatial terrain, 10D manifolds, .e64 containers,
//! dynamic hardware resource governor, and distributed display architectures.

pub mod display;
pub mod e64;
pub mod geospatial;
pub mod governor;
pub mod specimen;

pub use display::{CompanionDeviceRole, CompanionSyncMessage, FrameLockPacket, FrustumWedge};
pub use e64::{E64Header, E64HoloRecord, E64PointRecord};
pub use geospatial::*;
pub use governor::{
    GovernorConfig, GovernorMode, GpuTelemetrySample, HardwareMemoryClass, LlmWorkloadPhase,
    TerrainBudget, VramAndComputeGovernor,
};
pub use specimen::{GhostGumPhenology, GhostGumSpecimenLoupe};
