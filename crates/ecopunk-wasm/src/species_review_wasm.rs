//! Thin WASM projection of the existing species model construction and 10D owner.
use wasm_bindgen::prelude::*;

/// In-memory checked authoring session; browser edits evaluate source directly.
#[wasm_bindgen]
pub struct SpeciesReviewSession {
    subject: u8,
    source: ecopunk_terrain::species_review_source::SpeciesReviewSourceSession,
}
#[wasm_bindgen]
impl SpeciesReviewSession {
    #[wasm_bindgen(constructor)]
    pub fn new(subject: u8) -> Result<SpeciesReviewSession, JsValue> {
        if subject > 1 { return Err(JsValue::from_str("Unsupported authored species")); }
        Ok(Self {subject,source:Default::default()})
    }
    pub fn revision(&self) -> u32 { self.source.revision() as u32 }
    pub fn submit(&mut self, source: &str, expected_revision: u32) -> Result<JsValue, JsValue> {
        if self.source.revision() != expected_revision as u64 {
            return Err(JsValue::from_str("Source edit is stale; current revision retained"));
        }
        let subject = self.subject;
        let asset = self.source.submit(source, |authored| ecopunk_terrain::species_authored_review::build(subject, authored))
            .map_err(|e|JsValue::from_str(&e))?;
        let result = js_sys::Object::new();
        js_sys::Reflect::set(&result,&"bytes".into(),&js_sys::Uint8Array::from(asset.render.bytes.as_slice()).into())?;
        js_sys::Reflect::set(&result,&"bundle".into(),&js_sys::Uint8Array::from(asset.bundle.as_slice()).into())?;
        js_sys::Reflect::set(&result,&"metadata".into(),&js_sys::JSON::parse(&asset.render.metadata.to_string())?)?;
        Ok(result.into())
    }
}

#[wasm_bindgen]
pub fn build_species_review_asset(subject: u8) -> Result<JsValue, JsValue> {
    let asset = ecopunk_terrain::species_review::build_species_review_10d(subject)
        .map_err(|e| JsValue::from_str(&e))?;
    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &"bytes".into(), &js_sys::Uint8Array::from(asset.bytes.as_slice()).into())?;
    let metadata = js_sys::JSON::parse(&asset.metadata.to_string())?;
    js_sys::Reflect::set(&result, &"metadata".into(), &metadata)?;
    Ok(result.into())
}
