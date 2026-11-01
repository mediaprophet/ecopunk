//! Authored regression cases. Execution is deferred until coding is complete.
use super::*;
use crate::{
    species_authored_review::{self, AuthoredReviewAsset},
    species_review_resources::query::{FactCursor, FactPattern, QueryError, QueryLimits},
    species_review_source::{AuthoredSpeciesSource, SpeciesReviewSourceSession},
};
use qualia_core_db::q_hash;
use sha2::{Digest, Sha256};
use std::sync::OnceLock;
const BASE: &str = include_str!(
    "../../../../assets/production-studies/swamp-wallaby-review-r1/authoring/swamp-wallaby-r3.vibe"
);
fn fixture() -> &'static (AuthoredSpeciesSource, AuthoredReviewAsset) {
    static ASSET: OnceLock<(AuthoredSpeciesSource, AuthoredReviewAsset)> = OnceLock::new();
    ASSET.get_or_init(|| {
        let mut session = SpeciesReviewSourceSession::default();
        session
            .submit(BASE, |a| species_authored_review::build(1, a))
            .unwrap();
        session
            .submit(crate::animal_motion::STUDY_EDIT, |a| {
                Ok((copy_source(a), species_authored_review::build(1, a)?))
            })
            .unwrap()
    })
}
fn copy_source(a: &AuthoredSpeciesSource) -> AuthoredSpeciesSource {
    AuthoredSpeciesSource {
        data: a.data.clone(),
        sources: a.sources.clone(),
        revision: a.revision,
        source_sha256: a.source_sha256.clone(),
    }
}
fn build(disclosure: KnowledgeDisclosure) -> ResourceData {
    let (a, asset) = fixture();
    build_authored_knowledge(
        1,
        a,
        asset,
        &[],
        disclosure,
        KnowledgeLimits::default(),
        &AtomicBool::new(false),
    )
    .unwrap()
}
fn count(resource: &ResourceData, predicate: &str) -> usize {
    let resources =
        ReviewResources::admit(vec![resource.clone()], ReviewResourceLimits::default()).unwrap();
    let sha = Sha256::digest(&resource.bytes).into();
    let mut cursor = FactCursor::default();
    let mut count = 0;
    loop {
        let page = resources
            .inspect_page(
                RESOURCE_KEY,
                sha,
                FactPattern {
                    predicate: Some(q_hash(&graph::term(predicate))),
                    ..Default::default()
                },
                cursor,
                QueryLimits::default(),
            )
            .unwrap();
        for row in page.rows {
            assert!(row.raw_quin().verify_ecc_parity());
            assert_eq!(row.predicate.namespace.as_deref(), Some(NAMESPACE));
            count += 1;
        }
        match page.next {
            Some(next) => cursor = next,
            None => return count,
        }
    }
}
#[test]
fn actual_authored_parts_joints_materials_clips_and_reference_namespaces_are_native_queryable() {
    let resource = build(KnowledgeDisclosure::PublicAuthoredMetadata);
    let (source, _) = fixture();
    let typed = crate::swamp_wallaby_anatomical_r3::parse_source(
        &serde_json::to_string(&source.data).unwrap(),
    )
    .unwrap();
    assert_eq!(count(&resource, "hasJoint"), typed.joints.len());
    assert_eq!(count(&resource, "hasMaterial"), typed.palettes.len());
    assert_eq!(
        count(&resource, "hasPart"),
        typed.lofts.len() + typed.ellipsoids.len() + typed.ears.len()
    );
    assert_eq!(
        count(&resource, "hasSourceReference"),
        typed.references.len()
    );
    assert_eq!(
        count(&resource, "hasTrack"),
        typed
            .motion
            .as_ref()
            .unwrap()
            .clips
            .iter()
            .map(|c| c.tracks.len())
            .sum::<usize>()
    );
    assert_eq!(
        count(&resource, "hasContactConstraint"),
        typed
            .motion
            .as_ref()
            .unwrap()
            .clips
            .iter()
            .map(|c| c.contacts.len())
            .sum::<usize>()
    );
    assert_eq!(count(&resource, "requiresRepresentation"), 3);
    assert_eq!(count(&resource, "declaresFamily"), 0);
    assert_eq!(count(&resource, "coreLifecycleStageCode"), 0);
    assert!(count(&resource, "positionX") >= typed.joints.len());
}
#[test]
fn default_restricted_graph_cannot_be_made_public_by_query_arguments() {
    let resource = build(KnowledgeDisclosure::default());
    let sha = Sha256::digest(&resource.bytes).into();
    let resources =
        ReviewResources::admit(vec![resource], ReviewResourceLimits::default()).unwrap();
    let result = resources.inspect_page(
        RESOURCE_KEY,
        sha,
        FactPattern::default(),
        FactCursor::default(),
        QueryLimits::default(),
    );
    assert!(matches!(result, Err(QueryError::AccessDenied)));
}
#[test]
fn stale_history_revision_or_evaluated_source_cannot_acquire_current_model_graph() {
    let (a, asset) = fixture();
    for mutation in 0..3 {
        let mut stale = copy_source(a);
        match mutation {
            0 => stale.source_sha256 = "0".repeat(64),
            1 => stale.revision += 1,
            _ => stale.data["palettes"][0]["roughness"] = serde_json::json!(0.123),
        }
        assert!(build_authored_knowledge(
            1,
            &stale,
            asset,
            &[],
            KnowledgeDisclosure::Restricted,
            KnowledgeLimits::default(),
            &AtomicBool::new(false)
        )
        .is_err());
    }
}
#[test]
fn caps_cancellation_and_inline_collision_refuse_without_resource() {
    let (a, asset) = fixture();
    let mut limits = KnowledgeLimits::default();
    limits.statements = 1;
    assert!(build_authored_knowledge(
        1,
        a,
        asset,
        &[],
        KnowledgeDisclosure::Restricted,
        limits,
        &AtomicBool::new(false)
    )
    .is_err());
    assert!(build_authored_knowledge(
        1,
        a,
        asset,
        &[],
        KnowledgeDisclosure::Restricted,
        KnowledgeLimits::default(),
        &AtomicBool::new(true)
    )
    .is_err());
    let mut graph = graph::Graph::new(KnowledgeLimits::default());
    let inline = qualia_core_db::resolver::pack_typed_literal("1", "xsd:integer").unwrap();
    graph.lex.insert(inline, "urn:fixture:collision".into());
    assert!(graph.integer("urn:fixture:s", "urn:fixture:p", 1).is_err());
}
#[test]
fn botanical_roles_radii_and_attachments_project_actual_existing_source_fields() {
    let mut session = SpeciesReviewSourceSession::default();
    session.submit(include_str!("../../../../assets/production-studies/white-gum-review-r1/authoring/white-gum-r2.vibe"),|source|{
        let evaluated=serde_json::to_vec(&source.data).unwrap();
        let mut graph=graph::Graph::new(KnowledgeLimits::default());
        super::botanical::populate(&mut graph,"urn:fixture:study","urn:fixture:model",&evaluated,&AtomicBool::new(false))?;
        let resource=finish(graph,KnowledgeLimits::default(),&AtomicBool::new(false))?;
        assert_eq!(count(&resource,"hasPart"),source.data["botanical"]["organs"].as_array().unwrap().len()+source.data["sprays"].as_array().unwrap().len());
        assert_eq!(count(&resource,"sourceRadius"),source.data["radii_m"].as_array().unwrap().len());
        assert_eq!(count(&resource,"attachesToEdgeEnd"),source.data["sprays"].as_array().unwrap().len());
        Ok(())
    }).unwrap();
}
#[test]
fn explicit_native_family_state_is_retained_without_inventing_missing_variants() {
    use qualia_core_db::{
        bundle::{
            BiologicalPhenotype, BundleReader, BundleWriter, LodTier, ModelVariantDescriptor,
            StructuralState, TwinFamilyManifest,
        },
        entity_view::entity_ontologies::LifecycleStage,
    };
    let (source, base) = fixture();
    let mut family = TwinFamilyManifest::new(
        "urn:fixture:family",
        "Declared test family",
        "fauna",
        "declared-juvenile",
    );
    family.add_variant(ModelVariantDescriptor {
        variant_id: "declared-juvenile".into(),
        phenotype: BiologicalPhenotype::Unspecified,
        stage: LifecycleStage::Juvenile,
        lod: LodTier::Lod0,
        state: StructuralState::Pristine,
        asset_key: "review/model.10d".into(),
        entity_ontology_key: None,
        procedural_seed: None,
    });
    let reader = BundleReader::parse(&base.bundle).unwrap();
    let mut writer = BundleWriter::new();
    for e in reader.entries() {
        writer
            .add_file(
                &e.key,
                &e.kind,
                reader.get(&e.key).unwrap().to_vec(),
                e.meta.clone(),
            )
            .unwrap();
    }
    writer.add_family_manifest(&family).unwrap();
    let asset = AuthoredReviewAsset {
        render: crate::species_review::SpeciesReviewAsset {
            bytes: base.render.bytes.clone(),
            metadata: base.render.metadata.clone(),
        },
        bundle: writer.build().unwrap(),
    };
    let resource = build_authored_knowledge(
        1,
        source,
        &asset,
        &[],
        KnowledgeDisclosure::PublicAuthoredMetadata,
        KnowledgeLimits::default(),
        &AtomicBool::new(false),
    )
    .unwrap();
    assert_eq!(count(&resource, "hasDeclaredVariant"), 1);
    assert_eq!(count(&resource, "coreLifecycleStageCode"), 1);
    assert_eq!(count(&resource, "corePhenotypeCode"), 1);
    assert_eq!(count(&resource, "coreFamilySchemaSource"), 1);
}
#[test]
fn reopened_owned_graph_binds_private_source_model_revision_and_base_snapshot() {
    let (source, asset) = fixture();
    let resource = build(KnowledgeDisclosure::Restricted);
    let receipt = validate_authored_binding(
        &resource,
        &source.source_sha256,
        source.revision,
        &asset.render.bytes,
        KnowledgeLimits::default(),
        &AtomicBool::new(false),
    )
    .unwrap();
    assert!(receipt.restricted);
    assert_eq!(
        receipt.base_package_sha256,
        format!("{:x}", Sha256::digest(&asset.bundle))
    );
    assert_eq!(
        receipt.graph_sha256,
        <[u8; 32]>::from(Sha256::digest(&resource.bytes))
    );
    assert!(validate_authored_binding(
        &resource,
        &source.source_sha256,
        source.revision + 1,
        &asset.render.bytes,
        KnowledgeLimits::default(),
        &AtomicBool::new(false)
    )
    .is_err());
    assert!(validate_authored_binding(
        &resource,
        &"f".repeat(64),
        source.revision,
        &asset.render.bytes,
        KnowledgeLimits::default(),
        &AtomicBool::new(false)
    )
    .is_err());
    let mut different = asset.render.bytes.clone();
    different[0] ^= 1;
    assert!(validate_authored_binding(
        &resource,
        &source.source_sha256,
        source.revision,
        &different,
        KnowledgeLimits::default(),
        &AtomicBool::new(false)
    )
    .is_err());
}
#[test]
fn legacy_thin_or_competing_model_binding_needs_explicit_migration() {
    let (source, asset) = fixture();
    let index = crate::species_hmc_review::inspect_admitted(
        1,
        asset,
        ReviewResourceLimits::default(),
        &AtomicBool::new(false),
    )
    .unwrap();
    let thin = build_review_knowledge(
        &index,
        &[],
        KnowledgeLimits::default(),
        &AtomicBool::new(false),
    )
    .unwrap();
    assert!(validate_authored_binding(
        &thin,
        &source.source_sha256,
        source.revision,
        &asset.render.bytes,
        KnowledgeLimits::default(),
        &AtomicBool::new(false)
    )
    .is_err());
    let mut graph = graph::Graph::new(KnowledgeLimits::default());
    graph::populate(&mut graph, &index, &[], &AtomicBool::new(false)).unwrap();
    let study = format!(
        "urn:ecopunk:species-study-snapshot:{}",
        index.package_sha256
    );
    let model = graph::entry(&study, "review/model.10d");
    graph
        .triple(
            &study,
            &graph::term("snapshotKind"),
            &graph::term("OwnedGraphExcludedBaseHmcSnapshot"),
        )
        .unwrap();
    graph
        .triple(
            &study,
            &graph::term("knowledgeScope"),
            &graph::term("AuthoredModelSemantics"),
        )
        .unwrap();
    graph
        .triple(
            &model,
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
            &graph::term("EditableModelRepresentation"),
        )
        .unwrap();
    graph
        .triple(
            &model,
            &graph::term("representationStatus"),
            &graph::term("AuthoredStudyUnaccepted"),
        )
        .unwrap();
    graph
        .triple(
            &model,
            "http://www.w3.org/ns/prov#wasDerivedFrom",
            &format!("urn:sha256:{}", "f".repeat(64)),
        )
        .unwrap();
    let competing = finish(graph, KnowledgeLimits::default(), &AtomicBool::new(false)).unwrap();
    assert!(validate_authored_binding(
        &competing,
        &source.source_sha256,
        source.revision,
        &asset.render.bytes,
        KnowledgeLimits::default(),
        &AtomicBool::new(false)
    )
    .is_err());
}
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn full_verifier_is_exact_file_bound_and_returns_actual_native_receipt() {
    let resource = build(KnowledgeDisclosure::PublicAuthoredMetadata);
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "ecopunk-species-knowledge-{}-{stamp}.q42",
        std::process::id()
    ));
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .unwrap();
    file.write_all(&resource.bytes).unwrap();
    drop(file);
    let result = verify_generated_at(
        &resource,
        &path,
        KnowledgeLimits::default(),
        &AtomicBool::new(false),
    );
    // Another structurally valid generated resource, not invalid magic: this
    // must fail association with the staged public file, not only parsing.
    let changed = build(KnowledgeDisclosure::Restricted);
    assert!(verify_generated_at(
        &changed,
        &path,
        KnowledgeLimits::default(),
        &AtomicBool::new(false)
    )
    .is_err());
    std::fs::remove_file(path).unwrap();
    assert_eq!(
        result.unwrap().overall,
        qualia_core_db::q42_volume::CheckStatus::Pass
    );
}
