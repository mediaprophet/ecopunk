//! Native authored semantic resource regeneration through the existing source
//! transaction. This is not an import/adoption of outside biological evidence.
use super::*;
use crate::species_knowledge::{self, KnowledgeDisclosure, KnowledgeLimits, SourceNamespaceRef};
use crate::species_review_source::AuthoredSpeciesSource;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct CorpusOptions<'a> {
    /// The exact declared reference inventory for this authored snapshot.
    /// Acquired evidence partitions are separate retained native resources.
    pub sources: &'a [SourceNamespaceRef<'a>],
    pub disclosure: KnowledgeDisclosure,
    pub knowledge: KnowledgeLimits,
}
impl Default for CorpusOptions<'_> {
    fn default() -> Self {
        Self { sources: &[], disclosure: KnowledgeDisclosure::Restricted,
            knowledge: KnowledgeLimits::default() }
    }
}
pub(crate) fn copy_retained_corpus_inputs(resources: &ReviewResources,
    limits: ReviewResourceLimits, knowledge: KnowledgeLimits) -> Result<Vec<ResourceData>, String> {
    let bytes = resources.entries().iter().try_fold(0usize, |n, r|
        n.checked_add(r.bytes.len()).and_then(|n| n.checked_add(r.meta.as_ref().map_or(0, Vec::len))))
        .ok_or("Corpus resource copy overflow")?;
    if bytes.checked_mul(2).and_then(|n| n.checked_add(knowledge.cold_bytes))
        .is_none_or(|n| n > limits.cold_work_bytes) {
        return Err("Retained corpus plus graph construction exceeds cold budget".into());
    }
    // A matching key/marker cannot prove every assertion is generated or safe
    // to discard. Preserve ALL partitions byte/key/metadata-exact, including
    // earlier authored snapshots and any custom assertions they contain.
    Ok(resources.entries().to_vec())
}
pub(crate) fn generate_corpus_resources(subject: u8, authored: &AuthoredSpeciesSource,
    asset: &AuthoredReviewAsset, mut retained: Vec<ResourceData>, limits: ReviewResourceLimits,
    options: &CorpusOptions<'_>, cancel: &AtomicBool) -> Result<Vec<ResourceData>, String> {
    let retained_bytes = retained.iter().try_fold(0usize, |n, r| n.checked_add(r.bytes.len())
        .and_then(|n| n.checked_add(r.meta.as_ref().map_or(0, Vec::len))))
        .ok_or("Retained corpus size overflow")?;
    let envelope = asset.bundle.len().checked_mul(4)
        .and_then(|n| retained_bytes.checked_mul(2).and_then(|r| n.checked_add(r)))
        .and_then(|n| n.checked_add(options.knowledge.cold_bytes))
        .ok_or("Corpus construction envelope overflow")?;
    if envelope > limits.cold_work_bytes || cancel.load(Ordering::Acquire) {
        return Err("Corpus candidate exceeds cold budget or was cancelled".into());
    }
    // The envelope above already admits package inspection alongside graph
    // construction. Do not incorrectly reuse the graph-only cold allowance
    // as the full package inspector's separate envelope.
    let mut graph = species_knowledge::build_authored_knowledge_with_review_limits(
        subject, authored, asset, options.sources, options.disclosure,
        options.knowledge, limits, cancel)?;
    graph.key = species_knowledge::authored_resource_key(authored.revision, &authored.source_sha256)?;
    if retained.iter().any(|r| r.key == graph.key) {
        return Err("Authored graph snapshot key already exists; prior resources retained".into());
    }
    species_knowledge::validate_authored_binding(&graph, &authored.source_sha256,
        authored.revision, &asset.render.bytes, options.knowledge, cancel)?;
    retained.push(graph);
    Ok(retained)
}
impl SpeciesReviewPackageSession {
    /// Edit and regenerate the actual authored semantic partition atomically.
    /// All prior partitions survive unchanged; current bindings select the new
    /// revision rather than labelling older snapshots current. Full biological
    /// corpus acquisition/adoption and native Full verification remain distinct.
    pub fn submit_with_corpus(&mut self, source: &str, expected_revision: u64,
        options: &CorpusOptions<'_>, cancel: &AtomicBool) -> Result<AuthoredReviewAsset, String> {
        if self.revision() != expected_revision || cancel.load(Ordering::Acquire) {
            return Err("Corpus source request is stale or cancelled".into());
        }
        let (subject, limits) = self.corpus_profile();
        let retained = copy_retained_corpus_inputs(self.resources(), limits, options.knowledge)?;
        self.submit_with_authored_resources_prepared(source, expected_revision,
            crate::species_authored_review::build,
            |authored, asset| generate_corpus_resources(subject, authored, asset, retained,
                limits, options, cancel),
            |_, _| if cancel.load(Ordering::Acquire) { Err("Corpus edit cancelled".into()) } else { Ok(()) })
            .map(|(asset, ())| asset)
    }
}
#[cfg(test)]
#[path = "corpus_tests.rs"]
mod tests;
#[cfg(test)]
#[path = "corpus_large_tests.rs"]
mod large_tests;
