//! Browser binding of the same source-bound Rust motion and .10d export owners.
use ecopunk_terrain::{
    animal_motion::{MotionLimits, PoseExportLimits},
    animal_motion_review::AnimalMotionReviewSession,
    species_review_resources::ReviewResourceLimits,
    wallaby_rig::RigLimits,
};
use std::sync::atomic::AtomicBool;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SpeciesMotionReviewSession {
    session: AnimalMotionReviewSession,
}
#[wasm_bindgen]
impl SpeciesMotionReviewSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SpeciesMotionReviewSession, JsValue> {
        Ok(Self {
            session: AnimalMotionReviewSession::new(
                ReviewResourceLimits::default(),
                RigLimits::default(),
                MotionLimits::default(),
            )
            .map_err(error)?,
        })
    }
    pub fn revision(&self) -> u32 {
        self.session.revision() as u32
    }
    pub fn reopen(bytes: &[u8]) -> Result<SpeciesMotionReviewSession, JsValue> {
        Ok(Self { session: AnimalMotionReviewSession::reopen(bytes,
            ReviewResourceLimits::default(), MotionLimits::default(), &AtomicBool::new(false)).map_err(error)? })
    }
    /// Returned model and package belong to exactly the source revision admitted
    /// by this call. A rejected edit leaves the previous revision/player active.
    pub fn submit(&mut self, source: &str, expected_revision: u32) -> Result<JsValue, JsValue> {
        let asset = self
            .session
            .submit(
                source,
                u64::from(expected_revision),
                &AtomicBool::new(false),
            )
            .map_err(error)?;
        let result = js_sys::Object::new();
        set(
            &result,
            "model",
            js_sys::Uint8Array::from(asset.render.bytes.as_slice()).into(),
        )?;
        set(
            &result,
            "package",
            js_sys::Uint8Array::from(asset.bundle.as_slice()).into(),
        )?;
        set(
            &result,
            "manifest",
            JsValue::from_str(&asset.render.metadata.to_string()),
        )?;
        set(
            &result,
            "revision",
            JsValue::from_f64(self.session.revision() as f64),
        )?;
        Ok(result.into())
    }
    pub fn clip_names(&self) -> js_sys::Array {
        self.session
            .clip_names()
            .into_iter()
            .map(JsValue::from_str)
            .collect()
    }
    /// Returns a resource the existing viewer can load; all deformation and
    /// frame generation happen in Rust/Core. No JavaScript skinning is required.
    pub fn sample_10d(&mut self, clip: &str, time_s: f64) -> Result<JsValue, JsValue> {
        let asset = self
            .session
            .sample_10d(
                clip,
                time_s,
                PoseExportLimits::default(),
                &AtomicBool::new(false),
            )
            .map_err(error)?;
        let result = js_sys::Object::new();
        set(
            &result,
            "model",
            js_sys::Uint8Array::from(asset.bytes.as_slice()).into(),
        )?;
        set(
            &result,
            "manifest",
            JsValue::from_str(&asset.metadata.to_string()),
        )?;
        Ok(result.into())
    }
    pub fn select_clip(&mut self, clip: &str, fade_s: f64) -> Result<(), JsValue> {
        self.session.select_clip_checked(clip, fade_s).map_err(error)
    }
}
fn error(message: String) -> JsValue {
    JsValue::from_str(&message)
}
fn set(object: &js_sys::Object, key: &str, value: JsValue) -> Result<(), JsValue> {
    js_sys::Reflect::set(object, &JsValue::from_str(key), &value).map(|_| ())
}
