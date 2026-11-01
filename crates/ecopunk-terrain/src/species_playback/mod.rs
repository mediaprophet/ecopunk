//! Source-owned native production playback. Geometry/contacts stay in Core's
//! MotionPlayer; Q42/HMC and source transactions retain their existing owners.
pub mod time;
pub mod sound;
mod transport;
pub mod frame_bridge;
mod presentation;
mod pose_view;
pub use pose_view::PlaybackPoseContext;
mod projection_budget;
pub use projection_budget::NativeProjectionBudget;
#[cfg(test)]
mod tests;
pub use transport::{PlaybackRate, PlaybackTimeline};
use crate::{
    animal_motion::{MotionLimits, MotionPlayer},
    animal_motion_review::AnimalMotionReviewSession,
    species_authored_review::AuthoredReviewAsset,
    species_review_resources::{CorpusOptions, ReviewResourceLimits, ReviewResources},
    wallaby_rig::RigLimits,
};
use std::sync::atomic::{AtomicBool, Ordering};

/// The controller's transport is ephemeral view state. No direct mutable access
/// to the source session is exposed, so a host cannot edit behind its binding.
pub struct SpeciesPlaybackSession {
    session: AnimalMotionReviewSession,
    timeline: Option<PlaybackTimeline>,
    generation: u64,
    resource_limits: ReviewResourceLimits,
}
impl SpeciesPlaybackSession {
    pub fn new(resources: ReviewResourceLimits, rig: RigLimits, motion: MotionLimits) -> Result<Self, String> {
        Ok(Self { session: AnimalMotionReviewSession::new(resources, rig, motion)?,
            timeline: None, generation: 0, resource_limits: resources })
    }
    pub fn revision(&self) -> u64 { self.session.revision() }
    pub fn source_generation(&self) -> u64 { self.generation }
    pub fn asset(&self) -> Option<&AuthoredReviewAsset> { self.session.asset() }
    pub fn resources(&self) -> &ReviewResources { self.session.resources() }
    pub fn source_cells(&self) -> &[String] { self.session.sources() }
    pub fn timeline(&self) -> Option<&PlaybackTimeline> { self.timeline.as_ref() }
    pub fn clip_names(&self) -> Vec<&str> { self.session.clip_names() }
    pub fn clip_descriptor(&self, clip: &str) -> Option<crate::animal_motion::MotionClipDescriptor<'_>> {
        self.session.clip_descriptor(clip)
    }
    pub fn runtime_retired(&self) -> bool { self.session.runtime_retired() }
    pub fn runtime_binding(&self) -> Option<([u8; 32], [u8; 32])> {
        self.session.runtime_binding()
    }
    pub fn vibe_source_sha256(&self) -> Option<&str> { self.session.vibe_source_sha256() }
    /// Disposable playback from an immutable native document snapshot. Native
    /// source revision, undo, authority and save remain the caller's owners.
    pub fn from_authoritative_source_cells(cells: &[String], resources: ReviewResources,
        resource_limits: ReviewResourceLimits, rig: RigLimits, motion: MotionLimits,
        options: &CorpusOptions<'_>, budget: NativeProjectionBudget,
        cancel: &AtomicBool) -> Result<Self, String> {
        Self::from_authoritative_source_cells_at_generation(cells, resources,
            resource_limits, rig, motion, options, 1, budget, cancel)
    }
    /// A long-lived host reserves a monotonically increasing generation across
    /// projection replacement/retirement, including identical-source undo/reopen.
    pub fn from_authoritative_source_cells_at_generation(cells: &[String], resources: ReviewResources,
        resource_limits: ReviewResourceLimits, rig: RigLimits, motion: MotionLimits,
        options: &CorpusOptions<'_>, generation: u64, budget: NativeProjectionBudget,
        cancel: &AtomicBool) -> Result<Self, String> {
        if generation == 0 { return Err("Native projection generation must be nonzero".into()); }
        if cancel.load(Ordering::Acquire) { return Err("Native source projection cancelled".into()); }
        budget.admit(cells, &resources, resource_limits, rig, motion, options)?;
        let session = AnimalMotionReviewSession::from_authoritative_source_cells(
            cells, resources, resource_limits, rig, motion, options, cancel)?;
        let timeline = if let Some(name) = session.clip_names().first() {
            Some(PlaybackTimeline::new(session.clip_descriptor(name).ok_or("Native clip missing")?)
                .map_err(str::to_owned)?)
        } else { None };
        Ok(Self { session, timeline, generation, resource_limits })
    }
    pub fn submit(&mut self, source: &str, expected_revision: u64,
        options: &CorpusOptions<'_>, cancel: &AtomicBool) -> Result<&AuthoredReviewAsset, String> {
        if self.revision() != expected_revision || cancel.load(Ordering::Acquire) {
            return Err("Playback source request is stale or cancelled".into());
        }
        let generation = self.generation.checked_add(1).ok_or("Source generation exhausted")?;
        let retained = crate::species_review_resources::copy_retained_corpus_inputs(
            self.session.resources(), self.resource_limits, options.knowledge)?;
        let limits = self.resource_limits;
        let timeline = self.session.submit_with_authored_resources_prepared(source, expected_revision,
            |authored, asset| crate::species_review_resources::generate_corpus_resources(
                1, authored, asset, retained, limits, options, cancel),
            Self::prepare_timeline, cancel)?;
        // Only infallible assignments follow source/resource/player promotion.
        self.timeline = timeline;
        self.generation = generation;
        Ok(self.session.asset().expect("Source/player candidate was promoted together"))
    }
    fn prepare_timeline(player: Option<&MotionPlayer>) -> Result<Option<PlaybackTimeline>, String> {
        let Some(player) = player else { return Ok(None); };
        let name = player.clip_names().next().ok_or("Admitted player has no authored clips")?;
        let descriptor = player.clip_descriptor(name).ok_or("Admitted clip descriptor missing")?;
        PlaybackTimeline::new(descriptor).map(Some).map_err(str::to_owned)
    }
    pub fn reopen(bytes: &[u8], resources: ReviewResourceLimits, motion: MotionLimits,
        cancel: &AtomicBool) -> Result<Self, String> {
        Self::reopen_with_knowledge(bytes, resources, motion,
            crate::species_knowledge::KnowledgeLimits::default(), cancel)
    }
    pub fn reopen_with_knowledge(bytes: &[u8], resources: ReviewResourceLimits, motion: MotionLimits,
        knowledge: crate::species_knowledge::KnowledgeLimits, cancel: &AtomicBool) -> Result<Self, String> {
        Self::reopen_at_generation(bytes,resources,motion,knowledge,1,cancel)
    }
    pub fn reopen_at_generation(bytes:&[u8],resources:ReviewResourceLimits,motion:MotionLimits,
        knowledge:crate::species_knowledge::KnowledgeLimits,generation:u64,cancel:&AtomicBool)->Result<Self,String> {
        if generation==0 {return Err("Reopen generation must be nonzero".into());}
        let session = AnimalMotionReviewSession::reopen(bytes, resources, motion, cancel)?;
        let history_sha = session.vibe_source_sha256().ok_or("Restored source history missing")?;
        let key = crate::species_knowledge::authored_resource_key(session.revision(), history_sha)?;
        let graph = session.resources().entries().iter().find(|r| r.key == key)
            .ok_or("Current authored Q42 snapshot is missing; use the legacy inspector for an older study")?;
        crate::species_knowledge::validate_authored_binding(graph, history_sha, session.revision(),
            &session.asset().ok_or("Restored model missing")?.render.bytes, knowledge, cancel)?;
        // Reopen restores source/resources and creates a fresh paused view.
        // It does not silently restore an old device or active playback job.
        let timeline = if let Some(name) = session.clip_names().first() {
            Some(PlaybackTimeline::new(session.clip_descriptor(name).ok_or("Restored clip missing")?)
                .map_err(str::to_owned)?)
        } else { None };
        Ok(Self { session, timeline, generation, resource_limits: resources })
    }
    pub fn replace_from_bundle(&mut self, bytes: &[u8], motion: MotionLimits,
        knowledge: crate::species_knowledge::KnowledgeLimits, cancel: &AtomicBool) -> Result<(), String> {
        let generation = self.generation.checked_add(1).ok_or("Source generation exhausted")?;
        let mut next = Self::reopen_with_knowledge(bytes, self.resource_limits, motion, knowledge, cancel)?;
        next.generation = generation;
        *self = next;
        Ok(())
    }
    fn timeline_mut(&mut self) -> Result<&mut PlaybackTimeline, String> {
        self.timeline.as_mut().ok_or_else(|| "This source has no authored motion".into())
    }
    pub fn play(&mut self) -> Result<(), String> { self.timeline_mut()?.play().map_err(str::to_owned) }
    pub fn pause(&mut self) -> Result<(), String> { self.timeline_mut()?.pause().map_err(str::to_owned) }
    pub fn seek(&mut self, tick: time::Tick) -> Result<(), String> {
        self.timeline_mut()?.seek(tick).map_err(str::to_owned)
    }
    pub fn set_rate(&mut self, rate: PlaybackRate) -> Result<(), String> {
        self.timeline_mut()?.set_rate(rate).map_err(str::to_owned)
    }
    pub fn set_looping(&mut self, looping: bool) -> Result<(), String> {
        self.timeline_mut()?.set_looping(looping).map_err(str::to_owned)
    }
    pub fn select_clip(&mut self, clip: &str) -> Result<(), String> {
        let descriptor = self.session.clip_descriptor(clip).ok_or("Requested clip is not authored")?;
        self.timeline.as_mut().ok_or("No playback timeline")?.select(descriptor).map_err(str::to_owned)
    }
    pub fn rebind_runtime(&mut self, cancel: &AtomicBool) -> Result<(), String> {
        // Preflight the epoch bump so no post-rebind failure changes the contract.
        let timeline = self.timeline.as_ref().ok_or("No playback timeline")?;
        timeline.epoch().checked_add(1).ok_or("Transport epoch exhausted")?;
        let position = timeline.position();
        self.session.rebind_runtime(cancel)?;
        self.timeline.as_mut().expect("Timeline retained").seek(position).map_err(str::to_owned)
    }
}
