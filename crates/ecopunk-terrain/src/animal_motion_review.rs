//! Source-to-model binding for the named-joint motion review runtime.
//! The existing Vibe session, native resources and HMC owner remain authoritative.
use crate::{
    animal_motion::{MotionFrame, MotionLimits, MotionPlayer},
    species_authored_review::AuthoredReviewAsset,
    species_review_resources::{
        ResourceData, ReviewResourceLimits, ReviewResources, SpeciesReviewPackageSession,
    },
    wallaby_rig::RigLimits,
};
use std::sync::atomic::{AtomicBool, Ordering};
#[path = "animal_motion_review_reopen.rs"]
mod reopen;
#[path = "animal_motion_review_controls.rs"]
mod controls;
#[path = "animal_motion_review_projection.rs"]
mod projection;

struct Candidate {
    asset: AuthoredReviewAsset,
    player: Option<MotionPlayer>,
    vibe_source_sha256: String,
}

/// A failed or stale source submission keeps the preceding package and player.
/// Playback errors retire that player; a successful source submission replaces it.
pub struct AnimalMotionReviewSession {
    source: SpeciesReviewPackageSession,
    current: Option<Candidate>,
    rig_limits: RigLimits,
    motion_limits: MotionLimits,
}
impl AnimalMotionReviewSession {
    pub fn new(
        resources: ReviewResourceLimits,
        rig: RigLimits,
        motion: MotionLimits,
    ) -> Result<Self, String> {
        Ok(Self {
            source: SpeciesReviewPackageSession::new(1, resources)?,
            current: None,
            rig_limits: rig,
            motion_limits: motion,
        })
    }
    pub fn revision(&self) -> u64 {
        self.source.revision()
    }
    pub fn resources(&self) -> &ReviewResources {
        self.source.resources()
    }
    pub fn sources(&self) -> &[String] {
        self.source.sources()
    }
    pub fn asset(&self) -> Option<&AuthoredReviewAsset> {
        self.current.as_ref().map(|v| &v.asset)
    }
    pub fn vibe_source_sha256(&self) -> Option<&str> {
        self.current.as_ref().map(|v| v.vibe_source_sha256.as_str())
    }
    pub fn clip_names(&self) -> Vec<&str> {
        self.current
            .as_ref()
            .and_then(|v| v.player.as_ref())
            .map_or_else(Vec::new, |v| v.clip_names().collect())
    }
    /// Build, retain native resources and bind the compiled named-joint clips in
    /// one detached source transaction. No external model/source pair is accepted.
    pub fn submit(
        &mut self,
        source: &str,
        expected_revision: u64,
        cancel: &AtomicBool,
    ) -> Result<&AuthoredReviewAsset, String> {
        self.submit_transaction::<fn(&crate::species_review_source::AuthoredSpeciesSource,
            &AuthoredReviewAsset) -> Result<Vec<ResourceData>, String>>(
            source, expected_revision, None, cancel)
    }
    /// Native Q42/M64 resource replacement, source, model and playback are
    /// prepared together. No resource or source is promoted on failure.
    pub fn submit_with_resources(
        &mut self,
        source: &str,
        expected_revision: u64,
        resources: Vec<ResourceData>,
        cancel: &AtomicBool,
    ) -> Result<&AuthoredReviewAsset, String> {
        self.submit_transaction(source, expected_revision, Some(move |_: &crate::species_review_source::AuthoredSpeciesSource,
            _: &AuthoredReviewAsset| Ok(resources)), cancel)
    }
    /// Generate Q42/M64 from this exact source/model candidate before its
    /// resource inventory and dependent runtime are promoted together.
    pub fn submit_with_generated_resources(
        &mut self,
        source: &str,
        expected_revision: u64,
        generate: impl FnOnce(&AuthoredReviewAsset) -> Result<Vec<ResourceData>, String>,
        cancel: &AtomicBool,
    ) -> Result<&AuthoredReviewAsset, String> {
        self.submit_transaction(source, expected_revision,
            Some(move |_: &crate::species_review_source::AuthoredSpeciesSource, asset: &AuthoredReviewAsset| generate(asset)), cancel)
    }
    /// Generate semantic resources from the source-owner's exact checked values
    /// and model in the same detached promotion as the dependent runtime.
    pub fn submit_with_authored_resources(
        &mut self,
        source: &str,
        expected_revision: u64,
        generate: impl FnOnce(&crate::species_review_source::AuthoredSpeciesSource,
            &AuthoredReviewAsset) -> Result<Vec<ResourceData>, String>,
        cancel: &AtomicBool,
    ) -> Result<&AuthoredReviewAsset, String> {
        self.submit_transaction(source, expected_revision, Some(generate), cancel)
    }
    fn submit_transaction<F: FnOnce(&crate::species_review_source::AuthoredSpeciesSource,
        &AuthoredReviewAsset) -> Result<Vec<ResourceData>, String>>(
        &mut self,
        source: &str,
        expected_revision: u64,
        resources: Option<F>,
        cancel: &AtomicBool,
    ) -> Result<&AuthoredReviewAsset, String> {
        self.submit_prepared_transaction(source, expected_revision, resources, |_| Ok(()), cancel)?;
        self.asset().ok_or_else(|| "Prepared candidate missing".to_owned())
    }
    /// Dependent controller admission also completes before source promotion.
    /// A host cannot publish a source/player and only then discover that its
    /// timeline or dependent resource contract refuses the new candidate.
    pub fn submit_with_authored_resources_prepared<T>(
        &mut self,
        source: &str,
        expected_revision: u64,
        generate: impl FnOnce(&crate::species_review_source::AuthoredSpeciesSource,
            &AuthoredReviewAsset) -> Result<Vec<ResourceData>, String>,
        prepare_runtime: impl FnOnce(Option<&MotionPlayer>) -> Result<T, String>,
        cancel: &AtomicBool,
    ) -> Result<T, String> {
        self.submit_prepared_transaction(source, expected_revision, Some(generate), prepare_runtime, cancel)
    }
    fn submit_prepared_transaction<T, F: FnOnce(&crate::species_review_source::AuthoredSpeciesSource,
        &AuthoredReviewAsset) -> Result<Vec<ResourceData>, String>>(
        &mut self,
        source: &str,
        expected_revision: u64,
        resources: Option<F>,
        prepare_runtime: impl FnOnce(Option<&MotionPlayer>) -> Result<T, String>,
        cancel: &AtomicBool,
    ) -> Result<T, String> {
        if cancel.load(Ordering::Acquire) {
            return Err("Motion source submission cancelled".into());
        }
        let (rig, motion) = (self.rig_limits, self.motion_limits);
        let build = |subject, authored: &crate::species_review_source::AuthoredSpeciesSource|
            crate::species_authored_review::build_with_limits(subject, authored,
                qualia_core_db::specialized_libs::computational_geometry::branch_graph_surface::BranchGraphLimits::default(), rig);
        let prepare = |authored: &crate::species_review_source::AuthoredSpeciesSource,
            asset: &AuthoredReviewAsset| {
            if cancel.load(Ordering::Acquire) { return Err("Motion source submission cancelled".into()); }
            let data = serde_json::to_string(&authored.data).map_err(|e| e.to_string())?;
            let typed = crate::swamp_wallaby_anatomical_r3::parse_source(&data)?;
            let player = if typed.motion.is_some() {
                Some(MotionPlayer::admit(&asset.render.bytes, &typed, rig, motion, cancel)?)
            } else { None };
            let dependent = prepare_runtime(player.as_ref())?;
            if cancel.load(Ordering::Acquire) { return Err("Motion source submission cancelled".into()); }
            Ok((player, authored.source_sha256.clone(), dependent))
        };
        let (asset, (player, digest, dependent)) = if let Some(generate) = resources {
            self.source.submit_with_authored_resources_prepared(source, expected_revision,
                build, generate, prepare)?
        } else {
            self.source.submit_prepared(source, expected_revision, build, prepare)?
        };
        self.current = Some(Candidate {
            asset,
            player,
            vibe_source_sha256: digest,
        });
        Ok(dependent)
    }
    pub fn sample_at(
        &mut self,
        clip: &str,
        time_s: f64,
        cancel: &AtomicBool,
    ) -> Result<MotionFrame<'_>, String> {
        self.current
            .as_mut()
            .and_then(|v| v.player.as_mut())
            .ok_or("No admitted motion source")?
            .sample_at(clip, time_s, cancel)
            .map_err(|e| format!("Motion frame withheld: {e:?}"))
    }
    pub fn advance(&mut self, dt_s: f64, cancel: &AtomicBool) -> Result<MotionFrame<'_>, String> {
        self.current
            .as_mut()
            .and_then(|v| v.player.as_mut())
            .ok_or("No admitted motion source")?
            .advance(dt_s, cancel)
            .map_err(|e| format!("Motion frame withheld: {e:?}"))
    }
    pub fn select_clip(&mut self, clip: &str, fade_s: f64) -> Result<(), String> {
        self.current
            .as_mut()
            .and_then(|v| v.player.as_mut())
            .ok_or("No admitted motion source")?
            .select_clip(clip, fade_s)
            .map_err(|e| format!("Motion selection refused: {e:?}"))
    }
    /// Canonical posed derivative for the existing asset review route. The
    /// editable rig master stays in the HMC; its bind rig is not applied twice.
    pub fn sample_10d(
        &mut self,
        clip: &str,
        time_s: f64,
        limits: crate::animal_motion::PoseExportLimits,
        cancel: &AtomicBool,
    ) -> Result<crate::species_review::SpeciesReviewAsset, String> {
        let current = self.current.as_mut().ok_or("No admitted motion source")?;
        let frame = current
            .player
            .as_mut()
            .ok_or("No admitted motion clips")?
            .sample_at(clip, time_s, cancel)
            .map_err(|e| format!("Motion frame withheld: {e:?}"))?;
        crate::animal_motion::export_frame_10d(&current.asset.render.bytes, &frame, limits, cancel)
    }
}
