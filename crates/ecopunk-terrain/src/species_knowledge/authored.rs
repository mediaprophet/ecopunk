//! Source-owned model semantics, not a scientific observation or biological corpus.
use super::{
    graph::{self, entry, escaped, term, Graph},
    *,
};
use crate::{
    species_authored_review::AuthoredReviewAsset, species_review_source::AuthoredSpeciesSource,
};
use qualia_core_db::bundle::BundleReader;
use sha2::{Digest, Sha256};

/// Default is restricted, even when the supplied source happens to use public
/// URLs. Public is an explicit host declaration about this projection's INPUT
/// metadata, not a clearance/rights grant over any referenced source content.
#[derive(Clone, Copy, Default)]
pub enum KnowledgeDisclosure {
    #[default]
    Restricted,
    PublicAuthoredMetadata,
}

/// Invoke inside the existing detached source/model transaction. The source
/// owner must have produced `asset` from this exact `authored` candidate. This
/// checks its retained source bytes/revision and canonical model/package before
/// emitting named species-study, authored parts/materials/joints/clip/state and
/// representation relations. No additional source edit or circular final-HMC
/// digest is required: graph provenance names the exact BASE HMC excluding this
/// new owned partition; existing foreign corpus partitions may be present.
///
/// Geometry operators and source replay remain the existing owners. This graph
/// is authoritative for its declared authored semantic bindings, NOT adopted
/// measured biology, source rights, completion claims or replacement geometry.
pub fn build_authored_knowledge(
    subject: u8,
    authored: &AuthoredSpeciesSource,
    asset: &AuthoredReviewAsset,
    sources: &[SourceNamespaceRef<'_>],
    disclosure: KnowledgeDisclosure,
    limits: KnowledgeLimits,
    cancel: &AtomicBool,
) -> Result<ResourceData, String> {
    // Standalone callers retain their existing combined cold envelope. The
    // resource transaction below has a separately admitted package envelope.
    build_authored_knowledge_with_review_limits(subject, authored, asset, sources,
        disclosure, limits, ReviewResourceLimits {
            cold_work_bytes: limits.cold_bytes, ..Default::default()
        }, cancel)
}

pub(crate) fn build_authored_knowledge_with_review_limits(
    subject: u8,
    authored: &AuthoredSpeciesSource,
    asset: &AuthoredReviewAsset,
    sources: &[SourceNamespaceRef<'_>],
    disclosure: KnowledgeDisclosure,
    limits: KnowledgeLimits,
    review_limits: ReviewResourceLimits,
    cancel: &AtomicBool,
) -> Result<ResourceData, String> {
    if cancel.load(Ordering::Acquire) {
        return Err("Species knowledge cancelled".into());
    }
    let envelope = reservation(limits)?;
    if limits.statements == 0
        || limits.statements > 16_384
        || limits.lexicon_bytes > 1024 * 1024
        || limits.output_bytes > 16 * 1024 * 1024
        || envelope
            .checked_add(8 * 1024 * 1024)
            .is_none_or(|n| n > limits.cold_bytes)
    {
        return Err("Authored knowledge exceeds caller construction envelope".into());
    }
    let source_bytes = authored
        .sources
        .iter()
        .try_fold(0usize, |total, source| total.checked_add(source.len()))
        .ok_or("Authored source history byte count overflow")?;
    if source_bytes > 8 * 1024 * 1024
        || authored.sources.is_empty()
        || authored.sources.len() > 128
        || authored.revision != authored.sources.len() as u64
    {
        return Err("Authored knowledge source history is outside session profile".into());
    }
    let history = bounded_json(&authored.sources, 16 * 1024 * 1024)?;
    if format!("{:x}", Sha256::digest(&history)) != authored.source_sha256 {
        return Err("Authored knowledge source-history digest differs".into());
    }
    drop(history);
    let index = crate::species_hmc_review::inspect_admitted(
        subject,
        asset,
        review_limits,
        cancel,
    )?;
    let current_key = super::authored_resource_key(authored.revision, &authored.source_sha256)?;
    if index
        .resources
        .iter()
        .any(|r| r.key == RESOURCE_KEY || r.key == current_key)
    {
        return Err(
            "Authored graph generation requires a base candidate excluding the owned graph".into(),
        );
    }
    if index.source_sha256 != authored.source_sha256
        || index.source_revision != authored.revision.to_string()
    {
        return Err("Authored knowledge source revision/digest differs from admitted model".into());
    }
    let evaluated = bounded_json(&authored.data, 2_000_000)?;
    let reader = BundleReader::parse(&asset.bundle)
        .map_err(|e| format!("Admitted source package: {e:?}"))?;
    if reader.get("authoring/evaluated-source.json") != Some(evaluated.as_slice()) {
        return Err("Authored knowledge evaluated source bytes differ from retained source".into());
    }
    for (i, source) in authored.sources.iter().enumerate() {
        if reader.get(&format!("authoring/submissions/{:04}.vibe", i + 1))
            != Some(source.as_bytes())
        {
            return Err("Authored knowledge exact Vibe resource is absent/different".into());
        }
    }
    let mut graph = Graph::new(limits);
    graph.restricted = matches!(disclosure, KnowledgeDisclosure::Restricted);
    graph::populate(&mut graph, &index, sources, cancel)?;
    let study = format!(
        "urn:ecopunk:species-study-snapshot:{}",
        index.package_sha256
    );
    graph.triple(
        &study,
        &term("knowledgeScope"),
        &term("AuthoredModelSemantics"),
    )?;
    graph.triple(
        &study,
        &term("snapshotKind"),
        &term("OwnedGraphExcludedBaseHmcSnapshot"),
    )?;
    graph.triple(&study, &term("rightsStatus"), &term("RightsUnassessed"))?;
    graph.triple(
        &study,
        &term("publicationStatus"),
        &term(if graph.restricted {
            "RestrictedProjection"
        } else {
            "HostDeclaredPublicMetadata"
        }),
    )?;
    let model = entry(&study, "review/model.10d");
    graph.triple(&model, &rdf("type"), &term("EditableModelRepresentation"))?;
    graph.triple(
        &model,
        &term("representationStatus"),
        &term("AuthoredStudyUnaccepted"),
    )?;
    for profile in ["Film", "OpenWorldGame", "Cartoon"] {
        let representation = format!("{study}:required-representation:{profile}");
        graph.triple(&study, &term("requiresRepresentation"), &representation)?;
        graph.triple(
            &representation,
            &rdf("type"),
            &term("RequiredRepresentation"),
        )?;
        graph.triple(&representation, &term("deliveryProfile"), &term(profile))?;
        graph.triple(
            &representation,
            &term("claimStatus"),
            &term("MissingOrUnassessed"),
        )?;
    }
    if let Some(family) = index.family.as_ref() {
        let family_node = format!("{study}:family:{}", escaped(&family.family_id));
        graph.triple(&study, &term("declaresFamily"), &family_node)?;
        graph.triple(&family_node, &rdf("type"), &term("DeclaredModelFamily"))?;
        for variant in &family.variants {
            let v = format!("{family_node}:variant:{}", escaped(&variant.variant_id));
            graph.triple(&family_node, &term("hasDeclaredVariant"), &v)?;
            graph.triple(&v, &rdf("type"), &term("DeclaredModelVariant"))?;
            graph.triple(
                &v,
                &term("representationResource"),
                &entry(&study, &variant.asset_key),
            )?;
            graph.triple(&v, &term("claimStatus"), &term("DeclaredUnaccepted"))?;
            // Exact Core enum codes from the source-admitted family, never
            // LifecycleStage::default/from_u8 fallback for a missing variant.
            graph.integer(&v, &term("coreLifecycleStageCode"), variant.stage as usize)?;
            graph.integer(&v, &term("corePhenotypeCode"), variant.phenotype as usize)?;
            graph.integer(&v, &term("coreLodCode"), variant.lod as usize)?;
            graph.integer(&v, &term("coreStructuralStateCode"), variant.state as usize)?;
            graph.triple(&v,&term("coreFamilySchemaSource"),&format!("urn:sha256:{:x}",Sha256::digest(include_bytes!("../../../../../qualiaDB/crates/qualia-core-db/src/bundle/variant_matrix.rs"))))?;
            graph.triple(&v,&term("coreLifecycleSchemaSource"),&format!("urn:sha256:{:x}",Sha256::digest(include_bytes!("../../../../../qualiaDB/crates/qualia-core-db/src/entity_view/entity_ontologies.rs"))))?;
            if let Some(key) = variant.entity_ontology_key.as_ref() {
                graph.triple(&v, &term("declaredKnowledgeResource"), &entry(&study, key))?;
            }
        }
    }
    match subject {
        0 => super::botanical::populate(&mut graph, &study, &model, &evaluated, cancel)?,
        1 => super::wallaby::populate(&mut graph, &study, &model, &evaluated, &index, cancel)?,
        _ => return Err("Unsupported authored species knowledge profile".into()),
    }
    // Explicit migration boundary: these retained fields are not silently
    // discarded, adopted as scientific facts or falsely reported Q42-native.
    for (key, scope) in [
        ("authoring/source-session.json", "SourceReplayCompatibility"),
        ("authoring/evaluated-source.json", "TypedGeometryAdapter"),
        ("review/manifest.json", "ReviewCompatibility"),
    ] {
        let legacy = entry(&study, key);
        graph.triple(&study, &term("retainsCompatibilityResource"), &legacy)?;
        graph.triple(&legacy, &term("remainingAuthority"), &term(scope))?;
    }
    if reader.get("authoring/anatomy.json").is_some() {
        graph.triple(
            &entry(&study, "authoring/anatomy.json"),
            &term("remainingAuthority"),
            &term("AnatomyStreamsAndVertexBindings"),
        )?;
    }
    if cancel.load(Ordering::Acquire) {
        return Err("Authored knowledge cancelled before retention".into());
    }
    finish(graph, limits, cancel)
}

pub(super) fn rdf(local: &str) -> String {
    format!("http://www.w3.org/1999/02/22-rdf-syntax-ns#{local}")
}
pub(super) fn node(
    g: &mut Graph,
    study: &str,
    kind: &str,
    id: &str,
    path: &str,
) -> Result<String, String> {
    if id.is_empty() || id.len() > 1024 || path.len() > 2048 {
        return Err("Authored semantic identifier/path exceeds profile".into());
    }
    let node = format!("{study}:{}:{}", escaped(kind), escaped(id));
    g.triple(&node, &rdf("type"), &term(kind))?;
    g.triple(
        &node,
        &term("claimStatus"),
        &term("AuthoredStudyUnaccepted"),
    )?;
    g.triple(
        &node,
        &term("sourcePath"),
        &format!(
            "{}#{}",
            entry(study, "authoring/evaluated-source.json"),
            escaped(path)
        ),
    )?;
    g.triple(
        &node,
        &term("sourceIdentifier"),
        &format!("urn:ecopunk:source-local-identifier:{}", escaped(id)),
    )?;
    Ok(node)
}
pub(super) fn references(g: &mut Graph, study: &str, refs: &[String]) -> Result<(), String> {
    if refs.len() > 128 {
        return Err("Authored source reference count exceeds profile".into());
    }
    for (i, reference) in refs.iter().enumerate() {
        let record = format!("{study}:authored-source-reference:{i}");
        let rest = reference
            .strip_prefix("https://")
            .or_else(|| reference.strip_prefix("http://"))
            .ok_or("Authored reference requires explicit HTTP(S) source namespace")?;
        let authority = rest.split('/').next().unwrap_or("");
        if authority.is_empty()
            || authority.len() > 253
            || !authority
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b".-:".contains(&b))
        {
            return Err("Authored reference domain cannot define a source namespace".into());
        }
        let scheme = if reference.starts_with("https:") {
            "https"
        } else {
            "http"
        };
        g.triple(study, &term("hasSourceReference"), &record)?;
        g.triple(&record, &rdf("type"), &term("SourceNamespaceDeclaration"))?;
        g.triple(&record, "http://purl.org/dc/terms/source", reference)?;
        g.triple(
            &record,
            &term("sourceNamespace"),
            &format!("{scheme}://{authority}/"),
        )?;
        g.triple(&record, &term("claimStatus"), &term("ReferenceOnly"))?;
        g.triple(
            &record,
            &term("namespaceMethod"),
            &term("SourceDomainFallback"),
        )?;
        g.triple(&record, &term("rightsStatus"), &term("RightsUnassessed"))?;
    }
    Ok(())
}
pub(super) fn vector(g: &mut Graph, node: &str, prefix: &str, xyz: [f32; 3]) -> Result<(), String> {
    for (axis, value) in ["X", "Y", "Z"].into_iter().zip(xyz) {
        g.number_f32(node, &term(&format!("{prefix}{axis}")), value)?;
    }
    Ok(())
}

fn bounded_json(value: &impl serde::Serialize, limit: usize) -> Result<Vec<u8>, String> {
    struct Sink {
        bytes: Vec<u8>,
        limit: usize,
    }
    impl std::io::Write for Sink {
        fn write(&mut self, input: &[u8]) -> std::io::Result<usize> {
            if input.len() > self.limit.saturating_sub(self.bytes.len()) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Knowledge source serialization bound exceeded",
                ));
            }
            self.bytes.try_reserve(input.len()).map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Knowledge source allocation refused",
                )
            })?;
            self.bytes.extend_from_slice(input);
            Ok(input.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut sink = Sink {
        bytes: Vec::new(),
        limit,
    };
    serde_json::to_writer(&mut sink, value)
        .map_err(|e| format!("Canonical source projection: {e}"))?;
    Ok(sink.bytes)
}
