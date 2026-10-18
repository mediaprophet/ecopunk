//! Distributed Display Architectures & Companion Protocols
//! Powers full-dome clusters (600x 4K) and multi-screen companion ecosystems.

pub mod companion;
pub mod frame_lock;
pub mod frustum;

pub use companion::{CompanionDeviceRole, CompanionSyncMessage};
pub use frame_lock::{FrameLockPacket, FRAME_LOCK_MAGIC, FRAME_LOCK_VERSION};
pub use frustum::FrustumWedge;
