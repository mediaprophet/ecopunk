//! Asymmetric Companion Device Synchronization Protocol
//! Coordinates primary displays (Smart TV / Desktop) with companion tablets, phones, and remote social peers.

use serde::{Deserialize, Serialize};

/// Functional role played by a device in the user's multi-screen ecosystem.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanionDeviceRole {
    /// 100% immersive, zero-HUD natural environmental viewport (Smart TV / Main Monitor)
    PrimaryVista,
    /// Tactical GIS map, subterranean strata core-slicer, and 3D Specimen Loupe (Tablet)
    TacticalTablet,
    /// Tactile remote: vehicle throttle, winch controls, expedition journal (Mobile Phone)
    MobileRemote,
    /// Networked participant in a different physical location (Social Peer)
    SocialPeer,
}

/// Messages exchanged across local WebRTC DataChannels / WebSockets between companion devices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompanionSyncMessage {
    /// Primary vista informs tablet that user approached an ecological entity
    ProximityWakeSpecimen {
        taxon_id_hash: u64,
        entity_name: String,
        distance_meters: f32,
    },
    /// Tablet informs simulation of user interaction in specimen loupe (e.g. growth scrubber)
    SpecimenGrowthScrub {
        taxon_id_hash: u64,
        growth_scale_factor: f32,
        season_month: f32,
    },
    /// Mobile phone sends tactile vehicle steering or winch inputs
    VehicleControlInput {
        steering_delta: f32,
        throttle_normalized: f32,
        winch_speed: f32,
    },
    /// User excavated or modified terrain (committed as Merkle-CRDT mutation)
    TerrainMutation {
        chunk_index: u64,
        local_pos: [f32; 3],
        volume_delta_m3: f32,
        material_type: u8,
        lamport_clock: u64,
    },
    /// Telemetry status update from host governor to companion dashboard
    GovernorTelemetryUpdate {
        vram_pressure_pct: u8,
        llm_decode_toks: f32,
        gpu_temperature_c: u8,
    },
}
