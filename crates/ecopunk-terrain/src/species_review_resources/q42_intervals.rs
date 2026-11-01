//! Additional narrow package-profile range checks over canonical Q42 fields.
//! No query-index consistency or native VerifyLevel::Full receipt is asserted.
use qualia_core_db::q42::q42_volume::{
    OpfsSliceRangeSource, Q42RangeVolume, FLAG_BLOCKS_LZ4, FLAG_FIELD_POSTINGS, FLAG_FIELD_RANGES,
    FLAG_OBJECT_SORTED, FLAG_PERMISSIVE_COMMONS, FLAG_SANCTUARY, FLAG_VOLUME_ROOT, HEADER_SIZE,
};
pub(super) fn validate(volume: &Q42RangeVolume<OpfsSliceRangeSource<&[u8]>>) -> Result<(), String> {
    let h = volume.header();
    let flags = h.flags;
    let supported = FLAG_BLOCKS_LZ4
        | FLAG_OBJECT_SORTED
        | FLAG_VOLUME_ROOT
        | FLAG_FIELD_RANGES
        | FLAG_FIELD_POSTINGS
        | FLAG_PERMISSIVE_COMMONS
        | FLAG_SANCTUARY;
    if flags & !supported != 0 {
        return Err("Q42 attachment has unsupported profile flags".into());
    }
    let mut spans = [(0u64, 0u64); 10];
    spans[0] = (0, HEADER_SIZE as u64);
    let mut count = 1;
    let mut push = |start: u64, len: u64| -> Result<(), String> {
        if len == 0 {
            return Ok(());
        }
        let end = start
            .checked_add(len)
            .filter(|&end| end <= volume.source_length())
            .ok_or("Q42 attachment range outside file")?;
        if start < HEADER_SIZE as u64 {
            return Err("Q42 attachment section overlaps header".into());
        }
        spans[count] = (start, end);
        count += 1;
        Ok(())
    };
    for (start, len) in [
        (h.lex_offset, h.lex_length),
        (h.bidx_offset, h.bidx_length),
        (h.block_dir_offset, h.block_dir_length),
        (h.data_offset, h.data_length),
        (h.temporal_index_offset, h.temporal_index_length),
        (h.dag_root_offset, h.dag_root_length),
    ] {
        push(start, len)?;
    }
    // These reserved-slot pairs are the canonical header's documented current
    // volume_manifest/field_range_index/field_postings ranges, not new schema.
    let reserved = h._reserved;
    for (flag, start) in [
        (FLAG_VOLUME_ROOT, 0),
        (FLAG_FIELD_RANGES, 16),
        (FLAG_FIELD_POSTINGS, 32),
    ] {
        if flags & flag != 0 {
            push(
                u64::from_le_bytes(reserved[start..start + 8].try_into().unwrap()),
                u64::from_le_bytes(reserved[start + 8..start + 16].try_into().unwrap()),
            )?;
        }
    }
    spans[..count].sort_unstable();
    if spans[..count].windows(2).any(|p| p[0].1 > p[1].0) {
        return Err("Q42 attachment named sections overlap".into());
    }
    let mut blocks = Vec::with_capacity(
        usize::try_from(volume.block_count()).map_err(|_| "Q42 block count overflow")?,
    );
    let data_length = h.data_length;
    for index in 0..volume.block_count() as usize {
        let entry = volume
            .block_directory_entry(index)
            .map_err(|e| format!("Q42 attachment directory: {e}"))?;
        let start = entry.rel_offset;
        let end = start
            .checked_add(u64::from(entry.comp_len))
            .filter(|&end| end <= data_length)
            .ok_or("Q42 attachment block outside data section")?;
        blocks.push((start, end));
    }
    blocks.sort_unstable();
    if blocks.windows(2).any(|p| p[0].1 > p[1].0) {
        return Err("Q42 attachment compressed blocks overlap".into());
    }
    Ok(())
}
