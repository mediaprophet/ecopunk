use super::*;
use crate::{
    species_authored_review::{self, AuthoredReviewAsset},
    species_review_resources::ReviewResourceLimits,
    species_review_source::SpeciesReviewSourceSession,
};
use qualia_core_db::{
    bundle::{BundleReader, BundleWriter, ModelVariantDescriptor, TwinFamilyManifest},
    entity_view::entity_ontologies::LifecycleStage,
};
use std::sync::{atomic::AtomicBool, OnceLock};
const BASE: &str = include_str!(
    "../../../../assets/production-studies/swamp-wallaby-review-r1/authoring/swamp-wallaby-r3.vibe"
);
fn fixture() -> &'static AuthoredReviewAsset {
    static ASSET: OnceLock<AuthoredReviewAsset> = OnceLock::new();
    ASSET.get_or_init(|| {
        let mut s = SpeciesReviewSourceSession::default();
        s.submit(BASE, |a| species_authored_review::build(1, a))
            .unwrap();
        s.submit(crate::animal_motion::STUDY_EDIT, |a| {
            species_authored_review::build(1, a)
        })
        .unwrap()
    })
}
fn copy_with_family(family: &TwinFamilyManifest) -> AuthoredReviewAsset {
    let asset = fixture();
    let reader = BundleReader::parse(&asset.bundle).unwrap();
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
    writer.add_family_manifest(family).unwrap();
    AuthoredReviewAsset {
        render: crate::species_review::SpeciesReviewAsset {
            bytes: asset.render.bytes.clone(),
            metadata: asset.render.metadata.clone(),
        },
        bundle: writer.build().unwrap(),
    }
}
#[test]
fn actual_hmc_model_sources_and_clips_index_without_inventing_missing_graph_or_forms() {
    let asset = fixture();
    let index = inspect_admitted(
        1,
        asset,
        ReviewResourceLimits::default(),
        &AtomicBool::new(false),
    )
    .unwrap();
    assert!(index.family.is_none());
    assert_eq!(index.models.len(), 1);
    assert_eq!(index.models[0].clips.len(), 3);
    assert_eq!(
        index.resources.iter().filter(|r| r.kind == "vibe").count(),
        2
    );
    assert!(!index
        .resources
        .iter()
        .any(|r| r.kind == "q42" || r.kind == "m64"));
    assert!(index
        .coverage
        .iter()
        .filter(|r| r.dimension == "phenotype"
            || r.dimension == "lifecycle_stage"
            || r.dimension == "parent_offspring_relation"
            || r.dimension == "reproductive_condition")
        .all(|r| r.status == "missing_or_unassessed"));
    let clip = index.models[0]
        .clips
        .iter()
        .find(|c| c.source_name.as_deref() == Some("hop-step-study"))
        .unwrap();
    assert_eq!(clip.duration_s, 1.2);
    assert!(clip.id.parse::<u64>().unwrap() > 9_007_199_254_740_991);
    assert!(index
        .select_clip_exact(&index.package_sha256, "review/model.10d", &clip.id)
        .is_ok());
    assert!(index
        .select_clip_exact("stale", "review/model.10d", &clip.id)
        .is_err());
    assert!(index
        .select_variant_exact(&index.package_sha256, "female")
        .is_err());
    let projected = serde_json::to_value(&index).unwrap();
    assert!(projected["models"][0]["clips"][0]["id"].is_string());
}
#[test]
fn native_family_projection_selects_exact_member_without_sex_stage_or_parental_fallback() {
    let mut family = TwinFamilyManifest::new(
        "urn:authored:test:wallaby-family",
        "Authored family test",
        "fauna",
        "female-study",
    );
    family.add_variant(ModelVariantDescriptor {
        variant_id: "female-study".into(),
        phenotype: BiologicalPhenotype::Female,
        stage: LifecycleStage::Adult,
        lod: LodTier::Lod0,
        state: StructuralState::Pristine,
        asset_key: "review/model.10d".into(),
        entity_ontology_key: None,
        procedural_seed: Some(u64::MAX),
    });
    let asset = copy_with_family(&family);
    let index = inspect_admitted(
        1,
        &asset,
        ReviewResourceLimits::default(),
        &AtomicBool::new(false),
    )
    .unwrap();
    let selected = index
        .select_variant_exact(&index.package_sha256, "female-study")
        .unwrap();
    assert_eq!(
        selected.procedural_seed.as_deref(),
        Some("18446744073709551615")
    );
    assert!(index
        .select_variant_exact(&index.package_sha256, "male-study")
        .is_err());
    assert_eq!(
        index
            .coverage
            .iter()
            .find(|r| r.dimension == "phenotype" && r.value == "female")
            .unwrap()
            .status,
        "declared_unaccepted"
    );
    assert_eq!(
        index
            .coverage
            .iter()
            .find(|r| r.value == "pregnancy")
            .unwrap()
            .status,
        "missing_or_unassessed"
    );
    assert_eq!(
        index
            .coverage
            .iter()
            .find(|r| r.value == "joey")
            .unwrap()
            .status,
        "missing_or_unassessed"
    );
    family.variants[0].entity_ontology_key = Some("missing.q42".into());
    assert!(inspect_admitted(
        1,
        &copy_with_family(&family),
        ReviewResourceLimits::default(),
        &AtomicBool::new(false)
    )
    .is_err());
}
#[test]
fn asset_binding_budget_and_cancellation_refuse_without_mutation() {
    let asset = fixture();
    assert!(inspect_admitted(
        1,
        asset,
        ReviewResourceLimits {
            package_bytes: 1,
            ..Default::default()
        },
        &AtomicBool::new(false)
    )
    .is_err());
    assert!(inspect_admitted(
        1,
        asset,
        ReviewResourceLimits::default(),
        &AtomicBool::new(true)
    )
    .is_err());
    let mut wrong = AuthoredReviewAsset {
        render: crate::species_review::SpeciesReviewAsset {
            bytes: asset.render.bytes.clone(),
            metadata: asset.render.metadata.clone(),
        },
        bundle: asset.bundle.clone(),
    };
    wrong.render.metadata["title"] = "different source-bound title".into();
    let error = inspect_admitted(
        1,
        &wrong,
        ReviewResourceLimits::default(),
        &AtomicBool::new(false),
    )
    .err()
    .unwrap();
    assert!(error.contains("model/manifest do not bind"));
    wrong.render.metadata = asset.render.metadata.clone();
    wrong.render.bytes[64] ^= 1;
    let error = inspect_admitted(
        1,
        &wrong,
        ReviewResourceLimits::default(),
        &AtomicBool::new(false),
    )
    .err()
    .unwrap();
    assert!(error.contains("model/manifest do not bind"));
}
#[test]
fn flora_coverage_does_not_impose_animal_pregnancy_or_mammalian_sex_assumptions() {
    let rows = coverage::build(0, None, &[]);
    assert!(rows.iter().any(|r| r.value == "seed_or_spore"));
    assert!(rows.iter().any(|r| r.value == "developmental_transition"));
    assert!(!rows
        .iter()
        .any(|r| r.value == "pregnancy" || r.value == "joey" || r.value == "male"));
}
