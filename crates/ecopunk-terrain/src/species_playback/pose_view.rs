//! Borrow the admitted player at the existing cursor for sound/host projections.
use super::*;
use crate::animal_motion::MotionFrame;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaybackPoseContext {
    pub model_sha256: [u8; 32],
    pub source_sha256: [u8; 32],
    pub source_generation: u64,
    pub transport_epoch: u64,
    pub frame_sequence: u64,
    pub clip_id: u64,
    pub timeline_tick: time::Tick,
    pub clip_tick: time::Tick,
    pub rate: PlaybackRate,
    pub playing: bool,
    pub looping: bool,
}
impl SpeciesPlaybackSession {
    /// An audio/headless consumer may be the transport owner. Prepare and
    /// publish through the callback; commit the SAME cursor only on acceptance.
    /// Render/DSP/device consumers must retire any partially prepared epoch on error.
    pub fn advance_with_motion_pose<T>(&mut self, elapsed: time::Tick,
        prepare: impl for<'a> FnOnce(&MotionFrame<'a>, PlaybackPoseContext) -> Result<T, String>,
        cancel: &AtomicBool) -> Result<T, String> {
        if cancel.load(Ordering::Acquire) || self.runtime_retired() {
            return Err("Motion consumer is retired or cancelled".into());
        }
        let mut context = self.pose_context()?;
        let timeline = self.timeline.as_ref().ok_or("No playback timeline")?;
        let plan = timeline.plan(elapsed).map_err(str::to_owned)?;
        context.frame_sequence = plan.sequence;
        context.timeline_tick = plan.timeline;
        context.clip_tick = plan.position;
        context.playing = plan.playing;
        let seconds = timeline.sample_seconds(plan);
        let motion = self.session.sample_at_checked(timeline.clip(), seconds, cancel)?;
        let accepted = prepare(&motion, context)?;
        // Publication has already been accepted. Do not return a cancellation
        // error after that commit point while pretending nothing was delivered.
        self.timeline.as_mut().expect("Timeline retained").commit(plan);
        Ok(accepted)
    }
    pub fn pose_context(&self) -> Result<PlaybackPoseContext, String> {
        let (model_sha256, source_sha256) = self.session.runtime_binding()
            .ok_or("No admitted source motion")?;
        let timeline = self.timeline.as_ref().ok_or("No playback timeline")?;
        let clip_id = self.clip_descriptor(timeline.clip()).ok_or("No admitted selected clip")?.id;
        Ok(PlaybackPoseContext {
            model_sha256, source_sha256, source_generation: self.generation,
            transport_epoch: timeline.epoch(), frame_sequence: timeline.sequence(),
            clip_id,
            timeline_tick: timeline.timeline(), clip_tick: timeline.position(),
            rate: timeline.rate(), playing: timeline.playing(), looping: timeline.looping(),
        })
    }
    pub fn pose_context_is_current(&self, context: PlaybackPoseContext) -> bool {
        !self.runtime_retired() && self.pose_context().is_ok_and(|now| now == context)
    }
    /// No second player or cursor advance. The callback may prepare caller-owned
    /// data, but must not enqueue it: native authority is rechecked at publication.
    /// A returned token is only usable while this complete context remains current.
    pub fn with_current_motion_pose<T>(&mut self, expected: PlaybackPoseContext,
        prepare: impl for<'a> FnOnce(&MotionFrame<'a>, PlaybackPoseContext) -> Result<T, String>,
        cancel: &AtomicBool) -> Result<T, String> {
        if cancel.load(Ordering::Acquire) || !self.pose_context_is_current(expected) {
            return Err("Motion pose projection is stale or cancelled".into());
        }
        let timeline = self.timeline.as_ref().ok_or("No playback timeline")?;
        let plan = timeline.plan(time::Tick::ZERO).map_err(str::to_owned)?;
        let seconds = timeline.sample_seconds(plan);
        let motion = self.session.sample_at_checked(timeline.clip(), seconds, cancel)?;
        let prepared = prepare(&motion, expected)?;
        if cancel.load(Ordering::Acquire) { return Err("Motion pose projection cancelled".into()); }
        Ok(prepared)
    }
}
