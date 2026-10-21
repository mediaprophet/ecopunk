//! `ecopunk-wasm` — QualiaDB (`QualiaPortal` + `.10d` + `.q42` + `.hmc` + `.m64`)
//! and `VibeScript` (`vibe-0.1` + `.qbc`) WASM Engine Shell for East Coast Australia.
//!
//! Zero handwritten JS shaders, zero inline JS geometry, zero JS physics substitutes:
//! - Renders all 3D geometry and water via `qualia_core_db::QualiaPortal` (`portal_init_webgpu` / `portal_init_webgl2`).
//! - Executes `vibe/ecopunk_continuum_drive.vibe` and `vibe/ecopunk_biological_lab.vibe` through `vibe::load_program`,
//!   `vibe::bytecode::compile`, and `EcopunkVibeHost` (`impl vibe::Host`).
//! - Synthesizes and loads verified `.10d` Living Containers for all 8 corridor study sites and all 5
//!   botanical/zoological educational dissection scales (`botanical_anatomy` + `zoological_anatomy`).

pub mod vibe_host;

use ecopunk_terrain::botanical_anatomy::{
    build_scale1_whole_organism_10d, build_scale2_stem_bark_wedge_10d,
    build_scale3_floral_and_gumnut_dissection_10d, build_scale4_leaf_venation_and_histology_10d,
    DevelopmentalStage,
};
use ecopunk_terrain::botanical_ontology_q42::CANONICAL_ANATOMICAL_PINS;
use ecopunk_terrain::exploration_controller::{
    ContinuumExplorationSession, ExplorationMode, ExplorationTickInput,
};
use ecopunk_terrain::fauna_animation_rig::FaunaBiomechanicalArchetype;
use ecopunk_terrain::scene_builder_10d::{
    build_procedural_textures, build_sector_10d, ProceduralTextureAsset,
};
use ecopunk_terrain::world_continuum::{RegionalCorridorWorld, StudySiteId};
use ecopunk_terrain::zoological_anatomy::build_scale5_vertebrate_anatomy_10d;
use vibe::bytecode::{compile as compile_qbc, encode_chunk};
use vibe::{eval_function, load_program, Env, Program, Value};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

use crate::vibe_host::EcopunkVibeHost;

/// Compile a `vibe-0.1` source module into `VBC1` `.qbc` binary bytecode after full AST & effect type-checking.
pub fn compile_vibe_source_to_qbc(src: &str) -> Result<Vec<u8>, String> {
    let prog = load_program(src).map_err(|diags| format!("VibeScript typecheck failed: {diags:?}"))?;
    let chunk = compile_qbc(&prog).map_err(|d| format!("QBC compile failed: {d:?}"))?;
    Ok(encode_chunk(&chunk))
}

/// Convert a `vibe::Value` into a `serde_json::Value` for structured telemetry inspection.
pub fn vibe_value_to_json(v: &Value) -> serde_json::Value {
    match v {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::I64(n) => serde_json::json!(*n),
        Value::U64(n) => serde_json::json!(*n),
        Value::F64(n) => serde_json::json!(*n),
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::List(items) => {
            serde_json::Value::Array(items.iter().map(vibe_value_to_json).collect())
        }
        Value::Record(map) => {
            let mut obj = serde_json::Map::new();
            for (k, val) in map {
                obj.insert(k.clone(), vibe_value_to_json(val));
            }
            serde_json::Value::Object(obj)
        }
        other => serde_json::Value::String(format!("{other:?}")),
    }
}

/// Build the sealed `.10d` Living Container bytes and metadata for any of the 5 educational dissection scales.
pub fn build_dissection_scale_10d(
    scale: u8,
    explode_01: f32,
    cutaway_01: f32,
    anthesis_01: f32,
    peel_01: f32,
    archetype_idx: u8,
) -> Result<(Vec<u8>, serde_json::Value), String> {
    match scale.clamp(1, 5) {
        1 => {
            let organ = build_scale1_whole_organism_10d(
                "corymbia-aparrerinja",
                DevelopmentalStage::Mature,
            )?;
            let meta = serde_json::json!({
                "scale": 1,
                "scale_name": organ.scale.name(),
                "physical_range": organ.scale.physical_range(),
                "vertex_count": organ.vertex_count(),
                "triangle_count": organ.triangle_count(),
                "tissue_layer_count": organ.tissue_slices.len(),
                "container_bytes": organ.container_bytes.len(),
            });
            Ok((organ.container_bytes, meta))
        }
        2 => {
            let organ = build_scale2_stem_bark_wedge_10d(
                "corymbia-aparrerinja",
                explode_01.clamp(0.0, 1.0),
            )?;
            let meta = serde_json::json!({
                "scale": 2,
                "scale_name": organ.scale.name(),
                "physical_range": organ.scale.physical_range(),
                "vertex_count": organ.vertex_count(),
                "triangle_count": organ.triangle_count(),
                "tissue_layer_count": organ.tissue_slices.len(),
                "container_bytes": organ.container_bytes.len(),
            });
            Ok((organ.container_bytes, meta))
        }
        3 => {
            let organ = build_scale3_floral_and_gumnut_dissection_10d(
                "corymbia-aparrerinja",
                anthesis_01.clamp(0.0, 1.0),
                cutaway_01.clamp(0.0, 1.0),
            )?;
            let meta = serde_json::json!({
                "scale": 3,
                "scale_name": organ.scale.name(),
                "physical_range": organ.scale.physical_range(),
                "vertex_count": organ.vertex_count(),
                "triangle_count": organ.triangle_count(),
                "tissue_layer_count": organ.tissue_slices.len(),
                "container_bytes": organ.container_bytes.len(),
            });
            Ok((organ.container_bytes, meta))
        }
        4 => {
            let organ = build_scale4_leaf_venation_and_histology_10d(
                "corymbia-aparrerinja",
                cutaway_01.clamp(0.0, 1.0),
            )?;
            let meta = serde_json::json!({
                "scale": 4,
                "scale_name": organ.scale.name(),
                "physical_range": organ.scale.physical_range(),
                "vertex_count": organ.vertex_count(),
                "triangle_count": organ.triangle_count(),
                "tissue_layer_count": organ.tissue_slices.len(),
                "container_bytes": organ.container_bytes.len(),
            });
            Ok((organ.container_bytes, meta))
        }
        _ => {
            let arch = match archetype_idx % 5 {
                0 => FaunaBiomechanicalArchetype::MacropodSaltatorial,
                1 => FaunaBiomechanicalArchetype::VombatidFossorial,
                2 => FaunaBiomechanicalArchetype::MonotremeTeleostAmphibian,
                3 => FaunaBiomechanicalArchetype::RaptorSoaring,
                _ => FaunaBiomechanicalArchetype::PhascolarctidArboreal,
            };
            let peel = peel_01.clamp(0.0, 1.0);
            let osteo = 1.0_f32;
            let myo = (1.0 - (peel - 0.45).max(0.0) * 1.5).clamp(0.15, 1.0);
            let integ = (1.0 - peel * 1.6).clamp(0.0, 1.0);
            let vert = build_scale5_vertebrate_anatomy_10d(
                arch,
                osteo,
                myo,
                integ,
                explode_01.clamp(0.0, 1.0),
            );
            let meta = serde_json::json!({
                "scale": 5,
                "scale_name": "Scale 5: Comparative Vertebrate 3-Layer Anatomy (Osteology / Myology / Integument)",
                "physical_range": "150 mm – 2.2 m",
                "archetype": format!("{arch:?}"),
                "vertex_count": vert.vertex_count,
                "triangle_count": vert.triangle_count,
                "tissue_layer_count": vert.submesh_ranges.len(),
                "container_bytes": vert.container_bytes.len(),
            });
            Ok((vert.container_bytes, meta))
        }
    }
}

/// Map one of the 8 continuum study sites (`0..=7`) to its canonical `.10d` sector key and title in `ecopunk-terrain`.
pub fn sector_key_for_study_site(site_index: u8) -> (&'static str, &'static str) {
    match site_index {
        0 | 1 => ("talbingo", "Talbingo & Blowering Riparian Corridor"),
        2 => ("jounama_karst", "Jounama Creek Valley & Bogong Tors"),
        3 | 4 => ("yarrangobilly", "Yarrangobilly Silurian Limestone Karst Gorge"),
        5 => ("three_mile_dam", "Three Mile Dam Subalpine Snow Gum & Sphagnum Bog"),
        6 => ("sydney_sandstone", "Sydney Hawkesbury Sandstone Gully"),
        _ => ("noosa_headland", "Noosa Coastal Pandanus Basalt Headland"),
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn init_webgpu(canvas: HtmlCanvasElement) -> Result<bool, JsValue> {
    qualia_core_db::render::portal::portal_init_webgpu(canvas).await
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_webgl2(canvas: HtmlCanvasElement) -> Result<bool, JsValue> {
    qualia_core_db::render::portal::portal_init_webgl2(canvas)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct EcopunkPortal {
    inner: qualia_core_db::QualiaPortal,
    session: ContinuumExplorationSession,
    vibe_host: EcopunkVibeHost,
    textures: Vec<ProceduralTextureAsset>,
    drive_prog: Option<Program>,
    lab_prog: Option<Program>,
    drive_qbc_bytes: Vec<u8>,
    lab_qbc_bytes: Vec<u8>,
    active_experience: u8,
    active_site_loaded: u8,
    active_pin_index: usize,
    anthesis_01: f32,
    peel_01: f32,
    archetype_idx: u8,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl EcopunkPortal {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<EcopunkPortal, JsValue> {
        let mut inner = qualia_core_db::QualiaPortal::new(canvas)?;
        inner.set_preserve_authored_frame(false);
        inner.set_shadows_enabled(true);
        inner.set_screen_space_ao_enabled(true);
        inner.set_screen_space_ao(0.48, 0.72, 0.018);
        inner.set_sky_preset(0);
        inner.set_lighting(0.45, 0.82, 0.38, 1.55, 0.34);

        let textures = build_procedural_textures();
        for (idx, tex) in textures.iter().enumerate() {
            let _ = inner.load_texture_encoded(
                &tex.sha256_bytes,
                "image/png",
                &tex.png_bytes,
                (idx as u8) % 4,
                0.5,
            );
        }

        let world = RegionalCorridorWorld::new();
        let session = ContinuumExplorationSession::new(world);
        let vibe_host = EcopunkVibeHost::new();

        let mut portal = EcopunkPortal {
            inner,
            session,
            vibe_host,
            textures,
            drive_prog: None,
            lab_prog: None,
            drive_qbc_bytes: Vec::new(),
            lab_qbc_bytes: Vec::new(),
            active_experience: 0,
            active_site_loaded: 255,
            active_pin_index: 0,
            anthesis_01: 0.65,
            peel_01: 0.35,
            archetype_idx: 0,
        };
        portal.load_continuum_site_scene(0)?;
        Ok(portal)
    }

    /// Return the active QualiaPortal render tier (2 = WebGPU, 1 = WebGL2, 0 = CPU fallback).
    pub fn tier(&self) -> u8 {
        self.inner.tier()
    }

    /// Admit an in-memory `.q42` semantic graph volume slice into `EcopunkVibeHost` (`LocalHost::with_q42_pack`).
    pub fn admit_q42_pack(&mut self, q42_bytes: &[u8]) -> usize {
        self.vibe_host = EcopunkVibeHost::with_q42_pack(q42_bytes);
        q42_bytes.len()
    }

    /// Compile and register the two canonical `vibe-0.1` modules (`ecopunk_continuum_drive.vibe` and `ecopunk_biological_lab.vibe`),
    /// returning their `.qbc` bytecode verification receipt as JSON.
    pub fn load_vibe_modules(
        &mut self,
        continuum_drive_vibe: &str,
        biological_lab_vibe: &str,
    ) -> Result<String, JsValue> {
        let drive_prog = load_program(continuum_drive_vibe)
            .map_err(|e| JsValue::from_str(&format!("drive .vibe error: {e:?}")))?;
        let drive_chunk = compile_qbc(&drive_prog)
            .map_err(|e| JsValue::from_str(&format!("drive .qbc error: {e:?}")))?;
        let drive_qbc = encode_chunk(&drive_chunk);

        let lab_prog = load_program(biological_lab_vibe)
            .map_err(|e| JsValue::from_str(&format!("lab .vibe error: {e:?}")))?;
        let lab_chunk = compile_qbc(&lab_prog)
            .map_err(|e| JsValue::from_str(&format!("lab .qbc error: {e:?}")))?;
        let lab_qbc = encode_chunk(&lab_chunk);

        let receipt = serde_json::json!({
            "vibe_language": "vibe-0.1",
            "continuum_drive_qbc_bytes": drive_qbc.len(),
            "biological_lab_qbc_bytes": lab_qbc.len(),
            "catalog_invoke_id_count": vibe::catalog::INVOKE_ID_COUNT,
        });

        self.drive_prog = Some(drive_prog);
        self.lab_prog = Some(lab_prog);
        self.drive_qbc_bytes = drive_qbc;
        self.lab_qbc_bytes = lab_qbc;

        Ok(receipt.to_string())
    }

    /// Synthesize and upload a sealed `.10d` sector manifold + water surface for one of the 8 corridor study sites (`0..=7`).
    pub fn load_continuum_site_scene(&mut self, site_index: u8) -> Result<String, JsValue> {
        let clamped = site_index.min(7);
        if let Some(site_id) = StudySiteId::from_index(clamped as usize) {
            self.session.teleport_to_study_site(site_id);
        }
        let (sector_key, sector_title) = sector_key_for_study_site(clamped);
        let sector = build_sector_10d(sector_key, sector_title, &self.textures, false)
            .map_err(|e| JsValue::from_str(&format!("build_sector_10d({sector_key}): {e}")))?;

        self.inner.set_preserve_authored_frame(false);
        let _receipt = self.inner.load_10d(&sector.scene_10d_bytes)?;

        let flat_water: Vec<f32> = sector.water_positions.iter().flat_map(|p| *p).collect();
        self.inner
            .set_water_geometry(&flat_water, &sector.water_indices)?;
        self.inner.set_water_surface(
            sector.water_color[0],
            sector.water_color[1],
            sector.water_color[2],
            sector.water_alpha,
            0.015,
            2.8,
            2,
        );
        self.inner.set_sky_preset(sector.sky_preset);
        self.inner.set_lighting(
            sector.sun_dir[0],
            sector.sun_dir[1],
            sector.sun_dir[2],
            1.52,
            0.30,
        );
        self.inner.set_camera_target(
            sector.cam_yaw,
            sector.cam_pitch,
            sector.cam_zoom,
            sector.cam_target[0],
            sector.cam_target[1],
            sector.cam_target[2],
        )?;

        self.active_experience = 0;
        self.active_site_loaded = clamped;

        let site_desc = &self.session.world.sites[clamped as usize];
        let out = serde_json::json!({
            "experience": "world_continuum",
            "site_index": clamped,
            "site_name": site_desc.name,
            "corridor_km": site_desc.corridor_km,
            "elevation_ahd_m": site_desc.elevation_ahd_m,
            "vegetation_community": site_desc.vegetation_community,
            "geology_description": site_desc.geology_description,
            "sector_id": sector.sector_id,
            "scene_10d_bytes": sector.scene_10d_bytes.len(),
            "scene_crc32c": format!("0x{:08X}", sector.scene_crc32c),
            "vertex_count": sector.positions.len(),
            "triangle_count": sector.indices.len() / 3,
            "portal_tier": self.inner.tier(),
        });
        Ok(out.to_string())
    }

    /// Synthesize and upload a sealed `.10d` Living Container for any of the 5 educational dissection scales (`1..=5`),
    /// and evaluate `ecopunk_biological_lab.vibe` (`evaluate_dissection_lab_state`) through `EcopunkVibeHost`!
    pub fn load_dissection_scale(
        &mut self,
        scale: u8,
        explode_01: f32,
        cutaway_01: f32,
        anthesis_01: f32,
        peel_01: f32,
        archetype_idx: u8,
        pin_index: u8,
    ) -> Result<String, JsValue> {
        let s = scale.clamp(1, 5);
        self.session.loupe.active_scale = s;
        self.session.loupe.explode_layers_01 = explode_01.clamp(0.0, 1.0);
        self.session.loupe.cutaway_plane_01 = cutaway_01.clamp(0.0, 1.0);
        self.anthesis_01 = anthesis_01.clamp(0.0, 1.0);
        self.peel_01 = peel_01.clamp(0.0, 1.0);
        self.archetype_idx = archetype_idx;
        self.active_pin_index = pin_index as usize;
        self.active_experience = 1;

        let (bytes_10d, mesh_meta) = build_dissection_scale_10d(
            s,
            self.session.loupe.explode_layers_01,
            self.session.loupe.cutaway_plane_01,
            self.anthesis_01,
            self.peel_01,
            self.archetype_idx,
        )
        .map_err(|e| JsValue::from_str(&e))?;

        self.inner.set_preserve_authored_frame(false);
        let _ = self.inner.load_10d(&bytes_10d)?;
        self.inner.set_water_geometry(&[], &[])?;
        self.inner.set_camera_target(0.62, 0.28, 2.15, 0.0, 0.0, 0.0)?;

        let vibe_eval_json = if let Some(prog) = &self.lab_prog {
            let mut env = Env::default();
            let args = vec![
                Value::I64(s as i64),
                Value::F64(self.session.loupe.explode_layers_01 as f64),
                Value::F64(self.session.loupe.cutaway_plane_01 as f64),
                Value::F64(self.anthesis_01 as f64),
                Value::F64(self.peel_01 as f64),
                Value::I64(pin_index as i64),
                Value::I64(self.session.loupe.phenology_month as i64),
            ];
            match eval_function(
                prog,
                "evaluate_dissection_lab_state",
                args,
                &mut self.vibe_host,
                &mut env,
            ) {
                Ok(v) => vibe_value_to_json(&v),
                Err(d) => serde_json::json!({ "error": format!("{d:?}") }),
            }
        } else {
            serde_json::Value::Null
        };

        let scale_pins: Vec<serde_json::Value> = CANONICAL_ANATOMICAL_PINS
            .iter()
            .filter(|p| p.scale == s)
            .map(|p| {
                serde_json::json!({
                    "pin_id": p.pin_id,
                    "scale": p.scale,
                    "ontology_curie": p.ontology_curie,
                    "scientific_term": p.scientific_term,
                    "biophysical_formula": p.biophysical_formula,
                    "educational_summary": p.educational_summary,
                    "indigenous_custodial_context": p.indigenous_custodial_context,
                    "anchor_local_xyz": p.anchor_local_xyz,
                })
            })
            .collect();

        let selected_pin = if scale_pins.is_empty() {
            serde_json::Value::Null
        } else {
            scale_pins[self.active_pin_index % scale_pins.len()].clone()
        };

        let out = serde_json::json!({
            "experience": "biological_dissection_lab",
            "mesh_10d": mesh_meta,
            "vibe_evaluation": vibe_eval_json,
            "scale_pins": scale_pins,
            "selected_pin": selected_pin,
            "portal_tier": self.inner.tier(),
        });
        Ok(out.to_string())
    }

    /// Orbit/zoom the `QualiaPortal` camera directly.
    pub fn set_orbit_camera(
        &mut self,
        yaw: f32,
        pitch: f32,
        zoom: f32,
        tx: f32,
        ty: f32,
        tz: f32,
    ) -> Result<(), JsValue> {
        self.session.camera_yaw = yaw;
        self.session.camera_pitch = pitch;
        self.inner.set_camera_target(yaw, pitch, zoom, tx, ty, tz)
    }

    /// Toggle between RoverDriving and OnFootExploration (`[E]`) at any coordinate in the 26.5 km continuum.
    pub fn toggle_dismount(&mut self) -> String {
        let mut input = ExplorationTickInput::default();
        input.toggle_dismount = true;
        let snap = self.session.step_tick_zero_heap(&input, 1.0 / 64.0);
        serde_json::json!({
            "mode": format!("{:?}", snap.mode),
            "rover_position": snap.rover_position,
            "walker_position": snap.walker_position,
            "corridor_km": snap.current_corridor_km,
        })
        .to_string()
    }

    /// Step one frame of the 64 Hz `ContinuumExplorationSession` + `VibeScript` policy + `QualiaPortal::tick(canvas, dt_ms)`.
    pub fn step_and_render(
        &mut self,
        canvas: &HtmlCanvasElement,
        dt_ms: f32,
        throttle: f32,
        steer: f32,
        brake: f32,
        walk_fwd: f32,
        walk_strafe: f32,
        sprint: bool,
        cam_yaw: f32,
        cam_pitch: f32,
        cam_zoom: f32,
    ) -> Result<String, JsValue> {
        let dt_s = (dt_ms * 0.001).clamp(1.0 / 240.0, 0.05);

        let tick_input = ExplorationTickInput {
            throttle,
            steer,
            brake,
            wish_dir: [walk_fwd, walk_strafe],
            look_delta_yaw: 0.0,
            look_delta_pitch: 0.0,
            sprint,
            jump: false,
            crouch: false,
            toggle_dismount: false,
            toggle_loupe: false,
            cycle_camera: false,
            loupe_scale_change: None,
            loupe_cutaway_delta: 0.0,
            loupe_explode_delta: 0.0,
        };

        let snap = self.session.step_tick_zero_heap(&tick_input, dt_s);

        if self.active_experience == 0 {
            let nearest_idx = snap.nearest_study_site as u8;
            if nearest_idx != self.active_site_loaded && snap.nearest_site_distance_m < 450.0 {
                let saved_rover = self.session.rover.state;
                let saved_walker = self.session.walker.state.clone();
                let saved_mode = self.session.mode;
                let saved_odo = self.session.total_odometer_km;
                let _ = self.load_continuum_site_scene(nearest_idx);
                self.session.rover.state = saved_rover;
                self.session.walker.state = saved_walker;
                self.session.mode = saved_mode;
                self.session.total_odometer_km = saved_odo;
            }

            let active_pos = if snap.mode == ExplorationMode::RoverDriving {
                snap.rover_position
            } else {
                snap.walker_position
            };
            let site_origin =
                self.session.world.sites[self.active_site_loaded.min(7) as usize].world_pos_xz;
            let local_dx = ((active_pos[0] - site_origin[0]) * 0.004).clamp(-0.65, 0.65);
            let local_dz = ((active_pos[2] - site_origin[1]) * 0.004).clamp(-0.65, 0.65);
            self.inner
                .set_camera_target(cam_yaw, cam_pitch, cam_zoom, local_dx, -0.04, local_dz)?;
        } else {
            self.inner
                .set_camera_target(cam_yaw, cam_pitch, cam_zoom, 0.0, 0.0, 0.0)?;
        }

        let vibe_tick = if let Some(prog) = &self.drive_prog {
            let mut env = Env::default();
            if snap.mode == ExplorationMode::RoverDriving {
                let args = vec![
                    Value::F64(snap.rover_position[0] as f64),
                    Value::F64(snap.rover_position[2] as f64),
                    Value::F64(self.session.rover.state.heading as f64),
                    Value::F64((snap.rover_speed_kmh / 3.6) as f64),
                    Value::F64(throttle as f64),
                    Value::F64(steer as f64),
                    Value::F64(dt_s as f64),
                ];
                eval_function(
                    prog,
                    "step_continuum_rover_tick",
                    args,
                    &mut self.vibe_host,
                    &mut env,
                )
                .ok()
                .map(|v| vibe_value_to_json(&v))
            } else {
                let args = vec![
                    Value::F64(snap.walker_position[0] as f64),
                    Value::F64(snap.walker_position[2] as f64),
                    Value::F64(cam_yaw as f64),
                    Value::F64(walk_fwd as f64),
                    Value::F64(walk_strafe as f64),
                    Value::Bool(sprint),
                    Value::F64(dt_s as f64),
                ];
                eval_function(
                    prog,
                    "step_continuum_walker_tick",
                    args,
                    &mut self.vibe_host,
                    &mut env,
                )
                .ok()
                .map(|v| vibe_value_to_json(&v))
            }
        } else {
            None
        };

        self.inner.tick(canvas.clone(), dt_ms)?;

        let site_desc = &self.session.world.sites[snap.nearest_study_site as usize];
        let telemetry = serde_json::json!({
            "experience": if self.active_experience == 0 { "world_continuum" } else { "biological_dissection_lab" },
            "mode": format!("{:?}", snap.mode),
            "portal_tier": self.inner.tier(),
            "rover_position": snap.rover_position,
            "rover_speed_kmh": snap.rover_speed_kmh,
            "walker_position": snap.walker_position,
            "walker_submersion": format!("{:?}", snap.walker_submersion),
            "total_odometer_km": snap.total_odometer_km,
            "corridor_km": snap.current_corridor_km,
            "nearest_site_index": snap.nearest_study_site as u8,
            "nearest_site_name": site_desc.name,
            "nearest_site_distance_m": snap.nearest_site_distance_m,
            "elevation_ahd_m": snap.elevation_ahd_m,
            "soil_sinkage_mm": snap.soil_sinkage_mm,
            "ground_material": format!("{:?}", snap.ground_material),
            "active_scale": self.session.loupe.active_scale,
            "vibe_tick": vibe_tick,
        });
        Ok(telemetry.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_and_eval_both_vibe_modules() {
        let drive_src = include_str!("../../../vibe/ecopunk_continuum_drive.vibe");
        let lab_src = include_str!("../../../vibe/ecopunk_biological_lab.vibe");

        let drive_qbc =
            compile_vibe_source_to_qbc(drive_src).expect("drive .vibe must compile to .qbc");
        let lab_qbc =
            compile_vibe_source_to_qbc(lab_src).expect("lab .vibe must compile to .qbc");
        assert!(drive_qbc.starts_with(b"VBC1"));
        assert!(lab_qbc.starts_with(b"VBC1"));

        let mut host = EcopunkVibeHost::new();
        let mut env = Env::default();

        let drive_prog = load_program(drive_src).unwrap();
        let drive_res = eval_function(
            &drive_prog,
            "step_continuum_rover_tick",
            vec![
                Value::F64(1450.0),
                Value::F64(-980.0),
                Value::F64(0.25),
                Value::F64(12.0),
                Value::F64(0.8),
                Value::F64(0.1),
                Value::F64(1.0 / 64.0),
            ],
            &mut host,
            &mut env,
        )
        .expect("step_continuum_rover_tick must succeed");
        let json_drive = vibe_value_to_json(&drive_res);
        assert!(json_drive.get("rover").is_some());
        assert!(json_drive.get("nearest_site").is_some());

        let lab_prog = load_program(lab_src).unwrap();
        let mut env2 = Env::default();
        let lab_res = eval_function(
            &lab_prog,
            "evaluate_dissection_lab_state",
            vec![
                Value::I64(3),
                Value::F64(0.35),
                Value::F64(0.5),
                Value::F64(0.75),
                Value::F64(0.25),
                Value::I64(0),
                Value::I64(11),
            ],
            &mut host,
            &mut env2,
        )
        .expect("evaluate_dissection_lab_state must succeed");
        let json_lab = vibe_value_to_json(&lab_res);
        assert_eq!(json_lab.get("scale").and_then(|v| v.as_i64()), Some(3));
        assert!(json_lab.get("active_ontology_pin").is_some());
    }

    #[test]
    fn test_all_5_dissection_scales_build_valid_10d() {
        for scale in 1..=5 {
            let (bytes, meta) = build_dissection_scale_10d(scale, 0.25, 0.35, 0.65, 0.4, 0)
                .unwrap_or_else(|e| panic!("scale {scale} failed: {e}"));
            assert!(
                bytes.len() > 10_000,
                "scale {scale} .10d bytes too small: {}",
                bytes.len()
            );
            assert!(meta.get("vertex_count").and_then(|v| v.as_u64()).unwrap_or(0) >= 1800);
        }
    }
}
