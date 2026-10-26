//! ADR 0014 `FLAG_PAYLOAD_CATALOGUE` (`0x0080`) section encoding and decoding for `.q42` volumes.
//!
//! Provides byte-exact binary and CBOR parity with [`crate::bundle::cbor_ld_manifest::PayloadCatalogueEntry`]
//! for multi-language prose definitions, SHACL shape pack descriptors, and provenance sidecars
//! stored inside `.q42` v4 volumes.
//!
//! Header reserved slot layout when `FLAG_PAYLOAD_CATALOGUE` (`0x0080`) is set:
//! - `_reserved[48..56]` = `payload_catalogue_offset` (`u64` little-endian)
//! - `_reserved[56..64]` = `payload_catalogue_length` (`u64` little-endian)

use crate::bundle::cbor_ld_manifest::PayloadCatalogueEntry;
use crate::q42_volume::{Q42VolumeHeader, FLAG_PAYLOAD_CATALOGUE};

pub const PAYLOAD_CATALOGUE_MAGIC: [u8; 4] = *b"PCAT";
pub const PAYLOAD_CATALOGUE_VERSION: u32 = 1;
pub const PAYLOAD_CATALOGUE_HEADER_BYTES: usize = 16;

/// Encode a slice of [`PayloadCatalogueEntry`] records into a self-describing `PCAT` v1 section.
///
/// Layout:
/// - `[0..4]`: `"PCAT"` magic
/// - `[4..8]`: version (`1u32` LE)
/// - `[8..12]`: entry count (`u32` LE)
/// - `[12..16]`: CRC-32C of the CBOR payload (`u32` LE)
/// - `[16..]`: deterministic CBOR array of `PayloadCatalogueEntry`
pub fn encode_payload_catalogue_section(
    entries: &[PayloadCatalogueEntry],
) -> Result<Vec<u8>, String> {
    let count = u32::try_from(entries.len())
        .map_err(|_| "payload catalogue entry count exceeds u32::MAX".to_string())?;
    let mut cbor_bytes = Vec::new();
    ciborium::into_writer(entries, &mut cbor_bytes)
        .map_err(|e| format!("failed to encode payload catalogue CBOR: {e}"))?;
    let crc = crc32c::crc32c(&cbor_bytes);

    let mut out = Vec::with_capacity(PAYLOAD_CATALOGUE_HEADER_BYTES + cbor_bytes.len());
    out.extend_from_slice(&PAYLOAD_CATALOGUE_MAGIC);
    out.extend_from_slice(&PAYLOAD_CATALOGUE_VERSION.to_le_bytes());
    out.extend_from_slice(&count.to_le_bytes());
    out.extend_from_slice(&crc.to_le_bytes());
    out.extend_from_slice(&cbor_bytes);
    Ok(out)
}

/// Decode and CRC-32C-verify a `PCAT` v1 section back into [`PayloadCatalogueEntry`] records.
pub fn decode_payload_catalogue_section(
    section_bytes: &[u8],
) -> Result<Vec<PayloadCatalogueEntry>, String> {
    if section_bytes.len() < PAYLOAD_CATALOGUE_HEADER_BYTES {
        return Err("payload catalogue section shorter than 16-byte header".to_string());
    }
    if &section_bytes[0..4] != &PAYLOAD_CATALOGUE_MAGIC {
        return Err("invalid payload catalogue magic (expected PCAT)".to_string());
    }
    let version = u32::from_le_bytes(section_bytes[4..8].try_into().unwrap());
    if version != PAYLOAD_CATALOGUE_VERSION {
        return Err(format!("unsupported payload catalogue version {version}"));
    }
    let expected_count = u32::from_le_bytes(section_bytes[8..12].try_into().unwrap()) as usize;
    let expected_crc = u32::from_le_bytes(section_bytes[12..16].try_into().unwrap());
    let cbor_payload = &section_bytes[PAYLOAD_CATALOGUE_HEADER_BYTES..];

    let actual_crc = crc32c::crc32c(cbor_payload);
    if actual_crc != expected_crc {
        return Err(format!(
            "payload catalogue CRC-32C mismatch: expected {expected_crc:#010x}, got {actual_crc:#010x}"
        ));
    }

    let entries: Vec<PayloadCatalogueEntry> = ciborium::from_reader(cbor_payload)
        .map_err(|e| format!("failed to decode payload catalogue CBOR: {e}"))?;
    if entries.len() != expected_count {
        return Err(format!(
            "payload catalogue entry count mismatch: header {expected_count} vs decoded {}",
            entries.len()
        ));
    }
    Ok(entries)
}

/// Stamp `FLAG_PAYLOAD_CATALOGUE` (`0x0080`) and the `(offset, length)` pair into `header._reserved[48..64]`.
pub fn set_header_payload_catalogue_range(
    header: &mut Q42VolumeHeader,
    offset: u64,
    length: u64,
) {
    header.flags |= FLAG_PAYLOAD_CATALOGUE;
    header._reserved[48..56].copy_from_slice(&offset.to_le_bytes());
    header._reserved[56..64].copy_from_slice(&length.to_le_bytes());
}

/// Read the `(offset, length)` range for `FLAG_PAYLOAD_CATALOGUE` (`0x0080`) from `header._reserved[48..64]`.
pub fn header_payload_catalogue_range(header: &Q42VolumeHeader) -> Option<(u64, u64)> {
    let flags = { header.flags };
    if flags & FLAG_PAYLOAD_CATALOGUE == 0 {
        return None;
    }
    let reserved = header._reserved;
    let offset = u64::from_le_bytes(reserved[48..56].try_into().unwrap());
    let length = u64::from_le_bytes(reserved[56..64].try_into().unwrap());
    if length == 0 {
        return None;
    }
    Some((offset, length))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_catalogue_section_roundtrips_with_crc32c_and_header_range() {
        let entries = vec![
            PayloadCatalogueEntry {
                payload_key: "ns/human-rights/udhr-article-1.md".to_string(),
                mime_type: "text/markdown".to_string(),
                language: Some("en".to_string()),
                role: "normative_prose".to_string(),
                byte_length: 1024,
                sha256_hex: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                    .to_string(),
            },
            PayloadCatalogueEntry {
                payload_key: "shapes/natural_agent.shacl.ttl".to_string(),
                mime_type: "text/turtle".to_string(),
                language: None,
                role: "shacl_shape_pack".to_string(),
                byte_length: 4096,
                sha256_hex: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                    .to_string(),
            },
        ];

        let encoded = encode_payload_catalogue_section(&entries).unwrap();
        let decoded = decode_payload_catalogue_section(&encoded).unwrap();
        assert_eq!(decoded, entries);

        // Verify tamper detection via CRC-32C
        let mut corrupted = encoded.clone();
        let last = corrupted.len() - 1;
        corrupted[last] ^= 0x55;
        assert!(decode_payload_catalogue_section(&corrupted).is_err());

        // Verify header range stamping
        let mut header = Q42VolumeHeader::new_v4(256, 64, 320, 64, 384, 16, 400, 4096, 1, 40960, 853);
        assert_eq!(header_payload_catalogue_range(&header), None);
        set_header_payload_catalogue_range(&mut header, 4496, encoded.len() as u64);
        assert_eq!(
            header_payload_catalogue_range(&header),
            Some((4496, encoded.len() as u64))
        );
    }
}
