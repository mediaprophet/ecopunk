//! 64 Hz Vehicle Driving, Dismount Anywhere, 3D Walking & Seamless Loupe Transition (`exploration_controller.rs`).
//!
//! Epic 5 of the Ecopunk Biological Education Assets and Explorable World Plan.
//! Unites:
//! - 64 Hz 4-wheel rover vehicle kinematics with Ackermann steering, independent spring-damper
//!   suspension, and Bekker geotechnical soil compaction/sinkage across the multi-kilometer corridor.
//! - Dismount anywhere: step out of the field vehicle at ANY coordinate in the 26.5 km world
//!   (zero coordinate pens or 11m×14m clamps) and transition into unrestricted 3D capsule walking,
//!   rock hopping, river wading, swimming, and subterranean karst cave exploration.
//! - Proximity-triggered field study prompts when near botanical or zoological specimens.
//! - Seamless 5-scale Specimen Loupe transition zooming from whole organism (`Scale 1`) down to
//!   cellular stomatal histology (`Scale 4`) or vertebrate osteology/myology (`Scale 5`).

use qualia_sim::character::{CharacterConfig, CharacterController};
use qualia_sim::hydro::SubmersionState;
use qualia_sim::kinematics::{RoverKinematics, VehicleChassisConfig, VehicleState, WheelConfig};

use crate::world_continuum::{RegionalCorridorWorld, StudySiteId, SurfaceMaterialKind};

// ============================================================================
// 1. Exploration & Camera Modes
// ============================================================================

/// Four-mode seamless exploration state machine.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExplorationMode {
    /// Mode 0: Driving the 4WD field vehicle across the highway, fire-trails, or off-road.
    RoverDriving = 0,
    /// Mode 1: Unrestricted 3D on-foot capsule bushwalking, rock climbing, and cave exploring.
    OnFootExploration = 1,
    /// Mode 2: In close proximity to a tagged botanical, zoological, or geological study target.
    FieldStudySiteInspection = 2,
    /// Mode 3: Interactive 5-scale 3D anatomical dissection and microscopic histology loupe.
    Interactive3dSpecimenLoupe = 3,
}

/// Dynamic exploration pursuit camera modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExplorationCameraMode {
    /// Third-person spring-damper chase camera behind 4WD rover.
    RoverChaseCam = 0,
    /// First-person driver's cab / hood camera inside the vehicle.
    RoverCabFirstPerson = 1,
    /// Third-person over-the-shoulder camera trailing the on-foot bushwalker.
    WalkerThirdPerson = 2,
    /// First-person eye-level camera with simulated field binoculars / loupe optics.
    WalkerFirstPersonFieldOptics = 3,
    /// Precision orbital inspection camera for 5-scale anatomical specimen loupe.
    LoupeOrbitalMicroscope = 4,
}

// ============================================================================
// 2. Specimen Loupe Dissection State
// ============================================================================

/// Dynamic anatomical dissection, tissue peel, and phenological state in the 3D Specimen Loupe.
#[derive(Debug, Clone, PartialEq)]
pub struct LoupeDissectionState {
    /// Active biological scale (1..=5):
    /// - 1: Whole Organism Macroform (Canopy / Habit)
    /// - 2: Stem, Bark Wedge & Secondary Xylem Radial Cross-Section
    /// - 3: Reproductive Organ Dissection (Flower / Operculum / Woody Gumnut)
    /// - 4: Foliar Lamina Venation & Cellular Microscopic Histology
    /// - 5: Comparative Vertebrate 3-Layer Anatomy (Osteology / Myology / Integument)
    pub active_scale: u8,
    /// Exploded view layer separation ratio in `[0.0, 1.0]`.
    pub explode_layers_01: f32,
    /// Anatomical cutaway slicing plane progress in `[0.0, 1.0]`.
    pub cutaway_plane_01: f32,
    /// Seasonal phenology month (`1..=12`, 1 = January, 12 = December).
    pub phenology_month: u8,
    /// Ontogenetic developmental progress in `[0.0, 1.0]` (Seedling -> Ancient).
    pub ontogenetic_progress_01: f32,
    /// Skeletal system (osteology) layer visibility in `[0.0, 1.0]`.
    pub osteology_vis_01: f32,
    /// Muscular system (myology) layer visibility in `[0.0, 1.0]`.
    pub myology_vis_01: f32,
    /// External skin/pelage/plumage (integument) layer visibility in `[0.0, 1.0]`.
    pub integument_vis_01: f32,
    /// Active biological ontology CURIE (e.g. `po:0009010`, `po:0000032`, `uberon:0002415`).
    pub selected_ontology_curie: &'static str,
}

impl Default for LoupeDissectionState {
    fn default() -> Self {
        Self {
            active_scale: 1,
            explode_layers_01: 0.0,
            cutaway_plane_01: 0.0,
            phenology_month: 1,
            ontogenetic_progress_01: 0.65,
            osteology_vis_01: 1.0,
            myology_vis_01: 1.0,
            integument_vis_01: 1.0,
            selected_ontology_curie: "po:0009010",
        }
    }
}

// ============================================================================
// 3. Tick Inputs & Telemetry Snapshots
// ============================================================================

/// Input state for driving the 4WD rover.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RoverInput {
    pub throttle: f32,
    pub steer: f32,
    pub brake: f32,
}

/// Input state for the on-foot 3D character.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CharacterInput {
    pub wish_dir: [f32; 2],
    pub sprint: bool,
    pub jump: bool,
    pub crouch: bool,
}

/// Unified tick input packet delivered at 64 Hz.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ExplorationTickInput {
    /// Rover throttle input in `[-1.0, 1.0]` (negative is reverse).
    pub throttle: f32,
    /// Rover steering input in `[-1.0, 1.0]` (positive is right turn).
    pub steer: f32,
    /// Rover brake input in `[0.0, 1.0]`.
    pub brake: f32,
    /// Character walking wish direction `[forward, strafe_right]` in `[-1.0, 1.0]`.
    pub wish_dir: [f32; 2],
    /// Mouse / analog look delta yaw in radians.
    pub look_delta_yaw: f32,
    /// Mouse / analog look delta pitch in radians.
    pub look_delta_pitch: f32,
    /// Sprint key held.
    pub sprint: bool,
    /// Jump key pressed on this tick.
    pub jump: bool,
    /// Crouch key held.
    pub crouch: bool,
    /// Dismount / mount key `[E]` pressed on this tick.
    pub toggle_dismount: bool,
    /// Inspect specimen in 3D Loupe key `[F]` pressed on this tick.
    pub toggle_loupe: bool,
    /// Cycle camera mode key `[C]` pressed on this tick.
    pub cycle_camera: bool,
    /// Requested change to loupe dissection scale (1..=5).
    pub loupe_scale_change: Option<u8>,
    /// Incremental delta applied to cutaway plane slider.
    pub loupe_cutaway_delta: f32,
    /// Incremental delta applied to layer explosion slider.
    pub loupe_explode_delta: f32,
}

/// Rich telemetry snapshot output by each 64 Hz exploration tick.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplorationTelemetrySnapshot {
    /// Current exploration mode.
    pub mode: ExplorationMode,
    /// Current active camera mode.
    pub camera_mode: ExplorationCameraMode,
    /// World-space camera eye position `[x, y, z]` in meters.
    pub camera_eye: [f32; 3],
    /// World-space camera look target position `[x, y, z]` in meters.
    pub camera_target: [f32; 3],
    /// Rover world position `[x, y, z]` in meters.
    pub rover_position: [f32; 3],
    /// Rover world velocity vector `[vx, vy, vz]` in m/s.
    pub rover_velocity: [f32; 3],
    /// Rover forward speed in km/h.
    pub rover_speed_kmh: f32,
    /// Bushwalker on-foot world position `[x, y, z]` in meters.
    pub walker_position: [f32; 3],
    /// Bushwalker world velocity vector `[vx, vy, vz]` in m/s.
    pub walker_velocity: [f32; 3],
    /// Bushwalker water submersion state (Dry, Wading, Swimming, Diving).
    pub walker_submersion: SubmersionState,
    /// True if bushwalker is contacting ground.
    pub walker_is_grounded: bool,
    /// Total cumulative vehicle odometer distance in kilometers.
    pub total_odometer_km: f32,
    /// Current position along the 26.5 km road corridor in kilometers.
    pub current_corridor_km: f32,
    /// Nearest high-detail ecological study site.
    pub nearest_study_site: StudySiteId,
    /// Euclidean distance in meters to the nearest study site.
    pub nearest_site_distance_m: f32,
    /// Bekker geotechnical soil compaction sinkage in millimeters.
    pub soil_sinkage_mm: f32,
    /// Current terrain surface material kind.
    pub ground_material: SurfaceMaterialKind,
    /// Ground elevation in meters AHD.
    pub elevation_ahd_m: f32,
    /// Water surface elevation if in or near water.
    pub water_surface_m: Option<f32>,
    /// Karst cave ceiling overhang elevation in meters AHD (`f32::INFINITY` outdoors).
    pub cave_ceiling_m: f32,
    /// Contextual prompt shown to the user when near a study target.
    pub study_target_prompt: Option<&'static str>,
    /// Active biological scale in 3D Loupe.
    pub active_scale: u8,
    /// Active biological ontology CURIE.
    pub active_ontology_curie: &'static str,
}

// ============================================================================
// 4. Continuum Exploration Session
// ============================================================================

/// Unified 64 Hz interactive exploration session.
///
/// Integrates the 26.5 km multi-kilometer terrain corridor, 4-wheel rover vehicle kinematics,
/// unrestricted 3D on-foot character controller, Bekker soil mechanics, hydro buoyancy,
/// and the 5-scale Specimen Loupe.
pub struct ContinuumExplorationSession {
    /// The multi-kilometer terrain continuum and road network.
    pub world: RegionalCorridorWorld,
    /// Dynamic 4-wheel vehicle kinematics simulator.
    pub rover: RoverKinematics,
    /// Unrestricted 3D kinematic bushwalker character controller.
    pub walker: CharacterController,
    /// Current exploration mode.
    pub mode: ExplorationMode,
    /// Current camera mode.
    pub camera_mode: ExplorationCameraMode,
    /// 5-scale anatomical Specimen Loupe state.
    pub loupe: LoupeDissectionState,
    /// Parked pose of the vehicle while player is exploring on foot.
    pub parked_rover: Option<VehicleState>,
    /// Bekker soil sinkage depth in millimeters.
    pub soil_sinkage_mm: f32,
    /// Cumulative odometer in kilometers.
    pub total_odometer_km: f32,
    /// Nearest study site.
    pub nearest_site: StudySiteId,
    /// Distance in meters to nearest study site.
    pub nearest_site_dist_m: f32,
    /// Contextual user action prompt.
    pub study_target_prompt: Option<&'static str>,
    /// Camera orbital / look yaw in radians.
    pub camera_yaw: f32,
    /// Camera orbital / look pitch in radians.
    pub camera_pitch: f32,
}

impl ContinuumExplorationSession {
    /// Creates a new exploration session starting at Site 0 (The Pines Foreshore).
    pub fn new(world: RegionalCorridorWorld) -> Self {
        let site0 = &world.sites[0];
        let start_pos = [site0.world_pos_xz[0], site0.elevation_ahd_m + 0.6, site0.world_pos_xz[1]];

        let chassis = VehicleChassisConfig::default();
        let wheel = WheelConfig::default();
        let rover = RoverKinematics::new(chassis, wheel, start_pos);

        let char_cfg = CharacterConfig::default();
        let walker = CharacterController::new(char_cfg, [start_pos[0] + 2.2, start_pos[1], start_pos[2]]);

        Self {
            world,
            rover,
            walker,
            mode: ExplorationMode::RoverDriving,
            camera_mode: ExplorationCameraMode::RoverChaseCam,
            loupe: LoupeDissectionState::default(),
            parked_rover: None,
            soil_sinkage_mm: 1.2,
            total_odometer_km: 0.0,
            nearest_site: StudySiteId::ThePinesForeshore,
            nearest_site_dist_m: 0.0,
            study_target_prompt: None,
            camera_yaw: 0.0,
            camera_pitch: 0.25,
        }
    }

    /// Teleports the player and field vehicle instantly to any of the 8 study sites.
    pub fn teleport_to_study_site(&mut self, site: StudySiteId) {
        let desc = &self.world.sites[site.site_index()];
        let (elev, _, _) =
            self.world.sample_world_elevation_and_normal_zero_heap(desc.world_pos_xz[0], desc.world_pos_xz[1]);

        let target_pos = [desc.world_pos_xz[0], elev + 0.6, desc.world_pos_xz[1]];

        self.rover.state.position = target_pos;
        self.rover.state.velocity = [0.0, 0.0, 0.0];
        self.rover.state.steering_angle = 0.0;
        self.rover.state.pitch = 0.0;
        self.rover.state.roll = 0.0;

        let right = [self.rover.state.heading.cos(), 0.0, -self.rover.state.heading.sin()];
        self.walker.state.position = [
            target_pos[0] + right[0] * 2.2,
            elev,
            target_pos[2] + right[2] * 2.2,
        ];
        self.walker.state.velocity = [0.0, 0.0, 0.0];

        self.nearest_site = site;
        self.nearest_site_dist_m = 0.0;
        self.parked_rover = None;
    }

    /// Advances the exploration simulation by one deterministic 64 Hz timestep (`dt_s = 1/64`).
    ///
    /// Evaluates vehicle suspension, on-foot locomotion, Bekker compaction, water buoyancy,
    /// cave overhang collisions, and Specimen Loupe dissection with zero heap allocations.
    pub fn step_tick_zero_heap(
        &mut self,
        input: &ExplorationTickInput,
        dt_s: f32,
    ) -> ExplorationTelemetrySnapshot {
        // Update general look angles
        self.camera_yaw += input.look_delta_yaw;
        self.camera_pitch = (self.camera_pitch + input.look_delta_pitch).clamp(-1.3, 1.3);

        // 1. Advance simulation according to current exploration mode
        match self.mode {
            ExplorationMode::RoverDriving => {
                // Step 4-wheel rover vehicle suspension & Ackermann kinematics
                self.rover.step(input.throttle, input.steer, input.brake, dt_s, |wx, wz| {
                    let (y, norm, _) = self.world.sample_world_elevation_and_normal_zero_heap(wx, wz);
                    (y, norm)
                });

                // Compute Bekker geotechnical soil compaction sinkage
                let rpos = self.rover.state.position;
                let (_, _, mat) = self.world.sample_world_elevation_and_normal_zero_heap(rpos[0], rpos[2]);

                self.soil_sinkage_mm = match mat {
                    SurfaceMaterialKind::RoadTarmac => 1.2,
                    SurfaceMaterialKind::GraniteBedrock | SurfaceMaterialKind::CoastalBasalt => 1.5,
                    SurfaceMaterialKind::LimestoneKarst | SurfaceMaterialKind::SandstoneLedge => 2.0,
                    SurfaceMaterialKind::FireTrailGravel => 5.5,
                    SurfaceMaterialKind::RiverPebble => 9.0,
                    SurfaceMaterialKind::AlluvialLoam | SurfaceMaterialKind::ForestHumusLoam => {
                        let soil = self.world.sample_soil_profile(rpos[0], rpos[2]);
                        let c = soil.surface().cohesion_kpa.max(1.0);
                        (120.0 / (c + 5.0)).clamp(4.0, 22.0)
                    }
                    SurfaceMaterialKind::SubalpinePeatBog => 32.0,
                };

                // Track cumulative odometer
                self.total_odometer_km = (self.rover.state.odometer / 1000.0) as f32;

                // Track nearest site
                let (site, dist) = self.world.locate_nearest_study_site_zero_heap(rpos[0], rpos[2]);
                self.nearest_site = site;
                self.nearest_site_dist_m = dist;
                self.study_target_prompt = None;

                // Handle Camera cycle
                if input.cycle_camera {
                    self.camera_mode = match self.camera_mode {
                        ExplorationCameraMode::RoverChaseCam => ExplorationCameraMode::RoverCabFirstPerson,
                        _ => ExplorationCameraMode::RoverChaseCam,
                    };
                }

                // Handle Dismount Anywhere (Key [E])
                // Allows stepping out at ANY coordinate across the entire 26.5 km continuum
                if input.toggle_dismount {
                    let speed_m_s = self.rover.speed_kmh() / 3.6;
                    if speed_m_s < 2.0 {
                        // Park vehicle at its exact 3D world pose
                        self.parked_rover = Some(self.rover.state);

                        // Spawn bushwalker 2.2 m beside the driver's door
                        let heading = self.rover.state.heading;
                        let right = [heading.cos(), 0.0, -heading.sin()];
                        let spawn_x = rpos[0] + right[0] * 2.2;
                        let spawn_z = rpos[2] + right[2] * 2.2;
                        let (spawn_y, _, _) =
                            self.world.sample_world_elevation_and_normal_zero_heap(spawn_x, spawn_z);

                        self.walker.state.position = [spawn_x, spawn_y, spawn_z];
                        self.walker.state.velocity = [0.0, 0.0, 0.0];
                        self.walker.state.yaw = heading;

                        self.mode = ExplorationMode::OnFootExploration;
                        self.camera_mode = ExplorationCameraMode::WalkerThirdPerson;
                    }
                }
            }

            ExplorationMode::OnFootExploration | ExplorationMode::FieldStudySiteInspection => {
                // Update character look
                self.walker.state.yaw += input.look_delta_yaw;
                self.walker.state.pitch = (self.walker.state.pitch + input.look_delta_pitch).clamp(-1.4, 1.4);

                // Step 3D character controller with fluid locomotion
                let _ = self.walker.step_with_water(
                    input.wish_dir,
                    input.sprint,
                    input.jump,
                    input.crouch,
                    dt_s,
                    |wx, wz| {
                        let (y, norm, _) = self.world.sample_world_elevation_and_normal_zero_heap(wx, wz);
                        (y, norm)
                    },
                    |wx, wz| {
                        self.world
                            .sample_water_surface_zero_heap(wx, wz)
                            .unwrap_or(-1000.0)
                    },
                );

                // Enforce Yarrangobilly karst cave ceiling overhang collision
                let ceil_y = self.world.sample_ceiling_overhang_zero_heap(
                    self.walker.state.position[0],
                    self.walker.state.position[2],
                );
                let char_head = self.walker.state.position[1] + self.walker.config.height;
                if char_head > ceil_y {
                    self.walker.state.position[1] = ceil_y - self.walker.config.height;
                    if self.walker.state.velocity[1] > 0.0 {
                        self.walker.state.velocity[1] = 0.0;
                    }
                }

                // Update nearest study site
                let (site, dist) = self.world.locate_nearest_study_site_zero_heap(
                    self.walker.state.position[0],
                    self.walker.state.position[2],
                );
                self.nearest_site = site;
                self.nearest_site_dist_m = dist;

                // Contextual study site inspection proximity check (< 25 m from site target)
                if dist <= 25.0 {
                    self.mode = ExplorationMode::FieldStudySiteInspection;
                    self.study_target_prompt = Some("[F] Examine Specimen in 3D Botanical Loupe");
                } else {
                    if self.mode == ExplorationMode::FieldStudySiteInspection {
                        self.mode = ExplorationMode::OnFootExploration;
                    }
                    self.study_target_prompt = None;
                }

                // Handle Camera cycle
                if input.cycle_camera {
                    self.camera_mode = match self.camera_mode {
                        ExplorationCameraMode::WalkerThirdPerson => {
                            ExplorationCameraMode::WalkerFirstPersonFieldOptics
                        }
                        _ => ExplorationCameraMode::WalkerThirdPerson,
                    };
                }

                // Handle Re-entering the Rover (Key [E] within 3.5 m of parked rover)
                if input.toggle_dismount {
                    if let Some(parked) = self.parked_rover {
                        let dx = self.walker.state.position[0] - parked.position[0];
                        let dz = self.walker.state.position[2] - parked.position[2];
                        let dist_to_rover = (dx * dx + dz * dz).sqrt();
                        if dist_to_rover <= 3.5 {
                            self.rover.state = parked;
                            self.parked_rover = None;
                            self.mode = ExplorationMode::RoverDriving;
                            self.camera_mode = ExplorationCameraMode::RoverChaseCam;
                        }
                    }
                }

                // Handle Entering Specimen Loupe (Key [F])
                if input.toggle_loupe {
                    self.mode = ExplorationMode::Interactive3dSpecimenLoupe;
                    self.camera_mode = ExplorationCameraMode::LoupeOrbitalMicroscope;
                }
            }

            ExplorationMode::Interactive3dSpecimenLoupe => {
                // Adjust scale if requested (1..=5)
                if let Some(scale) = input.loupe_scale_change {
                    self.loupe.active_scale = scale.clamp(1, 5);
                }

                // Adjust cutaway and layer explosion sliders
                self.loupe.cutaway_plane_01 =
                    (self.loupe.cutaway_plane_01 + input.loupe_cutaway_delta).clamp(0.0, 1.0);
                self.loupe.explode_layers_01 =
                    (self.loupe.explode_layers_01 + input.loupe_explode_delta).clamp(0.0, 1.0);

                // Update active ontology CURIE based on scale and site
                self.loupe.selected_ontology_curie = match self.loupe.active_scale {
                    1 => self.world.sites[self.nearest_site.site_index()].target_ontology_curie,
                    2 => "po:0004708", // Bark & secondary xylem
                    3 => "po:0009084", // Pericarp / operculum
                    4 => "po:0000032", // Stomatal complex
                    5 => "uberon:0002415", // Tail / vertebral osteology
                    _ => "po:0009010",
                };

                // Exit loupe back to exploration (Key [F])
                if input.toggle_loupe {
                    self.mode = if self.nearest_site_dist_m <= 25.0 {
                        ExplorationMode::FieldStudySiteInspection
                    } else {
                        ExplorationMode::OnFootExploration
                    };
                    self.camera_mode = ExplorationCameraMode::WalkerThirdPerson;
                }
            }
        }

        // 2. Compute camera placement according to camera mode
        let (camera_eye, camera_target) = self.compute_camera_pose();

        // 3. Assemble and return telemetry snapshot
        let active_pos = match self.mode {
            ExplorationMode::RoverDriving => self.rover.state.position,
            _ => self.walker.state.position,
        };
        let (elev_ahd, _, mat) =
            self.world.sample_world_elevation_and_normal_zero_heap(active_pos[0], active_pos[2]);
        let water_m = self.world.sample_water_surface_zero_heap(active_pos[0], active_pos[2]);
        let cave_ceil = self.world.sample_ceiling_overhang_zero_heap(active_pos[0], active_pos[2]);
        let corr_km = self.world.sites[self.nearest_site.site_index()].corridor_km;

        ExplorationTelemetrySnapshot {
            mode: self.mode,
            camera_mode: self.camera_mode,
            camera_eye,
            camera_target,
            rover_position: self.rover.state.position,
            rover_velocity: self.rover.state.velocity,
            rover_speed_kmh: self.rover.speed_kmh(),
            walker_position: self.walker.state.position,
            walker_velocity: self.walker.state.velocity,
            walker_submersion: self.walker.state.submersion,
            walker_is_grounded: self.walker.state.is_grounded,
            total_odometer_km: self.total_odometer_km,
            current_corridor_km: corr_km,
            nearest_study_site: self.nearest_site,
            nearest_site_distance_m: self.nearest_site_dist_m,
            soil_sinkage_mm: self.soil_sinkage_mm,
            ground_material: mat,
            elevation_ahd_m: elev_ahd,
            water_surface_m: water_m,
            cave_ceiling_m: cave_ceil,
            study_target_prompt: self.study_target_prompt,
            active_scale: self.loupe.active_scale,
            active_ontology_curie: self.loupe.selected_ontology_curie,
        }
    }

    /// Computes world-space camera eye and target vectors for current mode.
    fn compute_camera_pose(&self) -> ([f32; 3], [f32; 3]) {
        match self.camera_mode {
            ExplorationCameraMode::RoverChaseCam => {
                let rpos = self.rover.state.position;
                let h = self.rover.state.heading;
                let forward = [h.sin(), 0.0, h.cos()];
                let eye = [rpos[0] - forward[0] * 7.5, rpos[1] + 2.8, rpos[2] - forward[2] * 7.5];
                let target = [rpos[0], rpos[1] + 1.2, rpos[2]];
                (eye, target)
            }

            ExplorationCameraMode::RoverCabFirstPerson => {
                let rpos = self.rover.state.position;
                let h = self.rover.state.heading;
                let forward = [h.sin(), 0.0, h.cos()];
                let right = [h.cos(), 0.0, -h.sin()];
                let eye = [
                    rpos[0] + right[0] * 0.4 + forward[0] * 0.2,
                    rpos[1] + 1.35,
                    rpos[2] + right[2] * 0.4 + forward[2] * 0.2,
                ];
                let target = [eye[0] + forward[0] * 12.0, eye[1], eye[2] + forward[2] * 12.0];
                (eye, target)
            }

            ExplorationCameraMode::WalkerThirdPerson => {
                let wpos = self.walker.state.position;
                let yaw = self.walker.state.yaw;
                let forward = [yaw.sin(), 0.0, yaw.cos()];
                let eye = [wpos[0] - forward[0] * 3.2, wpos[1] + 2.0, wpos[2] - forward[2] * 3.2];
                let target = [wpos[0], wpos[1] + 1.5, wpos[2]];
                (eye, target)
            }

            ExplorationCameraMode::WalkerFirstPersonFieldOptics => {
                let eye = self.walker.eye_position();
                let yaw = self.walker.state.yaw;
                let pitch = self.walker.state.pitch;
                let forward = [yaw.sin() * pitch.cos(), -pitch.sin(), yaw.cos() * pitch.cos()];
                let target = [
                    eye[0] + forward[0] * 15.0,
                    eye[1] + forward[1] * 15.0,
                    eye[2] + forward[2] * 15.0,
                ];
                (eye, target)
            }

            ExplorationCameraMode::LoupeOrbitalMicroscope => {
                let focus = [
                    self.walker.state.position[0],
                    self.walker.state.position[1] + 1.2,
                    self.walker.state.position[2],
                ];
                let orbit_radius = match self.loupe.active_scale {
                    1 => 10.0,
                    2 => 2.4,
                    3 => 0.55,
                    4 => 0.075,
                    5 => 1.8,
                    _ => 1.0,
                };
                let yaw = self.camera_yaw;
                let pitch = self.camera_pitch.clamp(-1.2, 1.2);
                let eye = [
                    focus[0] + orbit_radius * yaw.cos() * pitch.cos(),
                    focus[1] + orbit_radius * pitch.sin(),
                    focus[2] + orbit_radius * yaw.sin() * pitch.cos(),
                ];
                (eye, focus)
            }
        }
    }
}

// ============================================================================
// 5. Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rover_driving_along_corridor() {
        let world = RegionalCorridorWorld::new();
        let mut session = ContinuumExplorationSession::new(world);

        assert_eq!(session.mode, ExplorationMode::RoverDriving);
        assert_eq!(session.nearest_site, StudySiteId::ThePinesForeshore);

        // Accelerate forward for 64 ticks (1 second)
        let drive_input = ExplorationTickInput {
            throttle: 1.0,
            steer: 0.0,
            brake: 0.0,
            ..Default::default()
        };

        for _ in 0..64 {
            let snap = session.step_tick_zero_heap(&drive_input, 1.0 / 64.0);
            assert_eq!(snap.mode, ExplorationMode::RoverDriving);
        }

        assert!(session.rover.speed_kmh() > 5.0, "Rover should accelerate forward");
        assert!(session.total_odometer_km > 0.0, "Odometer must accumulate distance");
        assert!(session.soil_sinkage_mm > 0.0, "Bekker sinkage must be computed");
    }

    #[test]
    fn test_dismount_anywhere_at_arbitrary_coordinates() {
        let world = RegionalCorridorWorld::new();
        let mut session = ContinuumExplorationSession::new(world);

        // Teleport to an arbitrary point midway along the corridor (km 4.2 Jounama)
        session.teleport_to_study_site(StudySiteId::JounamaCreekValley);
        assert_eq!(session.rover.state.position[0], 3400.0);
        assert_eq!(session.rover.state.position[2], -1850.0);

        // Bring speed to zero and press [E] to dismount
        let dismount_input = ExplorationTickInput {
            toggle_dismount: true,
            ..Default::default()
        };

        let snap = session.step_tick_zero_heap(&dismount_input, 1.0 / 64.0);

        // Must dismount cleanly at (3400, -1850) without 11x14m clamp error!
        assert_eq!(snap.mode, ExplorationMode::OnFootExploration);
        assert_eq!(session.mode, ExplorationMode::OnFootExploration);
        assert!(session.parked_rover.is_some(), "Rover must be parked");

        let walker_x = session.walker.state.position[0];
        let walker_z = session.walker.state.position[2];
        assert!(
            (walker_x - 3400.0).abs() < 5.0,
            "Walker must spawn beside parked rover at exact coordinates: {walker_x}"
        );
        assert!(
            (walker_z - (-1850.0)).abs() < 5.0,
            "Walker must spawn beside parked rover at exact coordinates: {walker_z}"
        );
    }

    #[test]
    fn test_walker_submersion_in_water() {
        let world = RegionalCorridorWorld::new();
        let mut session = ContinuumExplorationSession::new(world);

        // Teleport to Three Mile Dam (water level at 1458.0 m)
        session.teleport_to_study_site(StudySiteId::ThreeMileDamSnowGumBog);

        // Dismount
        let dismount_input = ExplorationTickInput {
            toggle_dismount: true,
            ..Default::default()
        };
        session.step_tick_zero_heap(&dismount_input, 1.0 / 64.0);
        assert_eq!(session.mode, ExplorationMode::OnFootExploration);

        // Walk toward the dam pond
        session.walker.state.position = [14200.0, 1457.5, -8100.0];

        let walk_input = ExplorationTickInput {
            wish_dir: [1.0, 0.0],
            ..Default::default()
        };

        let snap = session.step_tick_zero_heap(&walk_input, 1.0 / 64.0);
        assert!(
            snap.walker_submersion == SubmersionState::Wading
                || snap.walker_submersion == SubmersionState::Swimming,
            "Walker in dam pond must experience water submersion, got: {:?}",
            snap.walker_submersion
        );
    }

    #[test]
    fn test_karst_cave_ceiling_collision() {
        let world = RegionalCorridorWorld::new();
        let mut session = ContinuumExplorationSession::new(world);

        // Teleport to Yarrangobilly Karst Gorge
        session.teleport_to_study_site(StudySiteId::YarrangobillyKarstGorge);

        // Dismount
        let dismount_input = ExplorationTickInput {
            toggle_dismount: true,
            ..Default::default()
        };
        session.step_tick_zero_heap(&dismount_input, 1.0 / 64.0);

        // Place walker inside the cave arch
        session.walker.state.position = [7820.0, 946.0, -4210.0];
        session.walker.state.velocity = [0.0, 10.0, 0.0]; // jumping upward

        let tick_input = ExplorationTickInput {
            jump: true,
            ..Default::default()
        };
        let snap = session.step_tick_zero_heap(&tick_input, 1.0 / 64.0);

        // Head must be clamped below cave ceiling
        assert!(snap.cave_ceiling_m.is_finite());
        assert!(
            session.walker.state.position[1] + session.walker.config.height <= snap.cave_ceiling_m + 0.1,
            "Walker head must not penetrate cave roof: pos_y={}, ceil={}",
            session.walker.state.position[1],
            snap.cave_ceiling_m
        );
    }

    #[test]
    fn test_specimen_loupe_5_scales_and_ontology_transition() {
        let world = RegionalCorridorWorld::new();
        let mut session = ContinuumExplorationSession::new(world);

        // Dismount
        let dismount = ExplorationTickInput {
            toggle_dismount: true,
            ..Default::default()
        };
        session.step_tick_zero_heap(&dismount, 1.0 / 64.0);

        // Toggle into Specimen Loupe [F]
        let enter_loupe = ExplorationTickInput {
            toggle_loupe: true,
            ..Default::default()
        };
        let snap1 = session.step_tick_zero_heap(&enter_loupe, 1.0 / 64.0);
        assert_eq!(snap1.mode, ExplorationMode::Interactive3dSpecimenLoupe);
        assert_eq!(session.camera_mode, ExplorationCameraMode::LoupeOrbitalMicroscope);

        // Switch to Scale 4 (Cellular Histology)
        let scale4_input = ExplorationTickInput {
            loupe_scale_change: Some(4),
            ..Default::default()
        };
        let snap4 = session.step_tick_zero_heap(&scale4_input, 1.0 / 64.0);
        assert_eq!(snap4.active_scale, 4);
        assert_eq!(snap4.active_ontology_curie, "po:0000032"); // Stomatal complex

        // Switch to Scale 5 (Vertebrate 3-Layer Anatomy)
        let scale5_input = ExplorationTickInput {
            loupe_scale_change: Some(5),
            ..Default::default()
        };
        let snap5 = session.step_tick_zero_heap(&scale5_input, 1.0 / 64.0);
        assert_eq!(snap5.active_scale, 5);
        assert_eq!(snap5.active_ontology_curie, "uberon:0002415"); // Vertebral osteology

        // Toggle out of Specimen Loupe [F]
        let exit_loupe = ExplorationTickInput {
            toggle_loupe: true,
            ..Default::default()
        };
        let snap_exit = session.step_tick_zero_heap(&exit_loupe, 1.0 / 64.0);
        assert_ne!(snap_exit.mode, ExplorationMode::Interactive3dSpecimenLoupe);
    }

    #[test]
    fn test_remount_parked_rover() {
        let world = RegionalCorridorWorld::new();
        let mut session = ContinuumExplorationSession::new(world);

        // Dismount
        let dismount = ExplorationTickInput {
            toggle_dismount: true,
            ..Default::default()
        };
        session.step_tick_zero_heap(&dismount, 1.0 / 64.0);
        assert_eq!(session.mode, ExplorationMode::OnFootExploration);

        // Walker is right beside parked rover (< 3.5 m)
        // Press [E] to re-enter
        let remount = ExplorationTickInput {
            toggle_dismount: true,
            ..Default::default()
        };
        let snap = session.step_tick_zero_heap(&remount, 1.0 / 64.0);
        assert_eq!(snap.mode, ExplorationMode::RoverDriving);
        assert_eq!(session.mode, ExplorationMode::RoverDriving);
        assert!(session.parked_rover.is_none());
    }
}
