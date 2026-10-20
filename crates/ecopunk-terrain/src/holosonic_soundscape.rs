//! Scene-Synchronized 3D Holosonic Wavefield & HRTF Audio Graph with Shared SDF + BVH Occlusion.
//!
//! Upgrades `ecopunk-terrain` with Epic 8 capabilities from `qualia-core-db`:
//! - Multi-bus scene audio graph integrating continuous Holosonic wavefield synthesis,
//!   HRTF binaural spatialization, DF2T biquad occlusion filtering, 6-tap early reflections,
//!   and stereo Freeverb late reverberation.
//! - Australian native biocenosis acoustic emitters:
//!   - Tumut River riffles (Hydrodynamic fluid resonance)
//!   - Yarrangobilly karst cave drips (Reverberant droplet ping)
//!   - Snow Gum wind canopy (*Eucalyptus pauciflora* foliage sibilance)
//!   - Superb Lyrebird mimicry (*Menura novaehollandiae*, 1250 Hz FM sweeps)
//!   - Laughing Kookaburra chorus (*Dacelo novaeguineae*, 680 Hz staccato calls)
//!   - Southern Cassowary infrasound boom (*Casuarius casuarius*, 23 Hz sub-audible resonant boom)
//!   - Yellow-Bellied Glider gurgle (*Petaurus australis*, 1450 Hz bubbling call)
//!   - Southern Corroboree Frog squelch (*Pseudophryne corroboree*, 215 Hz sphagnum squelch)
//!   - Gould's Wattled Bat sweep (*Chalinolobus gouldii*, 34 kHz echolocation down-sweep)
//! - Shared analytical SDF (sphere, capsule, rounded box) and BVH AABB acoustic obstacles
//!   with material-specific transmission absorption and reflection albedo:
//!   - Rock/Lithology (Yarrangobilly karst limestone canyon cliffs)
//!   - Timber/Trunk (Snow Gum trunks)
//!   - Foliage/Canopy (Subalpine scrub and sphagnum moss bogs)
//!   - Water Surface (Tumut River specular reflection)
//! - Zero-heap execution emitting 48-byte `HolosonicSonicToken48` records for `.q42` telemetry.

use qualia_core_db::audio::holosonic::{
    HolosonicEmitter, HolosonicModality, HolosonicPointSample, HolosonicSonicToken48,
    HolosonicWavefield,
};
use qualia_core_db::audio::scene_audio_graph::{
    evaluate_acoustic_occlusion_zero_heap, AcousticMaterial, AcousticObstacle,
    AcousticObstacleShape, AcousticOcclusionResult, AudioListenerPose, HolosonicNode,
    SceneAudioBusGraph,
};
use qualia_core_db::render::pga::Motor;
use qualia_core_db::sonic_token::SonicToken;

/// Maximum obstacles tracked in the zero-heap soundscape scene.
pub const MAX_SOUNDSCAPE_OBSTACLES: usize = 32;

/// Standard sample rate for high-fidelity Holosonic binaural rendering (48 kHz).
pub const HOLOSONIC_SAMPLE_RATE_48K: f32 = 48_000.0;

/// Freestanding helper to emit a 48-byte `HolosonicSonicToken48` telemetry record and
/// 64-bit packed `SonicToken` for `.q42` U3 telemetry ring buffers.
#[inline]
pub fn emit_sonic_token(
    wavefield: &HolosonicWavefield,
    emitter_idx: usize,
    origin_quin_hash: u64,
    tensor_index: u32,
    deontic_permit: u8,
    occlusion_gain: f32,
    sample: HolosonicPointSample,
    listener_pos: [f32; 3],
    time_s: f32,
) -> (HolosonicSonicToken48, SonicToken) {
    wavefield.emit_sonic_token(
        emitter_idx,
        origin_quin_hash,
        tensor_index,
        deontic_permit,
        occlusion_gain,
        sample,
        listener_pos,
        time_s,
    )
}

/// Australian Biocenosis Holosonic Soundscape & 3D Acoustic Wavefield Manager.
pub struct EcopunkHolosonicSoundscape {
    /// Internal zero-heap 4-bus scene audio graph.
    pub graph: SceneAudioBusGraph,
    /// Analytical SDF and BVH acoustic obstacles in the local sector.
    obstacles: [AcousticObstacle; MAX_SOUNDSCAPE_OBSTACLES],
    /// Active obstacle count.
    obstacle_count: usize,
    /// Sector identifier (e.g. "kosciuszko-yarrangobilly").
    pub sector_id: [u8; 32],
    /// Length of `sector_id` string.
    pub sector_id_len: usize,
    /// Cached latest point sample.
    pub latest_sample: HolosonicPointSample,
}

impl Default for EcopunkHolosonicSoundscape {
    fn default() -> Self {
        Self::new()
    }
}

impl EcopunkHolosonicSoundscape {
    /// Creates an empty Holosonic soundscape with default listener at origin.
    pub fn new() -> Self {
        let dummy_obstacle = AcousticObstacle {
            shape: AcousticObstacleShape::AnalyticalSphere {
                center: [0.0, 0.0, 0.0],
                radius: 0.0,
            },
            material: AcousticMaterial::RockLithology,
            thickness_m: 1.0,
            smooth_blend_radius: 0.25,
        };

        Self {
            graph: SceneAudioBusGraph::new(),
            obstacles: [dummy_obstacle; MAX_SOUNDSCAPE_OBSTACLES],
            obstacle_count: 0,
            sector_id: [0u8; 32],
            sector_id_len: 0,
            latest_sample: HolosonicPointSample {
                pressure_l: 0.0,
                pressure_r: 0.0,
                interference_gain: 1.0,
                active_emitter_count: 0,
            },
        }
    }

    /// Initializes and configures an Australian biocenosis soundscape for the given sector.
    pub fn with_australian_soundscape(sector_id: &str) -> Self {
        let mut soundscape = Self::new();
        soundscape.populate_sector_soundscape(sector_id);
        soundscape
    }

    /// Configures the sector-specific acoustic emitters and analytical SDF/AABB obstacles.
    pub fn populate_sector_soundscape(&mut self, sector_id: &str) {
        self.graph.clear_emitters();
        self.obstacle_count = 0;

        let bytes = sector_id.as_bytes();
        let len = bytes.len().min(32);
        self.sector_id[..len].copy_from_slice(&bytes[..len]);
        self.sector_id_len = len;

        // ─────────────────────────────────────────────────────────────────────
        // 1. Sector Environmental Emitters
        // ─────────────────────────────────────────────────────────────────────

        // A. Tumut River Riffles: Hydrodynamic fluid resonance across granite riverbed
        let tumut_river = HolosonicNode {
            emitter: HolosonicEmitter::new(
                [0.0, -1.5, 15.0],
                340.0,
                0.75,
                0.0,
                0.85,
                65.0,
                HolosonicModality::WaterResonance,
            ),
            node_id: 101,
            origin_quin_hash: 0x5455_4D55_545F_5249, // "TUMUT_RI"
            deontic_permit: 0,
            is_ambient: false,
            sdf_primitive: 1, // RoundedBox / River corridor
            reserved_u8: 0,
            velocity: [1.2, 0.0, 0.4], // Water surface velocity vector
            sdf_half_extents: [12.0, 0.5, 40.0],
        };
        self.graph.push_emitter(tumut_river);

        // B. Yarrangobilly Karst Cave Drips: Reverberant calcium carbonate droplet resonance
        let karst_drips = HolosonicNode {
            emitter: HolosonicEmitter::new(
                [22.0, -4.0, 30.0],
                520.0,
                0.55,
                0.35,
                0.96,
                40.0,
                HolosonicModality::WaterResonance,
            ),
            node_id: 102,
            origin_quin_hash: 0x5941_5252_414E_474F, // "YARRANGO"
            deontic_permit: 0,
            is_ambient: false,
            sdf_primitive: 0, // Sphere pool
            reserved_u8: 0,
            velocity: [0.0, -9.8, 0.0], // Falling droplet gravity vector
            sdf_half_extents: [1.5, 1.5, 1.5],
        };
        self.graph.push_emitter(karst_drips);

        // C. Snow Gum Wind Canopy (*Eucalyptus pauciflora* wind sibilance)
        let snow_gum_wind = HolosonicNode {
            emitter: HolosonicEmitter::new(
                [-14.0, 10.0, -18.0],
                220.0,
                0.68,
                0.15,
                0.72,
                85.0,
                HolosonicModality::FloraWind,
            ),
            node_id: 103,
            origin_quin_hash: 0x534E_4F57_5F47_554D, // "SNOW_GUM"
            deontic_permit: 0,
            is_ambient: false,
            sdf_primitive: 2, // Capsule branch canopy
            reserved_u8: 0,
            velocity: [3.5, 0.2, -1.8], // Alpine wind gust velocity
            sdf_half_extents: [8.0, 6.0, 8.0],
        };
        self.graph.push_emitter(snow_gum_wind);

        // ─────────────────────────────────────────────────────────────────────
        // 2. Australian Fauna Calls
        // ─────────────────────────────────────────────────────────────────────

        // D. Superb Lyrebird (*Menura novaehollandiae*): Complex mimicry sweeps
        let lyrebird = HolosonicNode {
            emitter: HolosonicEmitter::new(
                [28.0, 1.2, 42.0],
                1250.0,
                0.88,
                0.0,
                0.93,
                95.0,
                HolosonicModality::Bioacoustic,
            ),
            node_id: 201,
            origin_quin_hash: 0x4C59_5245_4249_5244, // "LYREBIRD"
            deontic_permit: 0,
            is_ambient: false,
            sdf_primitive: 0,
            reserved_u8: 0,
            velocity: [0.0, 0.0, 0.0],
            sdf_half_extents: [0.4, 0.4, 0.4],
        };
        self.graph.push_emitter(lyrebird);

        // E. Laughing Kookaburra (*Dacelo novaeguineae*): Staccato laughing chorus
        let kookaburra = HolosonicNode {
            emitter: HolosonicEmitter::new(
                [-22.0, 14.0, 26.0],
                680.0,
                0.90,
                0.45,
                0.89,
                110.0,
                HolosonicModality::Bioacoustic,
            ),
            node_id: 202,
            origin_quin_hash: 0x4B4F_4F4B_4142_5552, // "KOOKABUR"
            deontic_permit: 0,
            is_ambient: false,
            sdf_primitive: 0,
            reserved_u8: 0,
            velocity: [0.0, 0.0, 0.0],
            sdf_half_extents: [0.35, 0.35, 0.35],
        };
        self.graph.push_emitter(kookaburra);

        // F. Southern Cassowary (*Casuarius casuarius*): 23 Hz infrasound resonant boom
        let cassowary = HolosonicNode {
            emitter: HolosonicEmitter::new(
                [45.0, 0.8, -48.0],
                23.0, // 23 Hz sub-audible infrasound boom
                0.98,
                0.0,
                0.99,
                180.0, // Infrasound propagates extensive distances
                HolosonicModality::Bioacoustic,
            ),
            node_id: 203,
            origin_quin_hash: 0x4341_5353_4F57_4152, // "CASSOWAR"
            deontic_permit: 0,
            is_ambient: false,
            sdf_primitive: 0,
            reserved_u8: 0,
            velocity: [0.5, 0.0, 0.0],
            sdf_half_extents: [0.9, 1.4, 0.9],
        };
        self.graph.push_emitter(cassowary);

        // G. Yellow-Bellied Glider (*Petaurus australis*): High screech & bubbling gurgle
        let glider = HolosonicNode {
            emitter: HolosonicEmitter::new(
                [-8.0, 18.0, -32.0],
                1450.0,
                0.62,
                0.2,
                0.82,
                70.0,
                HolosonicModality::Bioacoustic,
            ),
            node_id: 204,
            origin_quin_hash: 0x474C_4944_4552_5F59, // "GLIDER_Y"
            deontic_permit: 0,
            is_ambient: false,
            sdf_primitive: 0,
            reserved_u8: 0,
            velocity: [4.0, -0.8, 2.0], // Volant glide trajectory
            sdf_half_extents: [0.25, 0.25, 0.4],
        };
        self.graph.push_emitter(glider);

        // H. Southern Corroboree Frog (*Pseudophryne corroboree*): Sphagnum squelch
        let frog = HolosonicNode {
            emitter: HolosonicEmitter::new(
                [6.0, 0.1, 10.0],
                215.0,
                0.78,
                0.0,
                0.96,
                24.0,
                HolosonicModality::Bioacoustic,
            ),
            node_id: 205,
            origin_quin_hash: 0x434F_5252_4F42_4F52, // "CORROBOR"
            deontic_permit: 0,
            is_ambient: false,
            sdf_primitive: 0,
            reserved_u8: 0,
            velocity: [0.0, 0.0, 0.0],
            sdf_half_extents: [0.05, 0.05, 0.05],
        };
        self.graph.push_emitter(frog);

        // I. Gould's Wattled Bat (*Chalinolobus gouldii*): 34 kHz echolocation sweep
        // Normalized base frequency for telemetry and audible downmixing
        let bat = HolosonicNode {
            emitter: HolosonicEmitter::new(
                [0.0, 24.0, 8.0],
                3400.0, // Scaled for audio bus representation (34 kHz ultrasonic down-sweep)
                0.58,
                0.85,
                0.97,
                45.0,
                HolosonicModality::Bioacoustic,
            ),
            node_id: 206,
            origin_quin_hash: 0x4241_545F_474F_554C, // "BAT_GOUL"
            deontic_permit: 0,
            is_ambient: false,
            sdf_primitive: 0,
            reserved_u8: 0,
            velocity: [-12.0, 1.2, 8.0], // Rapid aerial flight vector
            sdf_half_extents: [0.15, 0.15, 0.15],
        };
        self.graph.push_emitter(bat);

        // ─────────────────────────────────────────────────────────────────────
        // 3. Analytical SDF and BVH Acoustic Obstacles
        // ─────────────────────────────────────────────────────────────────────

        // Obstacle 1: Yarrangobilly Karst Limestone Canyon Wall (RockLithology)
        self.add_obstacle(AcousticObstacle {
            shape: AcousticObstacleShape::AnalyticalRoundedBox {
                center: [18.0, 6.0, 12.0],
                half_extents: [4.0, 18.0, 28.0],
                round_radius: 1.5,
            },
            material: AcousticMaterial::RockLithology,
            thickness_m: 8.0,
            smooth_blend_radius: 0.5,
        });

        // Obstacle 2: Ancient Snow Gum Trunk (TimberTrunk)
        self.add_obstacle(AcousticObstacle {
            shape: AcousticObstacleShape::AnalyticalCapsule {
                a: [-7.0, 0.0, -10.0],
                b: [-7.0, 15.0, -10.0],
                radius: 1.2,
            },
            material: AcousticMaterial::TimberTrunk,
            thickness_m: 2.4,
            smooth_blend_radius: 0.35,
        });

        // Obstacle 3: Subalpine Sphagnum Bog & Scrub Canopy (FoliageCanopy)
        self.add_obstacle(AcousticObstacle {
            shape: AcousticObstacleShape::AnalyticalSphere {
                center: [7.0, 1.5, 11.0],
                radius: 5.5,
            },
            material: AcousticMaterial::FoliageCanopy,
            thickness_m: 4.2,
            smooth_blend_radius: 0.75,
        });

        // Obstacle 4: Tumut River Water Surface (WaterSurface reflection plane)
        self.add_obstacle(AcousticObstacle {
            shape: AcousticObstacleShape::Aabb {
                min: [-15.0, -1.8, -20.0],
                max: [15.0, -1.2, 50.0],
            },
            material: AcousticMaterial::WaterSurface,
            thickness_m: 1.2,
            smooth_blend_radius: 0.2,
        });
    }

    /// Appends an acoustic obstacle to the soundscape scene.
    pub fn add_obstacle(&mut self, obstacle: AcousticObstacle) -> bool {
        if self.obstacle_count >= MAX_SOUNDSCAPE_OBSTACLES {
            return false;
        }
        self.obstacles[self.obstacle_count] = obstacle;
        self.obstacle_count += 1;
        true
    }

    /// Appends a new Holosonic emitter node.
    pub fn add_emitter(&mut self, node: HolosonicNode) -> bool {
        self.graph.push_emitter(node)
    }

    /// Sets the observer listener pose directly in world coordinates.
    pub fn set_listener_pose(
        &mut self,
        position: [f32; 3],
        forward: [f32; 3],
        up: [f32; 3],
        velocity: [f32; 3],
        manifold_w: f32,
    ) {
        let f_len = (forward[0] * forward[0] + forward[1] * forward[1] + forward[2] * forward[2]).sqrt();
        let f_norm = if f_len > 1e-6 {
            [forward[0] / f_len, forward[1] / f_len, forward[2] / f_len]
        } else {
            [0.0, 0.0, -1.0]
        };

        let u_len = (up[0] * up[0] + up[1] * up[1] + up[2] * up[2]).sqrt();
        let u_norm = if u_len > 1e-6 {
            [up[0] / u_len, up[1] / u_len, up[2] / u_len]
        } else {
            [0.0, 1.0, 0.0]
        };

        // Right vector = forward x up
        let right = [
            f_norm[1] * u_norm[2] - f_norm[2] * u_norm[1],
            f_norm[2] * u_norm[0] - f_norm[0] * u_norm[2],
            f_norm[0] * u_norm[1] - f_norm[1] * u_norm[0],
        ];

        let yaw_rad = f_norm[0].atan2(-f_norm[2]);

        self.graph.listener = AudioListenerPose {
            position,
            forward: f_norm,
            up: u_norm,
            right,
            velocity,
            yaw_rad,
            manifold_w,
        };
    }

    /// Sets the observer listener pose synchronized from a camera PGA [`Motor`].
    #[inline]
    pub fn set_listener_motor(&mut self, motor: Motor, velocity: [f32; 3], manifold_w: f32) {
        self.graph.sync_listener_from_motor(motor, velocity, manifold_w);
    }

    /// Evaluates zero-heap acoustic occlusion for an arbitrary point against the soundscape's obstacles.
    #[inline]
    pub fn evaluate_occlusion_for_point(&self, emitter_pos: [f32; 3]) -> AcousticOcclusionResult {
        let mut result = AcousticOcclusionResult::default();
        evaluate_acoustic_occlusion_zero_heap(
            self.graph.listener.position,
            emitter_pos,
            &self.obstacles[..self.obstacle_count],
            &mut result,
        );
        result
    }

    /// Synthesizes and mixes a stereo frame buffer of Holosonic audio across all four buses in zero heap.
    ///
    /// Evaluates continuous wave superposition, DF2T biquad occlusion, 6-tap early reflections,
    /// and stereo Freeverb late reverberation, emitting telemetry tokens into `graph.telemetry_bus`.
    pub fn render_frame_zero_heap(
        &mut self,
        time_s: f32,
        sample_rate: f32,
        out_l: &mut [f32],
        out_r: &mut [f32],
    ) -> HolosonicPointSample {
        let sample = self.graph.render_quantum_zero_heap(
            time_s,
            sample_rate,
            &self.obstacles[..self.obstacle_count],
            out_l,
            out_r,
        );
        self.latest_sample = sample;
        sample
    }

    /// Drains emitted 48-byte `HolosonicSonicToken48` telemetry records into `out_tokens`.
    pub fn drain_telemetry_tokens(&mut self, out_tokens: &mut [HolosonicSonicToken48]) -> usize {
        let mut count = 0usize;
        while count < out_tokens.len() {
            match self.graph.telemetry_bus.pop() {
                Some(tok) => {
                    out_tokens[count] = tok;
                    count += 1;
                }
                None => break,
            }
        }
        count
    }

    /// Returns the active emitter count.
    #[inline]
    pub fn emitter_count(&self) -> usize {
        self.graph.emitter_count
    }

    /// Returns the active obstacle count.
    #[inline]
    pub fn obstacle_count(&self) -> usize {
        self.obstacle_count
    }

    /// Returns a slice of active acoustic obstacles.
    #[inline]
    pub fn obstacles(&self) -> &[AcousticObstacle] {
        &self.obstacles[..self.obstacle_count]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::sonic_token::SONIC_MAGIC;

    #[test]
    fn soundscape_initializes_with_australian_fauna_and_landscape() {
        let soundscape = EcopunkHolosonicSoundscape::with_australian_soundscape("kosciuszko-east");
        assert!(soundscape.emitter_count() >= 9, "Should register all 9 Australian biocenosis emitters");
        assert!(soundscape.obstacle_count() >= 4, "Should register 4 SDF/BVH obstacles");

        // Verify Lyrebird, Cassowary, River, and Frog nodes are present
        let mut found_cassowary = false;
        let mut found_lyrebird = false;
        let mut found_river = false;
        for i in 0..soundscape.emitter_count() {
            let node = &soundscape.graph.emitters[i];
            if node.node_id == 203 && node.emitter.base_freq_hz == 23.0 {
                found_cassowary = true;
            }
            if node.node_id == 201 && node.emitter.base_freq_hz == 1250.0 {
                found_lyrebird = true;
            }
            if node.node_id == 101 {
                found_river = true;
            }
        }
        assert!(found_cassowary, "Cassowary 23 Hz emitter must be present");
        assert!(found_lyrebird, "Lyrebird 1250 Hz emitter must be present");
        assert!(found_river, "Tumut River riffles emitter must be present");
    }

    #[test]
    fn canyon_and_tree_trunk_occlusion_attenuates_audio() {
        let mut soundscape = EcopunkHolosonicSoundscape::new();
        // Add a dense rock obstacle at z = 10.0
        soundscape.add_obstacle(AcousticObstacle {
            shape: AcousticObstacleShape::AnalyticalRoundedBox {
                center: [0.0, 0.0, 10.0],
                half_extents: [10.0, 10.0, 2.0],
                round_radius: 0.5,
            },
            material: AcousticMaterial::RockLithology,
            thickness_m: 4.0,
            smooth_blend_radius: 0.25,
        });

        soundscape.set_listener_pose([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [0.0; 3], 0.0);

        // Occluded emitter behind the rock wall at z = 20.0
        let occ = soundscape.evaluate_occlusion_for_point([0.0, 0.0, 20.0]);
        assert!(occ.hit_count > 0, "Ray through rock wall must register hit");
        assert!(occ.direct_gain < 0.25, "Rock wall must significantly attenuate direct gain");
        assert!(occ.lowpass_cutoff_hz < 5000.0, "Rock wall must low-pass filter acoustic energy");

        // Unoccluded emitter off to the side at x = 50.0
        let unocc = soundscape.evaluate_occlusion_for_point([50.0, 0.0, 0.0]);
        assert_eq!(unocc.hit_count, 0);
        assert!((unocc.direct_gain - 1.0).abs() < 1e-4);
    }

    #[test]
    fn render_frame_produces_binaural_audio_and_emits_telemetry_tokens() {
        let mut soundscape = EcopunkHolosonicSoundscape::with_australian_soundscape("yarrangobilly");
        soundscape.set_listener_pose([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [0.0; 3], 0.0);

        let mut out_l = [0.0_f32; 128];
        let mut out_r = [0.0_f32; 128];

        let sample = soundscape.render_frame_zero_heap(0.0, HOLOSONIC_SAMPLE_RATE_48K, &mut out_l, &mut out_r);

        assert!(sample.active_emitter_count > 0, "Active emitters should contribute");
        let max_l = out_l.iter().fold(0.0_f32, |m, &x| m.max(x.abs()));
        let max_r = out_r.iter().fold(0.0_f32, |m, &x| m.max(x.abs()));
        assert!(max_l > 1e-5 || max_r > 1e-5, "Stereo audio frame should contain acoustic energy");

        // Drain telemetry tokens
        let mut tokens = [HolosonicSonicToken48::default(); 16];
        let drained = soundscape.drain_telemetry_tokens(&mut tokens);
        assert!(drained > 0, "Rendering should emit telemetry tokens for active emitters");
        assert_eq!(tokens[0].flags, SONIC_MAGIC as u16);
    }

    #[test]
    fn cassowary_infrasound_low_freq_registered() {
        let soundscape = EcopunkHolosonicSoundscape::with_australian_soundscape("daintree-corridor");
        let cassowary_node = soundscape
            .graph
            .emitters
            .iter()
            .take(soundscape.emitter_count())
            .find(|n| n.node_id == 203)
            .expect("Cassowary node present");
        assert_eq!(cassowary_node.emitter.base_freq_hz, 23.0);
        assert_eq!(cassowary_node.emitter.modality, HolosonicModality::Bioacoustic);
        assert!(cassowary_node.emitter.coherence_radius >= 150.0);
    }
}
