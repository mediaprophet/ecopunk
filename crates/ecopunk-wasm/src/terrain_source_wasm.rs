//! Thin source-terrain route in the existing Ecopunk portal. Rendering remains
//! QualiaPortal-owned; old illustrative driving/water is not applied to a DEM.
use crate::{configure_default_scene_environment, load_single_10d_colored, EcopunkPortal};
use ecopunk_terrain::terrain_source::TerrainSourceLimits;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
impl EcopunkPortal {
    /// Opt-in source terrain replaces the active illustrative scene. The exact
    /// source and compiled surface can be recovered separately for packaging.
    pub fn load_source_terrain(
        &mut self,
        source_m64: &[u8],
        licence_declaration: &str,
    ) -> Result<String, JsValue> {
        let scene = crate::vibe_host::EcopunkVibeHost::prepare_source_terrain(
            source_m64,
            licence_declaration,
            TerrainSourceLimits::default(),
        )
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let hex = |digest: [u8; 32]| {
            digest
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>()
        };
        let receipt=serde_json::json!({"schema":"ecopunk.source-terrain-scene/1",
            "frame":scene.source().frame(),"source_sha256":hex(scene.source().source_sha256()),
            "surface_sha256":hex(scene.surface_sha256()),"axes":"X east; Y up; Z south",
            "origin":"first source-cell centre; Y relative to declared origin elevation",
            "placement_method":"canonical-decoded-triangle-plane/v1",
            "scope":"Unaccepted source/frame/licence declarations; neutral display material; no generated actors, road or water; no geographic reprojection",
            "surface_bytes":scene.bytes().len(),"source_bytes":scene.source().source_m64().len()}).to_string();
        // Portal calls mutate presentation and are not a transaction. Retire
        // evidence/placement before the first mutation, even if upload fails.
        self.active_experience = 3;
        self.vibe_host.clear_source_terrain();
        self.rover_local_positions.clear();
        self.walker_local_positions.clear();
        self.rover_pose_buffer.clear();
        self.walker_pose_buffer.clear();
        self.inner.set_water_geometry(&[], &[])?;
        self.inner.set_preserve_authored_frame(false);
        let asset_id = format!("urn:sha256:{}", hex(scene.surface_sha256()));
        let loaded = load_single_10d_colored(&mut self.inner, scene.bytes(), &asset_id)?;
        let count = js_sys::Reflect::get(&loaded, &JsValue::from_str("organs_loaded"))?.as_f64();
        if count != Some(1.0) {
            return Err(JsValue::from_str(
                "Source terrain requires exactly one admitted surface; preview retired",
            ));
        }
        configure_default_scene_environment(
            &mut self.inner,
            "source_terrain",
            [0.48, 0.78, 0.44],
            1,
            1.56,
            0.28,
            0.06,
        )?;
        self.inner
            .set_camera_target(0.62, 0.55, 2.4, 0.0, 0.0, 0.0)?;
        let (min, max) = scene.bounds();
        self.orbit_gc = std::array::from_fn(|i| (min[i] + max[i]) * 0.5);
        self.orbit_scale = 1.7
            / (0..3)
                .map(|i| max[i] - min[i])
                .fold(0.0_f32, f32::max)
                .max(1e-6);
        self.vibe_host.adopt_source_terrain(scene);
        self.active_experience = 2;
        Ok(receipt)
    }
    pub fn source_terrain_surface(&self) -> Result<Vec<u8>, JsValue> {
        Ok(self.active_source_terrain()?.bytes().to_vec())
    }
    pub fn source_terrain_m64(&self) -> Result<Vec<u8>, JsValue> {
        Ok(self.active_source_terrain()?.source().source_m64().to_vec())
    }
    pub fn source_terrain_placement(&self, east_m: f64, south_m: f64) -> Result<String, JsValue> {
        let result = self
            .active_source_terrain()?
            .sample_local(east_m, south_m)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        serde_json::to_string(&result).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

impl EcopunkPortal {
    fn active_source_terrain(
        &self,
    ) -> Result<&ecopunk_terrain::terrain_source::TerrainSurfaceScene, JsValue> {
        if self.active_experience != 2 {
            return Err(JsValue::from_str("No active source-terrain presentation"));
        }
        self.vibe_host
            .source_terrain()
            .ok_or_else(|| JsValue::from_str("Source terrain retired"))
    }
    pub(crate) fn step_source_terrain(
        &mut self,
        canvas: &HtmlCanvasElement,
        dt_ms: f32,
        yaw: f32,
        pitch: f32,
        zoom: f32,
    ) -> Result<String, JsValue> {
        self.active_source_terrain()?;
        if !dt_ms.is_finite() || !(0.0..=1000.0).contains(&dt_ms)
            || !yaw.is_finite() || !pitch.is_finite() || !zoom.is_finite() {
            return Err(JsValue::from_str("Source-terrain frame requires finite camera and 0..=1000 ms interval"));
        }
        self.inner
            .set_camera_target(yaw, pitch, zoom, 0.0, 0.0, 0.0)?;
        self.inner.tick(canvas.clone(), dt_ms)?;
        Ok("{\"experience\":\"source_terrain\",\"physics\":\"not_adopted\"}".into())
    }
}
