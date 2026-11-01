//! Timed PCM clip projection over the existing soundscape graph.
use super::EcopunkHolosonicSoundscape;
use qualia_core_db::audio::scene_audio_graph::pcm_schedule::{
    ClipScheduleBudget, ClipScheduleError, ClipScheduleReport, ClipScheduleState,
    ClipScheduleWorkspace, PcmClip,
};
impl EcopunkHolosonicSoundscape {
    /// Render host-admitted clip windows using caller-owned scheduling buffers.
    /// Frame time/generation come from the existing timeline/transport owner;
    /// this does not persist a second session or decode/stream source resources.
    pub fn render_scheduled_pcm_frame_zero_heap(
        &self,
        state: &mut ClipScheduleState<'_>,
        workspace: &mut ClipScheduleWorkspace<'_>,
        clips: &[PcmClip<'_>],
        block_start: u64,
        transport_generation: u64,
        budget: ClipScheduleBudget,
        out_l: &mut [f32],
        out_r: &mut [f32],
    ) -> Result<ClipScheduleReport, ClipScheduleError> {
        self.graph.render_scheduled_pcm_quantum_zero_heap(
            state,
            workspace,
            clips,
            block_start,
            transport_generation,
            self.obstacles(),
            budget,
            out_l,
            out_r,
        )
    }
}
