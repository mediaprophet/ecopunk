//! Source regression cases for the source-to-runtime dependent transaction.
//! Authored only; no execution result is claimed.
use crate::{
    animal_motion::{MotionLimits, STUDY_EDIT},
    animal_motion_review::AnimalMotionReviewSession,
    species_review_resources::ReviewResourceLimits,
    wallaby_rig::RigLimits,
};
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicBool, Ordering};
const BASE: &str = include_str!(
    "../../../assets/production-studies/swamp-wallaby-review-r1/authoring/swamp-wallaby-r3.vibe"
);
#[test]
fn caller_projection_budget_refusal_keeps_empty_source_and_runtime() {
    let mut rig = RigLimits::default();
    rig.cold_bytes = 1;
    let mut session = AnimalMotionReviewSession::new(
        ReviewResourceLimits::default(),
        rig,
        MotionLimits::default(),
    )
    .unwrap();
    assert!(session.submit(BASE, 0, &AtomicBool::new(false)).is_err());
    assert_eq!(session.revision(), 0);
    assert!(session.sources().is_empty());
    assert!(session.asset().is_none());
    assert!(session.clip_names().is_empty());
}
#[test]
fn failed_stale_cancelled_edit_preserves_source_package_and_existing_player() {
    let cancel = AtomicBool::new(false);
    let mut session = AnimalMotionReviewSession::new(
        ReviewResourceLimits::default(),
        RigLimits::default(),
        MotionLimits::default(),
    )
    .unwrap();
    session.submit(BASE, 0, &cancel).unwrap();
    session.submit(STUDY_EDIT, 1, &cancel).unwrap();
    let digest = Sha256::digest(&session.asset().unwrap().render.bytes);
    let bundle_digest = Sha256::digest(&session.asset().unwrap().bundle);
    let source_digest = session.vibe_source_sha256().unwrap().to_owned();
    let expected = session
        .sample_at("head-look-study", 0.5, &cancel)
        .unwrap()
        .positions
        .to_vec();
    assert!(session.submit(STUDY_EDIT, 1, &cancel).is_err());
    assert!(session
        .submit("species_asset = { unsupported:true };", 2, &cancel)
        .is_err());
    cancel.store(true, Ordering::Release);
    assert!(session.submit(STUDY_EDIT, 2, &cancel).is_err());
    cancel.store(false, Ordering::Release);
    assert_eq!(session.revision(), 2);
    assert_eq!(session.sources().len(), 2);
    assert_eq!(session.vibe_source_sha256(), Some(source_digest.as_str()));
    assert_eq!(
        Sha256::digest(&session.asset().unwrap().render.bytes),
        digest
    );
    assert_eq!(
        Sha256::digest(&session.asset().unwrap().bundle),
        bundle_digest
    );
    assert_eq!(
        session
            .sample_at("head-look-study", 0.5, &cancel)
            .unwrap()
            .positions,
        expected.as_slice()
    );
    // Runtime cancellation withholds a frame and retires its handle, while source
    // revision/package remain intact. A new successful source transaction recovers.
    cancel.store(true, Ordering::Release);
    assert!(session.advance(0., &cancel).is_err());
    cancel.store(false, Ordering::Release);
    assert!(session.sample_at("head-look-study", 0.5, &cancel).is_err());
    assert_eq!(session.revision(), 2);
    session.submit(STUDY_EDIT, 2, &cancel).unwrap();
    assert_eq!(session.revision(), 3);
    assert!(session.sample_at("head-look-study", 0.5, &cancel).is_ok());
}
#[test]
fn hmc_fresh_motion_runtime_preserves_revision_sources_and_editable_clips() {
    let cancel = AtomicBool::new(false);
    let mut session = AnimalMotionReviewSession::new(
        ReviewResourceLimits::default(),
        RigLimits::default(),
        MotionLimits::default(),
    )
    .unwrap();
    session.submit(BASE, 0, &cancel).unwrap();
    session.submit(STUDY_EDIT, 1, &cancel).unwrap();
    let bundle = session.asset().unwrap().bundle.clone();
    let source_digest = session.vibe_source_sha256().unwrap().to_owned();
    let positions = session
        .sample_at("head-look-study", 0.5, &cancel)
        .unwrap()
        .positions
        .to_vec();
    let mut reopened = AnimalMotionReviewSession::reopen(
        &bundle,
        ReviewResourceLimits::default(),
        MotionLimits::default(),
        &cancel,
    )
    .unwrap();
    assert_eq!(reopened.revision(), 2);
    assert_eq!(reopened.sources(), session.sources());
    assert_eq!(reopened.vibe_source_sha256(), Some(source_digest.as_str()));
    assert_eq!(reopened.asset().unwrap().bundle, bundle);
    assert_eq!(
        reopened
            .resources()
            .inventory()
            .into_iter()
            .map(|r| (
                r.key,
                r.kind,
                r.bytes,
                r.digest,
                r.metadata_bytes,
                r.metadata_digest
            ))
            .collect::<Vec<_>>(),
        session
            .resources()
            .inventory()
            .into_iter()
            .map(|r| (
                r.key,
                r.kind,
                r.bytes,
                r.digest,
                r.metadata_bytes,
                r.metadata_digest
            ))
            .collect::<Vec<_>>()
    );
    assert_eq!(
        reopened
            .sample_at("head-look-study", 0.5, &cancel)
            .unwrap()
            .positions,
        positions.as_slice()
    );
    let revised = STUDY_EDIT.replace("id:\"head-look-study\"", "id:\"head-look-revised\"");
    assert_ne!(revised, STUDY_EDIT);
    reopened.submit(&revised, 2, &cancel).unwrap();
    assert_eq!(reopened.revision(), 3);
    assert!(reopened
        .sample_at("head-look-revised", 0.5, &cancel)
        .is_ok());
    cancel.store(true, Ordering::Release);
    assert!(AnimalMotionReviewSession::reopen(
        &bundle,
        ReviewResourceLimits::default(),
        MotionLimits::default(),
        &cancel
    )
    .is_err());
}
