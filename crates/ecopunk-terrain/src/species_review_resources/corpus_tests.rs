//! Source-only regressions for preserving prior evidence and snapshot authority.
//! Execution is deferred with the rest of the implementation wave.
use super::*;
use qualia_core_db::{q_hash, NQuin, q42_volume::UnifiedVolumeBuilder, bundle::BundleReader};
use std::collections::HashMap;

fn foreign_partition_at_reserved_key() -> ResourceData {
    let terms = ["urn:fixture:custom-source:item", "http://purl.org/dc/terms/source",
        "urn:fixture:custom-source:evidence"];
    let lex: HashMap<_,_> = terms.iter().map(|s| (q_hash(s), s.to_string())).collect();
    let (s,p,o) = (q_hash(terms[0]),q_hash(terms[1]),q_hash(terms[2]));
    let record = NQuin { subject:s, predicate:p, object:o, context:0, metadata:0,
        parity:NQuin::calculate_parity(s,p,o,0,0) };
    let mut writer = UnifiedVolumeBuilder::with_lex_map(&lex).unwrap();
    writer.push_block(0, &[record]).unwrap();
    ResourceData { key:species_knowledge::RESOURCE_KEY.into(), kind:"q42".into(),
        bytes:writer.finish_to_bytes(), meta:Some(vec![0xa0]) }
}
#[test]
fn graph_regeneration_appends_snapshots_and_never_deletes_foreign_reserved_assertions() {
    const BASE:&str = include_str!("../../../../assets/production-studies/swamp-wallaby-review-r1/authoring/swamp-wallaby-r3.vibe");
    let cancel = AtomicBool::new(false);
    let mut session = SpeciesReviewPackageSession::new(1, ReviewResourceLimits::default()).unwrap();
    let foreign = foreign_partition_at_reserved_key();
    session.submit_with_resources(BASE, 0, vec![foreign.clone()]).unwrap();
    let second = session.submit_with_corpus("= species_asset", 1, &CorpusOptions::default(), &cancel).unwrap();
    let second_reader = BundleReader::parse(&second.bundle).unwrap();
    assert_eq!(second_reader.get(&foreign.key), Some(foreign.bytes.as_slice()));
    assert_eq!(second_reader.entry(&foreign.key).unwrap().meta, foreign.meta);
    let earlier = session.resources().entries().iter().find(|r| r.key != foreign.key).unwrap().clone();
    let third = session.submit_with_corpus("= species_asset", 2, &CorpusOptions::default(), &cancel).unwrap();
    let third_reader = BundleReader::parse(&third.bundle).unwrap();
    assert_eq!(third_reader.get(&foreign.key), Some(foreign.bytes.as_slice()));
    assert_eq!(third_reader.get(&earlier.key), Some(earlier.bytes.as_slice()));
    assert_eq!(session.resources().entries().len(), 3);
    let reopened = crate::species_playback::SpeciesPlaybackSession::reopen(&third.bundle,
        ReviewResourceLimits::default(), crate::animal_motion::MotionLimits::default(), &cancel).unwrap();
    assert_eq!(reopened.revision(), 3);
    assert_eq!(reopened.resources().entries().len(), 3);
    assert_eq!(reopened.asset().unwrap().bundle, third.bundle);
}
#[test]
fn failed_corpus_construction_keeps_revision_source_and_resources() {
    const BASE:&str = include_str!("../../../../assets/production-studies/swamp-wallaby-review-r1/authoring/swamp-wallaby-r3.vibe");
    let cancel = AtomicBool::new(false);
    let mut session = SpeciesReviewPackageSession::new(1, ReviewResourceLimits::default()).unwrap();
    let foreign = foreign_partition_at_reserved_key();
    session.submit_with_resources(BASE, 0, vec![foreign.clone()]).unwrap();
    let mut options = CorpusOptions::default();
    options.knowledge.statements = 0;
    assert!(session.submit_with_corpus("= species_asset", 1, &options, &cancel).is_err());
    assert_eq!(session.revision(), 1);
    assert_eq!(session.resources().entries().len(), 1);
    assert_eq!(session.resources().entries()[0].bytes, foreign.bytes);
    assert!(session.submit_with_corpus("= species_asset", 0, &CorpusOptions::default(), &cancel).is_err());
    assert_eq!(session.revision(), 1);
}
