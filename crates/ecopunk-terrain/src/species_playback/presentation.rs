//! Publish the one source player's borrowed attributes into the existing full
//! WebGPU mesh. These receipts describe submission, not presented pixels.
use super::*;
use crate::species_playback::time::Tick;
use super::transport::PosePlan;
use qualia_core_db::render::{gpu::PortalGpu, source_frame::{
    SourceFrameBindLimits, SourceFrameBinding, SourceFrameError, SourceFrameReceipt, SourceMeshFrame,
}};

impl SpeciesPlaybackSession {
    /// Read-only context guard for a renderer projection. Typed motion-source
    /// hashes are checked separately from the retained Vibe history digest.
    pub fn binding_is_current(&self, binding: SourceFrameBinding) -> bool {
        self.checked_binding(binding).is_ok()
            && self.timeline.as_ref().is_some_and(|t| t.epoch() == binding.transport_epoch)
    }
    fn checked_binding(&self, binding: SourceFrameBinding) -> Result<(), String> {
        let (model, source) = self.session.runtime_binding().ok_or("No admitted motion player")?;
        if binding.source_generation != self.generation || binding.model_sha256 != model
            || binding.source_sha256 != source {
            return Err("Renderer binding is from a different source or resident model".into());
        }
        Ok(())
    }
    fn bind_inputs(&self, cancel: &AtomicBool) -> Result<(&[u8], [u8; 32], u64), String> {
        if cancel.load(Ordering::Acquire) { return Err("Playback bind cancelled".into()); }
        self.timeline.as_ref().ok_or("No authored motion to bind")?;
        let (_, source) = self.session.runtime_binding().ok_or("No admitted motion player")?;
        Ok((&self.session.asset().ok_or("No current source model")?.render.bytes, source, self.generation))
    }
    /// The host first uploads this exact single model in authored identity
    /// coordinates through the existing loader. Normalized/compact/multiasset
    /// resident scenes refuse instead of being relabelled as this source.
    pub fn bind_gpu(&self, gpu: &mut PortalGpu, limits: SourceFrameBindLimits,
        cancel: &AtomicBool) -> Result<SourceFrameBinding, String> {
        let (model, source, generation) = self.bind_inputs(cancel)?;
        let epoch = self.timeline.as_ref().ok_or("No playback timeline")?.epoch();
        gpu.bind_source_motion_model_at_epoch(model, source, generation, epoch, limits)
            .map_err(|e| e.to_string())
    }
    pub fn present_gpu(&mut self, gpu: &mut PortalGpu, elapsed: Tick,
        cancel: &AtomicBool) -> Result<Option<SourceFrameReceipt>, String> {
        if cancel.load(Ordering::Acquire) { return Err("Frame request cancelled".into()); }
        let plan = self.timeline.as_ref().ok_or("No playback timeline")?.plan(elapsed)
            .map_err(str::to_owned)?;
        let mut binding = gpu.source_motion_binding().ok_or("No current renderer source binding")?;
        self.checked_binding(binding)?;
        let epoch = self.timeline.as_ref().ok_or("No playback timeline")?.epoch();
        if binding.transport_epoch < epoch {
            binding = gpu.advance_source_motion_epoch(binding, epoch).map_err(|e| e.to_string())?;
        }
        if binding.transport_epoch != epoch { return Err("Renderer transport epoch differs".into()); }
        self.present_with(plan, binding, |frame| gpu.apply_source_motion_frame(frame), cancel)
    }
    fn present_with<F>(&mut self, plan: PosePlan, binding: SourceFrameBinding,
        publish: F, cancel: &AtomicBool) -> Result<Option<SourceFrameReceipt>, String>
    where F: for<'a> FnOnce(SourceMeshFrame<'a>) -> Result<SourceFrameReceipt, SourceFrameError> {
        let timeline = self.timeline.as_ref().ok_or("No playback timeline")?;
        if !plan.required {
            self.timeline.as_mut().expect("Timeline retained").commit(plan);
            return Ok(None);
        }
        let seconds = timeline.sample_seconds(plan);
        // No new container, mesh copy, second player or JS evaluator here.
        let motion = self.session.sample_at_checked(timeline.clip(), seconds, cancel)?;
        let frame = frame_bridge::frame(&motion, binding, plan.sequence, plan.timeline.0 as u64)
            .map_err(|e| e.to_string())?;
        if cancel.load(Ordering::Acquire) { return Err("Frame publication cancelled".into()); }
        let receipt = publish(frame).map_err(|e| e.to_string())?;
        // The unique mutable borrow spans plan/sample/publication. No concurrent
        // source/seek command can invalidate it between submission and commit.
        self.timeline.as_mut().expect("Timeline retained").commit(plan);
        Ok(Some(receipt))
    }
    #[cfg(target_arch = "wasm32")]
    pub fn bind_portal(&self, portal: &mut qualia_core_db::QualiaPortal,
        limits: SourceFrameBindLimits, cancel: &AtomicBool) -> Result<SourceFrameBinding, String> {
        let (model, source, generation) = self.bind_inputs(cancel)?;
        let epoch = self.timeline.as_ref().ok_or("No playback timeline")?.epoch();
        portal.bind_source_motion_model_at_epoch(model, source, generation, epoch, limits)
            .map_err(|e| e.to_string())
    }
    #[cfg(target_arch = "wasm32")]
    pub fn present_portal(&mut self, portal: &mut qualia_core_db::QualiaPortal,
        elapsed: Tick, cancel: &AtomicBool) -> Result<Option<SourceFrameReceipt>, String> {
        if cancel.load(Ordering::Acquire) { return Err("Frame request cancelled".into()); }
        let plan = self.timeline.as_ref().ok_or("No playback timeline")?.plan(elapsed)
            .map_err(str::to_owned)?;
        let mut binding = portal.source_motion_binding().ok_or("No current renderer source binding")?;
        self.checked_binding(binding)?;
        let epoch = self.timeline.as_ref().ok_or("No playback timeline")?.epoch();
        if binding.transport_epoch < epoch {
            binding = portal.advance_source_motion_epoch(binding, epoch).map_err(|e| e.to_string())?;
        }
        if binding.transport_epoch != epoch { return Err("Renderer transport epoch differs".into()); }
        self.present_with(plan, binding, |frame| portal.apply_source_motion_frame(frame), cancel)
    }
}
