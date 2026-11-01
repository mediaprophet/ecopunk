//! Regression for real groom assets that exceed the graph-only file envelope.
use super::*;
use std::sync::atomic::AtomicBool;

#[test]
fn larger_groom_uses_admitted_package_budget_and_reopens_exactly() {
    const BASE:&str=include_str!("../../../../assets/production-studies/swamp-wallaby-review-r1/authoring/swamp-wallaby-r3.vibe");
    const GROOM:&str=r#"species_asset = record.merge(species_asset, {
      coat:record.merge(species_asset.coat,{enabled:false}),
      groom:{seed:1707,density_per_m2:6000,max_strands:24000,segments:3,
        length_m:[0.005,0.01],width_m:[0.0003,0.0005],lift_m:0.002,
        direction:[0,-1,0],guard_fraction:0.1,root_tone:0.7,tip_tone:1.2,
        max_influence_loss:0.02,material:0,surface_materials:[0,1,2,3],regions:[],
        cold_bytes:134217728,work_units:100000000}
    });"#;
    let limits=ReviewResourceLimits::default();
    let options=CorpusOptions::default();let cancel=AtomicBool::new(false);
    let mut session=SpeciesReviewPackageSession::new(1,limits).unwrap();
    session.submit_with_corpus(BASE,0,&options,&cancel).unwrap();
    let asset=session.submit_with_corpus(GROOM,1,&options,&cancel).unwrap();
    // This is actual authored geometry/HMC, not artificial opaque file padding.
    assert!(asset.bundle.len()*4+1024*1024>options.knowledge.cold_bytes,
        "fixture no longer reaches the old incorrect graph-only inspector limit");
    assert!(asset.bundle.len()*4+options.knowledge.cold_bytes<limits.cold_work_bytes);
    let reopened=crate::species_playback::SpeciesPlaybackSession::reopen(
        &asset.bundle,limits,crate::animal_motion::MotionLimits::default(),&cancel).unwrap();
    assert_eq!(reopened.revision(),2);
    assert_eq!(reopened.asset().unwrap().bundle,asset.bundle);
    assert!(reopened.resources().entries().iter().any(|r|r.kind=="q42"));
}
