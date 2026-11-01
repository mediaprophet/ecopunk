//! Q42 query regressions for authored wallaby serial loft recipes.
//! Source assertions describe retained generator controls, not biological proof.
use super::*;
use crate::{
    species_hmc_review::{CatalogueReview, ModelReview, PackageReviewIndex},
    species_review_resources::query::{FactCursor, FactPattern, QueryLimits},
};
use qualia_core_db::q_hash;
use sha2::{Digest, Sha256};

fn source_text(with_chain: bool, explicit_empty: bool) -> String {
    let mut source = r3::parse_source(r3::DEFAULT_SOURCE).unwrap();
    source.motion = None;
    if with_chain {
        source.loft_chains = vec![r3::LoftChain {
            id: "hind-continuous".into(),
            members: vec![
                r3::LoftChainMember { loft_id: "thigh-integument".into(), first_section: 0, last_section: 3 },
                r3::LoftChainMember { loft_id: "calf-integument".into(), first_section: 1, last_section: 3 },
                r3::LoftChainMember { loft_id: "hind-pes".into(), first_section: 1, last_section: 5 },
            ],
            radial_segments: 16,
            subdivisions: 3,
        }];
    }
    let mut value = serde_json::to_value(source).unwrap();
    if explicit_empty {
        value["loft_chains"] = serde_json::json!([]);
    }
    value.to_string()
}

fn q42(text: &str) -> ResourceData {
    let index = PackageReviewIndex {
        schema: "fixture",
        package_sha256: "0".repeat(64),
        package_bytes: 0,
        subject: 1,
        source_revision: "1".into(),
        source_sha256: "1".repeat(64),
        catalogue: CatalogueReview { id: "fixture".into(), version: "1".into(), sha256: "2".repeat(64) },
        resources: Vec::new(),
        models: vec![ModelReview {
            key: "review/model.10d".into(), sha256: "3".repeat(64), vertices: 0, triangles: 0,
            section_types: Vec::new(), clips: Vec::new(),
        }],
        family: None,
        coverage: Vec::new(),
        scope: "fixture",
    };
    let mut graph = graph::Graph::new(KnowledgeLimits::default());
    populate(
        &mut graph,
        "urn:fixture:study",
        "urn:fixture:model",
        text.as_bytes(),
        &index,
        &AtomicBool::new(false),
    )
    .unwrap();
    finish(graph, KnowledgeLimits::default(), &AtomicBool::new(false)).unwrap()
}

fn rows(resource: &ResourceData, predicate: &str) -> Vec<crate::species_review_resources::query::FactView> {
    let resources = ReviewResources::admit(vec![resource.clone()], ReviewResourceLimits::default()).unwrap();
    let digest = Sha256::digest(&resource.bytes).into();
    let mut cursor = FactCursor::default();
    let mut result = Vec::new();
    loop {
        let page = resources.inspect_page(
            RESOURCE_KEY,
            digest,
            FactPattern { predicate: Some(q_hash(&graph::term(predicate))), ..Default::default() },
            cursor,
            QueryLimits::default(),
        ).unwrap();
        result.extend(page.rows);
        match page.next { Some(next) => cursor = next, None => return result }
    }
}

fn subject_rows(resource: &ResourceData, subject: &str, predicate: &str) -> usize {
    let resources = ReviewResources::admit(vec![resource.clone()], ReviewResourceLimits::default()).unwrap();
    let digest = Sha256::digest(&resource.bytes).into();
    let page = resources.inspect_page(
        RESOURCE_KEY,
        digest,
        FactPattern {
            subject: Some(q_hash(subject)),
            predicate: Some(q_hash(&graph::term(predicate))),
            ..Default::default()
        },
        FactCursor::default(),
        QueryLimits::default(),
    ).unwrap();
    page.rows.len()
}

fn assert_exact_double(resource: &ResourceData, subject: &str, predicate: &str, expected: f64) {
    let resources = ReviewResources::admit(vec![resource.clone()], ReviewResourceLimits::default()).unwrap();
    let digest = Sha256::digest(&resource.bytes).into();
    let page = resources.inspect_page(
        RESOURCE_KEY,
        digest,
        FactPattern {
            subject: Some(q_hash(subject)),
            predicate: Some(q_hash(&graph::term(predicate))),
            ..Default::default()
        },
        FactCursor::default(),
        QueryLimits::default(),
    ).unwrap();
    assert_eq!(page.rows.len(), 1, "expected one {predicate} value");
    let lexical = page.rows[0].object.lexical.as_deref().expect("xsd:double must remain in the Q42 lexical map");
    let (_, actual) = qualia_core_db::resolver::typed_double::decode(lexical).unwrap();
    assert_eq!(actual.to_bits(), expected.to_bits(), "lossless xsd:double roundtrip for {predicate}");
    assert!(page.rows[0].literal.is_none(), "xsd:double belongs to the full-precision lexical path");
}

type TermMeaning = (u64, Option<String>, Option<String>);
type SemanticFact = (
    [u64; 6],
    TermMeaning,
    TermMeaning,
    TermMeaning,
    TermMeaning,
    Option<String>,
    bool,
);

fn all_semantic_facts(resource: &ResourceData) -> Vec<SemanticFact> {
    let resources = ReviewResources::admit(vec![resource.clone()], ReviewResourceLimits::default()).unwrap();
    let digest = Sha256::digest(&resource.bytes).into();
    let mut cursor = FactCursor::default();
    let mut facts = Vec::new();
    loop {
        let page = resources.inspect_page(
            RESOURCE_KEY,
            digest,
            FactPattern::default(),
            cursor,
            QueryLimits::default(),
        ).unwrap();
        for row in page.rows {
            facts.push((
                row.words,
                (row.subject.raw, row.subject.namespace, row.subject.lexical),
                (row.predicate.raw, row.predicate.namespace, row.predicate.lexical),
                (row.object.raw, row.object.namespace, row.object.lexical),
                (row.context.raw, row.context.namespace, row.context.lexical),
                row.literal.map(|literal| format!("{literal:?}")),
                row.embedded_object,
            ));
        }
        match page.next {
            Some(next) => cursor = next,
            None => {
                facts.sort();
                return facts;
            }
        }
    }
}

#[test]
fn authored_serial_recipe_and_ordered_inclusive_windows_are_q42_queryable() {
    let resource = q42(&source_text(true, false));
    let members = rows(&resource, "hasOrderedChainMember");
    assert_eq!(members.len(), 3);
    let mut lofts: Vec<_> = rows(&resource, "memberLoftPart")
        .into_iter().filter_map(|row| row.object.lexical).collect();
    lofts.sort();
    assert_eq!(lofts, vec![
        "urn:fixture:study:LoftPart:calf-integument",
        "urn:fixture:study:LoftPart:hind-pes",
        "urn:fixture:study:LoftPart:thigh-integument",
    ]);
    let integers = |predicate| {
        let mut values: Vec<_> = rows(&resource, predicate).into_iter().filter_map(|row| {
            match row.literal { Some(qualia_core_db::resolver::InlineLiteral::Integer(value)) => Some(value), _ => None }
        }).collect();
        values.sort();
        values
    };
    assert_eq!(integers("memberOrdinal"), vec![0, 1, 2]);
    assert_eq!(integers("firstSourceSectionInclusive"), vec![0, 1, 1]);
    assert_eq!(integers("lastSourceSectionInclusive"), vec![3, 3, 5]);
    assert_eq!(integers("radialSegments"), vec![16]);
    assert_eq!(integers("subdivisions"), vec![3]);
    assert_eq!(rows(&resource, "recipeStatus").len(), 1);

    let chain_iri = "urn:fixture:study:AuthoredSerialLoftChainRecipe:hind-continuous";
    assert_eq!(subject_rows(&resource, chain_iri, "sourcePath"), 1);
    for index in 0..3 {
        let member_iri = format!(
            "urn:fixture:study:AuthoredLoftChainMember:hind-continuous%3Amember%3A{index}"
        );
        assert_eq!(subject_rows(&resource, &member_iri, "sourcePath"), 1);
    }
    let subject = q_hash(chain_iri);
    let predicate = q_hash(&graph::term("representationResource"));
    let object = q_hash("urn:fixture:model");
    let resources = ReviewResources::admit(vec![resource.clone()], ReviewResourceLimits::default()).unwrap();
    let digest = Sha256::digest(&resource.bytes).into();
    let page = resources.inspect_page(
        RESOURCE_KEY,
        digest,
        FactPattern { subject: Some(subject), predicate: Some(predicate), object: Some(object), context: None },
        FactCursor::default(),
        QueryLimits::default(),
    ).unwrap();
    assert_eq!(page.rows.len(), 1);
    assert!(page.rows[0].raw_quin().verify_ecc_parity());
}

#[test]
fn absent_and_explicitly_empty_recipe_keep_the_same_q42_projection() {
    let absent = q42(&source_text(false, false));
    let explicit_empty = q42(&source_text(false, true));
    assert!(rows(&absent, "hasLoftChainRecipe").is_empty());
    assert!(rows(&explicit_empty, "hasLoftChainRecipe").is_empty());
    let absent_facts = all_semantic_facts(&absent);
    let empty_facts = all_semantic_facts(&explicit_empty);
    assert!(
        absent_facts == empty_facts,
        "complete Q42 query facts differ (absent={}, explicit-empty={}); records withheld for bounded diagnostics",
        absent_facts.len(),
        empty_facts.len(),
    );
}

#[test]
fn explicit_portable_numeric_profile_roundtrips_method_provenance_into_q42() {
    let mut source = r3::parse_source(&source_text(false, false)).unwrap();
    assert_eq!(source.numeric_profile, None);
    source.numeric_profile = Some(r3::NumericProfile::PortableSoftwareV1);
    let explicit = q42(&serde_json::to_string(&source).unwrap());
    let legacy = q42(&source_text(false, false));
    let explicit_facts = all_semantic_facts(&explicit);
    let legacy_facts = all_semantic_facts(&legacy);
    let added: Vec<_> = explicit_facts
        .iter()
        .filter(|fact| !legacy_facts.contains(*fact))
        .collect();
    assert_eq!(added.len(), 8);
    assert!(legacy_facts.iter().all(|fact| explicit_facts.contains(fact)));

    let profile = rows(&explicit, "declaresNumericExecutionProfile");
    assert_eq!(profile.len(), 1);
    let profile_id = graph::term("PortableSoftwareV1");
    assert_eq!(profile[0].object.lexical.as_deref(), Some(profile_id.as_str()));
    let method = rows(&explicit, "usesNumericMethodPolicy");
    assert_eq!(method.len(), 1);
    let method_id = graph::term("LibmBoundedAnatomySinCosPowV1");
    assert_eq!(method[0].object.lexical.as_deref(), Some(method_id.as_str()));
    let implementation = rows(&explicit, "implementedBySoftware");
    assert_eq!(implementation.len(), 1);
    assert_eq!(
        implementation[0].object.lexical.as_deref(),
        Some("https://crates.io/crates/libm/0.2.16"),
    );
    let scope = rows(&explicit, "numericMethodScope");
    assert_eq!(scope.len(), 1);
    let scope_id = graph::term("BoundedAnatomySinCosPow");
    assert_eq!(scope[0].object.lexical.as_deref(), Some(scope_id.as_str()));
    let status = rows(&explicit, "numericProfileStatus");
    assert_eq!(status.len(), 1);
    let status_id = graph::term("SourceDeclaredMethodNotExecutionProof");
    assert_eq!(status[0].object.lexical.as_deref(), Some(status_id.as_str()));

    for predicate in [
        "declaresNumericExecutionProfile",
        "usesNumericMethodPolicy",
        "implementedBySoftware",
        "numericMethodScope",
        "numericProfileStatus",
    ] {
        assert!(
            rows(&legacy, predicate).is_empty(),
            "legacy omitted profile unexpectedly emits {predicate}",
        );
    }
}

#[test]
fn authored_groom_recipe_roundtrips_controls_and_preserves_legacy_absence() {
    let mut source = r3::parse_source(&source_text(false, false)).unwrap();
    assert!(source.groom.is_none());
    source.coat.enabled = false;
    // Compare only optional groom adoption. The disabled legacy coat must be
    // identical on both sides; its enabled fact is independently authoritative.
    let legacy_source = serde_json::to_string(&source).unwrap();
    source.groom = Some(r3::Groom {
        seed: 17,
        density_per_m2: 20.125,
        max_strands: 16,
        segments: 3,
        length_m: [0.005, 0.01],
        width_m: [0.0005, 0.001],
        lift_m: 0.002,
        direction: [0.0, -1.0, 0.0],
        guard_fraction: 0.1,
        root_tone: 0.9,
        tip_tone: 1.1,
        max_influence_loss: 0.0125,
        material: 0,
        surface_materials: vec![0],
        regions: vec![r3::GroomRegion {
            id: "trunk-study".into(),
            surface_materials: vec![0],
            bounds_min_m: [-0.5, 0.1, -0.5],
            bounds_max_m: [0.5, 1.0, 0.5],
            direction: [0.0, -1.0, 0.0],
            density_scale: 1.2,
            length_scale: 0.8,
            width_scale: 1.1,
        }],
        cold_bytes: 8 * 1024 * 1024,
        work_units: 1_000_000,
    });
    let legacy = q42(&legacy_source);
    let explicit = q42(&serde_json::to_string(&source).unwrap());
    assert!(rows(&legacy, "hasGroomRecipe").is_empty());
    let legacy_facts = all_semantic_facts(&legacy);
    let explicit_facts = all_semantic_facts(&explicit);
    assert!(legacy_facts.iter().all(|fact| explicit_facts.contains(fact)));
    let recipes = rows(&explicit, "hasGroomRecipe");
    assert_eq!(recipes.len(), 1);
    let recipe = recipes[0].object.lexical.as_deref().unwrap();
    assert_exact_double(&explicit, recipe, "densityPerSquareMetre", 20.125);
    assert_exact_double(&explicit, recipe, "maximumInfluenceLoss", 0.0125);
    for predicate in [
        "groomRecipeStatus", "geometryMethod", "groomSamplingRule", "groomCurveRule",
        "geometryOwner", "seed", "densityPerSquareMetre", "maximumStrands",
        "segmentsPerStrand", "strandLengthMinimum", "strandLengthMaximum",
        "strandWidthMinimum", "strandWidthMaximum", "lift", "directionX", "directionY",
        "directionZ", "guardFraction", "rootTone", "tipTone", "maximumInfluenceLoss",
        "coldReservationBytes", "workUnitBudget", "groomMaterial",
    ] {
        assert_eq!(subject_rows(&explicit, recipe, predicate), 1, "missing {predicate}");
    }
    let geometry_method = rows(&explicit, "geometryMethod");
    let expected_geometry_method = graph::term("AreaPointRegionRejectionNativeBezierFibresV1");
    assert_eq!(geometry_method.len(), 1);
    assert_eq!(geometry_method[0].object.lexical.as_deref(), Some(expected_geometry_method.as_str()));
    assert_eq!(rows(&explicit, "hasGroomRegion").len(), 1);
    assert_eq!(rows(&explicit, "selectsSurfaceMaterial").len(), 1);
    assert_eq!(subject_rows(&explicit, recipe, "densityPerSquareMetre"), 1);
    assert_eq!(subject_rows(&explicit, recipe, "maximumInfluenceLoss"), 1);
    let sampling = rows(&explicit, "groomSamplingRule");
    assert_eq!(sampling.len(), 1);
    let sampling_method = graph::term("AreaWeightedPointRegionRejectionBarycentricRoots");
    assert_eq!(sampling[0].object.lexical.as_deref(), Some(sampling_method.as_str()));
    assert_eq!(rows(&explicit, "groomCurveRule").len(), 1);
    assert_eq!(rows(&explicit, "selectsRegionSurfaceMaterial").len(), 1);
    assert_eq!(rows(&legacy, "densityPerSquareMetre").len(), 0);
    let regions = rows(&explicit, "hasGroomRegion");
    let region = regions[0].object.lexical.as_deref().unwrap();
    for predicate in [
        "regionOrdinal", "boundsMinimumX", "boundsMinimumY", "boundsMinimumZ",
        "boundsMaximumX", "boundsMaximumY", "boundsMaximumZ", "directionX",
        "directionY", "directionZ", "densityScale", "lengthScale", "widthScale",
    ] {
        assert_eq!(subject_rows(&explicit, region, predicate), 1, "missing {predicate}");
    }
}

#[test]
fn authored_surface_junction_recipe_is_queryable_and_not_execution_or_biology_proof() {
    let mut source = r3::parse_source(&source_text(true, false)).unwrap();
    source.surface_junctions = Some(r3::JunctionSet {
        cold_bytes: 16 * 1024 * 1024,
        work_units: 100_000_000,
        recipes: vec![r3::JunctionRecipe {
            id: "left-hind-graft-study".into(),
            torso_loft: "axial-integument".into(),
            limb_chain: "hind-continuous".into(),
            mirrored: false,
            body_rows: [12, 16],
            body_column_start: 5,
            body_column_count: 4,
            limb_cut_row: 10,
            rotation: 2,
            subdivisions: 3,
            body_tangent_scale: 0.75,
            limb_tangent_scale: 1.125,
            material: 0,
            max_influence_loss: 0.05,
            min_boundary_normal_dot: 0.25,
            interior_bulge_m: None,
        }],
    });
    let legacy = q42(&source_text(true, false));
    let explicit = q42(&serde_json::to_string(&source).unwrap());
    assert!(rows(&legacy, "hasSurfaceJunctionSet").is_empty());
    let legacy_facts = all_semantic_facts(&legacy);
    let explicit_facts = all_semantic_facts(&explicit);
    assert!(legacy_facts.iter().all(|fact| explicit_facts.contains(fact)));
    let sets = rows(&explicit, "hasSurfaceJunctionSet");
    let set = sets[0].object.lexical.as_deref().unwrap();
    assert_eq!(subject_rows(&explicit, set, "junctionColdReservationBytes"), 1);
    assert_eq!(subject_rows(&explicit, set, "junctionWorkUnitBudget"), 1);
    assert_eq!(rows(&explicit, "hasJunctionRecipe").len(), 1);
    let recipes = rows(&explicit, "hasJunctionRecipe");
    let recipe = recipes[0].object.lexical.as_deref().unwrap();
    let none_method = rows(&explicit, "interiorBulgeMethod");
    assert_eq!(none_method.len(), 1);
    assert_eq!(none_method[0].object.lexical.as_deref(), Some(graph::term("CubicInteriorUnmodified").as_str()));
    assert_eq!(subject_rows(&explicit, recipe, "interiorBulgeXMetres"), 0);
    let mut folded = source.clone();
    folded.surface_junctions.as_mut().unwrap().recipes[0].interior_bulge_m = Some([0.0123456789012345, -0.0234567890123456, 0.0345678901234567]);
    let folded_graph = q42(&serde_json::to_string(&folded).unwrap());
    let folded_recipes = rows(&folded_graph, "hasJunctionRecipe");
    let folded_recipe = folded_recipes[0].object.lexical.as_deref().unwrap();
    let folded_method = rows(&folded_graph, "interiorBulgeMethod");
    assert_eq!(folded_method.len(), 1);
    assert_eq!(folded_method[0].object.lexical.as_deref(), Some(graph::term("DegreeElevatedCubicModelVectorInteriorBulgeV1").as_str()));
    let unit = rows(&folded_graph, "interiorBulgeUnit");
    assert_eq!(unit.len(), 1);
    assert_eq!(unit[0].object.lexical.as_deref(), Some(graph::term("Metre").as_str()));
    let frame = rows(&folded_graph, "interiorBulgeFrame");
    assert_eq!(frame.len(), 1);
    assert_eq!(frame[0].object.lexical.as_deref(), Some(graph::term("SourceRestModelXYZ").as_str()));
    assert_exact_double(&folded_graph, folded_recipe, "interiorBulgeXMetres", 0.0123456789012345);
    assert_exact_double(&folded_graph, folded_recipe, "interiorBulgeYMetres", -0.0234567890123456);
    assert_exact_double(&folded_graph, folded_recipe, "interiorBulgeZMetres", 0.0345678901234567);
    assert_eq!(subject_rows(&folded_graph, folded_recipe, "interiorBulgeXMetres"), 1);
    let explicit_facts = all_semantic_facts(&explicit);
    let folded_facts = all_semantic_facts(&folded_graph);
    // Authoring a bulge changes its method assertion. All other original
    // semantics survive; retaining the old method would be a false assertion.
    let method_predicate = graph::term("interiorBulgeMethod");
    let changed:Vec<_> = explicit_facts.iter().filter(|fact|
        fact.2.2.as_deref()==Some(method_predicate.as_str())).collect();
    assert_eq!(changed.len(),1);
    assert!(!folded_facts.contains(changed[0]));
    assert!(explicit_facts.iter().filter(|fact|
        fact.2.2.as_deref()!=Some(method_predicate.as_str()))
        .all(|fact| folded_facts.contains(fact)));
    assert_exact_double(&explicit, recipe, "bodyTangentScale", 0.75);
    assert_exact_double(&explicit, recipe, "limbTangentScale", 1.125);
    assert_exact_double(&explicit, recipe, "maxJunctionInfluenceLoss", 0.05);
    assert_exact_double(&explicit, recipe, "minBoundaryNormalDot", 0.25);
    for predicate in [
        "recipeOrdinal", "representationResource", "junctionRecipeStatus",
        "junctionContinuityStatus", "junctionMethod", "torsoLoftPart", "limbLoftChainRecipe",
        "mirroredJunction", "bodyRowStartInclusive", "bodyRowEndInclusive",
        "bodyColumnStartCyclic", "bodyColumnCount", "limbCutRow", "ringRotation",
        "junctionSubdivisions", "bodyTangentScale", "limbTangentScale",
        "maxJunctionInfluenceLoss", "minBoundaryNormalDot", "junctionMaterial",
    ] {
        assert_eq!(subject_rows(&explicit, recipe, predicate), 1, "missing {predicate}");
    }
    assert!(rows(&legacy, "bodyTangentScale").is_empty());
    assert!(rows(&explicit, "acceptedBiologicalContinuity").is_empty());
}









