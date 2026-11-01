//! Same-player motion attachment projection into the existing Core PCM field.
//! The host supplies source-authorized anchors and already decoded PCM. No
//! second transport, player, media admission, codec or device queue lives here.
use super::{PlaybackRate, PlaybackPoseContext, SpeciesPlaybackSession, time::{Tick, TICKS_PER_SECOND}};
use crate::animal_motion::MotionFrame;
use qualia_core_db::{audio::scene_audio_graph::{AudioListenerPose, SceneAudioBusGraph,
    AcousticObstacle, MAX_SCENE_AUDIO_EMITTERS,
    pcm_schedule::{ClipScheduleBudget, ClipScheduleState, ClipScheduleWorkspace, PcmClip},
    source_coupling::{self, EmitterPose, SoundBinding, SoundBlockMode, SoundBlockReceipt,
        SoundError, SoundPose, SoundScopeState}},
    render::{source_frame::SourceFrameBinding, skeletal::Motor}};
#[path="sound_tests.rs"]
#[cfg(test)]
mod tests;

pub const PCM_RATE: u32 = 48_000;
pub fn validate_rate(rate: PlaybackRate) -> Result<(), SoundError> {
    rate.validate().map_err(|_| SoundError::UnsupportedRate)?;
    if rate.numerator != rate.denominator { return Err(SoundError::UnsupportedRate); }
    Ok(())
}
pub fn exact_source_frame(tick: Tick) -> Result<u64, SoundError> {
    if tick.0<0 { return Err(SoundError::Clock); }
    let value=u128::try_from(tick.0).map_err(|_| SoundError::Clock)?
        .checked_mul(u128::from(PCM_RATE)).ok_or(SoundError::Clock)?;
    if value % (TICKS_PER_SECOND as u128) != 0 { return Err(SoundError::Clock); }
    u64::try_from(value/(TICKS_PER_SECOND as u128)).map_err(|_| SoundError::Clock)
}

impl SpeciesPlaybackSession {
    /// Preflight the coupled profile before any transport/epoch mutation.
    pub fn set_rate_with_sound(&mut self, rate: PlaybackRate) -> Result<(), String> {
        validate_rate(rate).map_err(|e| e.to_string())?;
        self.set_rate(rate)
    }
    pub fn seek_with_sound(&mut self, tick: Tick) -> Result<(), String> {
        exact_source_frame(tick).map_err(|e| e.to_string())?;
        self.seek(tick)
    }
    pub fn select_clip_with_sound(&mut self, clip: &str) -> Result<(), String> {
        let descriptor = self.clip_descriptor(clip).ok_or("Requested sound-coupled clip is not authored")?;
        let duration = super::time::seconds_to_tick(descriptor.duration_s).map_err(str::to_owned)?;
        exact_source_frame(duration).map_err(|e| e.to_string())?;
        self.select_clip(clip)
    }
}

#[derive(Clone, Copy)]
pub enum Anchor {
    WorldMeters([f64;3]),
    Joint { joint_identifier:u64, local_m:[f64;3] },
}
#[derive(Clone, Copy)]
pub struct EmitterAnchor {
    pub node_id:u32,
    pub origin_quin_hash:u64,
    pub anchor:Anchor,
}
#[derive(Clone, Copy)]
pub enum ListenerAnchor {
    World(AudioListenerPose),
    Joint { joint_identifier:u64, local_m:[f64;3] },
}

fn attachment(motion:&MotionFrame<'_>, identifier:u64) -> Result<Motor,SoundError> {
    let mut found=None;
    for (i,&id) in motion.joint_ids.iter().enumerate() {
        if id==identifier {
            if found.is_some() { return Err(SoundError::Pose); }
            found=Some(motion.world_motors[i]);
        }
    }
    found.ok_or(SoundError::Pose)
}
fn point(p:[f64;3]) -> Result<[f32;3],SoundError> {
    if p.iter().any(|v| !v.is_finite() || v.abs()>1_000_000.) { return Err(SoundError::Pose); }
    Ok(p.map(|v| v as f32))
}
fn joint_position(motion:&MotionFrame<'_>, id:u64, local:[f64;3], model_to_world:Motor)
    -> Result<([f32;3],Motor),SoundError> {
    if local.iter().any(|v| !v.is_finite() || v.abs()>1_000_000.) { return Err(SoundError::Pose); }
    let motor=model_to_world.mul(&attachment(motion,id)?);
    Ok((point(motor.transform_point(local))?,motor))
}
fn direction(motor:Motor, axis:[f64;3]) -> Result<[f32;3],SoundError> {
    let origin=motor.transform_point([0.;3]);
    let tip=motor.transform_point(axis);
    let d=std::array::from_fn::<_,3,_>(|i| tip[i]-origin[i]);
    let length=d.iter().map(|v| v*v).sum::<f64>().sqrt();
    if !length.is_finite() || (length-1.).abs()>0.02 { return Err(SoundError::Pose); }
    Ok(d.map(|v| (v/length) as f32))
}

/// Zero-order-held pose for one bounded audio block. The borrowed world motors
/// are the SAME post-contact/IK evaluation as the native skin frame.
pub fn prepare_pose(motion:&MotionFrame<'_>, visual:SourceFrameBinding, binding:SoundBinding,
    context:PlaybackPoseContext, anchors:&[EmitterAnchor], listener:ListenerAnchor,
    model_to_world:Motor, max_joint_visits:usize) -> Result<SoundPose,SoundError> {
    if visual.model_sha256!=binding.model_sha256 || visual.source_sha256!=binding.source_sha256
        || visual.source_generation!=binding.source_generation || visual.transport_epoch!=binding.transport_epoch
        || visual.mesh_generation==0 {return Err(SoundError::Binding);}
    prepare_pose_for_source(motion,binding,context,anchors,listener,model_to_world,max_joint_visits)
}

/// Headless audio uses the same admitted source pose; no fabricated renderer
/// binding or visible-frame claim is required to prepare an acoustic block.
pub fn prepare_pose_for_source(motion:&MotionFrame<'_>, binding:SoundBinding,
    context:PlaybackPoseContext, anchors:&[EmitterAnchor], listener:ListenerAnchor,
    model_to_world:Motor, max_joint_visits:usize) -> Result<SoundPose,SoundError> {
    binding.validate()?;
    if motion.model_sha256!=binding.model_sha256 || motion.source_sha256!=binding.source_sha256
        || motion.joint_ids.len()!=motion.world_motors.len()
        || motion.receipt.joints!=motion.joint_ids.len() || !motion.receipt.time.is_finite()
        || context.model_sha256!=binding.model_sha256 || context.source_sha256!=binding.source_sha256
        || context.source_generation!=binding.source_generation || context.transport_epoch!=binding.transport_epoch
        || context.timeline_tick.0<0 || context.frame_sequence==0
        || motion.receipt.clip_id!=context.clip_id { return Err(SoundError::Binding); }
    validate_rate(context.rate)?;
    if super::time::sample_seconds_to_tick(motion.receipt.time).map_err(|_|SoundError::Clock)? != context.clip_tick {
        return Err(SoundError::Clock);
    }
    if anchors.len()>MAX_SCENE_AUDIO_EMITTERS { return Err(SoundError::Budget); }
    let visits=anchors.len().checked_add(1).and_then(|n| n.checked_mul(motion.joint_ids.len()))
        .ok_or(SoundError::Budget)?;
    if visits>max_joint_visits { return Err(SoundError::Budget); }
    let mut emitters=[EmitterPose::default();MAX_SCENE_AUDIO_EMITTERS];
    for (slot,a) in anchors.iter().enumerate() {
        let position_m=match a.anchor {
            Anchor::WorldMeters(p)=>point(p)?,
            Anchor::Joint {joint_identifier,local_m}=>joint_position(motion,joint_identifier,local_m,model_to_world)?.0,
        };
        emitters[slot]=EmitterPose {node_id:a.node_id,origin_quin_hash:a.origin_quin_hash,position_m};
    }
    let listener=match listener {
        ListenerAnchor::World(p)=>p,
        ListenerAnchor::Joint {joint_identifier,local_m}=> {
            let (position,motor)=joint_position(motion,joint_identifier,local_m,model_to_world)?;
            let forward=direction(motor,[0.,0.,-1.])?;
            let right=direction(motor,[1.,0.,0.])?;
            let up=direction(motor,[0.,1.,0.])?;
            if forward[0]*forward[0]+forward[2]*forward[2]<1e-8 { return Err(SoundError::Pose); }
            AudioListenerPose {position,forward,right,up,yaw_rad:forward[0].atan2(-forward[2]),
                velocity:[0.;3],..Default::default()}
        }
    };
    Ok(SoundPose {binding,pose_sequence:context.frame_sequence,pose_tick:context.timeline_tick.0 as u64,listener,
        emitters,emitter_count:anchors.len()})
}

/// Output-device frame index is a separate explicit value. Never floor a seek
/// onto a sample; unit-rate admission must happen before shared rate mutation.
pub fn render_at_tick(graph:&mut SceneAudioBusGraph, scope:&mut SoundScopeState,
    state:&mut ClipScheduleState<'_>,workspace:&mut ClipScheduleWorkspace<'_>,clips:&[PcmClip<'_>],
    pose:&SoundPose, source_tick:Tick, first_output_frame:u64, mode:SoundBlockMode,rate:PlaybackRate,
    obstacles:&[AcousticObstacle],budget:ClipScheduleBudget,l:&mut[f32],r:&mut[f32],
    live:&dyn Fn(SoundBinding)->bool) -> Result<SoundBlockReceipt,SoundError> {
    validate_rate(rate)?;
    let start=exact_source_frame(source_tick)?;
    if mode==SoundBlockMode::Playing && pose.pose_tick!=source_tick.0 as u64
        || mode==SoundBlockMode::NaturalTail && pose.pose_tick>source_tick.0 as u64 {
        return Err(SoundError::Clock);
    }
    source_coupling::render_guarded(graph,scope,state,workspace,clips,pose,start,first_output_frame,
        mode,obstacles,budget,l,r,live)
}
