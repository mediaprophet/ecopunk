//! Public review of exact retained Q42 bytes. No graph adoption or trust grant.
//! Sequential scans deliberately avoid unqualified BIDX/FIDX query pruning.
use super::ReviewResources;
use qualia_core_db::{
    q42_lex::Q42LexMmap,
    q42_volume::{
        OpfsSliceRangeSource, Q42RangeQueryCursor, Q42RangeQueryPattern, Q42RangeQueryPlan,
        Q42RangeQueryStrategy, Q42RangeVolume, MAX_COMPRESSED_SUPERBLOCK_SIZE, SUPERBLOCK_SIZE,
    },
    resolver::{self, InlineLiteral},
    NQuin, PermissiveRoutingLane,
};
use sha2::{Digest, Sha256};

pub use qualia_core_db::q42_volume::{
    Q42RangeQueryCursor as FactCursor, Q42RangeQueryPattern as FactPattern,
};
#[derive(Clone, Copy, Debug)]
pub struct QueryLimits {
    pub page_records: usize,
    pub max_blocks: usize,
    pub lexical_bytes: usize,
    pub cold_bytes: usize,
}
impl Default for QueryLimits {
    fn default() -> Self {
        Self {
            page_records: 64,
            max_blocks: 256,
            lexical_bytes: 256 * 1024,
            cold_bytes: 2 * 1024 * 1024,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryError {
    Resource,
    StaleResource,
    Format,
    Budget,
    Cursor,
    AccessDenied,
}
#[derive(Clone, Debug)]
pub struct TermView {
    pub raw: u64,
    pub namespace: Option<String>,
    pub lexical: Option<String>,
}
#[derive(Clone, Debug)]
pub struct FactView {
    pub words: [u64; 6],
    pub subject: TermView,
    pub predicate: TermView,
    pub object: TermView,
    pub context: TermView,
    pub literal: Option<InlineLiteral>,
    pub embedded_object: bool,
}
impl FactView {
    pub fn raw_quin(&self) -> NQuin {
        let w = self.words;
        NQuin {
            subject: w[0],
            predicate: w[1],
            object: w[2],
            context: w[3],
            metadata: w[4],
            parity: w[5],
        }
    }
}
pub struct FactPage {
    pub resource_key: String,
    pub resource_sha256: [u8; 32],
    pub rows: Vec<FactView>,
    pub next: Option<FactCursor>,
}

pub(super) fn public_record(q: &NQuin) -> bool {
    q.get_sensitivity_byte() == NQuin::SENSITIVITY_PUBLIC
        && q.get_sensitivity_tier() == NQuin::SENSITIVITY_TIER_PUBLIC
        && q.identify_routing_lane() == PermissiveRoutingLane::PassthroughStandard
}
fn term(lex: &Q42LexMmap<'_>, raw: u64, remaining: &mut usize) -> Result<TermView, QueryError> {
    let Some((ns, local)) = lex.lookup_parts(raw) else {
        return Ok(TermView {
            raw,
            namespace: None,
            lexical: None,
        });
    };
    let len = ns
        .len()
        .checked_mul(2)
        .and_then(|n| n.checked_add(local.len()))
        .ok_or(QueryError::Budget)?;
    *remaining = remaining.checked_sub(len).ok_or(QueryError::Budget)?;
    Ok(TermView {
        raw,
        namespace: if ns.is_empty() { None } else { Some(ns.into()) },
        lexical: Some(format!("{ns}{local}")),
    })
}
impl ReviewResources {
    /// Every matching row is either returned publicly or the page is refused;
    /// no denied row is filtered into an apparently complete public result.
    /// This first profile offers no argument that can grant private clearance.
    pub fn inspect_page(
        &self,
        key: &str,
        expected: [u8; 32],
        pattern: Q42RangeQueryPattern,
        cursor: Q42RangeQueryCursor,
        limits: QueryLimits,
    ) -> Result<FactPage, QueryError> {
        let resource = self
            .entries()
            .iter()
            .find(|r| r.key == key && r.kind == "q42")
            .ok_or(QueryError::Resource)?;
        let actual: [u8; 32] = Sha256::digest(&resource.bytes).into();
        if actual != expected {
            return Err(QueryError::StaleResource);
        }
        if limits.page_records == 0 || limits.page_records > 512 {
            return Err(QueryError::Budget);
        }
        let charge =
            MAX_COMPRESSED_SUPERBLOCK_SIZE
                .checked_add(SUPERBLOCK_SIZE)
                .and_then(|n| {
                    n.checked_add(limits.page_records.checked_mul(
                        std::mem::size_of::<NQuin>() + std::mem::size_of::<FactView>(),
                    )?)
                })
                .and_then(|n| n.checked_add(limits.lexical_bytes))
                .ok_or(QueryError::Budget)?;
        if charge > limits.cold_bytes {
            return Err(QueryError::Budget);
        }
        let volume = Q42RangeVolume::open(OpfsSliceRangeSource::new(resource.bytes.as_slice()))
            .map_err(|_| QueryError::Format)?;
        if volume.block_count() > limits.max_blocks as u64 {
            return Err(QueryError::Budget);
        }
        if cursor.block_index >= volume.block_count() as usize
            && !(volume.block_count() == 0 && cursor == FactCursor::default())
        {
            return Err(QueryError::Cursor);
        }
        let header = volume.header();
        let start = usize::try_from(header.lex_offset).map_err(|_| QueryError::Format)?;
        let end = start
            .checked_add(usize::try_from(header.lex_length).map_err(|_| QueryError::Format)?)
            .ok_or(QueryError::Format)?;
        let lex = Q42LexMmap::from_bytes(resource.bytes.get(start..end).ok_or(QueryError::Format)?)
            .map_err(|_| QueryError::Format)?;
        let zero = NQuin {
            subject: 0,
            predicate: 0,
            object: 0,
            context: 0,
            metadata: 0,
            parity: 0,
        };
        let mut quins = vec![zero; limits.page_records];
        let mut compressed = vec![0; MAX_COMPRESSED_SUPERBLOCK_SIZE];
        let mut decoded = vec![0; SUPERBLOCK_SIZE];
        if cursor.block_index < volume.block_count() as usize {
            volume
                .read_superblock_into(cursor.block_index, &mut compressed, &mut decoded)
                .map_err(|_| QueryError::Format)?;
            let count = u64::from_le_bytes(decoded[16..24].try_into().unwrap()) as usize;
            if cursor.quin_offset > count {
                return Err(QueryError::Cursor);
            }
        }
        let page = volume
            .execute_query_page_into(
                Q42RangeQueryPlan {
                    pattern,
                    strategy: Q42RangeQueryStrategy::Sequential,
                },
                cursor,
                &mut compressed,
                &mut decoded,
                &mut quins,
            )
            .map_err(|_| QueryError::Format)?;
        if quins[..page.returned]
            .iter()
            .any(|q| !q.verify_ecc_parity())
        {
            return Err(QueryError::Format);
        }
        if quins[..page.returned].iter().any(|q| !public_record(q)) {
            return Err(QueryError::AccessDenied);
        }
        let mut remaining = limits.lexical_bytes;
        let mut rows = Vec::with_capacity(page.returned);
        for q in &quins[..page.returned] {
            let object = term(&lex, q.object, &mut remaining)?;
            let embedded = lex.lookup_embedded_triple(q.object).is_some();
            let literal = if object.lexical.is_none() && !embedded {
                resolver::classify_inline_literal(q.object)
            } else {
                None
            };
            rows.push(FactView {
                words: [
                    q.subject,
                    q.predicate,
                    q.object,
                    q.context,
                    q.metadata,
                    q.parity,
                ],
                subject: term(&lex, q.subject, &mut remaining)?,
                predicate: term(&lex, q.predicate, &mut remaining)?,
                object,
                context: term(&lex, q.context, &mut remaining)?,
                literal,
                embedded_object: embedded,
            });
        }
        Ok(FactPage {
            resource_key: key.into(),
            resource_sha256: actual,
            rows,
            next: page.next_cursor,
        })
    }
}
#[cfg(test)]
#[path = "query_tests.rs"]
mod tests;
