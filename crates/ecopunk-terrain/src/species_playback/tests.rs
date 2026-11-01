//! Meaningful source regressions; execution intentionally deferred until coding
//! is complete. No test result is implied by this module's presence.
use super::*;
use time::{Tick, TICKS_PER_SECOND, FrameRate};

fn descriptor(looping: bool) -> crate::animal_motion::MotionClipDescriptor<'static> {
    crate::animal_motion::MotionClipDescriptor { id: 42, name: "fixture", duration_s: 2., looping }
}
#[test]
fn exact_media_clocks_refuse_rounding_and_overflow() {
    assert_eq!(time::frame_tick(24, FrameRate::FPS_24).unwrap(), Tick(TICKS_PER_SECOND));
    assert_eq!(time::sample_tick(48_000, 48_000).unwrap(), Tick(TICKS_PER_SECOND));
    assert_eq!(time::frame_tick(24_000, FrameRate::FPS_23_976).unwrap(), Tick(1001 * TICKS_PER_SECOND));
    assert!(time::frame_tick(1, FrameRate { num: 11, den: 1 }).is_err());
    assert!(time::frame_tick(i64::MAX, FrameRate::FPS_24).is_err());
    assert!(time::frame_tick(-1, FrameRate::FPS_24).is_err());
    assert!(time::sample_tick(1, 0).is_err());
    assert!(time::sample_tick(1, 47_999).is_err());
}
#[test]
fn fractional_rate_and_clock_plan_commit_have_no_float_drift() {
    let mut clock = PlaybackTimeline::new(descriptor(true)).unwrap();
    clock.set_rate(PlaybackRate { numerator: 1, denominator: 3 }).unwrap();
    clock.play().unwrap();
    // Plans are detached. A rejected publication need not promote playhead.
    let _discarded = clock.plan(Tick(1)).unwrap();
    assert_eq!(clock.position(), Tick::ZERO);
    for _ in 0..3 { let plan = clock.plan(Tick(1)).unwrap(); clock.commit(plan); }
    assert_eq!(clock.position(), Tick(1));
    assert_eq!(clock.timeline(), Tick(1));
    let epoch = clock.epoch();
    clock.seek(Tick(TICKS_PER_SECOND)).unwrap();
    assert!(clock.epoch() > epoch);
    let before = (clock.position(), clock.epoch());
    assert!(clock.seek(Tick(-1)).is_err());
    assert_eq!((clock.position(), clock.epoch()), before);
}
#[test]
fn loop_and_end_policy_are_explicit_and_pause_does_not_advance() {
    let mut clock = PlaybackTimeline::new(descriptor(false)).unwrap();
    assert!(clock.set_looping(true).is_err());
    clock.play().unwrap();
    for _ in 0..2 { let plan = clock.plan(Tick(TICKS_PER_SECOND)).unwrap(); clock.commit(plan); }
    assert_eq!(clock.position(), clock.duration());
    assert!(!clock.playing());
    assert!(clock.play().is_err());
    clock.seek(Tick::ZERO).unwrap();
    clock.play().unwrap(); clock.pause().unwrap();
    let plan = clock.plan(Tick(TICKS_PER_SECOND)).unwrap(); clock.commit(plan);
    assert_eq!(clock.position(), Tick::ZERO);
    assert!(clock.plan(Tick(TICKS_PER_SECOND + 1)).is_err());
}
#[test]
fn stopping_a_loop_preserves_editorial_time_and_pause_preserves_fraction() {
    let mut clock = PlaybackTimeline::new(descriptor(true)).unwrap();
    clock.play().unwrap();
    for _ in 0..5 { let p=clock.plan(Tick(TICKS_PER_SECOND)).unwrap(); clock.commit(p); }
    assert_eq!(clock.position(), Tick(TICKS_PER_SECOND));
    clock.set_looping(false).unwrap();
    let p=clock.plan(Tick(TICKS_PER_SECOND)).unwrap(); clock.commit(p);
    assert_eq!(clock.position(), Tick(2*TICKS_PER_SECOND));
    assert_eq!(clock.timeline(), Tick(6*TICKS_PER_SECOND));
    assert!(!clock.playing());
    clock.seek(Tick::ZERO).unwrap();
    clock.set_rate(PlaybackRate { numerator:1, denominator:3 }).unwrap();
    clock.play().unwrap();
    let p=clock.plan(Tick(2)).unwrap(); clock.commit(p);
    clock.pause().unwrap();
    let p=clock.plan(Tick(100)).unwrap(); clock.commit(p);
    clock.play().unwrap();
    let p=clock.plan(Tick(1)).unwrap(); clock.commit(p);
    assert_eq!(clock.position(), Tick(1));
}
#[test]
fn invalid_ui_clip_and_time_keep_existing_player_recoverable() {
    const BASE: &str = include_str!("../../../../assets/production-studies/swamp-wallaby-review-r1/authoring/swamp-wallaby-r3.vibe");
    let cancel = AtomicBool::new(false);
    let mut session = AnimalMotionReviewSession::new(ReviewResourceLimits::default(),
        RigLimits::default(), MotionLimits::default()).unwrap();
    session.submit(BASE, 0, &cancel).unwrap();
    session.submit(crate::animal_motion::STUDY_EDIT, 1, &cancel).unwrap();
    let binding = session.runtime_binding();
    assert!(session.select_clip_checked("unavailable-behaviour", 0.).is_err());
    assert!(session.sample_at_checked("head-look-study", f64::NAN, &cancel).is_err());
    assert!(!session.runtime_retired());
    assert_eq!(session.runtime_binding(), binding);
    let before = session.sample_at_checked("head-look-study", 0.5, &cancel).unwrap().positions.to_vec();
    cancel.store(true, Ordering::Release);
    assert!(session.sample_at_checked("head-look-study", 0.5, &cancel).is_err());
    cancel.store(false, Ordering::Release);
    assert_eq!(session.sample_at_checked("head-look-study", 0.5, &cancel).unwrap().positions, before);
}

#[test]
fn changed_transport_epoch_requires_same_time_pose_and_fresh_owner_receipt() {
    use qualia_core_db::render::source_frame::{SourceFrameBinding,SourceFrameState,SourceMeshFrame,SourceFrameError};
    let mut clock=PlaybackTimeline::new(descriptor(true)).unwrap();
    let mut resident=SourceFrameState::default();
    let mut binding=SourceFrameBinding{model_sha256:[1;32],source_sha256:[2;32],
        source_generation:1,mesh_generation:1,transport_epoch:clock.epoch(),vertex_count:3,index_count:3};
    resident.bind(binding).unwrap();
    let positions=[[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]];
    let normals=[[0.,0.,1.];3];let tangents=[[1.,0.,0.,1.];3];
    let first=clock.plan(Tick::ZERO).unwrap();
    let frame=SourceMeshFrame{binding,frame_sequence:first.sequence,timeline_tick:0,
        positions:&positions,normals:&normals,tangents:&tangents};
    let (next,_)=resident.prepare(&frame,1).unwrap();resident=next;clock.commit(first);
    let commands:[fn(&mut PlaybackTimeline)->Result<(),&'static str>;4]=[
        PlaybackTimeline::play,PlaybackTimeline::pause,
        |t|t.set_rate(PlaybackRate{numerator:2,denominator:1}),|t|t.set_looping(false),
    ];
    for command in commands {
        assert!(!clock.plan(Tick::ZERO).unwrap().required);
        let before=(clock.epoch(),clock.sequence(),clock.position(),clock.timeline());
        command(&mut clock).unwrap();
        let plan=clock.plan(Tick::ZERO).unwrap();
        assert!(plan.required);assert_eq!(plan.sequence,before.1+1);
        assert_eq!(clock.epoch(),before.0+1);assert_eq!(plan.position,before.2);assert_eq!(plan.timeline,before.3);
        let stale=binding;
        binding=resident.advance_epoch(binding,clock.epoch()).unwrap();
        let old=SourceMeshFrame{binding:stale,frame_sequence:plan.sequence,timeline_tick:plan.timeline.0 as u64,
            positions:&positions,normals:&normals,tangents:&tangents};
        assert!(matches!(resident.prepare(&old,1),Err(SourceFrameError::Stale)));
        let frame=SourceMeshFrame{binding,..old};
        let (next,receipt)=resident.prepare(&frame,1).unwrap();
        assert_eq!(receipt.binding.transport_epoch,clock.epoch());assert_eq!(receipt.frame_sequence,plan.sequence);
        resident=next;clock.commit(plan);
        // Repeating an already-selected setting must not invent an epoch/pose.
        command(&mut clock).unwrap();
        assert_eq!(clock.epoch(),before.0+1);assert!(!clock.plan(Tick::ZERO).unwrap().required);
    }
}

#[test]
fn rejected_transport_commands_do_not_dirty_a_clean_pose() {
    let mut clock=PlaybackTimeline::new(descriptor(false)).unwrap();
    let p=clock.plan(Tick::ZERO).unwrap();clock.commit(p);
    let before=(clock.epoch(),clock.sequence(),clock.position(),clock.rate());
    assert!(clock.set_rate(PlaybackRate{numerator:0,denominator:1}).is_err());
    assert!(clock.set_looping(true).is_err());
    assert_eq!((clock.epoch(),clock.sequence(),clock.position(),clock.rate()),before);
    assert!(!clock.plan(Tick::ZERO).unwrap().required);
    clock.seek(clock.duration()).unwrap();let p=clock.plan(Tick::ZERO).unwrap();clock.commit(p);
    let epoch=clock.epoch();assert!(clock.play().is_err());assert_eq!(clock.epoch(),epoch);
    assert!(!clock.plan(Tick::ZERO).unwrap().required);
}
