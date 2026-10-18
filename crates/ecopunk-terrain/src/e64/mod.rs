//! .e64 Earth Point-Tensor & Entity Container Module.
//!
//! Canonical zero-copy serialization, validation, and layout guarantees
//! are provided upstream by QualiaDB (`qualia_core_db::q42::m64_matter`).

pub use qualia_core_db::q42::m64_matter::{
    E64Header, E64HoloRecord, E64PointRecord, E64_MAGIC,
    M64Header, M64HoloRecord, M64PointRecord, M64_MAGIC, M64_VERSION,
};

pub const E64_VERSION: u32 = 1;
