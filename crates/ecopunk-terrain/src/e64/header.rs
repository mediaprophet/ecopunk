//! Canonical .e64 Earth Point-Tensor Container Header (64 Bytes Aligned)
//! Sibling format to .p64 weight containers, optimized for massive geospatial strata,
//! LiDAR point clouds, and holographic entity phase-space manifolds.

use bytemuck::{Pod, Zeroable};

/// Magic identifier for .m64 matter container files: ASCII `m64\0`.
pub const M64_MAGIC: [u8; 4] = *b"m64\0";

/// Backward-compatible magic identifier for .e64 files: ASCII `e64\0`.
pub const E64_MAGIC: [u8; 4] = *b"e64\0";

/// Current container format version.
pub const M64_VERSION: u32 = 1;
pub const E64_VERSION: u32 = 1;

/// Type alias for M64Header
pub type M64Header = E64Header;

/// Fixed 64-Byte .e64 Container Header
#[repr(C, align(64))]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct E64Header {
    /// 0x00..0x04: ASCII Magic `e64\0`
    pub magic: [u8; 4],
    /// 0x04..0x08: Format version (starting at 1)
    pub version: u32,
    /// 0x08..0x10: Fixed 64-byte header size
    pub header_size: u64,
    /// 0x10..0x18: Total file size in bytes
    pub total_file_size: u64,
    /// 0x18..0x20: WGS84 Reference Origin: [latitude, longitude] in degrees
    pub spatial_origin: [f32; 2],
    /// 0x20..0x28: Elevation datum: [base_altitude_m, vertical_scale_factor]
    pub elevation_datum: [f32; 2],
    /// 0x28..0x30: Byte offset to chunk directory
    pub chunk_table_offset: u64,
    /// 0x30..0x38: Number of spatial chunk partitions
    pub chunk_count: u64,
    /// 0x38..0x3C: CRC-32C checksum of the first 56 bytes of this header
    pub header_crc32c: u32,
    /// 0x3C..0x40: Container flags / alignment padding
    pub container_flags: u32,
}

impl Default for E64Header {
    fn default() -> Self {
        Self {
            magic: E64_MAGIC,
            version: E64_VERSION,
            header_size: 64,
            total_file_size: 64,
            spatial_origin: [0.0, 0.0],
            elevation_datum: [0.0, 1.0],
            chunk_table_offset: 64,
            chunk_count: 0,
            header_crc32c: 0,
            container_flags: 0,
        }
    }
}

impl E64Header {
    /// Computes and updates the CRC-32C checksum for the header.
    pub fn update_crc(&mut self) {
        self.header_crc32c = 0;
        let bytes: &[u8] = bytemuck::bytes_of(self);
        // Compute CRC over the first 56 bytes (excluding checksum and flags)
        self.header_crc32c = crc32c::crc32c(&bytes[..56]);
    }

    /// Validates the container header magic, version, and CRC-32C checksum.
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.magic != M64_MAGIC && self.magic != E64_MAGIC {
            return Err("Invalid .m64 / .e64 magic identifier");
        }
        if self.version != M64_VERSION {
            return Err("Unsupported .m64 container version");
        }
        if self.header_size != 64 {
            return Err("Corrupted header size (must be 64 bytes)");
        }
        let bytes: &[u8] = bytemuck::bytes_of(self);
        let expected_crc = crc32c::crc32c(&bytes[..56]);
        if self.header_crc32c != expected_crc {
            return Err("Header CRC-32C checksum mismatch");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e64_header_size_and_alignment() {
        assert_eq!(std::mem::size_of::<E64Header>(), 64);
        assert_eq!(std::mem::align_of::<E64Header>(), 64);
    }

    #[test]
    fn test_e64_header_crc_roundtrip() {
        let mut header = E64Header::default();
        header.spatial_origin = [-23.70, 133.87]; // Alice Springs / MacDonnell Ranges
        header.elevation_datum = [545.0, 1.0];
        header.update_crc();

        assert!(header.validate().is_ok());

        // Mutate a byte to ensure CRC fails
        header.elevation_datum[0] = 546.0;
        assert!(header.validate().is_err());
    }
}
