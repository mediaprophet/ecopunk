//! Clock/profile regressions; authored only, not executed in this coding wave.
use super::*;
#[test]
fn sound_seeks_use_exact_native_ticks_and_never_floor() {
    assert_eq!(exact_source_frame(Tick(5_292_000)),Ok(1));
    assert_eq!(exact_source_frame(Tick(TICKS_PER_SECOND)),Ok(48_000));
    assert_eq!(exact_source_frame(Tick(5_291_999)),Err(SoundError::Clock));
    assert_eq!(exact_source_frame(Tick(-1)),Err(SoundError::Clock));
    assert_eq!(validate_rate(PlaybackRate {numerator:2,denominator:2}),Ok(()));
    assert_eq!(validate_rate(PlaybackRate {numerator:2,denominator:1}),Err(SoundError::UnsupportedRate));
}
#[test]
fn attachment_pose_must_match_the_actual_sampled_clip_and_cursor() {
    use qualia_core_db::render::skeletal::{PlaybackReceipt, RootMotionDelta};
    let ids=[11]; let motors=[Motor::identity()];
    let motion=MotionFrame {receipt:PlaybackReceipt {clip_id:7,time:0.,vertices:0,joints:1,reduced_motion:false},
        positions:&[],normals:&[],tangents:&[],joint_ids:&ids,world_motors:&motors,
        contacts:[crate::animal_motion::ContactReceipt::default();crate::animal_motion::MAX_CONTACTS],
        contact_count:0,root_delta:RootMotionDelta::default(),model_sha256:[1;32],source_sha256:[2;32]};
    let binding=SoundBinding {model_sha256:[1;32],source_sha256:[2;32],graph_sha256:[3;32],
        source_generation:1,graph_generation:1,transport_epoch:2};
    let visual=SourceFrameBinding {model_sha256:[1;32],source_sha256:[2;32],source_generation:1,
        mesh_generation:1,transport_epoch:2,vertex_count:0,index_count:0};
    let context=PlaybackPoseContext {model_sha256:[1;32],source_sha256:[2;32],source_generation:1,
        transport_epoch:2,frame_sequence:1,clip_id:7,timeline_tick:Tick(5_292_000),clip_tick:Tick(5_292_000),
        rate:PlaybackRate::default(),playing:true,looping:false};
    assert!(matches!(prepare_pose(&motion,visual,binding,context,&[],
        ListenerAnchor::World(AudioListenerPose::default()),Motor::identity(),1),Err(SoundError::Clock)));
    assert!(prepare_pose(&motion,visual,binding,PlaybackPoseContext {clip_tick:Tick::ZERO,..context},&[],
        ListenerAnchor::World(AudioListenerPose::default()),Motor::identity(),1).is_ok());
}
