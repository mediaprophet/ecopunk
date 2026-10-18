//! Deterministic 64-Byte UDP Multicast Frame-Lock Protocol
//! Powers industrial multi-projector full-domes (e.g. 600x 4K displays) with microsecond
//! synchronization and zero network geometry transmission.

use bytemuck::{Pod, Zeroable};

/// Frame-lock magic ASCII identifier: `FMLK`
pub const FRAME_LOCK_MAGIC: [u8; 4] = *b"FMLK";

/// Protocol version 1
pub const FRAME_LOCK_VERSION: u16 = 1;

/// Fixed 64-Byte Aligned Frame-Lock Packet broadcast over UDP multicast at 60/120 Hz.
#[repr(C, align(64))]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct FrameLockPacket {
    /// 0x00..0x04: ASCII Magic `FMLK`
    pub magic: [u8; 4],
    /// 0x04..0x06: Protocol version (1)
    pub protocol_version: u16,
    /// 0x06..0x08: Cluster Identifier
    pub cluster_id: u16,
    /// 0x08..0x10: Monotonic Lamport simulation clock tick
    pub lamport_clock: u64,
    /// 0x10..0x1C: Observer camera focal position (x, y, z) in meters relative to cluster datum (12 bytes)
    pub observer_pos: [f32; 3],
    /// 0x1C..0x20: Solar radiant irradiance & zenith factor (α)
    pub solar_irradiance_alpha: f32,
    /// 0x20..0x30: Camera orientation quaternion (qx, qy, qz, qw) (16 bytes)
    pub observer_rot: [f32; 4],
    /// 0x30..0x34: Active simulation quantum branch q (0 = ground truth)
    pub quantum_branch_q: u32,
    /// 0x34..0x38: Environmental PRNG seed (wind, rain turbulence, wildlife)
    pub environmental_seed: u32,
    /// 0x38..0x3C: Reserved for network flags
    pub network_flags: u32,
    /// 0x3C..0x40: CRC-32C checksum of bytes 0..60
    pub packet_crc32c: u32,
}

impl Default for FrameLockPacket {
    fn default() -> Self {
        Self {
            magic: FRAME_LOCK_MAGIC,
            protocol_version: FRAME_LOCK_VERSION,
            cluster_id: 1,
            lamport_clock: 0,
            observer_pos: [0.0, 0.0, 0.0],
            solar_irradiance_alpha: 1.0,
            observer_rot: [0.0, 0.0, 0.0, 1.0], // Identity quaternion
            quantum_branch_q: 0,
            environmental_seed: 42,
            network_flags: 0,
            packet_crc32c: 0,
        }
    }
}

impl FrameLockPacket {
    /// Updates the CRC-32C checksum over the 60-byte payload.
    pub fn update_crc(&mut self) {
        self.packet_crc32c = 0;
        let bytes: &[u8] = bytemuck::bytes_of(self);
        // Exclude CRC field at offset 60..64
        self.packet_crc32c = crc32c::crc32c(&bytes[..60]);
    }

    /// Validates the frame packet magic, version, and CRC-32C checksum.
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.magic != FRAME_LOCK_MAGIC {
            return Err("Invalid frame-lock magic");
        }
        if self.protocol_version != FRAME_LOCK_VERSION {
            return Err("Unsupported frame-lock protocol version");
        }
        let bytes: &[u8] = bytemuck::bytes_of(self);
        let expected_crc = crc32c::crc32c(&bytes[..60]);
        if self.packet_crc32c != expected_crc {
            return Err("Frame-lock CRC-32C checksum mismatch");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_lock_packet_size_and_alignment() {
        assert_eq!(std::mem::size_of::<FrameLockPacket>(), 64);
        assert_eq!(std::mem::align_of::<FrameLockPacket>(), 64);
    }

    #[test]
    fn test_frame_lock_crc() {
        let mut pkt = FrameLockPacket::default();
        pkt.lamport_clock = 1024;
        pkt.observer_pos = [120.5, 450.2, 540.0];
        pkt.update_crc();

        assert!(pkt.validate().is_ok());

        // Alter position
        pkt.observer_pos[0] += 0.1;
        assert!(pkt.validate().is_err());
    }
}
