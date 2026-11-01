//! Thin browser binding of native Q42/source/player/transport/publication owners.
//! It is a local data/source API, not a project role/clearance/adoption grant.
use ecopunk_terrain::{
    animal_motion::MotionLimits,
    species_knowledge::KnowledgeLimits,
    species_playback::{SpeciesPlaybackSession, PlaybackRate, time::{self, Tick, FrameRate}},
    species_review_resources::{CorpusOptions, ReviewResourceLimits},
    wallaby_rig::RigLimits,
};
use qualia_core_db::{QualiaPortal, render::source_frame::{SourceFrameBindLimits, SourceFrameReceipt}};
use std::sync::atomic::{AtomicBool, Ordering};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SpeciesProductionPlaybackSession {
    inner: SpeciesPlaybackSession,
    cancel: AtomicBool,
    last_submission: Option<SourceFrameReceipt>,
}
#[wasm_bindgen]
impl SpeciesProductionPlaybackSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsValue> {
        Ok(Self { inner: SpeciesPlaybackSession::new(ReviewResourceLimits::default(),
            RigLimits::default(), MotionLimits::default()).map_err(error)?,
            cancel: AtomicBool::new(false), last_submission: None })
    }
    /// Source, authored Q42 partition, model, runtime and transport are admitted
    /// before promotion. Restricted metadata is the default; no declassification
    /// or external evidence adoption occurs through this browser convenience API.
    pub fn submit(&mut self, source: &str, expected_revision: u32) -> Result<u32, JsValue> {
        self.inner.submit(source, u64::from(expected_revision),
            &CorpusOptions::default(), &self.cancel).map_err(error)?;
        self.last_submission = None;
        // Revision is bounded by the existing 128-submission source profile.
        // JS object/byte projections are separate queries, never fallible work
        // after this mutation has committed.
        Ok(self.inner.revision() as u32)
    }
    /// Failed production binding/replay preserves this instance's previous source.
    /// Successful replacement needs a fresh authored model upload and cold bind.
    pub fn reopen(&mut self, bytes: &[u8]) -> Result<u32, JsValue> {
        self.inner.replace_from_bundle(bytes, MotionLimits::default(),
            KnowledgeLimits::default(), &self.cancel).map_err(error)?;
        self.last_submission = None;
        Ok(self.inner.revision() as u32)
    }
    pub fn current_asset(&self) -> Result<JsValue, JsValue> {
        let asset = self.inner.asset().ok_or_else(|| error("No current source candidate"))?;
        let out = js_sys::Object::new();
        put(&out, "model", js_sys::Uint8Array::from(asset.render.bytes.as_slice()).into())?;
        put(&out, "package", js_sys::Uint8Array::from(asset.bundle.as_slice()).into())?;
        put(&out, "state", self.state()?)?;
        Ok(out.into())
    }
    /// Caller uses the EXISTING scene loader with one asset and authored-frame
    /// preservation before this bind. Arbitrary normalized scenes refuse.
    pub fn bind_portal(&mut self, portal: &mut QualiaPortal) -> Result<(), JsValue> {
        self.inner.bind_portal(portal, SourceFrameBindLimits::default(), &self.cancel)
            .map_err(error)?;
        self.last_submission = None;
        Ok(())
    }
    /// Read-only projection; failure here never reports a failed mutation.
    pub fn binding_state(&self, portal: &QualiaPortal) -> Result<JsValue, JsValue> {
        let Some(binding) = portal.source_motion_binding() else { return Ok(JsValue::NULL); };
        if !self.inner.binding_is_current(binding) {
            return Err(error("Renderer binding is not this source/transport context"));
        }
        let out = js_sys::Object::new();
        put(&out, "source_generation", text(binding.source_generation))?;
        put(&out, "mesh_generation", text(binding.mesh_generation))?;
        put(&out, "transport_epoch", text(binding.transport_epoch))?;
        put(&out, "scope", JsValue::from_str("Exact pose geometry/index binding plus admitted model/source hashes; material residency and rendering are separate"))?;
        Ok(out.into())
    }
    pub fn play(&mut self) -> Result<(), JsValue> { self.inner.play().map_err(error) }
    pub fn pause(&mut self) -> Result<(), JsValue> { self.inner.pause().map_err(error) }
    pub fn select_clip(&mut self, clip: &str) -> Result<(), JsValue> { self.inner.select_clip(clip).map_err(error) }
    pub fn seek_ticks(&mut self, tick: &str) -> Result<(), JsValue> {
        self.inner.seek(Tick(decimal_i64(tick)?)).map_err(error)
    }
    pub fn seek_frame(&mut self, frame: &str, numerator: u32, denominator: u32) -> Result<(), JsValue> {
        let tick = time::frame_tick(decimal_i64(frame)?, FrameRate { num: i64::from(numerator),
            den: i64::from(denominator) }).map_err(error)?;
        self.inner.seek(tick).map_err(error)
    }
    pub fn set_rate(&mut self, numerator: u32, denominator: u32) -> Result<(), JsValue> {
        self.inner.set_rate(PlaybackRate { numerator, denominator }).map_err(error)
    }
    pub fn set_looping(&mut self, looping: bool) -> Result<(), JsValue> { self.inner.set_looping(looping).map_err(error) }
    pub fn rebind_runtime(&mut self) -> Result<(), JsValue> { self.inner.rebind_runtime(&self.cancel).map_err(error) }
    /// No .10d serialization or JS skinning. Wide ticks/sequences use strings.
    pub fn present_ticks(&mut self, portal: &mut QualiaPortal, elapsed: &str) -> Result<bool, JsValue> {
        let receipt = self.inner.present_portal(portal, Tick(decimal_i64(elapsed)?), &self.cancel).map_err(error)?;
        let submitted = receipt.is_some();
        if submitted { self.last_submission = receipt; }
        Ok(submitted)
    }
    /// Platform RAF timing is quantized once at the native boundary. This is
    /// not the offline/editorial exact-frame API and commits no behaviour cues.
    pub fn present_elapsed_ms(&mut self, portal: &mut QualiaPortal, elapsed_ms: f64) -> Result<bool, JsValue> {
        if !elapsed_ms.is_finite() || !(0. ..=1000.).contains(&elapsed_ms) {
            return Err(error("Live frame elapsed time must be 0 through 1000 milliseconds"));
        }
        let tick = Tick((elapsed_ms * time::TICKS_PER_SECOND as f64 / 1000.).round() as i64);
        let receipt = self.inner.present_portal(portal, tick, &self.cancel).map_err(error)?;
        let submitted = receipt.is_some();
        if submitted { self.last_submission = receipt; }
        Ok(submitted)
    }
    pub fn last_submission(&self) -> Result<JsValue, JsValue> { receipt_view(self.last_submission) }
    pub fn cancel(&self) { self.cancel.store(true, Ordering::Release); }
    pub fn clear_cancel(&self) { self.cancel.store(false, Ordering::Release); }
    pub fn state(&self) -> Result<JsValue, JsValue> {
        let out = js_sys::Object::new();
        put(&out, "revision", text(self.inner.revision()))?;
        put(&out, "source_generation", text(self.inner.source_generation()))?;
        put(&out, "runtime_retired", JsValue::from_bool(self.inner.runtime_retired()))?;
        if let Some(timeline) = self.inner.timeline() {
            put(&out, "clip", JsValue::from_str(timeline.clip()))?;
            put(&out, "position_tick", text(timeline.position().0))?;
            put(&out, "timeline_tick", text(timeline.timeline().0))?;
            put(&out, "duration_tick", text(timeline.duration().0))?;
            put(&out, "transport_epoch", text(timeline.epoch()))?;
            put(&out, "sequence", text(timeline.sequence()))?;
            put(&out, "playing", JsValue::from_bool(timeline.playing()))?;
            put(&out, "looping", JsValue::from_bool(timeline.looping()))?;
        }
        let clips = js_sys::Array::new();
        for name in self.inner.clip_names() { clips.push(&JsValue::from_str(name)); }
        put(&out, "clips", clips.into())?;
        put(&out, "scope", JsValue::from_str("Native authored-source cursor; no device/pixel/audio/acceptance claim"))?;
        Ok(out.into())
    }
}
fn decimal_i64(value: &str) -> Result<i64, JsValue> {
    if value.is_empty() || value.len() > 19 || !value.bytes().all(|b| b.is_ascii_digit()) {
        return Err(error("Expected bounded nonnegative integer text"));
    }
    value.parse().map_err(|_| error("Integer clock overflow"))
}
fn receipt_view(receipt: Option<SourceFrameReceipt>) -> Result<JsValue, JsValue> {
    let Some(receipt) = receipt else { return Ok(JsValue::NULL); };
    let out = js_sys::Object::new();
    put(&out, "sequence", text(receipt.frame_sequence))?;
    put(&out, "timeline_tick", text(receipt.timeline_tick))?;
    put(&out, "source_generation", text(receipt.binding.source_generation))?;
    put(&out, "mesh_generation", text(receipt.binding.mesh_generation))?;
    put(&out, "transport_epoch", text(receipt.binding.transport_epoch))?;
    put(&out, "scope", JsValue::from_str("Native complete-pose submission; rendering and audio have separate receipts"))?;
    Ok(out.into())
}
fn text(value: impl ToString) -> JsValue { JsValue::from_str(&value.to_string()) }
fn error(value: impl ToString) -> JsValue { JsValue::from_str(&value.to_string()) }
fn put(out: &js_sys::Object, key: &str, value: JsValue) -> Result<(), JsValue> {
    if js_sys::Reflect::set(out, &JsValue::from_str(key), &value)? { Ok(()) }
    else { Err(error("Read-only browser projection property was refused")) }
}
