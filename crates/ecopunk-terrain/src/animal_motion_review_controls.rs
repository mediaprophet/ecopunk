//! UI preflight and recovery over the existing source-bound player.
use super::*;
use crate::animal_motion::MotionClipDescriptor;

impl AnimalMotionReviewSession {
    /// Exact runtime model and typed motion-source hashes; these are distinct
    /// from the retained Vibe history digest.
    pub fn runtime_binding(&self) -> Option<([u8; 32], [u8; 32])> {
        let player = self.current.as_ref()?.player.as_ref()?;
        Some((player.model_sha256(), player.source_sha256()))
    }
    pub fn clip_descriptor(&self, name: &str) -> Option<MotionClipDescriptor<'_>> {
        self.current.as_ref()?.player.as_ref()?.clip_descriptor(name)
    }
    pub fn runtime_retired(&self) -> bool {
        self.current.as_ref().and_then(|c| c.player.as_ref())
            .is_none_or(MotionPlayer::is_retired)
    }
    /// Unlike the low-level player's failure policy, an invalid UI selection
    /// never touches a valid player or requires an invented source edit.
    pub fn select_clip_checked(&mut self, clip: &str, fade_s: f64) -> Result<(), String> {
        if self.clip_descriptor(clip).is_none() || !fade_s.is_finite()
            || !(0. ..=10.).contains(&fade_s) {
            return Err("Requested clip or transition is unavailable".into());
        }
        self.select_clip(clip, fade_s)
    }
    pub fn sample_at_checked(&mut self, clip: &str, time_s: f64,
        cancel: &AtomicBool) -> Result<MotionFrame<'_>, String> {
        let descriptor = self.clip_descriptor(clip).ok_or("Requested clip is unavailable")?;
        if !time_s.is_finite() || time_s < 0. || time_s > descriptor.duration_s {
            return Err("Requested clip time is outside its authored interval".into());
        }
        // Cancellation of a UI request does not retire an otherwise valid
        // player. Cancellation detected inside a running kernel retains the
        // existing explicit runtime-retirement policy.
        if cancel.load(Ordering::Acquire) { return Err("Motion request cancelled".into()); }
        self.sample_at(clip, time_s, cancel)
    }
    /// A genuine runtime failure can rebind from the retained exact candidate
    /// without changing source revision, package or admitted resource inventory.
    pub fn rebind_runtime(&mut self, cancel: &AtomicBool) -> Result<(), String> {
        if cancel.load(Ordering::Acquire) { return Err("Runtime rebind cancelled".into()); }
        let current = self.current.as_ref().ok_or("No retained motion candidate")?;
        let reader = qualia_core_db::bundle::BundleReader::parse(&current.asset.bundle)
            .map_err(|e| format!("Retained candidate: {e:?}"))?;
        let bytes = reader.get("authoring/evaluated-source.json")
            .ok_or("Retained source projection missing")?;
        if bytes.len() > 16 * 1024 * 1024 { return Err("Retained source exceeds profile".into()); }
        let text = std::str::from_utf8(bytes).map_err(|_| "Retained source encoding")?;
        let source = crate::swamp_wallaby_anatomical_r3::parse_source(text)?;
        let player = if source.motion.is_some() {
            Some(MotionPlayer::admit(&current.asset.render.bytes, &source,
                self.rig_limits, self.motion_limits, cancel)?)
        } else { None };
        if cancel.load(Ordering::Acquire) { return Err("Runtime rebind cancelled".into()); }
        self.current.as_mut().ok_or("Retained candidate missing")?.player = player;
        Ok(())
    }
}
