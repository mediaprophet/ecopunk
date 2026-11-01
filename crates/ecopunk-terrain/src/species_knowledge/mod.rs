//! Native Q42 technical-study ontology; never JSON renamed as graph data.
//! Only source-owner-authorized technical metadata is supplied. Referencing an
//! attached digest does not declassify its contents or adopt source assertions.
mod authored;
mod binding;
mod botanical;
mod graph;
#[cfg(test)]
mod tests;
#[cfg(not(target_arch = "wasm32"))]
mod verify;
mod wallaby;
pub use authored::{build_authored_knowledge, KnowledgeDisclosure};
pub(crate) use authored::build_authored_knowledge_with_review_limits;
pub use binding::{authored_resource_key, validate_authored_binding, AuthoredKnowledgeBinding};
#[cfg(not(target_arch = "wasm32"))]
pub use verify::verify_generated_at;

use crate::{
    species_hmc_review::PackageReviewIndex,
    species_review_resources::{ResourceData, ReviewResourceLimits, ReviewResources},
};
use qualia_core_db::q42_volume::UnifiedVolumeBuilder;
use std::sync::atomic::{AtomicBool, Ordering};
pub const NAMESPACE: &str = "urn:ecopunk:species-review-knowledge:v1#";
pub const RESOURCE_KEY: &str = "resources/knowledge/species-review.q42";
#[derive(Clone, Copy)]
pub enum SourceFormat {
    Rdf,
    JsonCompatibility,
    Csv,
    OtherDeclared,
    Html,
}
impl SourceFormat {
    fn term(self) -> &'static str {
        match self {
            Self::Rdf => "RdfSource",
            Self::JsonCompatibility => "JsonCompatibilitySource",
            Self::Csv => "CsvSource",
            Self::OtherDeclared => "OtherDeclaredSource",
            Self::Html => "HtmlSource",
        }
    }
}
/// Source namespaces are declared explicitly even for non-RDF data. A reference
/// link is not evidence that its content has been fetched, licensed or adopted.
pub struct SourceNamespaceRef<'a> {
    pub source_iri: &'a str,
    pub namespace_iri: &'a str,
    pub format: SourceFormat,
}
#[derive(Clone, Copy)]
pub struct KnowledgeLimits {
    pub statements: usize,
    pub lexicon_bytes: usize,
    pub output_bytes: usize,
    pub cold_bytes: usize,
}
impl Default for KnowledgeLimits {
    fn default() -> Self {
        Self {
            statements: 16_384,
            lexicon_bytes: 512 * 1024,
            output_bytes: 8 * 1024 * 1024,
            cold_bytes: 64 * 1024 * 1024,
        }
    }
}
/// Emits the current canonical Q42 volume through Core's existing native volume/lexicon,
/// indexing and statement owners. The graph names the exact BASE package from
/// the supplied inspection; enclosing it in a later HMC never creates a circular
/// self-digest claim. Subsequent source edits may leave this snapshot stale.
///
/// Public context0 is used only for authored technical metadata in this bounded
/// profile. Caller source-policy admission must precede this operation; there is
/// no private/classified data read, clearance grant or source-content harvesting.
pub fn build_review_knowledge(
    index: &PackageReviewIndex,
    sources: &[SourceNamespaceRef<'_>],
    limits: KnowledgeLimits,
    cancel: &AtomicBool,
) -> Result<ResourceData, String> {
    if cancel.load(Ordering::Acquire) {
        return Err("Species knowledge generation cancelled".into());
    }
    let reservation = reservation(limits)?;
    if limits.statements == 0
        || limits.statements > 16_384
        || limits.lexicon_bytes > 1024 * 1024
        || limits.output_bytes > 16 * 1024 * 1024
        || reservation > limits.cold_bytes
    {
        return Err(
            "Species knowledge exceeds caller statement/lexicon/output/cold profile".into(),
        );
    }
    let mut graph = graph::Graph::new(limits);
    graph::populate(&mut graph, index, sources, cancel)?;
    finish(graph, limits, cancel)
}

pub(super) fn reservation(limits: KnowledgeLimits) -> Result<usize, String> {
    limits
        .statements
        .checked_mul(768)
        .and_then(|n| {
            limits
                .lexicon_bytes
                .checked_mul(16)
                .and_then(|b| n.checked_add(b))
        })
        .and_then(|n| {
            limits
                .output_bytes
                .checked_mul(3)
                .and_then(|b| n.checked_add(b))
        })
        .and_then(|n| n.checked_add(1024 * 1024))
        .ok_or_else(|| "Species knowledge reservation overflow".into())
}

pub(super) fn finish(
    mut graph: graph::Graph,
    limits: KnowledgeLimits,
    cancel: &AtomicBool,
) -> Result<ResourceData, String> {
    graph.facts.sort_unstable_by_key(|q| q.object);
    let mut writer = UnifiedVolumeBuilder::with_lex_map(&graph.lex)
        .map_err(|e| format!("Core Q42LEX encode: {e:?}"))?;
    if graph.restricted {
        writer = writer.with_sanctuary();
    }
    // Existing Core block owner rejects capacity/order errors;512 is below its
    // current850-record SuperBlock capacity, while the total count remains capped.
    for (seq, records) in graph.facts.chunks(512).enumerate() {
        if cancel.load(Ordering::Acquire) {
            return Err("Species knowledge generation cancelled".into());
        }
        writer
            .push_block(seq as u64, records)
            .map_err(|e| format!("Core Q42 statement block: {e}"))?;
    }
    let bytes = writer.finish_to_bytes();
    if bytes.len() > limits.output_bytes {
        return Err("Native species Q42 exceeds caller output cap".into());
    }
    if cancel.load(Ordering::Acquire) {
        return Err("Species knowledge generation cancelled before publication".into());
    }
    let resource = ResourceData {
        key: RESOURCE_KEY.into(),
        kind: "q42".into(),
        bytes,
        meta: None,
    };
    // Existing source-owner canonical graph admission is reused; it does not make
    // a biological/corpus adoption or a Full query-index/lex consistency claim.
    ReviewResources::admit(
        vec![resource.clone()],
        ReviewResourceLimits {
            resource_bytes: limits.output_bytes,
            total_bytes: limits.output_bytes,
            cold_work_bytes: limits.cold_bytes,
            ..Default::default()
        },
    )?;
    Ok(resource)
}

pub const ONTOLOGY_IRI: &str = "urn:ecopunk:species-review-knowledge:v1#Ontology";
pub const ONTOLOGY_TTL: &str = include_str!("ontology.ttl");
