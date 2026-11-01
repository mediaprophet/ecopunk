//! PCM clip projection over the existing scene soundscape; no source store.
use super::EcopunkHolosonicSoundscape;
use qualia_core_db::audio::scene_audio_graph::pcm::{
    PcmRenderBudget, PcmRenderError, PcmRenderReport, PcmRenderState, PcmVoiceBlock,
};
impl EcopunkHolosonicSoundscape {
    /// Process admitted caller-scheduled mono clips using this soundscape's
    /// current listener, emitter slots and obstacles. Retain `state` across
    /// blocks; inspect histories_reset and limiter counts. Scheduling, source
    /// rights, codec/streaming, clock, resampling and device output remain host
    /// responsibilities. The existing harmonic rendering method is unchanged.
    pub fn render_pcm_frame_zero_heap(&self, state: &mut PcmRenderState,
        voices: &[PcmVoiceBlock<'_>], budget: PcmRenderBudget,
        out_l: &mut [f32], out_r: &mut [f32]) -> Result<PcmRenderReport, PcmRenderError> {
        self.graph.render_pcm_quantum_zero_heap(state, voices, self.obstacles(), budget, out_l, out_r)
    }
}
