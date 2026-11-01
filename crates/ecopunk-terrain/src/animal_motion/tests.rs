use super::*;
use crate::{
    species_authored_review, species_review_source::SpeciesReviewSourceSession,
    swamp_wallaby_anatomical_r3::Source, wallaby_rig::RigLimits,
};
use qualia_core_db::{
    bundle::BundleReader, container_10d::skeletal_container::read_skeletal_section,
};
use std::sync::atomic::AtomicBool;
const BASE: &str = include_str!(
    "../../../../assets/production-studies/swamp-wallaby-review-r1/authoring/swamp-wallaby-r3.vibe"
);
fn fixture() -> (Source, species_authored_review::AuthoredReviewAsset) {
    let mut session = SpeciesReviewSourceSession::default();
    session
        .submit(BASE, |a| species_authored_review::build(1, a))
        .unwrap();
    session
        .submit(STUDY_EDIT, |a| {
            let source = serde_json::from_value(a.data.clone()).map_err(|e| e.to_string())?;
            Ok((source, species_authored_review::build(1, a)?))
        })
        .unwrap()
}
fn player(source: &Source, asset: &species_authored_review::AuthoredReviewAsset) -> MotionPlayer {
    MotionPlayer::admit(
        &asset.render.bytes,
        source,
        RigLimits::default(),
        MotionLimits::default(),
        &AtomicBool::new(false),
    )
    .unwrap()
}
#[test]
fn named_source_keys_canonical_master_and_reopened_edit() {
    let (source, asset) = fixture();
    let section = read_skeletal_section(&asset.render.bytes).unwrap();
    assert_eq!(section.counts().joints, 21);
    assert_eq!(section.counts().clips, 3);
    assert_eq!(
        section.clip(1).unwrap().id,
        clip_id(&source.identifier, "head-look-study")
    );
    let mut reopened = crate::species_review_reopen::reopen(1, &asset.bundle).unwrap();
    assert_eq!(reopened.session.revision(), 2);
    assert_eq!(reopened.session.sources()[1], STUDY_EDIT);
    let edit = STUDY_EDIT.replace("id:\"head-look-study\"", "id:\"head-look-revised\"");
    assert_ne!(edit, STUDY_EDIT);
    let changed = reopened.session.submit(&edit, 2).unwrap();
    let changed_section = read_skeletal_section(&changed.render.bytes).unwrap();
    assert_eq!(
        changed_section.clip(1).unwrap().id,
        clip_id(&source.identifier, "head-look-revised")
    );
    let reread = crate::species_review_reopen::reopen(1, &changed.bundle).unwrap();
    assert_eq!(reread.session.revision(), 3);
    let b = BundleReader::parse(&changed.bundle).unwrap();
    assert_eq!(
        b.get("review/model.10d"),
        Some(changed.render.bytes.as_slice())
    );
}
#[test]
fn deterministic_seek_head_motion_has_frames_and_resets_displacement() {
    let (source, asset) = fixture();
    let mut p = player(&source, &asset);
    let cancel = AtomicBool::new(false);
    let rest = p
        .sample_at("head-look-study", 0., &cancel)
        .unwrap()
        .positions
        .to_vec();
    let a = p.sample_at("head-look-study", 0.5, &cancel).unwrap();
    let first = a.positions.to_vec();
    assert!(first.iter().zip(&rest).any(|(a, b)| a != b));
    assert_eq!(a.root_delta.delta_translation, [0.; 3]);
    for (n, t) in a.normals.iter().zip(a.tangents) {
        let dot = (0..3).map(|i| n[i] * t[i]).sum::<f32>();
        assert!(dot.abs() < 1e-3);
        assert!(n.iter().chain(t.iter()).all(|v| v.is_finite()));
    }
    p.sample_at("head-look-study", 1.5, &cancel).unwrap();
    assert_eq!(
        p.sample_at("head-look-study", 0.5, &cancel)
            .unwrap()
            .positions,
        first.as_slice()
    );
    let fresh = player(&source, &asset).model_sha256();
    assert_eq!(p.model_sha256(), fresh);
}
#[test]
fn authored_contact_targets_checked_and_root_not_applied_twice() {
    let (source, asset) = fixture();
    let mut p = player(&source, &asset);
    let cancel = AtomicBool::new(false);
    for time in [0., 0.09, 0.179, 1.02, 1.1, 1.2] {
        let f = p.sample_at("hop-step-study", time, &cancel).unwrap();
        assert_eq!(f.contact_count, 2);
        assert!(f.contacts[..f.contact_count]
            .iter()
            .all(|c| c.error_m <= 0.003));
        assert_eq!(f.root_delta.delta_translation, [0.; 3]);
    }
    let start = p
        .sample_at("hop-step-study", 0.2, &cancel)
        .unwrap()
        .positions
        .to_vec();
    let f = p.advance(0.3, &cancel).unwrap();
    assert!(f.root_delta.delta_translation[2] > 0.);
    assert!(f.positions.iter().zip(start).any(|(a, b)| a != &b));
    p.select_clip("head-look-study", 0.25).unwrap();
    assert!(p.advance(0.1, &cancel).is_ok());
}
#[test]
fn source_model_mismatch_invalid_constraints_and_cancellation_refuse() {
    let (mut source, asset) = fixture();
    let cancel = AtomicBool::new(false);
    source.motion.as_mut().unwrap().clips[0].tracks[0].keys[1].rotation_wxyz = [1., 0., 0., 0.];
    assert!(MotionPlayer::admit(
        &asset.render.bytes,
        &source,
        RigLimits::default(),
        MotionLimits::default(),
        &cancel
    )
    .is_err());
    let (source, asset) = fixture();
    let mut p = player(&source, &asset);
    cancel.store(true, std::sync::atomic::Ordering::Release);
    assert!(matches!(
        p.advance(0., &cancel),
        Err(MotionError::Cancelled)
    ));
    cancel.store(false, std::sync::atomic::Ordering::Release);
    assert!(matches!(p.advance(0., &cancel), Err(MotionError::Retired)));
    let mut session = SpeciesReviewSourceSession::default();
    session
        .submit(BASE, |a| species_authored_review::build(1, a))
        .unwrap();
    // Geometric source validation admits authored targets; runtime refuses unreachable
    // residuals instead of publishing a falsely planted frame.
    let edit = STUDY_EDIT.replacen(
        "target_model_m:[-0.155,0,-0.07]",
        "target_model_m:[8,8,8]",
        1,
    );
    assert_ne!(edit, STUDY_EDIT);
    let pair = session
        .submit(&edit, |a| {
            Ok((
                serde_json::from_value::<Source>(a.data.clone()).map_err(|e| e.to_string())?,
                species_authored_review::build(1, a)?,
            ))
        })
        .unwrap();
    let mut p = player(&pair.0, &pair.1);
    assert!(matches!(
        p.sample_at("hop-step-study", 0., &cancel),
        Err(MotionError::Contact)
    ));
    let mut cap = MotionLimits::default();
    cap.max_keys = 1;
    assert!(MotionPlayer::admit(
        &asset.render.bytes,
        &source,
        RigLimits::default(),
        cap,
        &cancel
    )
    .is_err());
}
#[test]
fn invalid_joint_loop_quaternion_and_overlap_are_source_atomic() {
    for edit in [
        STUDY_EDIT.replace("joint_id:\"head\"", "joint_id:\"ear\""),
        STUDY_EDIT.replace("[0.96,0,0.28,0]", "[0.5,0,0.28,0]"),
        STUDY_EDIT.replace("end_s:0.18", "end_s:1.1"),
    ] {
        assert_ne!(edit, STUDY_EDIT);
        let mut session = SpeciesReviewSourceSession::default();
        session
            .submit(BASE, |a| species_authored_review::build(1, a))
            .unwrap();
        assert!(session
            .submit(&edit, |a| species_authored_review::build(1, a))
            .is_err());
        assert_eq!(session.revision(), 1);
    }
}
#[test]
fn inspection_export_retains_master_and_surface_streams_without_top_level_rig() {
    let (source, asset) = fixture();
    let cancel = AtomicBool::new(false);
    let mut p = player(&source, &asset);
    let f = p.sample_at("head-look-study", 0.5, &cancel).unwrap();
    let pose = export_frame_10d(
        &asset.render.bytes,
        &f,
        PoseExportLimits::default(),
        &cancel,
    )
    .unwrap();
    let h = qualia_core_db::container_10d::Container10dHeader::parse(&pose.bytes).unwrap();
    let sections = qualia_core_db::container_10d::parse_section_table(&pose.bytes, &h).unwrap();
    assert!(!sections
        .iter()
        .any(|d| d.typ() == Some(qualia_core_db::container_10d::SectionType::SkeletalAnimation)));
    let base_h =
        qualia_core_db::container_10d::Container10dHeader::parse(&asset.render.bytes).unwrap();
    let base =
        qualia_core_db::container_10d::parse_section_table(&asset.render.bytes, &base_h).unwrap();
    for kind in [
        qualia_core_db::container_10d::SectionType::Materials,
        qualia_core_db::container_10d::SectionType::TextureCoordinates,
        qualia_core_db::container_10d::SectionType::SurfaceReading,
    ] {
        let a = base.iter().find(|d| d.typ() == Some(kind)).unwrap();
        let b = sections.iter().find(|d| d.typ() == Some(kind)).unwrap();
        assert_eq!(
            &asset.render.bytes[a.byte_offset as usize..(a.byte_offset + a.byte_length) as usize],
            &pose.bytes[b.byte_offset as usize..(b.byte_offset + b.byte_length) as usize]
        );
    }
    let d = sections
        .iter()
        .find(|d| d.typ() == Some(qualia_core_db::container_10d::SectionType::ProvenanceSidecar))
        .unwrap();
    let v = qualia_core_db::container_10d::provenance_section::decode_provenance_section(
        &pose.bytes[d.byte_offset as usize..(d.byte_offset + d.byte_length) as usize],
    )
    .unwrap();
    assert_eq!(v.source_bytes(), asset.render.bytes.as_slice());
    assert!(v.vc().is_none());
    let receipt: serde_json::Value = ciborium::from_reader(&v.semantic_metadata()[3..]).unwrap();
    assert_eq!(receipt["clip_id"].as_u64(), Some(f.receipt.clip_id));
    assert_eq!(receipt["time_s"].as_f64(), Some(0.5));
    assert_eq!(
        receipt["contacts"].as_array().unwrap().len(),
        f.contact_count
    );
    assert!(export_frame_10d(
        &asset.render.bytes,
        &f,
        PoseExportLimits {
            max_container_bytes: 1,
            ..Default::default()
        },
        &cancel
    )
    .is_err());
}
#[test]
fn metadata_and_contact_copies_are_reserved_before_compiler_allocation() {
    let (source, _) = fixture();
    let motion = source.motion.as_ref().unwrap();
    let tracks = motion.clips.iter().map(|c| c.tracks.len()).sum::<usize>();
    let keys = motion
        .clips
        .iter()
        .flat_map(|c| &c.tracks)
        .map(|t| t.keys.len())
        .sum::<usize>();
    // This cap passed the former compact-array-only estimate but cannot retain
    // the simultaneously live scoped metadata and contact copies as well.
    let old_array_only = keys * 160 + tracks * 128;
    let mut cap = MotionLimits::default();
    cap.cold_bytes = old_array_only + 1;
    assert!(validation::validate(&source, motion, cap).is_err());
    assert!(validation::validate(&source, motion, MotionLimits::default()).is_ok());
}
