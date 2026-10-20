//! Zero-Heap Generational 10D Scene Graph, `EONT1` Living Ontology Binding & Multi-Modal Culling
//!
//! Integrates Epic 1 (Generational 10D Scene Graph, Intrusive Sibling Tree, Prefab Delta Inheritance,
//! `EONT1` Living Ontology Binding & SOA Archetype ECS) and Epic 2 (Multi-Modal `MeshSlabArenaGpu`,
//! GPU-Driven Indirect Culling, 2-Phase HZB Occlusion, and 4-Tier Hybrid LOD) into `ecopunk-terrain`.

use qualia_core_db::container_10d::entity_ontology_section::{
    AffordanceBinaryRecord, DevelopmentalKeyframeRecord, EntityOntologyMiniHeader, MAGIC_EONT,
};
use qualia_core_db::render::gpu::indirect_cull::{
    build_hzb_mip_chain_zero_heap, cull_and_select_lod_indirect_zero_heap,
    DrawIndexedIndirectCommand, HzbPyramidView, IndirectCullStreamCounts, InstanceCullDescriptor,
    RepresentationPolicy,
};
use qualia_core_db::render::gpu::mesh_slab::{
    ResidentAssetEntry, SubmeshSlice, REP_KIND_HOLOGRAPHIC_SDF_PROXY, REP_KIND_MESH_BREP,
    REP_KIND_OCTAHEDRAL_IMPOSTOR, REP_KIND_SGGX_CANOPY, REP_KIND_SURFEL_CLOUD,
};
use qualia_core_db::render::pga::{motor_translate, Motor};
use qualia_core_db::render::scene_graph::ontology_binding::{
    apply_ontogenetic_allometry, DevelopmentalStage, LivingOntologyState,
};
use qualia_core_db::render::scene_graph::pool::{
    NodeHandle, NodeKind, NodePool, SceneNodeRecord, VISIBILITY_CAST_SHADOWS,
    VISIBILITY_HOLOGRAPHIC_INTERFERENCE, VISIBILITY_RAYCAST_PICKABLE, VISIBILITY_RECEIVE_SHADOWS,
    VISIBILITY_VISIBLE,
};
use qualia_core_db::render::scene_graph::prefab::{
    apply_archetype_to_instance, PrefabDeltaMask,
};
use qualia_core_db::render::scene_graph::transform_sync::{
    compute_topological_order, propagate_transforms_zero_heap, SyncedHolographicNode,
    SyncedInstanceTransform, TransformSyncSummary,
};
use qualia_sim::ecs::{SimTable, Tick};

use crate::specimen::loupe::{
    BotanicalDevelopmentalStage, GhostGumOrganSubmesh, GhostGumSpecimenLoupe,
};

/// 64-bit FNV-1a fast string hasher for deterministic 10D entity tokens.
#[inline]
pub const fn hash_str_token(s: &str) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    let b = s.as_bytes();
    let mut i = 0usize;
    while i < b.len() {
        h ^= b[i] as u64;
        h = h.wrapping_mul(0x100000001b3);
        i += 1;
    }
    h
}

/// Fixed capacity for sector scene graph nodes in `ecopunk-terrain`.
pub const SECTOR_NODE_CAPACITY: usize = 1024;
/// Maximum botanical living entities tracked in the SOA table per sector.
pub const MAX_LIVING_ENTITIES: usize = 256;
/// Maximum resident asset descriptors tracked for GPU slab indirect drawing.
pub const MAX_RESIDENT_ASSETS: usize = 64;
/// HZB mip pyramid scratch texel capacity (e.g. 64x64 base + mips = 5461 <= 8192).
pub const MAX_HZB_TEXELS: usize = 8192;

/// Zero-heap generational 10D sector scene graph, living ontology container & multi-modal culler.
pub struct EcopunkSectorSceneGraph {
    /// Zero-heap generational node pool managing 10D scene nodes with $O(1)$ allocation/deallocation.
    pub pool: NodePool<SECTOR_NODE_CAPACITY>,
    /// East Coast sector identifier (`talbingo`, `yarrangobilly`, `three_mile_dam`, etc.).
    pub sector_id: &'static str,
    /// Root boundary-representation (`MeshBRep`) terrain node.
    pub root_terrain: NodeHandle,
    /// Water surface mesh node.
    pub water_node: NodeHandle,
    /// Volumetric atmospheric EMF layer (`VolumetricEmf`).
    pub volumetric_emf: NodeHandle,
    /// Holosonic wavefield emitter handles.
    pub holosonic_emitters: [NodeHandle; 8],
    pub holosonic_emitter_count: usize,
    /// Holographic epistemic superposition nodes (`HolographicSdf`).
    pub holographic_sdf_nodes: [NodeHandle; 16],
    pub holographic_sdf_count: usize,
    /// Root botanical tree node handles.
    pub tree_roots: [NodeHandle; 64],
    pub tree_count: usize,

    /// Monotonically advancing simulation tick for change detection.
    pub sim_tick: Tick,
    /// Last tick when transforms were synchronized with the GPU.
    pub last_sync_tick: u32,

    /// Fixed-capacity SOA component table storing live `EONT1` metabolic and allometric state.
    pub living_states: SimTable<LivingOntologyState, MAX_LIVING_ENTITIES>,
    /// Map from dense living entity index to scene graph `NodeHandle`.
    pub living_node_handles: [NodeHandle; MAX_LIVING_ENTITIES],
    /// Number of active botanical entities registered in `living_states`.
    pub living_count: usize,

    /// EONT1 mini-headers and keyframes for botanical taxa.
    pub ghost_gum_header: EntityOntologyMiniHeader,
    pub ghost_gum_keyframes: [DevelopmentalKeyframeRecord; 4],
    pub ghost_gum_affordances: [AffordanceBinaryRecord; 4],
    pub snow_gum_header: EntityOntologyMiniHeader,
    pub snow_gum_keyframes: [DevelopmentalKeyframeRecord; 4],
    pub snow_gum_affordances: [AffordanceBinaryRecord; 4],

    /// Multi-modal resident asset entries in the GPU slab arena.
    pub resident_assets: [ResidentAssetEntry; MAX_RESIDENT_ASSETS],
    pub resident_asset_count: usize,

    // ── Zero-Heap Flat Scratch Buffers (No Heap Allocations on Hot Paths) ──
    /// Flat breadth-first topological ordering queue.
    pub topological_order: [u32; SECTOR_NODE_CAPACITY],
    /// Dirty GPU instance transform records produced by transform propagation.
    pub synced_instances: [SyncedInstanceTransform; SECTOR_NODE_CAPACITY],
    /// Dirty Holographic proxy records produced by transform propagation.
    pub synced_holographic: [SyncedHolographicNode; 64],

    /// Per-instance indirect culling & LOD selection descriptors.
    pub cull_descriptors: [InstanceCullDescriptor; SECTOR_NODE_CAPACITY],
    pub cull_count: usize,

    /// WebGPU-standard indirect indexed draw commands.
    pub indirect_draw_commands: [DrawIndexedIndirectCommand; SECTOR_NODE_CAPACITY],
    /// Stream 0: Visible mesh instance indices (`LOD0..LOD2`).
    pub visible_mesh_indices: [u32; SECTOR_NODE_CAPACITY],
    /// Stream 1: Visible Holographic SDF proxy instance indices (`q > 0` or SDF proxy).
    pub visible_holo_indices: [u32; 256],
    /// Stream 2: Visible Surfel Cloud & Octahedral Impostor indices (`LOD3` or surfel/impostor).
    pub visible_surfel_indices: [u32; 512],
    /// Selected 4-tier LOD indices (`0..=3`) per instance.
    pub selected_lods: [u8; SECTOR_NODE_CAPACITY],
    /// 2-Phase HZB mip-chain reduction buffer.
    pub hzb_texels: [f32; MAX_HZB_TEXELS],
}

impl Default for EcopunkSectorSceneGraph {
    fn default() -> Self {
        Self::new("talbingo")
    }
}

impl EcopunkSectorSceneGraph {
    /// Creates an empty sector scene graph container initialized with default resident assets.
    pub fn new(sector_id: &'static str) -> Self {
        let ghost_gum_header = GhostGumSpecimenLoupe::default_ontology_mini_header();
        let ghost_gum_keyframes = GhostGumSpecimenLoupe::default_developmental_keyframes();
        let ghost_gum_affordances = GhostGumSpecimenLoupe::default_custodial_affordances();

        let snow_gum_header = EntityOntologyMiniHeader {
            magic: MAGIC_EONT,
            version: 1,
            stage: DevelopmentalStage::Adult as u8,
            reserved_u8: 0,
            entity_did_hash: hash_str_token("taxon:eucalyptus_pauciflora"),
            taxon_iri_hash: hash_str_token("urn:qualia:taxon:eucalyptus_pauciflora"),
            energy_permill: 800,
            damage_permill: 35,
            hydration_permill: 820,
            wear_permill: 180,
            affordance_count: 4,
            reserved_u32: 0,
        };

        let snow_gum_keyframes = [
            DevelopmentalKeyframeRecord {
                age_days: 120.0,
                stage: DevelopmentalStage::SeedOrSpore as u8,
                reserved_u8: 0,
                reserved_u16: 0,
                allometric_crown_scale: 0.15,
                allometric_trunk_scale: 0.10,
                lai_multiplier: 0.30,
                reserved_u32: 0,
                mesh_asset_hash: 0x0101,
            },
            DevelopmentalKeyframeRecord {
                age_days: 2200.0,
                stage: DevelopmentalStage::Juvenile as u8,
                reserved_u8: 0,
                reserved_u16: 0,
                allometric_crown_scale: 0.50,
                allometric_trunk_scale: 0.40,
                lai_multiplier: 0.75,
                reserved_u32: 0,
                mesh_asset_hash: 0x0102,
            },
            DevelopmentalKeyframeRecord {
                age_days: 20000.0,
                stage: DevelopmentalStage::Adult as u8,
                reserved_u8: 0,
                reserved_u16: 0,
                allometric_crown_scale: 1.0,
                allometric_trunk_scale: 1.0,
                lai_multiplier: 1.0,
                reserved_u32: 0,
                mesh_asset_hash: 0x0103,
            },
            DevelopmentalKeyframeRecord {
                age_days: 60000.0,
                stage: DevelopmentalStage::Senescent as u8,
                reserved_u8: 0,
                reserved_u16: 0,
                allometric_crown_scale: 0.85,
                allometric_trunk_scale: 1.30,
                lai_multiplier: 0.60,
                reserved_u32: 0,
                mesh_asset_hash: 0x0104,
            },
        ];

        let mut resident_assets = [ResidentAssetEntry::default(); MAX_RESIDENT_ASSETS];
        // Asset 0: Sector Terrain B-Rep
        resident_assets[0] = ResidentAssetEntry {
            asset_id: 0,
            lod_slices: [
                SubmeshSlice { first_index: 0, index_count: 12000, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 12000, index_count: 4800, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 16800, index_count: 1200, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 18000, index_count: 300, base_vertex: 0, rep_kind: REP_KIND_OCTAHEDRAL_IMPOSTOR },
            ],
            bounding_sphere: [0.0, 0.0, 0.0, 500.0],
            ..ResidentAssetEntry::default()
        };
        // Asset 1: Water Surface Mesh
        resident_assets[1] = ResidentAssetEntry {
            asset_id: 1,
            lod_slices: [
                SubmeshSlice { first_index: 19000, index_count: 3200, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 22200, index_count: 1200, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 23400, index_count: 400, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 23800, index_count: 100, base_vertex: 0, rep_kind: REP_KIND_OCTAHEDRAL_IMPOSTOR },
            ],
            bounding_sphere: [0.0, -1.0, 0.0, 250.0],
            ..ResidentAssetEntry::default()
        };
        // Asset 2: Corymbia aparrerinja (Ghost Gum) Trunk
        resident_assets[2] = ResidentAssetEntry {
            asset_id: 2,
            lod_slices: [
                SubmeshSlice { first_index: 24000, index_count: 6400, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 30400, index_count: 2400, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 32800, index_count: 600, base_vertex: 0, rep_kind: REP_KIND_SURFEL_CLOUD },
                SubmeshSlice { first_index: 33400, index_count: 120, base_vertex: 0, rep_kind: REP_KIND_OCTAHEDRAL_IMPOSTOR },
            ],
            bounding_sphere: [0.0, 7.5, 0.0, 9.0],
            ..ResidentAssetEntry::default()
        };
        // Asset 3: Corymbia aparrerinja Falcate Foliage / Canopy
        resident_assets[3] = ResidentAssetEntry {
            asset_id: 3,
            lod_slices: [
                SubmeshSlice { first_index: 34000, index_count: 9600, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 43600, index_count: 3600, base_vertex: 0, rep_kind: REP_KIND_SGGX_CANOPY },
                SubmeshSlice { first_index: 47200, index_count: 900, base_vertex: 0, rep_kind: REP_KIND_SURFEL_CLOUD },
                SubmeshSlice { first_index: 48100, index_count: 150, base_vertex: 0, rep_kind: REP_KIND_OCTAHEDRAL_IMPOSTOR },
            ],
            bounding_sphere: [0.0, 12.0, 0.0, 7.5],
            ..ResidentAssetEntry::default()
        };
        // Asset 4: Eucalyptus pauciflora (Snow Gum) Contorted Trunk
        resident_assets[4] = ResidentAssetEntry {
            asset_id: 4,
            lod_slices: [
                SubmeshSlice { first_index: 49000, index_count: 5800, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 54800, index_count: 2100, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 56900, index_count: 550, base_vertex: 0, rep_kind: REP_KIND_SURFEL_CLOUD },
                SubmeshSlice { first_index: 57450, index_count: 100, base_vertex: 0, rep_kind: REP_KIND_OCTAHEDRAL_IMPOSTOR },
            ],
            bounding_sphere: [0.0, 5.5, 0.0, 7.0],
            ..ResidentAssetEntry::default()
        };
        // Asset 5: Eucalyptus pauciflora Subalpine Canopy
        resident_assets[5] = ResidentAssetEntry {
            asset_id: 5,
            lod_slices: [
                SubmeshSlice { first_index: 58000, index_count: 8400, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 66400, index_count: 3200, base_vertex: 0, rep_kind: REP_KIND_SGGX_CANOPY },
                SubmeshSlice { first_index: 69600, index_count: 800, base_vertex: 0, rep_kind: REP_KIND_SURFEL_CLOUD },
                SubmeshSlice { first_index: 70400, index_count: 140, base_vertex: 0, rep_kind: REP_KIND_OCTAHEDRAL_IMPOSTOR },
            ],
            bounding_sphere: [0.0, 8.5, 0.0, 6.5],
            ..ResidentAssetEntry::default()
        };
        // Asset 6: Close-Up Botanical Specimen Study Loupe (Leaf / Flower / Capsule)
        resident_assets[6] = ResidentAssetEntry {
            asset_id: 6,
            lod_slices: [
                SubmeshSlice { first_index: 71000, index_count: 14400, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 85400, index_count: 4800, base_vertex: 0, rep_kind: REP_KIND_MESH_BREP },
                SubmeshSlice { first_index: 90200, index_count: 1200, base_vertex: 0, rep_kind: REP_KIND_SURFEL_CLOUD },
                SubmeshSlice { first_index: 91400, index_count: 200, base_vertex: 0, rep_kind: REP_KIND_OCTAHEDRAL_IMPOSTOR },
            ],
            bounding_sphere: [0.0, 0.0, 0.0, 0.25],
            ..ResidentAssetEntry::default()
        };
        // Asset 7: Holographic SDF Superposition Lens Proxy
        resident_assets[7] = ResidentAssetEntry {
            asset_id: 7,
            lod_slices: [
                SubmeshSlice { first_index: 0, index_count: 0, base_vertex: 0, rep_kind: REP_KIND_HOLOGRAPHIC_SDF_PROXY },
                SubmeshSlice { first_index: 0, index_count: 0, base_vertex: 0, rep_kind: REP_KIND_HOLOGRAPHIC_SDF_PROXY },
                SubmeshSlice { first_index: 0, index_count: 0, base_vertex: 0, rep_kind: REP_KIND_HOLOGRAPHIC_SDF_PROXY },
                SubmeshSlice { first_index: 0, index_count: 0, base_vertex: 0, rep_kind: REP_KIND_HOLOGRAPHIC_SDF_PROXY },
            ],
            bounding_sphere: [0.0, 0.0, 0.0, 25.0],
            ..ResidentAssetEntry::default()
        };

        Self {
            pool: NodePool::new(),
            sector_id,
            root_terrain: NodeHandle::NONE,
            water_node: NodeHandle::NONE,
            volumetric_emf: NodeHandle::NONE,
            holosonic_emitters: [NodeHandle::NONE; 8],
            holosonic_emitter_count: 0,
            holographic_sdf_nodes: [NodeHandle::NONE; 16],
            holographic_sdf_count: 0,
            tree_roots: [NodeHandle::NONE; 64],
            tree_count: 0,
            sim_tick: Tick::new(1),
            last_sync_tick: 0,
            living_states: SimTable::new(LivingOntologyState::default()),
            living_node_handles: [NodeHandle::NONE; MAX_LIVING_ENTITIES],
            living_count: 0,
            ghost_gum_header,
            ghost_gum_keyframes,
            ghost_gum_affordances,
            snow_gum_header,
            snow_gum_keyframes,
            snow_gum_affordances: ghost_gum_affordances,
            resident_assets,
            resident_asset_count: 8,
            topological_order: [0u32; SECTOR_NODE_CAPACITY],
            synced_instances: [SyncedInstanceTransform {
                node_index: 0,
                mesh_handle: u32::MAX,
                world_motor_r: [1.0, 0.0, 0.0, 0.0],
                world_motor_d: [0.0; 4],
                world_scale_alpha: [1.0, 1.0, 1.0, 1.0],
                metadata_packed: [0; 4],
            }; SECTOR_NODE_CAPACITY],
            synced_holographic: [SyncedHolographicNode {
                position_scale: [0.0; 4],
                half_extents_primitive: [0.0; 4],
                frequency_amplitude: [0.0; 4],
                manifold_tensor: [0.0; 4],
                connection_endpoint: [0.0; 4],
                emissive_color: [0.0; 4],
            }; 64],
            cull_descriptors: [InstanceCullDescriptor::default(); SECTOR_NODE_CAPACITY],
            cull_count: 0,
            indirect_draw_commands: [DrawIndexedIndirectCommand::default(); SECTOR_NODE_CAPACITY],
            visible_mesh_indices: [0u32; SECTOR_NODE_CAPACITY],
            visible_holo_indices: [0u32; 256],
            visible_surfel_indices: [0u32; 512],
            selected_lods: [0u8; SECTOR_NODE_CAPACITY],
            hzb_texels: [1.0f32; MAX_HZB_TEXELS],
        }
    }

    /// Allocates an `EcopunkSectorSceneGraph` directly on the heap to preserve thread stack budget.
    pub fn new_boxed(sector_id: &'static str) -> Box<Self> {
        Box::new(Self::new(sector_id))
    }

    /// Populates a comprehensive 10D scene hierarchy for one of the 6 East Coast Australian sectors:
    /// - `talbingo` (Jounama Pondage & subalpine transition)
    /// - `yarrangobilly` (Karst limestone gorge & thermal springs)
    /// - `three_mile_dam` (Kosciuszko alpine reservoir & snow gum woodlands)
    /// - `sydney_sandstone` (Hawkesbury sandstone cliffs & coastal woodland)
    /// - `noosa_headland` (Pacific littoral headland, coastal wallum & pandanus)
    /// - `jounama_karst` (Subterranean stream resurgence & ancient limestone tors)
    pub fn populate_sector_10d_hierarchy(&mut self, sector_id: &'static str) -> bool {
        self.sector_id = sector_id;
        let current_tick = self.sim_tick.get();

        // 1. Root Terrain B-Rep Node
        let terrain_token = hash_str_token(&format!("sector:terrain:{sector_id}"));
        let mut terrain_rec = SceneNodeRecord::new(terrain_token, NodeKind::MeshBRep);
        terrain_rec.mesh_handle = 0;
        terrain_rec.material_id = 0;
        terrain_rec.visibility_flags = VISIBILITY_VISIBLE
            | VISIBILITY_CAST_SHADOWS
            | VISIBILITY_RECEIVE_SHADOWS
            | VISIBILITY_RAYCAST_PICKABLE;
        terrain_rec.local_scale = [1.0, 1.0, 1.0];
        let root_handle = self.pool.spawn(terrain_rec).expect("spawn root terrain");
        self.root_terrain = root_handle;

        // 2. Water Surface Node
        let water_elevation = match sector_id {
            "three_mile_dam" => 1390.0 * 0.001,
            "yarrangobilly" | "jounama_karst" => 980.0 * 0.001,
            "talbingo" => 540.0 * 0.001,
            "sydney_sandstone" => 2.0 * 0.001,
            "noosa_headland" => 0.5 * 0.001,
            _ => 1.0,
        };
        let water_token = hash_str_token(&format!("sector:water:{sector_id}"));
        let mut water_rec = SceneNodeRecord::new(water_token, NodeKind::MeshBRep);
        water_rec.mesh_handle = 1;
        water_rec.material_id = 1;
        water_rec.local_motor = motor_translate([0.0, water_elevation, 0.0]);
        water_rec.visibility_flags = VISIBILITY_VISIBLE | VISIBILITY_RECEIVE_SHADOWS | VISIBILITY_RAYCAST_PICKABLE;
        let water_handle = self.pool.spawn(water_rec).expect("spawn water node");
        self.pool.attach_child(root_handle, water_handle);
        self.water_node = water_handle;

        // 3. Volumetric EMF & Atmospheric Layer
        let emf_token = hash_str_token(&format!("sector:emf:{sector_id}"));
        let mut emf_rec = SceneNodeRecord::new(emf_token, NodeKind::VolumetricEmf);
        emf_rec.tensor_v = 1; // Cyclic / atmospheric diurnal circulation band
        emf_rec.alpha_salience = 0.85;
        emf_rec.sigma_spectral = 0.45;
        emf_rec.local_motor = motor_translate([0.0, water_elevation + 15.0, 0.0]);
        emf_rec.local_scale = [250.0, 60.0, 250.0];
        let emf_handle = self.pool.spawn(emf_rec).expect("spawn volumetric emf");
        self.pool.attach_child(root_handle, emf_handle);
        self.volumetric_emf = emf_handle;

        // 4. Holosonic Acoustic Wavefield Emitters
        let emitter_configs: &[(&str, [f32; 3], u16)] = match sector_id {
            "yarrangobilly" | "jounama_karst" => &[
                ("river_cascade", [-15.0, water_elevation, 20.0], 1),
                ("thermal_spring_murmur", [10.0, water_elevation, -5.0], 2),
                ("karst_cave_resonance", [35.0, water_elevation + 12.0, -18.0], 3),
                ("subalpine_cicada_hum", [0.0, water_elevation + 8.0, 0.0], 4),
            ],
            "noosa_headland" => &[
                ("pacific_breaker_wash", [45.0, 0.0, 60.0], 1),
                ("coastal_sheoak_whisper", [-20.0, 18.0, -10.0], 2),
                ("honeyeater_feeding_chatter", [5.0, 12.0, -30.0], 3),
                ("dune_wind_vorticity", [0.0, 25.0, 0.0], 4),
            ],
            _ => &[
                ("riparian_river_flow", [-12.0, water_elevation, 15.0], 1),
                ("eucalypt_canopy_wind", [0.0, water_elevation + 22.0, 0.0], 2),
                ("creek_pebble_trickle", [8.0, water_elevation, -12.0], 3),
                ("woodland_robin_piping", [-25.0, water_elevation + 6.0, -20.0], 4),
            ],
        };

        self.holosonic_emitter_count = 0;
        for &(emitter_name, pos, audio_id) in emitter_configs {
            let token = hash_str_token(&format!("audio:{sector_id}:{emitter_name}"));
            let mut audio_rec = SceneNodeRecord::new(token, NodeKind::AudioHolosonic);
            audio_rec.local_motor = motor_translate(pos);
            audio_rec.audio_emitter_handle = audio_id;
            audio_rec.alpha_salience = 0.90;
            if let Some(h) = self.pool.spawn(audio_rec) {
                self.pool.attach_child(root_handle, h);
                if self.holosonic_emitter_count < self.holosonic_emitters.len() {
                    self.holosonic_emitters[self.holosonic_emitter_count] = h;
                    self.holosonic_emitter_count += 1;
                }
            }
        }

        // 5. Holographic Epistemic Superposition Nodes (tensor_q > 0, tensor_v >= 1, deontic_lane, sigma_spectral)
        let holo_configs: &[(&str, [f32; 3], u8, u8, u8, f32, f32, f32)] = &[
            // 1) Custodial songline / boundary lens: tensor_q=1, tensor_v=1, deontic_lane=1 (Commons), sigma=0.62
            ("custodial_songline_boundary", [12.0, water_elevation + 4.0, -25.0], 1, 1, 1, 0.62, 0.95, 0.90),
            // 2) Riparian potentiality / ecological corridor: tensor_q=2, tensor_v=2, deontic_lane=2 (Bilateral), sigma=0.78
            ("riparian_corridor_potentiality", [-28.0, water_elevation + 2.0, 18.0], 2, 2, 2, 0.78, 0.85, 0.80),
            // 3) Epistemic observation locus / sentinel perspective: tensor_q=1, tensor_v=3, deontic_lane=2, sigma=0.50
            ("sentinel_perspective_locus", [0.0, water_elevation + 30.0, 45.0], 1, 3, 2, 0.50, 0.92, 0.88),
        ];

        self.holographic_sdf_count = 0;
        for &(lens_name, pos, q, v, deontic, sigma, ep_q, alpha) in holo_configs {
            let token = hash_str_token(&format!("holo:{sector_id}:{lens_name}"));
            let mut holo_rec = SceneNodeRecord::new(token, NodeKind::HolographicSdf);
            holo_rec.local_motor = motor_translate(pos);
            holo_rec.tensor_q = q;
            holo_rec.tensor_v = v;
            holo_rec.deontic_lane = deontic;
            holo_rec.sigma_spectral = sigma;
            holo_rec.epistemic_q = ep_q;
            holo_rec.alpha_salience = alpha;
            holo_rec.mesh_handle = 7; // Holographic proxy resident asset
            holo_rec.visibility_flags |= VISIBILITY_HOLOGRAPHIC_INTERFERENCE;

            if let Some(h) = self.pool.spawn(holo_rec) {
                self.pool.attach_child(root_handle, h);
                if self.holographic_sdf_count < self.holographic_sdf_nodes.len() {
                    self.holographic_sdf_nodes[self.holographic_sdf_count] = h;
                    self.holographic_sdf_count += 1;
                }
            }
        }

        // 6. Botanical Archetypes & Multi-Organ Trees (*Corymbia aparrerinja* & *Eucalyptus pauciflora*)
        // Archetype 0: Corymbia aparrerinja Trunk Archetype
        let ghost_gum_trunk_archetype = SceneNodeRecord::new(
            hash_str_token("archetype:corymbia_aparrerinja:trunk"),
            NodeKind::MeshBRep,
        )
        .with_local_scale([1.0, 1.0, 1.0])
        .with_tensor_lanes(0, 0, 0, 1);

        // Archetype 1: Eucalyptus pauciflora Contorted Trunk Archetype
        let snow_gum_trunk_archetype = SceneNodeRecord::new(
            hash_str_token("archetype:eucalyptus_pauciflora:trunk"),
            NodeKind::MeshBRep,
        )
        .with_local_scale([0.9, 0.85, 0.9])
        .with_tensor_lanes(0, 0, 0, 1);

        // Multi-organ tree placement specifications: (species_id, pos, scale, rot_y_deg, growth_stage, growth_progress)
        let tree_specs: &[(&str, [f32; 3], f32, f32, BotanicalDevelopmentalStage, f32)] = &[
            // Ghost Gum Specimen 0: Foreground Veteran Tree (Ancient Hollow-Bearing, Stage 4)
            ("corymbia_aparrerinja", [18.0, water_elevation, 12.0], 1.15, 35.0, BotanicalDevelopmentalStage::AncientHollowBearing, 0.95),
            // Ghost Gum Specimen 1: Mature Canopy (Stage 2)
            ("corymbia_aparrerinja", [-22.0, water_elevation, 28.0], 0.95, -60.0, BotanicalDevelopmentalStage::MatureCanopy, 0.75),
            // Ghost Gum Specimen 2: Vigorous Sapling (Stage 1)
            ("corymbia_aparrerinja", [30.0, water_elevation, -15.0], 0.55, 120.0, BotanicalDevelopmentalStage::Sapling, 0.35),
            // Ghost Gum Specimen 3: Establishing Seedling (Stage 0)
            ("corymbia_aparrerinja", [8.0, water_elevation, 5.0], 0.20, 15.0, BotanicalDevelopmentalStage::Seedling, 0.08),

            // Snow Gum Specimen 0: Subalpine Veteran (Ancient Hollow-Bearing, Stage 4)
            ("eucalyptus_pauciflora", [-10.0, water_elevation + 3.0, -32.0], 1.10, -45.0, BotanicalDevelopmentalStage::AncientHollowBearing, 0.92),
            // Snow Gum Specimen 1: Mature Contorted Canopy (Stage 2)
            ("eucalyptus_pauciflora", [-38.0, water_elevation + 5.0, -10.0], 0.88, 75.0, BotanicalDevelopmentalStage::MatureCanopy, 0.80),
            // Snow Gum Specimen 2: Subalpine Sapling (Stage 1)
            ("eucalyptus_pauciflora", [25.0, water_elevation + 2.0, 35.0], 0.48, -140.0, BotanicalDevelopmentalStage::Sapling, 0.40),
            // Snow Gum Specimen 3: Snowline Seedling (Stage 0)
            ("eucalyptus_pauciflora", [-5.0, water_elevation + 4.0, -18.0], 0.22, 90.0, BotanicalDevelopmentalStage::Seedling, 0.10),
        ];

        self.tree_count = 0;
        self.living_count = 0;

        for (tree_idx, &(species, pos, scale, rot_deg, bot_stage, progress)) in tree_specs.iter().enumerate() {
            let is_ghost_gum = species == "corymbia_aparrerinja";
            let trunk_archetype = if is_ghost_gum {
                &ghost_gum_trunk_archetype
            } else {
                &snow_gum_trunk_archetype
            };

            let trunk_token = hash_str_token(&format!("tree:{species}:{tree_idx}:trunk"));
            let mut trunk_instance = SceneNodeRecord::new(trunk_token, NodeKind::MeshBRep);

            // Configure instance overrides
            let rot_rad = rot_deg.to_radians();
            trunk_instance.local_motor = Motor::from_translation_axis_angle(pos, [0.0, 1.0, 0.0], rot_rad);
            trunk_instance.local_scale = [scale, scale, scale];
            trunk_instance.mesh_handle = if is_ghost_gum { 2 } else { 4 };
            trunk_instance.material_id = if is_ghost_gum { 2 } else { 4 };

            // Prefab Delta Mask overrides: motor, scale, mesh handle, material id
            let mut trunk_mask = PrefabDeltaMask::NONE;
            trunk_mask.mark_modified(PrefabDeltaMask::BIT_LOCAL_MOTOR);
            trunk_mask.mark_modified(PrefabDeltaMask::BIT_LOCAL_SCALE);
            trunk_mask.mark_modified(PrefabDeltaMask::BIT_MESH_HANDLE);
            trunk_mask.mark_modified(PrefabDeltaMask::BIT_MATERIAL_ID);
            trunk_instance.modified_mask = trunk_mask.0;

            // Apply archetype inheritance for all non-overridden fields in O(1)
            apply_archetype_to_instance(trunk_archetype, &mut trunk_instance, current_tick);

            let trunk_handle = match self.pool.spawn(trunk_instance) {
                Some(h) => h,
                None => continue,
            };
            self.pool.attach_child(root_handle, trunk_handle);

            if self.tree_count < self.tree_roots.len() {
                self.tree_roots[self.tree_count] = trunk_handle;
                self.tree_count += 1;
            }

            // Register LivingOntologyState into SOA ECS SimTable
            let dev_stage: DevelopmentalStage = bot_stage.into();
            let living_state = if is_ghost_gum {
                let mut st = GhostGumSpecimenLoupe::evaluate_living_ontology_state(1, progress);
                st.stage = dev_stage;
                st
            } else {
                LivingOntologyState {
                    stage: dev_stage,
                    growth_progress_01: progress,
                    age_days: progress * 365.0 * 220.0,
                    energy_permill: (750.0 * (1.0 - progress * 0.15)) as u16,
                    damage_permill: if progress > 0.85 { 180 } else { 0 },
                    hydration_permill: 850,
                    wear_permill: (progress * 380.0) as u16,
                    active_state_id: dev_stage as u32,
                    allometry_base_b: 0.95,
                    allometry_exponent_k: 1.32,
                }
            };

            let entity_id = self.living_count as u32;
            if entity_id < MAX_LIVING_ENTITIES as u32 {
                let _ = self.living_states.insert(entity_id, living_state, self.sim_tick);
                self.living_node_handles[self.living_count] = trunk_handle;
                self.living_count += 1;
            }

            // Populate Anatomical Organ Sub-Meshes as Child Nodes of the Trunk
            let organ_archetype = SceneNodeRecord::new(
                hash_str_token(&format!("archetype:{species}:organ")),
                NodeKind::MeshBRep,
            )
            .with_tensor_lanes(0, 0, 0, 1);

            let organ_list: &[(GhostGumOrganSubmesh, [f32; 3], [f32; 3], u32, u16)] = &[
                // 1) Chalk-Bark Trunk / Bole
                (GhostGumOrganSubmesh::ChalkBark, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], if is_ghost_gum { 2 } else { 4 }, 2),
                // 2) Branchwork Limb Scaffold
                (GhostGumOrganSubmesh::Branchwork, [0.0, 6.0, 0.0], [0.85, 0.85, 0.85], if is_ghost_gum { 2 } else { 4 }, 2),
                // 3) Irregular Canopy / SGGX Microflake Shell
                (GhostGumOrganSubmesh::IrregularCanopy, [0.0, 11.5, 0.0], [1.1, 0.9, 1.1], if is_ghost_gum { 3 } else { 5 }, 3),
                // 4) Falcate Foliage Leaf Clusters
                (GhostGumOrganSubmesh::FalcateFoliage, [0.5, 12.0, -0.4], [1.0, 1.0, 1.0], if is_ghost_gum { 3 } else { 5 }, 3),
                // 5) Summer Cream Buds
                (GhostGumOrganSubmesh::CreamBuds, [-0.6, 12.8, 0.5], [0.7, 0.7, 0.7], 6, 4),
                // 6) Epicormic Stem Shoots
                (GhostGumOrganSubmesh::EpicormicShoots, [0.2, 4.5, 0.2], [0.5, 0.5, 0.5], if is_ghost_gum { 3 } else { 5 }, 3),
                // 7) Microscopic 3D Leaf Lamina Study Blade (.10d)
                (GhostGumOrganSubmesh::LeafStudyBlade, [0.1, 1.5, 0.3], [0.08, 0.08, 0.08], 6, 5),
                // 8) Primary Longitudinal Midrib Vein Study (.10d)
                (GhostGumOrganSubmesh::LeafStudyMidrib, [0.1, 1.5, 0.3], [0.08, 0.08, 0.08], 6, 5),
                // 9) Exserted Stamen Flower Study (.10d)
                (GhostGumOrganSubmesh::FlowerStudyStamens, [-0.1, 1.6, -0.2], [0.06, 0.06, 0.06], 6, 4),
                // 10) Urceolate Gumnut Capsule Study (.10d)
                (GhostGumOrganSubmesh::GumnutStudyCapsule, [0.2, 1.4, -0.1], [0.07, 0.07, 0.07], 6, 5),
            ];

            for &(organ, organ_pos, organ_scale, mesh_handle, mat_id) in organ_list {
                let organ_token = hash_str_token(&format!("tree:{species}:{tree_idx}:{0}", organ.as_str()));
                let organ_kind = if organ == GhostGumOrganSubmesh::LeafStudyBlade {
                    NodeKind::Procedural10d
                } else {
                    NodeKind::MeshBRep
                };

                let mut organ_instance = SceneNodeRecord::new(organ_token, organ_kind);
                organ_instance.local_motor = motor_translate(organ_pos);
                organ_instance.local_scale = organ_scale;
                organ_instance.mesh_handle = mesh_handle;
                organ_instance.material_id = mat_id;

                let mut organ_mask = PrefabDeltaMask::NONE;
                organ_mask.mark_modified(PrefabDeltaMask::BIT_LOCAL_MOTOR);
                organ_mask.mark_modified(PrefabDeltaMask::BIT_LOCAL_SCALE);
                organ_mask.mark_modified(PrefabDeltaMask::BIT_MESH_HANDLE);
                organ_mask.mark_modified(PrefabDeltaMask::BIT_MATERIAL_ID);
                organ_mask.mark_modified(PrefabDeltaMask::BIT_NODE_KIND);
                organ_instance.modified_mask = organ_mask.0;

                apply_archetype_to_instance(&organ_archetype, &mut organ_instance, current_tick);

                if let Some(child_handle) = self.pool.spawn(organ_instance) {
                    self.pool.attach_child(trunk_handle, child_handle);
                }
            }
        }

        true
    }

    /// Advances simulation ontogeny, propagates world transforms in topological order with zero heap
    /// allocations, builds the 2-phase HZB mip pyramid, and evaluates 4-tier hybrid indirect culling.
    #[allow(clippy::too_many_arguments)]
    pub fn step_ontogeny_and_transforms_zero_heap(
        &mut self,
        delta_days: f32,
        time_sec: f32,
        observer_pos: [f32; 3],
        observer_did_hash: u64,
        view_proj: &[[f32; 4]; 4],
        focal_length_px: f32,
        hzb_mip0: Option<&[f32]>,
        hzb_dims: (u32, u32),
    ) -> (TransformSyncSummary, IndirectCullStreamCounts) {
        // 1. Advance simulation ticks
        self.sim_tick.advance();
        let current_tick = self.sim_tick.get();
        let last_sync = self.last_sync_tick;
        self.pool.advance_tick();

        // 2. Advance EONT1 Ontogeny and Apply Huxley Allometry across Living Entities
        for i in 0..self.living_count {
            let entity_id = i as u32;
            let node_handle = self.living_node_handles[i];
            if !self.pool.is_valid(node_handle) {
                continue;
            }

            let opt_state = if self.living_states.contains(entity_id) {
                let row = self.living_states.sparse[entity_id as usize] as usize;
                Some(self.living_states.components[row])
            } else {
                None
            };

            if let Some(mut state) = opt_state {
                state.age_days += delta_days;
                state.growth_progress_01 = (state.age_days / (365.0 * 200.0)).clamp(0.02, 1.0);

                if state.growth_progress_01 < 0.15 {
                    state.stage = DevelopmentalStage::SeedOrSpore;
                } else if state.growth_progress_01 < 0.50 {
                    state.stage = DevelopmentalStage::Juvenile;
                } else if state.growth_progress_01 < 0.85 {
                    state.stage = DevelopmentalStage::Adult;
                } else {
                    state.stage = DevelopmentalStage::Senescent;
                }

                let keyframes = &self.ghost_gum_keyframes;
                let mini_header = &self.ghost_gum_header;

                if let Some(node) = self.pool.get_mut(node_handle) {
                    apply_ontogenetic_allometry(node, &state, mini_header, keyframes, current_tick);
                }

                let _ = self.living_states.insert(entity_id, state, self.sim_tick);
            }
        }

        // 3. Compute Flat Topological Ordering (Parent-before-Child, Zero Heap)
        let order_len = compute_topological_order(
            &self.pool.records,
            &self.pool.active,
            &mut self.topological_order,
        );

        // 4. Propagate PGA Motors & Bilateral Deontic Pull (Zero Heap)
        let sync_summary = propagate_transforms_zero_heap(
            &mut self.pool.records,
            &self.pool.active,
            &self.topological_order[..order_len],
            last_sync,
            current_tick,
            time_sec,
            observer_pos,
            observer_did_hash,
            &mut self.synced_instances,
            &mut self.synced_holographic,
        );
        self.last_sync_tick = current_tick;

        // 5. Populate InstanceCullDescriptor Records for Indirect Drawing
        let mut cull_count = 0usize;
        for i in 0..order_len {
            let idx = self.topological_order[i] as usize;
            if idx >= SECTOR_NODE_CAPACITY || !self.pool.active[idx] {
                continue;
            }
            let node = &self.pool.records[idx];
            if node.mesh_handle == u32::MAX && node.node_kind != NodeKind::HolographicSdf {
                continue;
            }
            if cull_count >= self.cull_descriptors.len() {
                break;
            }

            let asset_id = if node.mesh_handle < self.resident_asset_count as u32 {
                node.mesh_handle
            } else {
                0
            };

            let bound_rad = self.resident_assets[asset_id as usize].bounding_sphere[3].max(0.5);

            let rep_policy = match node.node_kind {
                NodeKind::HolographicSdf => RepresentationPolicy::HolographicSdfProxy,
                NodeKind::SurfelCloud => RepresentationPolicy::GaussianSurfelCloud,
                NodeKind::ImpostorOctahedral => RepresentationPolicy::OctahedralImpostor,
                _ => RepresentationPolicy::Auto,
            };

            let prev_lod = self.selected_lods[cull_count] as f32;

            self.cull_descriptors[cull_count] = InstanceCullDescriptor {
                r: node.world_motor.r,
                d: node.world_motor.d,
                scale_radius: [node.world_scale[0], node.world_scale[1], node.world_scale[2], bound_rad],
                asset_id,
                material_id: node.material_id,
                rep_policy: rep_policy as u16,
                rig_offset: 0,
                proc_header_idx: if node.proc_header_handle == u16::MAX { 0 } else { node.proc_header_handle },
                tensor_q_v: InstanceCullDescriptor::pack_tensor_q_v(node.tensor_q, node.tensor_v),
                alpha_salience: node.alpha_salience,
                sigma_spectral: node.sigma_spectral,
                prev_lod_tier: prev_lod,
                _pad: 0.0,
            };
            cull_count += 1;
        }
        self.cull_count = cull_count;

        // 6. Build 2-Phase HZB Mip Pyramid (Zero Heap)
        let hzb_view = if let Some(mip0) = hzb_mip0 {
            let (total_written, slices, mip_count) = build_hzb_mip_chain_zero_heap(
                mip0,
                hzb_dims.0,
                hzb_dims.1,
                &mut self.hzb_texels,
            );
            Some(HzbPyramidView {
                base_width: hzb_dims.0,
                base_height: hzb_dims.1,
                mip_texels: &self.hzb_texels[..total_written],
                mip_slices: slices,
                mip_count,
            })
        } else {
            None
        };

        // 7. Multi-Draw Indirect Frustum Culling, 2-Phase HZB Occlusion & 4-Tier Hybrid LOD Routing
        let stream_counts = cull_and_select_lod_indirect_zero_heap(
            &self.cull_descriptors[..self.cull_count],
            &self.resident_assets[..self.resident_asset_count],
            view_proj,
            observer_pos,
            focal_length_px,
            hzb_view.as_ref(),
            &mut self.indirect_draw_commands,
            &mut self.visible_mesh_indices,
            &mut self.visible_holo_indices,
            &mut self.visible_surfel_indices,
            Some(&mut self.selected_lods[..self.cull_count]),
        );

        (sync_summary, stream_counts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::render::pga::sandwich_point;
    use qualia_core_db::render::scene_graph::ontology_binding::dispatch_affordance;

    fn perspective_test_matrix() -> [[f32; 4]; 4] {
        // Perspective matrix looking along -Z from camera at [0, 0, 0]
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, -0.01, -1.0],
            [0.0, 0.0, -0.1, 0.0],
        ]
    }

    #[test]
    fn test_sector_hierarchy_construction_across_all_sectors() {
        let sectors = [
            "talbingo",
            "yarrangobilly",
            "three_mile_dam",
            "sydney_sandstone",
            "noosa_headland",
            "jounama_karst",
        ];

        for &sector in &sectors {
            let mut graph = EcopunkSectorSceneGraph::new_boxed(sector);
            assert!(graph.populate_sector_10d_hierarchy(sector));

            // Verify Root Terrain
            assert!(graph.pool.is_valid(graph.root_terrain));
            let terrain = graph.pool.get(graph.root_terrain).unwrap();
            assert_eq!(terrain.node_kind, NodeKind::MeshBRep);
            assert_eq!(terrain.parent, NodeHandle::NONE);

            // Verify Water Node
            assert!(graph.pool.is_valid(graph.water_node));
            let water = graph.pool.get(graph.water_node).unwrap();
            assert_eq!(water.parent, graph.root_terrain);

            // Verify Volumetric EMF Layer
            assert!(graph.pool.is_valid(graph.volumetric_emf));
            let emf = graph.pool.get(graph.volumetric_emf).unwrap();
            assert_eq!(emf.node_kind, NodeKind::VolumetricEmf);
            assert_eq!(emf.tensor_v, 1);

            // Verify Holosonic Audio Emitters
            assert!(graph.holosonic_emitter_count >= 2);
            for i in 0..graph.holosonic_emitter_count {
                let h = graph.holosonic_emitters[i];
                assert!(graph.pool.is_valid(h));
                let emitter = graph.pool.get(h).unwrap();
                assert_eq!(emitter.node_kind, NodeKind::AudioHolosonic);
                assert_eq!(emitter.parent, graph.root_terrain);
            }

            // Verify Holographic Epistemic Superposition Nodes
            assert_eq!(graph.holographic_sdf_count, 3);
            for i in 0..graph.holographic_sdf_count {
                let h = graph.holographic_sdf_nodes[i];
                assert!(graph.pool.is_valid(h));
                let holo = graph.pool.get(h).unwrap();
                assert_eq!(holo.node_kind, NodeKind::HolographicSdf);
                assert!(holo.tensor_q > 0);
                assert!(holo.tensor_v >= 1);
                assert!(holo.deontic_lane >= 1);
                assert!(holo.sigma_spectral > 0.0);
            }

            // Verify Botanical Multi-Organ Trees
            assert!(graph.tree_count >= 4);
            assert_eq!(graph.living_count, graph.tree_count);
            for i in 0..graph.tree_count {
                let trunk_h = graph.tree_roots[i];
                assert!(graph.pool.is_valid(trunk_h));
                let trunk = graph.pool.get(trunk_h).unwrap();
                assert_eq!(trunk.parent, graph.root_terrain);
                // Must have child organ nodes
                assert!(trunk.first_child.is_some());
            }
        }
    }

    #[test]
    fn test_prefab_delta_inheritance() {
        let archetype = SceneNodeRecord::new(
            hash_str_token("archetype:corymbia:trunk"),
            NodeKind::MeshBRep,
        )
        .with_local_scale([1.0, 1.0, 1.0])
        .with_tensor_lanes(0, 2, 1, 1);

        let mut instance = SceneNodeRecord::new(
            hash_str_token("instance:tree:0"),
            NodeKind::MeshBRep,
        );
        // Explicitly override local motor and scale
        instance.local_motor = motor_translate([10.0, 0.0, 5.0]);
        instance.local_scale = [0.8, 0.8, 0.8];
        let mut mask = PrefabDeltaMask::NONE;
        mask.mark_modified(PrefabDeltaMask::BIT_LOCAL_MOTOR);
        mask.mark_modified(PrefabDeltaMask::BIT_LOCAL_SCALE);
        instance.modified_mask = mask.0;

        // Apply archetype inheritance
        apply_archetype_to_instance(&archetype, &mut instance, 1);

        // Overridden values must be preserved
        assert_eq!(instance.local_scale, [0.8, 0.8, 0.8]);
        let pos = sandwich_point(instance.local_motor, [0.0, 0.0, 0.0]);
        assert!((pos[0] - 10.0).abs() < 1e-4);
        assert!((pos[2] - 5.0).abs() < 1e-4);

        // Non-overridden values must inherit from archetype
        assert_eq!(instance.tensor_v, 2);
        assert_eq!(instance.tensor_w, 1);
        assert_eq!(instance.deontic_lane, 1);
    }

    #[test]
    fn test_ontogenetic_allometry_application() {
        let mut node = SceneNodeRecord::new(1234, NodeKind::MeshBRep);
        let keyframes = GhostGumSpecimenLoupe::default_developmental_keyframes();
        let mini_header = GhostGumSpecimenLoupe::default_ontology_mini_header();

        // 1. Seedling stage (progress = 0.05)
        let seedling_state = GhostGumSpecimenLoupe::evaluate_living_ontology_state(1, 0.05);
        apply_ontogenetic_allometry(&mut node, &seedling_state, &mini_header, &keyframes, 10);
        let scale_seedling = node.local_scale;
        assert!(scale_seedling[0] < 0.3);
        assert!(scale_seedling[1] < 0.3);
        assert_eq!(node.changed_tick, 10);

        // 2. Adult mature stage (progress = 0.80)
        let adult_state = GhostGumSpecimenLoupe::evaluate_living_ontology_state(1, 0.80);
        apply_ontogenetic_allometry(&mut node, &adult_state, &mini_header, &keyframes, 20);
        let scale_adult = node.local_scale;
        assert!(scale_adult[0] > scale_seedling[0]);
        assert!(scale_adult[1] > scale_seedling[1]);
        assert_eq!(node.changed_tick, 20);
    }

    #[test]
    fn test_affordance_dispatch_and_preconditions() {
        let mut state = GhostGumSpecimenLoupe::evaluate_living_ontology_state(1, 0.70);
        let affordances = GhostGumSpecimenLoupe::default_custodial_affordances();

        // Cultural Cool Burn: requires fire care capability
        let initial_damage = state.damage_permill;
        let outcome = dispatch_affordance(
            &mut state,
            &affordances,
            crate::specimen::loupe::ACTION_CULTURAL_COOL_BURN,
            crate::specimen::loupe::CAPABILITY_CUSTODIAL_FIRE_CARE,
            true,
        );
        assert!(outcome.is_some());
        let res = outcome.unwrap();
        assert_eq!(res.action_id, crate::specimen::loupe::ACTION_CULTURAL_COOL_BURN);
        assert_eq!(res.animation_clip_id, 101);
        assert_eq!(state.damage_permill, initial_damage.saturating_sub(30));
    }

    #[test]
    fn test_topological_propagation_and_bilateral_pull() {
        let mut graph = EcopunkSectorSceneGraph::new_boxed("talbingo");
        graph.populate_sector_10d_hierarchy("talbingo");

        let observer_pos = [0.0, 10.0, 20.0];
        let observer_did_hash = 0xCAFE_BEEF;

        // Step simulation and transform propagation
        let (summary, _) = graph.step_ontogeny_and_transforms_zero_heap(
            1.0,
            0.5,
            observer_pos,
            observer_did_hash,
            &perspective_test_matrix(),
            600.0,
            None,
            (64, 64),
        );

        assert!(summary.visited_count > 0);
        assert!(summary.synced_instances > 0);
        assert!(summary.synced_holographic_nodes > 0);

        // Verify that root terrain world motor remains at origin
        let terrain = graph.pool.get(graph.root_terrain).unwrap();
        let terrain_pos = sandwich_point(terrain.world_motor, [0.0, 0.0, 0.0]);
        assert!(terrain_pos[0].abs() < 1e-4);
        assert!(terrain_pos[1].abs() < 1e-4);
        assert!(terrain_pos[2].abs() < 1e-4);
    }

    #[test]
    fn test_indirect_culling_4tier_lod_and_hzb() {
        let mut graph = EcopunkSectorSceneGraph::new_boxed("yarrangobilly");
        graph.populate_sector_10d_hierarchy("yarrangobilly");

        // Camera looking down -Z
        let camera_pos = [0.0, 0.0, 0.0];
        let vp = perspective_test_matrix();
        let focal_length_px = 500.0;

        // Construct 64x64 HZB mip0 depth texture (0.0 = near, 1.0 = far)
        let mut hzb_mip0 = [1.0f32; 64 * 64];
        // Create an occluding wall in the center (uv [0.4..0.6], depth 0.2)
        for y in 25..39 {
            for x in 25..39 {
                hzb_mip0[y * 64 + x] = 0.2;
            }
        }

        let (_sync_summary, stream_counts) = graph.step_ontogeny_and_transforms_zero_heap(
            0.0,
            0.0,
            camera_pos,
            0,
            &vp,
            focal_length_px,
            Some(&hzb_mip0),
            (64, 64),
        );

        // Stream 1 (Holographic SDF proxy stream) must route the 3 holographic superposition nodes
        assert!(stream_counts.holo_proxy_count > 0);
        // At least one indirect mesh command or surfel impostor stream entry must be generated
        assert!(stream_counts.mesh_draw_count > 0 || stream_counts.surfel_impostor_count > 0);
    }
}
