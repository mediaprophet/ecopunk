//! Source regression for end-only contacts on an authored-loop clip. Not run
//! while coding is open. It exercises both pose and final skinning phases.
use super::*;
use crate::{species_review_source::SpeciesReviewSourceSession,
    species_authored_review, swamp_wallaby_anatomical_r3::Source, wallaby_rig::RigLimits};
use std::sync::atomic::AtomicBool;

#[test]
fn explicit_loop_endpoint_keeps_end_contact_while_natural_advance_wraps() {
    let base = include_str!("../../../../assets/production-studies/swamp-wallaby-review-r1/authoring/swamp-wallaby-r3.vibe");
    let end_contact = "contacts:[{chain:[\"left_hip\",\"left_knee\",\"left_ankle\"],start_s:1.9,end_s:2.0,attachment_local_m:[0,-0.08,0],target_model_m:[-0.155,0,-0.07],pole_model_m:[-0.155,0.25,0.3],min_knee_angle_rad:0.4,max_knee_angle_rad:2.9,tolerance_m:0.003}]}";
    let edit = STUDY_EDIT.replacen("contacts:[]}", end_contact, 1);
    assert_ne!(edit, STUDY_EDIT);
    let mut session = SpeciesReviewSourceSession::default();
    session.submit(base, |a| species_authored_review::build(1, a)).unwrap();
    let (source, asset) = session.submit(&edit, |a| {
        let source: Source = serde_json::from_value(a.data.clone()).map_err(|e| e.to_string())?;
        Ok((source, species_authored_review::build(1, a)?))
    }).unwrap();
    let cancel = AtomicBool::new(false);
    let mut player = MotionPlayer::admit(&asset.render.bytes, &source,
        RigLimits::default(), MotionLimits::default(), &cancel).unwrap();
    assert_eq!(player.sample_at("head-look-study", 0., &cancel).unwrap().contact_count, 0);
    let end = player.sample_at("head-look-study", 2., &cancel).unwrap();
    assert_eq!(end.receipt.time, 2.);
    assert_eq!(end.contact_count, 1);
    assert!(end.contacts[0].error_m <= 0.003);
    let natural = player.advance(0., &cancel).unwrap();
    assert_eq!(natural.receipt.time, 0.);
    assert_eq!(natural.contact_count, 0);
}
