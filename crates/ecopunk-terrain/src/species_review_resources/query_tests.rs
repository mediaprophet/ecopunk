use super::super::{
    query_estimate::{CohortLimits, CohortRequest, EstimateError, EstimateMethod},
    ResourceData, ReviewResourceLimits, ReviewResources, SpeciesReviewPackageSession,
};
use super::*;
use qualia_core_db::{q42_volume::UnifiedVolumeBuilder, q_hash, resolver::pack_typed_literal};
use std::collections::HashMap;
fn fixture(conflict: bool, private: bool, wrong_unit: bool) -> (ReviewResources, CohortRequest) {
    let value = "urn:test:observed-radius";
    let unit = "http://qudt.org/schema/qudt/unit";
    let metre = "http://qudt.org/vocab/unit/M";
    let mut lex = HashMap::new();
    let mut facts = Vec::new();
    for term in [value, unit, metre, "http://qudt.org/vocab/unit/CM"] {
        lex.insert(q_hash(term), term.to_string());
    }
    for (i, number) in ["0.16", "0.18"].into_iter().enumerate() {
        let subject = format!("https://example.invalid/fixture/measurement-{i}");
        let id = q_hash(&subject);
        lex.insert(id, subject);
        let object = pack_typed_literal(number, "xsd:decimal").unwrap();
        let mut q = NQuin {
            subject: id,
            predicate: q_hash(value),
            object,
            context: 0,
            metadata: 0,
            parity: 0,
        };
        if private && i == 1 {
            q.set_sensitivity_tier(NQuin::SENSITIVITY_TIER_LEGAL);
        }
        q.parity = NQuin::calculate_parity(q.subject, q.predicate, q.object, q.context, q.metadata);
        facts.push(q);
        let object = q_hash(if wrong_unit && i == 1 {
            "http://qudt.org/vocab/unit/CM"
        } else {
            metre
        });
        let q = NQuin {
            subject: id,
            predicate: q_hash(unit),
            object,
            context: 0,
            metadata: 0,
            parity: NQuin::calculate_parity(id, q_hash(unit), object, 0, 0),
        };
        facts.push(q);
        if conflict && i == 0 {
            let object = pack_typed_literal("0.19", "xsd:decimal").unwrap();
            facts.push(NQuin {
                subject: id,
                predicate: q_hash(value),
                object,
                context: 0,
                metadata: 0,
                parity: NQuin::calculate_parity(id, q_hash(value), object, 0, 0),
            });
        }
    }
    facts.sort_unstable_by_key(|q| q.object);
    let mut builder = UnifiedVolumeBuilder::with_lex_map(&lex).unwrap();
    builder.push_block(0, &facts).unwrap();
    let bytes = builder.finish_to_bytes();
    let digest = Sha256::digest(&bytes).into();
    let key = "resources/knowledge/radius-fixture.q42";
    let resources = ReviewResources::admit(
        vec![ResourceData {
            key: key.into(),
            kind: "q42".into(),
            bytes,
            meta: None,
        }],
        ReviewResourceLimits::default(),
    )
    .unwrap();
    (
        resources,
        CohortRequest {
            resource_key: key.into(),
            resource_sha256: digest,
            subject: None,
            context: None,
            value_predicate: q_hash(value),
            unit_predicate: q_hash(unit),
            unit_iri: metre.into(),
            method: EstimateMethod::Median,
        },
    )
}
#[test]
fn public_sequential_pages_keep_exact_assertions_lexical_namespace_and_stale_guards() {
    let (resources, request) = fixture(true, false, false);
    let mut cursor = FactCursor::default();
    let mut count = 0;
    let mut limits = QueryLimits::default();
    limits.page_records = 1;
    loop {
        let page = resources
            .inspect_page(
                &request.resource_key,
                request.resource_sha256,
                FactPattern {
                    predicate: Some(request.value_predicate),
                    ..Default::default()
                },
                cursor,
                limits,
            )
            .unwrap();
        for fact in page.rows {
            count += 1;
            assert!(fact
                .subject
                .lexical
                .as_ref()
                .unwrap()
                .starts_with("https://example.invalid/fixture/"));
            assert!(fact.literal.is_some());
            assert!(fact.raw_quin().verify_ecc_parity());
        }
        match page.next {
            Some(next) => cursor = next,
            None => break,
        }
    }
    assert_eq!(count, 3);
    assert!(matches!(
        resources.propose_estimate(request.clone(), CohortLimits::default()),
        Err(EstimateError::ConflictingPremises)
    ));
    let mut wrong = request.resource_sha256;
    wrong[0] ^= 1;
    assert!(matches!(
        resources.inspect_page(
            &request.resource_key,
            wrong,
            FactPattern::default(),
            FactCursor::default(),
            limits
        ),
        Err(QueryError::StaleResource)
    ));
    assert!(matches!(
        resources.inspect_page(
            &request.resource_key,
            request.resource_sha256,
            FactPattern::default(),
            FactCursor {
                block_index: 0,
                quin_offset: 99999
            },
            limits
        ),
        Err(QueryError::Cursor)
    ));
}
#[test]
fn native_declared_statistics_refuse_denied_mixed_units_and_incomplete_cohorts() {
    let (resources, request) = fixture(false, false, false);
    let proposal = resources
        .propose_estimate(request.clone(), CohortLimits::default())
        .unwrap();
    assert!((proposal.value() - 0.17).abs() < 1e-12);
    assert_eq!(proposal.value_premises().len(), 2);
    assert_eq!(proposal.unit_premises().len(), 2);
    let mut mean = request.clone();
    mean.method = EstimateMethod::Mean;
    let mean = resources
        .propose_estimate(mean, CohortLimits::default())
        .unwrap();
    assert!((mean.value() - 0.17).abs() < 1e-12);
    assert_ne!(proposal.cohort_sha256(), mean.cohort_sha256());
    let (resources, request) = fixture(false, true, false);
    assert!(matches!(
        resources.propose_estimate(request, CohortLimits::default()),
        Err(EstimateError::Query(QueryError::AccessDenied))
    ));
    let (resources, request) = fixture(false, false, true);
    assert!(matches!(
        resources.propose_estimate(request, CohortLimits::default()),
        Err(EstimateError::Unit)
    ));
    let (resources, request) = fixture(false, false, false);
    let mut tiny = CohortLimits::default();
    tiny.query.page_records = 1;
    tiny.max_pages = 1;
    assert!(matches!(
        resources.propose_estimate(request.clone(), tiny),
        Err(EstimateError::Incomplete)
    ));
    let mut tiny = CohortLimits::default();
    tiny.max_values = 1;
    assert!(matches!(
        resources.propose_estimate(request.clone(), tiny),
        Err(EstimateError::Budget)
    ));
    let mut none = request;
    none.subject = Some(999);
    assert!(matches!(
        resources.propose_estimate(none, CohortLimits::default()),
        Err(EstimateError::Empty)
    ));
}
#[test]
fn manual_declared_radius_assignment_pins_actual_source_resources_and_reopens() {
    let (resources, request) = fixture(false, false, false);
    let proposal = resources
        .propose_estimate(request.clone(), CohortLimits::default())
        .unwrap();
    let source =
        include_str!("../../../../tools/species-production/white-gum-review-r1/white-gum-r2.vibe");
    let mut session = SpeciesReviewPackageSession::new(0, ReviewResourceLimits::default()).unwrap();
    let first = session
        .submit_with_resources(source, 0, resources.entries().to_vec())
        .unwrap();
    let manual = source
        .replace("let mut species_asset =", "species_asset =")
        .replace("    0.205,", &format!("    {},", proposal.value()));
    assert!(!manual.contains("    0.205,"));
    let next=session.submit_manual_radius_assignment(&manual,1,request.clone(),"collar","TEST ONLY: manually map this fixture radius cohort to the authored collar; no biological acceptance").unwrap();
    assert_ne!(next.render.bytes, first.render.bytes);
    assert_eq!(session.revision(), 2);
    let reopened = crate::species_review_reopen::reopen(0, &next.bundle).unwrap();
    assert_eq!(reopened.session.revision(), 2);
    assert!(reopened.session.sources()[1].contains("Unadopted public Q42 estimate proposal"));
    assert_eq!(
        reopened.session.resources().inventory(),
        session.resources().inventory()
    );
    assert!(session
        .submit_manual_radius_assignment("= species_asset", 1, request.clone(), "collar", "stale")
        .is_err());
    assert!(session
        .submit_manual_radius_assignment(
            "= species_asset",
            2,
            request.clone(),
            "unknown",
            "manual declaration"
        )
        .is_err());
    let mut cm = request;
    cm.unit_iri = "http://qudt.org/vocab/unit/CM".into();
    assert!(session
        .submit_manual_radius_assignment(
            "= species_asset",
            2,
            cm,
            "collar",
            "no implicit conversion"
        )
        .is_err());
    assert_eq!(session.revision(), 2);
}
