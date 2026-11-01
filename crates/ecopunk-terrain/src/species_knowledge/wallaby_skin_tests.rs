//! Native graph/query regressions authored without execution.
use super::*;
use crate::species_review_resources::query::{FactCursor, FactPattern, QueryLimits};
use qualia_core_db::q_hash;
use sha2::{Digest, Sha256};

fn fixture() -> (r3::Loft, Vec<String>) {
    let source = r3::parse_source(r3::DEFAULT_SOURCE).unwrap();
    let mut loft = source.lofts[0].clone();
    loft.mirror_joints = None;
    loft.skin_stations = None;
    loft.mirror_skin_stations = None;
    let joints = (0..source.joints.len()).map(|i| format!("urn:fixture:joint:{i}")).collect();
    (loft, joints)
}
fn resource(loft: &r3::Loft, joints: &[String]) -> ResourceData {
    let limits = KnowledgeLimits::default();
    let mut graph = Graph::new(limits);
    loft_skin(&mut graph, "urn:fixture:study", "urn:fixture:part", loft, 0,
        joints, &AtomicBool::new(false)).unwrap();
    super::super::finish(graph, limits, &AtomicBool::new(false)).unwrap()
}
fn rows(resource: &ResourceData, predicate: &str) -> Vec<crate::species_review_resources::query::FactView> {
    let inventory = ReviewResources::admit(vec![resource.clone()], ReviewResourceLimits::default()).unwrap();
    let hash = Sha256::digest(&resource.bytes).into();
    let mut cursor = FactCursor::default();
    let mut rows = Vec::new();
    loop {
        let page = inventory.inspect_page(RESOURCE_KEY, hash,
            FactPattern { predicate: Some(q_hash(&term(predicate))), ..Default::default() },
            cursor, QueryLimits::default()).unwrap();
        rows.extend(page.rows);
        match page.next { Some(next) => cursor = next, None => return rows }
    }
}
#[test]
fn explicit_primary_and_mirror_controls_are_native_queryable_without_zero_weight_claims() {
    let (mut loft, joints) = fixture();
    loft.skin_stations = Some(vec![r3::SkinStation { joints: [2, 0], weights: [1., 0.] }; loft.sections.len()]);
    loft.mirror_joints = Some([9, 11]);
    loft.mirror_skin_stations = Some(vec![r3::SkinStation { joints: [4, 5], weights: [0.25, 0.75] }; loft.sections.len()]);
    let graph = resource(&loft, &joints);
    let primary = rows(&graph, "influencedByJoint");
    assert_eq!(primary.len(), 1);
    assert_eq!(primary[0].object.lexical.as_deref(), Some(joints[2].as_str()));
    let mirror: BTreeSet<_> = rows(&graph, "mirrorInfluencedByJoint").into_iter()
        .map(|r| r.object.lexical.unwrap()).collect();
    assert_eq!(mirror, BTreeSet::from([joints[4].clone(), joints[5].clone()]));
    assert_eq!(rows(&graph, "hasSkinStation").len(), loft.sections.len());
    assert_eq!(rows(&graph, "hasMirrorSkinStation").len(), loft.sections.len());
    assert_eq!(rows(&graph, "sourceSkinWeight").len(), 4 * loft.sections.len());
    assert_eq!(rows(&graph, "influenceSlot").len(), 4 * loft.sections.len());
    assert!(rows(&graph, "sourcePath").iter().any(|r| r.object.lexical.as_deref()
        .is_some_and(|s| s.ends_with("#/lofts/0/mirror_skin_stations/0/weights/1"))));
    let weights: BTreeSet<_> = rows(&graph, "sourceSkinWeight").into_iter()
        .map(|r| r.object.raw).collect();
    for value in ["0", "0.25", "0.75", "1"] {
        assert!(weights.contains(&qualia_core_db::resolver::pack_typed_literal(
            value, "http://www.w3.org/2001/XMLSchema#float").unwrap()));
    }
}
#[test]
fn omitted_stations_retain_legacy_endpoint_projection() {
    let (loft, joints) = fixture();
    let graph = resource(&loft, &joints);
    let actual: BTreeSet<_> = rows(&graph, "influencedByJoint").into_iter()
        .map(|r| r.object.lexical.unwrap()).collect();
    assert_eq!(actual, loft.joints.map(|i| joints[i].clone()).into_iter().collect());
    assert!(rows(&graph, "hasSkinStation").is_empty());
    assert!(rows(&graph, "sourceSkinWeight").is_empty());
    assert!(rows(&graph, "skinInterpolationPolicy").is_empty());
}
#[test]
fn station_graph_preserves_cancellation_and_statement_budget_refusal() {
    let (mut loft, joints) = fixture();
    loft.skin_stations = Some(vec![r3::SkinStation { joints: [2, 0], weights: [1., 0.] }; loft.sections.len()]);
    let mut graph = Graph::new(KnowledgeLimits::default());
    assert!(loft_skin(&mut graph, "urn:fixture:study", "urn:fixture:part", &loft, 0,
        &joints, &AtomicBool::new(true)).is_err());
    let mut limits = KnowledgeLimits::default();
    limits.statements = 1;
    let mut graph = Graph::new(limits);
    assert!(loft_skin(&mut graph, "urn:fixture:study", "urn:fixture:part", &loft, 0,
        &joints, &AtomicBool::new(false)).is_err());
}
