//! Distributed Display Architectures & Companion Protocols.
//! Powers full-dome clusters (600x 4K) and multi-screen companion ecosystems.

pub use etalon_render::frame_lock::{
    CompanionDeviceRole, CompanionSyncMessage, ConicalDomeWedge, FrameLockPacket, FrustumWedge,
    FRAME_LOCK_MAGIC, FRAME_LOCK_VERSION,
};
