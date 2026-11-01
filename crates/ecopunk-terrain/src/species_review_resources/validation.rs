//! Canonical Q42/M64 validation. No graph import or authority mutation occurs.
use super::{ResourceData, ReviewResourceLimits};
#[path = "q42_intervals.rs"]
mod q42_intervals;
use qualia_core_db::{
    q42::m64_matter::{M64PayloadInterval, M64ReadLimits, M64Reader, M64_MAGIC},
    q42::q42_volume::{
        OpfsSliceRangeSource, Q42RangeVolume, MAX_COMPRESSED_SUPERBLOCK_SIZE, QUIN_SIZE,
        SUPERBLOCK_HEADER, SUPERBLOCK_SIZE,
    },
    NQuin,
};

pub(super) fn validate(
    resource: &ResourceData,
    limits: ReviewResourceLimits,
) -> Result<(), String> {
    match resource.kind.as_str() {
        "q42" => validate_q42(&resource.bytes, limits),
        "m64" => {
            let scratch_bytes = limits
                .m64_chunks
                .checked_mul(std::mem::size_of::<M64PayloadInterval>())
                .ok_or("M64 scratch overflow")?;
            if scratch_bytes > limits.cold_work_bytes {
                return Err("M64 scratch exceeds cold-work budget".into());
            }
            let mut scratch = vec![M64PayloadInterval::default(); limits.m64_chunks];
            let reader = M64Reader::parse(
                &resource.bytes,
                M64ReadLimits {
                    max_file_bytes: limits.resource_bytes,
                    max_chunks: limits.m64_chunks,
                    max_records: limits.m64_records,
                },
                &mut scratch,
            )
            .map_err(|e| format!("Retained M64 refused: {e}"))?;
            if reader.header().magic != M64_MAGIC {
                return Err(
                    "Expected an M64 resource; E64 requires its own explicit profile".into(),
                );
            }
            Ok(())
        }
        _ => Err("Unsupported native resource format".into()),
    }
}
fn validate_q42(bytes: &[u8], limits: ReviewResourceLimits) -> Result<(), String> {
    // Full unified v3/v4 volumes only: raw Quin arrays lack the retained
    // namespace/lexicon container contract and require explicit migration.
    let volume = Q42RangeVolume::open(OpfsSliceRangeSource::new(bytes))
        .map_err(|e| format!("Retained Q42 header: {e}"))?;
    let blocks = usize::try_from(volume.block_count()).map_err(|_| "Q42 block count overflow")?;
    let scratch = MAX_COMPRESSED_SUPERBLOCK_SIZE
        .checked_add(SUPERBLOCK_SIZE)
        .and_then(|n| n.checked_add(blocks.checked_mul(16)?))
        .ok_or("Q42 scratch overflow")?;
    if blocks > limits.q42_blocks || scratch > limits.cold_work_bytes {
        return Err("Q42 block/scratch budget exceeded".into());
    }
    q42_intervals::validate(&volume)?;
    let header = volume.header();
    if header.lex_length == 0 {
        return Err("Retained Q42 requires its source lexicon; no namespace is synthesized".into());
    }
    let start = usize::try_from(header.lex_offset).map_err(|_| "Q42 lexicon offset overflow")?;
    let len = usize::try_from(header.lex_length).map_err(|_| "Q42 lexicon length overflow")?;
    let end = start.checked_add(len).ok_or("Q42 lexicon span overflow")?;
    qualia_core_db::q42_lex::Q42LexMmap::from_bytes(
        bytes
            .get(start..end)
            .ok_or("Q42 lexicon outside resource")?,
    )
    .map_err(|e| format!("Retained Q42 lexicon: {e:?}"))?;
    let mut compressed = vec![0; MAX_COMPRESSED_SUPERBLOCK_SIZE];
    let mut decoded = vec![0; SUPERBLOCK_SIZE];
    let mut total = 0usize;
    for block in 0..blocks {
        volume
            .read_superblock_into(block, &mut compressed, &mut decoded)
            .map_err(|e| format!("Retained Q42 block: {e}"))?;
        let count = usize::try_from(u64::from_le_bytes(decoded[16..24].try_into().unwrap()))
            .map_err(|_| "Q42 Quin count overflow")?;
        total = total
            .checked_add(count)
            .ok_or("Q42 total Quin count overflow")?;
        if count > qualia_core_db::QUINS_PER_BLOCK || total > limits.q42_quins {
            return Err("Retained Q42 Quin budget exceeded".into());
        }
        for raw in decoded[SUPERBLOCK_HEADER..SUPERBLOCK_HEADER + count * QUIN_SIZE]
            .chunks_exact(QUIN_SIZE)
        {
            let quin = bytemuck::pod_read_unaligned::<NQuin>(raw);
            if !quin.verify_ecc_parity() {
                return Err("Retained Q42 Quin parity failed".into());
            }
        }
    }
    Ok(())
}
