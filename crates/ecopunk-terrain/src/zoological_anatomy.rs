//! Comparative Vertebrate Anatomy & 3-Layer Biomechanical Living Containers (`zoological_anatomy.rs`).
//!
//! Implements **Epic 2** (Scale 5 Comparative Vertebrate Anatomy) of the Biological Education Assets
//! and Explorable World master plan (`docs/plans/biological-education-assets-and-explorable-world-plan.md`).
//!
//! Covers all 13 Australian fauna biomechanical archetypes across 3 inspectable anatomical layers:
//! - **Layer 0 — Articulated Osteology** (8 regions including diprotodont rostrum, hypsodont dentition,
//!   keeled carina sternum, furcula, and characteristic marsupial/monotreme **epipubic bones**!).
//! - **Layer 1 — Major Myology & Tendons** (8 functional groups including the elastic
//!   **Achilles tendon spring** energy-recovery store, massive flight pectorals, and fossorial deltoids).
//! - **Layer 2 — External Integument** (high-resolution outer skin, pelage, plumage, or squamation).
//!
//! Generates >= 1,800 vertex high-resolution meshes bound to `FaunaSkeletalRig10d` (18 PGA joints),
//! seals them into `.10d` Living Containers with `MAT4`, `UV01`, `FNRM`, `SRD1`, `PROC1`, `EONT1`,
//! `MRP1` (costal respiration, gular vocalization, Achilles tendon strain), and `PROV`.
//! Also provides zero-heap layer peeling and exploded-view cross-fading.

use bytemuck;
use serde::{Deserialize, Serialize};

use qualia_core_db::container_10d::entity_ontology_section::{
    AffordanceBinaryRecord, DevelopmentalKeyframeRecord, EntityOntologyMiniHeader,
    AFFORDANCE_RECORD_SIZE, DEVELOPMENTAL_KEYFRAME_RECORD_SIZE,
    ENTITY_ONTOLOGY_MINI_HEADER_SIZE, ENTITY_ONTOLOGY_SECTION_VERSION, MAGIC_EONT,
};
use qualia_core_db::container_10d::header::{Container10dHeader, MAX_SECTION_COUNT};
use qualia_core_db::container_10d::integrity::seal_whole_file_crc32c;
use qualia_core_db::container_10d::material_container::attach_material_section;
use qualia_core_db::container_10d::material_section::{
    MaterialRecord, OpacityMode, ShadingModel, SubmeshRange,
};
use qualia_core_db::container_10d::morph_section::{
    encode_morph_section, morph_section_encoded_len, quantize_morph_delta, MorphSectionHeader,
    QuantizedMorphDelta, FLAG_MORPH_HAS_NORMALS, FLAG_MORPH_ONTOGENETIC_BIND, FLAG_MORPH_SPARSE,
    MAGIC_MRP1, MORPH_SECTION_VERSION,
};
use qualia_core_db::container_10d::procedural_synthesis::{
    encode_procedural_synthesis_section, ProceduralSynthesisHeader, FLAG_ALLOMETRIC_GROWTH,
    FLAG_CELLULAR_VORONOI, FLAG_IMPLICIT_FILLET, FLAG_PGA_MOTOR_DYNAMICS, MAGIC_PROC,
    PROCEDURAL_SYNTHESIS_HEADER_SIZE, PROCEDURAL_SYNTHESIS_SECTION_VERSION,
};
use qualia_core_db::container_10d::provenance_section::ProvenanceSidecar;
use qualia_core_db::container_10d::section::{
    encode_container, parse_section_table, AlignmentTier, SectionInput, SectionTableError,
    SectionType,
};
use qualia_core_db::container_10d::texture_coordinate_container::attach_texture_coordinate_section;
use qualia_core_db::q_hash;
use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::compile_10d::{
    compile_mesh_to_10d_with_extras_and_frames, Compile10dExtras,
};

use crate::fauna_animation_rig::{
    FaunaBiomechanicalArchetype, FaunaSkeletalRig10d, JOINT_CHEST, JOINT_HEAD,
    JOINT_JAW, JOINT_LEFT_ANKLE, JOINT_LEFT_ELBOW, JOINT_LEFT_HIP, JOINT_LEFT_KNEE,
    JOINT_LEFT_SHOULDER, JOINT_LEFT_WRIST_WING, JOINT_NECK, JOINT_PELVIS_ROOT, JOINT_RIGHT_ANKLE,
    JOINT_RIGHT_HIP, JOINT_RIGHT_KNEE, JOINT_SPINE, JOINT_TAIL_BASE, JOINT_TAIL_MID,
    JOINT_TAIL_TIP,
};

// ── Anatomical Layers ────────────────────────────────────────────────────────

/// 3 Primary Comparative Anatomical Layers for Scale 5 vertebrate dissection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZoologicalAnatomicalLayer {
    /// Layer 0: Articulated skeleton.
    Osteology,
    /// Layer 1: Functional muscle groups & elastic energy-recovery tendons.
    MyologyAndTendons,
    /// Layer 2: Pelage, plumage, squamation, or amphibian skin.
    ExternalIntegument,
}

// ── Osteology Regions ────────────────────────────────────────────────────────

/// 8 Comparative Vertebrate Osteology Regions (Layer 0).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VertebrateOsteologyRegion {
    /// Cranium & Rostrum (diprotodont diastema / monotreme electrosensory bill / raptor beak / lizard diapsid).
    CraniumAndRostrum,
    /// Mandible & Dentition (procumbent lower incisors / hypsodont molars / toothless keratinous pads).
    MandibleAndDentition,
    /// Cervical, Thoracic & Lumbar vertebral column (axial spine support).
    CervicalThoracicLumbarVertebrae,
    /// Ribcage & Sternum (including avian keeled carina & furcula wishbone).
    RibcageAndSternum,
    /// Pectoral Girdle & Forelimb (scapula, humerus, radius, ulna, carpals, pentadactyl digits / fossorial claws).
    PectoralGirdleAndForelimb,
    /// Pelvic Girdle & Epipubic Bones (ilium, ischium, pubis + characteristic marsupial/monotreme epipubic bones!).
    PelvicGirdleAndEpipubicBones,
    /// Hindlimb & Syndactyl Pes (femur, elongated tibia/fibula, calcaneum, elongated 4th metatarsal, syndactyl grooming digits II & III).
    HindlimbAndSyndactylPes,
    /// Caudal Vertebral Chain (muscular counterbalance/pentapedal fifth-limb tail in macropods; flattened rudder tail in platypus).
    CaudalVertebralChain,
}

impl VertebrateOsteologyRegion {
    pub const ALL: [Self; 8] = [
        Self::CraniumAndRostrum,
        Self::MandibleAndDentition,
        Self::CervicalThoracicLumbarVertebrae,
        Self::RibcageAndSternum,
        Self::PectoralGirdleAndForelimb,
        Self::PelvicGirdleAndEpipubicBones,
        Self::HindlimbAndSyndactylPes,
        Self::CaudalVertebralChain,
    ];

    pub const fn name(&self) -> &'static str {
        match self {
            Self::CraniumAndRostrum => "Cranium & Rostrum",
            Self::MandibleAndDentition => "Mandible & Dentition",
            Self::CervicalThoracicLumbarVertebrae => "Cervical, Thoracic & Lumbar Vertebrae",
            Self::RibcageAndSternum => "Ribcage & Sternum (Carina / Furcula)",
            Self::PectoralGirdleAndForelimb => "Pectoral Girdle & Forelimb",
            Self::PelvicGirdleAndEpipubicBones => "Pelvic Girdle & Epipubic Bones",
            Self::HindlimbAndSyndactylPes => "Hindlimb & Syndactyl Pes",
            Self::CaudalVertebralChain => "Caudal Vertebral Chain",
        }
    }
}

// ── Myology Groups ───────────────────────────────────────────────────────────

/// 8 Comparative Vertebrate Functional Myology Groups & Tendon Springs (Layer 1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VertebrateMyologyGroup {
    /// Gastrocnemius & Plantaris (power calf propulsion).
    GastrocnemiusAndPlantaris,
    /// Elastic Achilles Tendon Spring (stores and returns up to 65% metabolic work per hop/stride).
    ElasticAchillesTendonSpring,
    /// Gluteus & Biceps Femoris (massive thigh hip-extensor mass).
    GluteusAndBicepsFemoris,
    /// Iliopsoas & Caudofemoralis (deep hip flexor & tail-to-femur retractor).
    IliopsoasAndCaudofemoralis,
    /// Pectoralis Major & Supracoracoideus (massive chest flight/forelimb adductor).
    PectoralisMajorAndSupracoracoideus,
    /// Deltoid & Triceps Fossorial (powerful digging/climbing shoulder abductors).
    DeltoidAndTricepsFossorial,
    /// Masseter & Temporalis (jaw mastication muscles powering diprotodont shearing).
    MasseterAndTemporalis,
    /// Intercostals & Diaphragm (thoracic respiratory expansion & costal ventilation).
    IntercostalsAndDiaphragm,
}

impl VertebrateMyologyGroup {
    pub const ALL: [Self; 8] = [
        Self::GastrocnemiusAndPlantaris,
        Self::ElasticAchillesTendonSpring,
        Self::GluteusAndBicepsFemoris,
        Self::IliopsoasAndCaudofemoralis,
        Self::PectoralisMajorAndSupracoracoideus,
        Self::DeltoidAndTricepsFossorial,
        Self::MasseterAndTemporalis,
        Self::IntercostalsAndDiaphragm,
    ];

    pub const fn name(&self) -> &'static str {
        match self {
            Self::GastrocnemiusAndPlantaris => "Gastrocnemius & Plantaris",
            Self::ElasticAchillesTendonSpring => "Elastic Achilles Tendon Spring",
            Self::GluteusAndBicepsFemoris => "Gluteus & Biceps Femoris",
            Self::IliopsoasAndCaudofemoralis => "Iliopsoas & Caudofemoralis",
            Self::PectoralisMajorAndSupracoracoideus => "Pectoralis Major & Supracoracoideus",
            Self::DeltoidAndTricepsFossorial => "Deltoid & Triceps Fossorial",
            Self::MasseterAndTemporalis => "Masseter & Temporalis",
            Self::IntercostalsAndDiaphragm => "Intercostals & Diaphragm",
        }
    }
}

// ── Compiled Vertebrate Anatomy Container ────────────────────────────────────

/// Complete 3-Layer Comparative Anatomical Twin sealed into a `.10d` Living Container.
#[derive(Clone, Debug)]
pub struct CompiledVertebrateAnatomy10d {
    /// Biomechanical archetype.
    pub archetype: FaunaBiomechanicalArchetype,
    /// 18-joint skeletal rig and kinematics definition.
    pub rig: FaunaSkeletalRig10d,
    /// Fully sealed and CRC-32C verified `.10d` living container binary.
    pub container_bytes: Vec<u8>,
    /// Total vertices across all 3 layers (>= 1,800).
    pub vertex_count: usize,
    /// Total triangles across all 3 layers.
    pub triangle_count: usize,
    /// Index range for Osteology (Layer 0).
    pub osteology_vertex_range: std::ops::Range<usize>,
    /// Index range for Myology (Layer 1).
    pub myology_vertex_range: std::ops::Range<usize>,
    /// Index range for External Integument (Layer 2).
    pub integument_vertex_range: std::ops::Range<usize>,
    /// CPU vertex positions [x, y, z] in rest pose.
    pub positions: Vec<[f32; 3]>,
    /// CPU vertex normals [nx, ny, nz].
    pub normals: Vec<[f32; 3]>,
    /// CPU RGBA8 surface readings / colors [r, g, b, a].
    pub colors: Vec<[f32; 4]>,
    /// CPU texture coordinates [u, v].
    pub uvs: Vec<[f32; 2]>,
    /// Contiguous index stream for all submeshes.
    pub indices: Vec<u32>,
    /// 17 PBR material records (8 osteology, 8 myology, 1 integument).
    pub materials: Vec<MaterialRecord>,
    /// 17 submesh index intervals.
    pub submesh_ranges: Vec<SubmeshRange>,
    /// GPU joint indices (up to 4 joints per vertex).
    pub skin_joint_indices: Vec<[u16; 4]>,
    /// GPU joint blending weights (summing to 1.0).
    pub skin_weights: Vec<[f32; 4]>,
    /// Per-vertex anatomical layer classification.
    pub layer_tags: Vec<ZoologicalAnatomicalLayer>,
}

// ── Zero-Heap Layer Peeling & Exploded View ──────────────────────────────────

/// Evaluates interactive layer peeling and exploded-view displacement between
/// `ExternalIntegument`, `MyologyAndTendons`, and `Osteology` in zero heap allocations.
///
/// Modulates alpha opacity and radially displaces muscles and integument outward along
/// surface normals, revealing deeper anatomical layers without allocating memory.
///
/// Returns the number of vertices evaluated.
#[inline]
pub fn evaluate_anatomical_layer_peel_zero_heap(
    in_positions: &[[f32; 3]],
    in_normals: &[[f32; 3]],
    in_layer_tags: &[ZoologicalAnatomicalLayer],
    osteology_vis_01: f32,
    myology_vis_01: f32,
    integument_vis_01: f32,
    explode_displacement_01: f32,
    out_positions: &mut [[f32; 3]],
    out_alphas: &mut [f32],
) -> usize {
    let count = in_positions
        .len()
        .min(in_normals.len())
        .min(in_layer_tags.len())
        .min(out_positions.len())
        .min(out_alphas.len());

    let ost_vis = osteology_vis_01.clamp(0.0, 1.0);
    let myo_vis = myology_vis_01.clamp(0.0, 1.0);
    let int_vis = integument_vis_01.clamp(0.0, 1.0);
    let explode = explode_displacement_01.clamp(0.0, 1.0);

    for i in 0..count {
        let p = in_positions[i];
        let n = in_normals[i];
        let layer = in_layer_tags[i];

        let (alpha, disp_dist) = match layer {
            ZoologicalAnatomicalLayer::Osteology => (ost_vis, 0.0),
            ZoologicalAnatomicalLayer::MyologyAndTendons => (myo_vis, explode * 0.08),
            ZoologicalAnatomicalLayer::ExternalIntegument => (int_vis, explode * 0.18),
        };

        out_positions[i] = [
            p[0] + n[0] * disp_dist,
            p[1] + n[1] * disp_dist,
            p[2] + n[2] * disp_dist,
        ];
        out_alphas[i] = alpha;
    }

    count
}

// ── Mesh Accumulator & Geometry Generators ───────────────────────────────────

struct MeshAccumulator {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
    layer_tags: Vec<ZoologicalAnatomicalLayer>,
    joint_indices: Vec<[u16; 4]>,
    joint_weights: Vec<[f32; 4]>,
}

impl MeshAccumulator {
    fn new() -> Self {
        Self {
            positions: Vec::with_capacity(3600),
            normals: Vec::with_capacity(3600),
            colors: Vec::with_capacity(3600),
            uvs: Vec::with_capacity(3600),
            indices: Vec::with_capacity(7200),
            layer_tags: Vec::with_capacity(3600),
            joint_indices: Vec::with_capacity(3600),
            joint_weights: Vec::with_capacity(3600),
        }
    }

    fn push_cylinder(
        &mut self,
        p1: [f32; 3],
        p2: [f32; 3],
        r1: f32,
        r2: f32,
        radial_segs: usize,
        height_segs: usize,
        color: [f32; 4],
        layer: ZoologicalAnatomicalLayer,
        primary_joint: usize,
        secondary_joint: usize,
        primary_weight: f32,
    ) {
        let base_idx = self.positions.len() as u32;
        let axis = [p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2]];
        let height = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt().max(1e-5);
        let dir = [axis[0] / height, axis[1] / height, axis[2] / height];

        let up = if dir[1].abs() < 0.95 {
            [0.0, 1.0, 0.0]
        } else {
            [1.0, 0.0, 0.0]
        };
        let side = [
            dir[1] * up[2] - dir[2] * up[1],
            dir[2] * up[0] - dir[0] * up[2],
            dir[0] * up[1] - dir[1] * up[0],
        ];
        let side_len = (side[0] * side[0] + side[1] * side[1] + side[2] * side[2]).sqrt().max(1e-5);
        let u_vec = [side[0] / side_len, side[1] / side_len, side[2] / side_len];
        let v_vec = [
            dir[1] * u_vec[2] - dir[2] * u_vec[1],
            dir[2] * u_vec[0] - dir[0] * u_vec[2],
            dir[0] * u_vec[1] - dir[1] * u_vec[0],
        ];

        let r_segs = radial_segs.max(6);
        let h_segs = height_segs.max(1);

        for h in 0..=h_segs {
            let t = h as f32 / h_segs as f32;
            let center = [
                p1[0] + dir[0] * height * t,
                p1[1] + dir[1] * height * t,
                p1[2] + dir[2] * height * t,
            ];
            let radius = r1 + (r2 - r1) * t;
            let w1 = primary_weight;
            let w2 = 1.0 - w1;

            for s in 0..=r_segs {
                let theta = (s as f32 / r_segs as f32) * core::f32::consts::TAU;
                let cos_t = theta.cos();
                let sin_t = theta.sin();

                let normal = [
                    u_vec[0] * cos_t + v_vec[0] * sin_t,
                    u_vec[1] * cos_t + v_vec[1] * sin_t,
                    u_vec[2] * cos_t + v_vec[2] * sin_t,
                ];
                let pos = [
                    center[0] + normal[0] * radius,
                    center[1] + normal[1] * radius,
                    center[2] + normal[2] * radius,
                ];

                self.positions.push(pos);
                self.normals.push(normal);
                self.colors.push(color);
                self.uvs.push([s as f32 / r_segs as f32, t]);
                self.layer_tags.push(layer);
                self.joint_indices.push([primary_joint as u16, secondary_joint as u16, 0, 0]);
                self.joint_weights.push([w1, w2, 0.0, 0.0]);
            }
        }

        let stride = (r_segs + 1) as u32;
        for h in 0..h_segs as u32 {
            for s in 0..r_segs as u32 {
                let i00 = base_idx + h * stride + s;
                let i10 = i00 + stride;
                let i01 = i00 + 1;
                let i11 = i10 + 1;

                self.indices.extend_from_slice(&[i00, i10, i11, i00, i11, i01]);
            }
        }
    }

    fn push_ellipsoid(
        &mut self,
        center: [f32; 3],
        radii: [f32; 3],
        u_segs: usize,
        v_segs: usize,
        color: [f32; 4],
        layer: ZoologicalAnatomicalLayer,
        primary_joint: usize,
        secondary_joint: usize,
        primary_weight: f32,
    ) {
        let base_idx = self.positions.len() as u32;
        let u_n = u_segs.max(8);
        let v_n = v_segs.max(6);

        for v in 0..=v_n {
            let phi = (v as f32 / v_n as f32) * core::f32::consts::PI;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            for u in 0..=u_n {
                let theta = (u as f32 / u_n as f32) * core::f32::consts::TAU;
                let cos_theta = theta.cos();
                let sin_theta = theta.sin();

                let nx = sin_phi * cos_theta;
                let ny = cos_phi;
                let nz = sin_phi * sin_theta;

                let px = center[0] + radii[0] * nx;
                let py = center[1] + radii[1] * ny;
                let pz = center[2] + radii[2] * nz;

                let normal = [
                    nx / radii[0].max(1e-4),
                    ny / radii[1].max(1e-4),
                    nz / radii[2].max(1e-4),
                ];
                let n_len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt().max(1e-5);
                let unit_normal = [normal[0] / n_len, normal[1] / n_len, normal[2] / n_len];

                self.positions.push([px, py, pz]);
                self.normals.push(unit_normal);
                self.colors.push(color);
                self.uvs.push([u as f32 / u_n as f32, v as f32 / v_n as f32]);
                self.layer_tags.push(layer);
                self.joint_indices.push([primary_joint as u16, secondary_joint as u16, 0, 0]);
                self.joint_weights.push([primary_weight, 1.0 - primary_weight, 0.0, 0.0]);
            }
        }

        let stride = (u_n + 1) as u32;
        for v in 0..v_n as u32 {
            for u in 0..u_n as u32 {
                let i00 = base_idx + v * stride + u;
                let i10 = i00 + stride;
                let i01 = i00 + 1;
                let i11 = i10 + 1;

                self.indices.extend_from_slice(&[i00, i10, i11, i00, i11, i01]);
            }
        }
    }

    fn push_box(
        &mut self,
        min: [f32; 3],
        max: [f32; 3],
        color: [f32; 4],
        layer: ZoologicalAnatomicalLayer,
        primary_joint: usize,
    ) {
        let base_idx = self.positions.len() as u32;

        let corners = [
            [min[0], min[1], min[2]],
            [max[0], min[1], min[2]],
            [max[0], max[1], min[2]],
            [min[0], max[1], min[2]],
            [min[0], min[1], max[2]],
            [max[0], min[1], max[2]],
            [max[0], max[1], max[2]],
            [min[0], max[1], max[2]],
        ];

        let center = [
            (min[0] + max[0]) * 0.5,
            (min[1] + max[1]) * 0.5,
            (min[2] + max[2]) * 0.5,
        ];

        for c in corners {
            let n = [c[0] - center[0], c[1] - center[1], c[2] - center[2]];
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt().max(1e-5);
            self.positions.push(c);
            self.normals.push([n[0] / len, n[1] / len, n[2] / len]);
            self.colors.push(color);
            self.uvs.push([0.5, 0.5]);
            self.layer_tags.push(layer);
            self.joint_indices.push([primary_joint as u16, 0, 0, 0]);
            self.joint_weights.push([1.0, 0.0, 0.0, 0.0]);
        }

        let face_indices = [
            0, 2, 1, 0, 3, 2, // -Z
            4, 5, 6, 4, 6, 7, // +Z
            0, 1, 5, 0, 5, 4, // -Y
            3, 6, 2, 3, 7, 6, // +Y
            0, 4, 7, 0, 7, 3, // -X
            1, 2, 6, 1, 6, 5, // +X
        ];

        for &idx in &face_indices {
            self.indices.push(base_idx + idx);
        }
    }
}

// ── PBR Material Builder Helpers ─────────────────────────────────────────────

fn make_pbr_material(
    id_str: &str,
    base_color: [f32; 4],
    roughness: f32,
    metallic: f32,
) -> MaterialRecord {
    let mut mat = MaterialRecord::legacy_default();
    mat.id = q_hash(id_str);
    mat.shading_model = ShadingModel::MetallicRoughness;
    mat.opacity_mode = OpacityMode::Opaque;
    mat.base_color = base_color;
    mat.roughness = roughness;
    mat.metallic = metallic;
    mat
}

// ── Main Anatomical 10D Builder ──────────────────────────────────────────────

/// Builds a high-resolution, 3-layer Comparative Vertebrate Anatomical Digital Twin
/// bound to `FaunaSkeletalRig10d` (18 PGA joints) and sealed into a `.10d` Living Container.
///
/// Features:
/// - >= 1,800 vertices across 8 Osteology regions, 8 Myology groups, and External Integument.
/// - Distinct submesh ranges for all 17 anatomical structures.
/// - Valid `.10d` container with `MAT4`, `UV01`, `FNRM`, `SRD1`, `PROC1`, `EONT1`, `MRP1`, and `PROV`.
/// - 3 Morph Targets: Costal Respiration, Gular Vocalization, and Achilles Tendon Strain.
pub fn build_scale5_vertebrate_anatomy_10d(
    archetype: FaunaBiomechanicalArchetype,
    osteology_vis_01: f32,
    myology_vis_01: f32,
    integument_vis_01: f32,
    explode_layer_01: f32,
) -> CompiledVertebrateAnatomy10d {
    let rig = FaunaSkeletalRig10d::new(archetype);
    let (body_len, hip_height) = archetype.default_dimensions();

    let mut acc = MeshAccumulator::new();
    let mut submesh_ranges = Vec::with_capacity(17);
    let mut materials = Vec::with_capacity(17);

    // Anatomical proportions
    let spine_len = (body_len * 0.32).max(0.10);
    let neck_len = (body_len * 0.18).max(0.06);
    let head_len = (body_len * 0.22).max(0.08);
    let tail_len = (body_len * 0.40).max(0.12);
    let half_width = (body_len * 0.14).max(0.04);
    let femur_len = (hip_height * 0.48).max(0.08);
    let tibia_len = (hip_height * 0.54).max(0.08);

    // ── LAYER 0: OSTEOLOGY (8 Regions) ───────────────────────────────────────
    let ost_start_v = acc.positions.len();

    // 1. Cranium & Rostrum
    let cranium_start = acc.indices.len() as u32;
    let cranium_head_pos = [0.0, hip_height + neck_len * 0.6, spine_len * 0.7 + neck_len];
    let rostrum_tip = [0.0, cranium_head_pos[1] - 0.02, cranium_head_pos[2] + head_len];

    // Braincase ellipsoid
    acc.push_ellipsoid(
        cranium_head_pos,
        [half_width * 0.55, half_width * 0.50, head_len * 0.42],
        12,
        8,
        [0.88, 0.86, 0.80, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_HEAD,
        JOINT_NECK,
        0.85,
    );

    // Snout / Rostrum / Beak
    let is_monotreme = archetype == FaunaBiomechanicalArchetype::MonotremeTeleostAmphibian;
    let is_avian = matches!(
        archetype,
        FaunaBiomechanicalArchetype::RaptorSoaring
            | FaunaBiomechanicalArchetype::PasserinePsittacine
            | FaunaBiomechanicalArchetype::RatiteBipedal
    );
    let rostrum_r1 = if is_monotreme { half_width * 0.45 } else { half_width * 0.35 };
    let rostrum_r2 = if is_avian { 0.005 } else { half_width * 0.22 };

    acc.push_cylinder(
        cranium_head_pos,
        rostrum_tip,
        rostrum_r1,
        rostrum_r2,
        10,
        3,
        [0.86, 0.84, 0.78, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_HEAD,
        JOINT_JAW,
        0.90,
    );

    // Zygomatic arches (L+R)
    acc.push_cylinder(
        [-half_width * 0.48, cranium_head_pos[1] - 0.02, cranium_head_pos[2] - 0.02],
        [-half_width * 0.42, cranium_head_pos[1] - 0.01, cranium_head_pos[2] + head_len * 0.35],
        0.008,
        0.006,
        8,
        2,
        [0.87, 0.85, 0.79, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_HEAD,
        JOINT_NECK,
        0.90,
    );
    acc.push_cylinder(
        [half_width * 0.48, cranium_head_pos[1] - 0.02, cranium_head_pos[2] - 0.02],
        [half_width * 0.42, cranium_head_pos[1] - 0.01, cranium_head_pos[2] + head_len * 0.35],
        0.008,
        0.006,
        8,
        2,
        [0.87, 0.85, 0.79, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_HEAD,
        JOINT_NECK,
        0.90,
    );

    let cranium_mat = make_pbr_material("mat_ost_cranium", [0.88, 0.86, 0.80, 1.0], 0.45, 0.05);
    materials.push(cranium_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: cranium_start,
        index_count: acc.indices.len() as u32 - cranium_start,
        material_id: cranium_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:cranium_rostrum"),
    });

    // 2. Mandible & Dentition (procumbent lower incisors / hypsodont molars)
    let mandible_start = acc.indices.len() as u32;
    let jaw_joint_l = [-half_width * 0.40, cranium_head_pos[1] - 0.04, cranium_head_pos[2]];
    let jaw_joint_r = [half_width * 0.40, cranium_head_pos[1] - 0.04, cranium_head_pos[2]];
    let chin_pos = [0.0, cranium_head_pos[1] - 0.07, rostrum_tip[2] - 0.02];

    acc.push_cylinder(
        jaw_joint_l,
        chin_pos,
        half_width * 0.16,
        half_width * 0.12,
        8,
        3,
        [0.87, 0.85, 0.79, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_JAW,
        JOINT_HEAD,
        0.85,
    );
    acc.push_cylinder(
        jaw_joint_r,
        chin_pos,
        half_width * 0.16,
        half_width * 0.12,
        8,
        3,
        [0.87, 0.85, 0.79, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_JAW,
        JOINT_HEAD,
        0.85,
    );

    // Procumbent lower incisors (characteristic of Diprotodontia)
    let incisor_tip = [0.0, chin_pos[1] + 0.015, chin_pos[2] + 0.035];
    acc.push_cylinder(
        chin_pos,
        incisor_tip,
        0.007,
        0.004,
        6,
        2,
        [0.94, 0.93, 0.90, 1.0], // Enamel ivory
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_JAW,
        JOINT_HEAD,
        0.95,
    );

    let mandible_mat = make_pbr_material("mat_ost_mandible", [0.90, 0.88, 0.82, 1.0], 0.32, 0.02);
    materials.push(mandible_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: mandible_start,
        index_count: acc.indices.len() as u32 - mandible_start,
        material_id: mandible_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:mandible_dentition"),
    });

    // 3. Cervical, Thoracic & Lumbar Vertebrae
    let vert_start = acc.indices.len() as u32;
    let pelvis_pos = [0.0, hip_height, 0.0];
    let chest_pos = [0.0, hip_height + 0.05, spine_len * 0.7];

    // Cervical vertebrae (neck)
    acc.push_cylinder(
        chest_pos,
        cranium_head_pos,
        half_width * 0.22,
        half_width * 0.18,
        8,
        5,
        [0.85, 0.83, 0.77, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_NECK,
        JOINT_CHEST,
        0.70,
    );

    // Thoracic & Lumbar vertebrae (dorsal column)
    acc.push_cylinder(
        pelvis_pos,
        chest_pos,
        half_width * 0.26,
        half_width * 0.22,
        8,
        6,
        [0.85, 0.83, 0.77, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_SPINE,
        JOINT_CHEST,
        0.55,
    );

    // Neural spines
    for s in 1..=5 {
        let t = s as f32 / 6.0;
        let c = [
            pelvis_pos[0] + (chest_pos[0] - pelvis_pos[0]) * t,
            pelvis_pos[1] + (chest_pos[1] - pelvis_pos[1]) * t,
            pelvis_pos[2] + (chest_pos[2] - pelvis_pos[2]) * t,
        ];
        acc.push_box(
            [c[0] - 0.005, c[1] + 0.015, c[2] - 0.015],
            [c[0] + 0.005, c[1] + 0.045, c[2] + 0.015],
            [0.84, 0.82, 0.76, 1.0],
            ZoologicalAnatomicalLayer::Osteology,
            JOINT_SPINE,
        );
    }

    let vert_mat = make_pbr_material("mat_ost_vertebrae", [0.85, 0.83, 0.77, 1.0], 0.50, 0.02);
    materials.push(vert_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: vert_start,
        index_count: acc.indices.len() as u32 - vert_start,
        material_id: vert_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:vertebrae"),
    });

    // 4. Ribcage & Sternum (including avian keeled carina & furcula)
    let rib_start = acc.indices.len() as u32;
    let sternum_y = chest_pos[1] - half_width * 0.75;
    let sternum_pos = [0.0, sternum_y, chest_pos[2] * 0.8];

    // 6 Rib pairs
    for r in 1..=6 {
        let t = r as f32 / 7.0;
        let spine_p = [
            pelvis_pos[0] + (chest_pos[0] - pelvis_pos[0]) * (0.3 + 0.6 * t),
            pelvis_pos[1] + (chest_pos[1] - pelvis_pos[1]) * (0.3 + 0.6 * t),
            pelvis_pos[2] + (chest_pos[2] - pelvis_pos[2]) * (0.3 + 0.6 * t),
        ];
        let sternum_p = [
            sternum_pos[0],
            sternum_pos[1],
            sternum_pos[2] - 0.04 * (3.0 - r as f32),
        ];

        let rib_w = half_width * 0.65;
        let rib_mid_l = [-rib_w, (spine_p[1] + sternum_p[1]) * 0.5, spine_p[2]];
        let rib_mid_r = [rib_w, (spine_p[1] + sternum_p[1]) * 0.5, spine_p[2]];

        acc.push_cylinder(spine_p, rib_mid_l, 0.007, 0.006, 6, 2, [0.84, 0.82, 0.76, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_CHEST, JOINT_SPINE, 0.7);
        acc.push_cylinder(rib_mid_l, sternum_p, 0.006, 0.005, 6, 2, [0.82, 0.80, 0.74, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_CHEST, JOINT_SPINE, 0.8);

        acc.push_cylinder(spine_p, rib_mid_r, 0.007, 0.006, 6, 2, [0.84, 0.82, 0.76, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_CHEST, JOINT_SPINE, 0.7);
        acc.push_cylinder(rib_mid_r, sternum_p, 0.006, 0.005, 6, 2, [0.82, 0.80, 0.74, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_CHEST, JOINT_SPINE, 0.8);
    }

    // Sternum / Keeled Carina (deep blade in birds, sternebrae in mammals)
    let carina_depth = if is_avian { half_width * 0.55 } else { half_width * 0.15 };
    acc.push_box(
        [sternum_pos[0] - 0.008, sternum_pos[1] - carina_depth, sternum_pos[2] - spine_len * 0.22],
        [sternum_pos[0] + 0.008, sternum_pos[1] + 0.01, sternum_pos[2] + spine_len * 0.22],
        [0.86, 0.84, 0.78, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_CHEST,
    );

    let rib_mat = make_pbr_material("mat_ost_ribcage", [0.83, 0.81, 0.75, 1.0], 0.42, 0.02);
    materials.push(rib_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: rib_start,
        index_count: acc.indices.len() as u32 - rib_start,
        material_id: rib_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:ribcage_sternum"),
    });

    // 5. Pectoral Girdle & Forelimb
    let pect_start = acc.indices.len() as u32;
    let shoulder_l = [-half_width * 0.70, chest_pos[1] - 0.02, chest_pos[2] * 0.9];
    let shoulder_r = [half_width * 0.70, chest_pos[1] - 0.02, chest_pos[2] * 0.9];
    let elbow_l = [-half_width * 0.80, chest_pos[1] - femur_len * 0.55, shoulder_l[2] - 0.04];
    let elbow_r = [half_width * 0.80, chest_pos[1] - femur_len * 0.55, shoulder_r[2] - 0.04];
    let wrist_l = [-half_width * 0.75, chest_pos[1] - femur_len * 0.95, elbow_l[2] + 0.08];
    let wrist_r = [half_width * 0.75, chest_pos[1] - femur_len * 0.95, elbow_r[2] + 0.08];

    // Scapulae (L+R)
    acc.push_box(
        [shoulder_l[0] - 0.015, shoulder_l[1], shoulder_l[2] - spine_len * 0.18],
        [shoulder_l[0] + 0.015, shoulder_l[1] + 0.06, shoulder_l[2] + 0.02],
        [0.86, 0.84, 0.78, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_LEFT_SHOULDER,
    );
    acc.push_box(
        [shoulder_r[0] - 0.015, shoulder_r[1], shoulder_r[2] - spine_len * 0.18],
        [shoulder_r[0] + 0.015, shoulder_r[1] + 0.06, shoulder_r[2] + 0.02],
        [0.86, 0.84, 0.78, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_LEFT_SHOULDER,
    );

    // Humerus, Radius, Ulna
    acc.push_cylinder(shoulder_l, elbow_l, half_width * 0.16, half_width * 0.14, 8, 3, [0.86, 0.84, 0.78, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_LEFT_SHOULDER, JOINT_LEFT_ELBOW, 0.6);
    acc.push_cylinder(shoulder_r, elbow_r, half_width * 0.16, half_width * 0.14, 8, 3, [0.86, 0.84, 0.78, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_LEFT_SHOULDER, JOINT_LEFT_ELBOW, 0.6);
    acc.push_cylinder(elbow_l, wrist_l, half_width * 0.12, half_width * 0.10, 8, 3, [0.86, 0.84, 0.78, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_LEFT_ELBOW, JOINT_LEFT_WRIST_WING, 0.6);
    acc.push_cylinder(elbow_r, wrist_r, half_width * 0.12, half_width * 0.10, 8, 3, [0.86, 0.84, 0.78, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_LEFT_ELBOW, JOINT_LEFT_WRIST_WING, 0.6);

    let pect_mat = make_pbr_material("mat_ost_pectoral", [0.86, 0.84, 0.78, 1.0], 0.40, 0.02);
    materials.push(pect_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: pect_start,
        index_count: acc.indices.len() as u32 - pect_start,
        material_id: pect_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:pectoral_forelimb"),
    });

    // 6. Pelvic Girdle & EPIPUBIC BONES (characteristic marsupial / monotreme trait)
    let pelvis_start = acc.indices.len() as u32;
    let ilium_l = [-half_width * 0.62, pelvis_pos[1] + 0.04, pelvis_pos[2] + 0.06];
    let ilium_r = [half_width * 0.62, pelvis_pos[1] + 0.04, pelvis_pos[2] + 0.06];
    let pubis_pos = [0.0, pelvis_pos[1] - half_width * 0.45, pelvis_pos[2] + 0.04];

    // Pelvic ring
    acc.push_cylinder(pelvis_pos, ilium_l, half_width * 0.20, half_width * 0.16, 8, 2, [0.87, 0.85, 0.79, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_PELVIS_ROOT, JOINT_LEFT_HIP, 0.7);
    acc.push_cylinder(pelvis_pos, ilium_r, half_width * 0.20, half_width * 0.16, 8, 2, [0.87, 0.85, 0.79, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_PELVIS_ROOT, JOINT_RIGHT_HIP, 0.7);
    acc.push_cylinder(pubis_pos, ilium_l, half_width * 0.14, half_width * 0.12, 6, 2, [0.87, 0.85, 0.79, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_PELVIS_ROOT, JOINT_LEFT_HIP, 0.8);
    acc.push_cylinder(pubis_pos, ilium_r, half_width * 0.14, half_width * 0.12, 6, 2, [0.87, 0.85, 0.79, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_PELVIS_ROOT, JOINT_RIGHT_HIP, 0.8);

    // ── EPIPUBIC BONES (Os Epipubicum) ──
    // Paired forward-pointing pelvic bones present in Marsupialia and Monotremata.
    let epipubic_len = half_width * 0.85;
    let epipubic_tip_l = [-half_width * 0.24, pubis_pos[1] + epipubic_len * 0.40, pubis_pos[2] + epipubic_len * 0.90];
    let epipubic_tip_r = [half_width * 0.24, pubis_pos[1] + epipubic_len * 0.40, pubis_pos[2] + epipubic_len * 0.90];

    acc.push_cylinder(
        [-0.02, pubis_pos[1], pubis_pos[2]],
        epipubic_tip_l,
        0.008,
        0.004,
        8,
        4,
        [0.89, 0.87, 0.81, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_PELVIS_ROOT,
        JOINT_SPINE,
        0.90,
    );
    acc.push_cylinder(
        [0.02, pubis_pos[1], pubis_pos[2]],
        epipubic_tip_r,
        0.008,
        0.004,
        8,
        4,
        [0.89, 0.87, 0.81, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_PELVIS_ROOT,
        JOINT_SPINE,
        0.90,
    );

    let pelvis_mat = make_pbr_material("mat_ost_pelvis_epipubic", [0.87, 0.85, 0.79, 1.0], 0.44, 0.02);
    materials.push(pelvis_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: pelvis_start,
        index_count: acc.indices.len() as u32 - pelvis_start,
        material_id: pelvis_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:pelvic_epipubic_bones"),
    });

    // 7. Hindlimb & Syndactyl Pes
    let hind_start = acc.indices.len() as u32;
    let hip_l = [-half_width * 0.65, pelvis_pos[1] - 0.02, pelvis_pos[2]];
    let hip_r = [half_width * 0.65, pelvis_pos[1] - 0.02, pelvis_pos[2]];
    let knee_l = [-half_width * 0.72, pelvis_pos[1] - femur_len, pelvis_pos[2] + femur_len * 0.45];
    let knee_r = [half_width * 0.72, pelvis_pos[1] - femur_len, pelvis_pos[2] + femur_len * 0.45];
    let ankle_l = [-half_width * 0.58, knee_l[1] - tibia_len, pelvis_pos[2] - 0.04];
    let ankle_r = [half_width * 0.58, knee_r[1] - tibia_len, pelvis_pos[2] - 0.04];
    let pes_tip_l = [ankle_l[0], ankle_l[1], ankle_l[2] + half_width * 1.4];
    let pes_tip_r = [ankle_r[0], ankle_r[1], ankle_r[2] + half_width * 1.4];

    // Femurs (L+R)
    acc.push_cylinder(hip_l, knee_l, half_width * 0.22, half_width * 0.18, 8, 3, [0.86, 0.84, 0.78, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_LEFT_HIP, JOINT_LEFT_KNEE, 0.6);
    acc.push_cylinder(hip_r, knee_r, half_width * 0.22, half_width * 0.18, 8, 3, [0.86, 0.84, 0.78, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_RIGHT_HIP, JOINT_RIGHT_KNEE, 0.6);

    // Tibia & Fibula (L+R)
    acc.push_cylinder(knee_l, ankle_l, half_width * 0.18, half_width * 0.14, 8, 4, [0.86, 0.84, 0.78, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_LEFT_KNEE, JOINT_LEFT_ANKLE, 0.6);
    acc.push_cylinder(knee_r, ankle_r, half_width * 0.18, half_width * 0.14, 8, 4, [0.86, 0.84, 0.78, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_RIGHT_KNEE, JOINT_RIGHT_ANKLE, 0.6);

    // Calcaneum / Heel bone (L+R)
    acc.push_box(
        [ankle_l[0] - 0.015, ankle_l[1], ankle_l[2] - half_width * 0.35],
        [ankle_l[0] + 0.015, ankle_l[1] + 0.03, ankle_l[2]],
        [0.85, 0.83, 0.77, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_LEFT_ANKLE,
    );
    acc.push_box(
        [ankle_r[0] - 0.015, ankle_r[1], ankle_r[2] - half_width * 0.35],
        [ankle_r[0] + 0.015, ankle_r[1] + 0.03, ankle_r[2]],
        [0.85, 0.83, 0.77, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_RIGHT_ANKLE,
    );

    // Main 4th Metatarsal & Hopping Digit IV
    acc.push_cylinder(ankle_l, pes_tip_l, half_width * 0.14, half_width * 0.09, 8, 3, [0.86, 0.84, 0.78, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_LEFT_ANKLE, JOINT_LEFT_KNEE, 0.9);
    acc.push_cylinder(ankle_r, pes_tip_r, half_width * 0.14, half_width * 0.09, 8, 3, [0.86, 0.84, 0.78, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_RIGHT_ANKLE, JOINT_RIGHT_KNEE, 0.9);

    // Syndactyl grooming digits II & III (paired medial digits)
    acc.push_cylinder(
        [ankle_l[0] + 0.015, ankle_l[1], ankle_l[2] + 0.02],
        [pes_tip_l[0] + 0.015, pes_tip_l[1], pes_tip_l[2] * 0.75],
        0.005,
        0.003,
        6,
        2,
        [0.88, 0.86, 0.80, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_LEFT_ANKLE,
        JOINT_LEFT_KNEE,
        0.95,
    );
    acc.push_cylinder(
        [ankle_r[0] - 0.015, ankle_r[1], ankle_r[2] + 0.02],
        [pes_tip_r[0] - 0.015, pes_tip_r[1], pes_tip_r[2] * 0.75],
        0.005,
        0.003,
        6,
        2,
        [0.88, 0.86, 0.80, 1.0],
        ZoologicalAnatomicalLayer::Osteology,
        JOINT_RIGHT_ANKLE,
        JOINT_RIGHT_KNEE,
        0.95,
    );

    let hind_mat = make_pbr_material("mat_ost_hindlimb_pes", [0.86, 0.84, 0.78, 1.0], 0.42, 0.02);
    materials.push(hind_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: hind_start,
        index_count: acc.indices.len() as u32 - hind_start,
        material_id: hind_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:hindlimb_syndactyl_pes"),
    });

    // 8. Caudal Vertebral Chain
    let tail_start = acc.indices.len() as u32;
    let tail_base = [0.0, pelvis_pos[1] - 0.02, -spine_len * 0.1];
    let tail_mid = [0.0, pelvis_pos[1] - femur_len * 0.45, -tail_len * 0.55];
    let tail_tip = [0.0, ankle_l[1] + 0.02, -tail_len];

    acc.push_cylinder(tail_base, tail_mid, half_width * 0.28, half_width * 0.18, 8, 4, [0.84, 0.82, 0.76, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_TAIL_BASE, JOINT_TAIL_MID, 0.6);
    acc.push_cylinder(tail_mid, tail_tip, half_width * 0.18, half_width * 0.06, 8, 4, [0.84, 0.82, 0.76, 1.0], ZoologicalAnatomicalLayer::Osteology, JOINT_TAIL_MID, JOINT_TAIL_TIP, 0.6);

    // Chevron bones (haemal arches on ventral tail vertebrae)
    for c in 1..=4 {
        let t = c as f32 / 5.0;
        let pos = [
            tail_base[0] + (tail_mid[0] - tail_base[0]) * t,
            tail_base[1] + (tail_mid[1] - tail_base[1]) * t - 0.015,
            tail_base[2] + (tail_mid[2] - tail_base[2]) * t,
        ];
        acc.push_box(
            [pos[0] - 0.005, pos[1] - 0.02, pos[2] - 0.01],
            [pos[0] + 0.005, pos[1], pos[2] + 0.01],
            [0.83, 0.81, 0.75, 1.0],
            ZoologicalAnatomicalLayer::Osteology,
            JOINT_TAIL_BASE,
        );
    }

    let tail_mat = make_pbr_material("mat_ost_caudal", [0.84, 0.82, 0.76, 1.0], 0.48, 0.02);
    materials.push(tail_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: tail_start,
        index_count: acc.indices.len() as u32 - tail_start,
        material_id: tail_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:caudal_vertebrae"),
    });

    let ost_end_v = acc.positions.len();

    // ── LAYER 1: MYOLOGY & TENDONS (8 Groups) ───────────────────────────────
    let myo_start_v = acc.positions.len();

    // 9. Gastrocnemius & Plantaris (power calf musculature)
    let gast_start = acc.indices.len() as u32;
    let gastro_center_l = [knee_l[0] * 1.02, (knee_l[1] + ankle_l[1]) * 0.58, knee_l[2] - half_width * 0.15];
    let gastro_center_r = [knee_r[1] * 0.0 + knee_r[0] * 1.02, (knee_r[1] + ankle_r[1]) * 0.58, knee_r[2] - half_width * 0.15];
    let gastro_radii = [half_width * 0.24, tibia_len * 0.38, half_width * 0.20];

    acc.push_ellipsoid(gastro_center_l, gastro_radii, 10, 6, [0.55, 0.18, 0.18, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_LEFT_KNEE, JOINT_LEFT_ANKLE, 0.7);
    acc.push_ellipsoid(gastro_center_r, gastro_radii, 10, 6, [0.55, 0.18, 0.18, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_RIGHT_KNEE, JOINT_RIGHT_ANKLE, 0.7);

    let gast_mat = make_pbr_material("mat_myo_gastrocnemius", [0.55, 0.18, 0.18, 1.0], 0.65, 0.0);
    materials.push(gast_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: gast_start,
        index_count: acc.indices.len() as u32 - gast_start,
        material_id: gast_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:gastrocnemius_plantaris"),
    });

    // 10. ELASTIC ACHILLES TENDON SPRING (stores and returns up to 65% metabolic work)
    let ach_start = acc.indices.len() as u32;
    let achilles_start_l = [gastro_center_l[0], gastro_center_l[1] - tibia_len * 0.28, gastro_center_l[2]];
    let achilles_start_r = [gastro_center_r[0], gastro_center_r[1] - tibia_len * 0.28, gastro_center_r[2]];
    let calcaneus_l = [ankle_l[0], ankle_l[1] + 0.01, ankle_l[2] - half_width * 0.28];
    let calcaneus_r = [ankle_r[0], ankle_r[1] + 0.01, ankle_r[2] - half_width * 0.28];
    let plantar_l = [ankle_l[0], ankle_l[1] - 0.01, ankle_l[2] + half_width * 0.8];
    let plantar_r = [ankle_r[0], ankle_r[1] - 0.01, ankle_r[2] + half_width * 0.8];

    // Tendon cylinder spanning calf -> tuber calcanei -> plantar fascia
    acc.push_cylinder(achilles_start_l, calcaneus_l, half_width * 0.06, half_width * 0.05, 8, 3, [0.92, 0.90, 0.85, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_LEFT_ANKLE, JOINT_LEFT_KNEE, 0.85);
    acc.push_cylinder(calcaneus_l, plantar_l, half_width * 0.05, half_width * 0.04, 8, 3, [0.92, 0.90, 0.85, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_LEFT_ANKLE, JOINT_LEFT_KNEE, 0.95);

    acc.push_cylinder(achilles_start_r, calcaneus_r, half_width * 0.06, half_width * 0.05, 8, 3, [0.92, 0.90, 0.85, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_RIGHT_ANKLE, JOINT_RIGHT_KNEE, 0.85);
    acc.push_cylinder(calcaneus_r, plantar_r, half_width * 0.05, half_width * 0.04, 8, 3, [0.92, 0.90, 0.85, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_RIGHT_ANKLE, JOINT_RIGHT_KNEE, 0.95);

    let ach_mat = make_pbr_material("mat_myo_achilles_tendon", [0.92, 0.90, 0.85, 1.0], 0.22, 0.08); // Collagenous sheen
    materials.push(ach_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: ach_start,
        index_count: acc.indices.len() as u32 - ach_start,
        material_id: ach_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:achilles_tendon_spring"),
    });

    // 11. Gluteus & Biceps Femoris (massive thigh extensor)
    let glut_start = acc.indices.len() as u32;
    let thigh_center_l = [(hip_l[0] + knee_l[0]) * 0.5 - 0.02, (hip_l[1] + knee_l[1]) * 0.5, (hip_l[2] + knee_l[2]) * 0.5];
    let thigh_center_r = [(hip_r[0] + knee_r[0]) * 0.5 + 0.02, (hip_r[1] + knee_r[1]) * 0.5, (hip_r[2] + knee_r[2]) * 0.5];
    let thigh_radii = [half_width * 0.32, femur_len * 0.42, half_width * 0.30];

    acc.push_ellipsoid(thigh_center_l, thigh_radii, 10, 6, [0.52, 0.16, 0.16, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_LEFT_HIP, JOINT_LEFT_KNEE, 0.7);
    acc.push_ellipsoid(thigh_center_r, thigh_radii, 10, 6, [0.52, 0.16, 0.16, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_RIGHT_HIP, JOINT_RIGHT_KNEE, 0.7);

    let glut_mat = make_pbr_material("mat_myo_gluteus", [0.52, 0.16, 0.16, 1.0], 0.62, 0.0);
    materials.push(glut_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: glut_start,
        index_count: acc.indices.len() as u32 - glut_start,
        material_id: glut_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:gluteus_biceps_femoris"),
    });

    // 12. Iliopsoas & Caudofemoralis (deep flexor & retractor)
    let ilio_start = acc.indices.len() as u32;
    let caudofem_l = [(hip_l[0] + tail_base[0]) * 0.5, hip_l[1] - 0.03, (hip_l[2] + tail_base[2]) * 0.5];
    let caudofem_r = [(hip_r[0] + tail_base[0]) * 0.5, hip_r[1] - 0.03, (hip_r[2] + tail_base[2]) * 0.5];
    let caudofem_radii = [half_width * 0.22, half_width * 0.18, spine_len * 0.24];

    acc.push_ellipsoid(caudofem_l, caudofem_radii, 8, 6, [0.50, 0.15, 0.15, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_PELVIS_ROOT, JOINT_TAIL_BASE, 0.7);
    acc.push_ellipsoid(caudofem_r, caudofem_radii, 8, 6, [0.50, 0.15, 0.15, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_PELVIS_ROOT, JOINT_TAIL_BASE, 0.7);

    let ilio_mat = make_pbr_material("mat_myo_iliopsoas", [0.50, 0.15, 0.15, 1.0], 0.60, 0.0);
    materials.push(ilio_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: ilio_start,
        index_count: acc.indices.len() as u32 - ilio_start,
        material_id: ilio_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:iliopsoas_caudofemoralis"),
    });

    // 13. Pectoralis Major & Supracoracoideus (massive chest flight/forelimb muscles)
    let pect_myo_start = acc.indices.len() as u32;
    let pect_center_l = [-half_width * 0.35, sternum_pos[1] + 0.03, chest_pos[2] * 0.85];
    let pect_center_r = [half_width * 0.35, sternum_pos[1] + 0.03, chest_pos[2] * 0.85];
    let pect_radii = [half_width * 0.38, half_width * 0.35, spine_len * 0.28];

    acc.push_ellipsoid(pect_center_l, pect_radii, 10, 6, [0.58, 0.20, 0.20, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_CHEST, JOINT_LEFT_SHOULDER, 0.7);
    acc.push_ellipsoid(pect_center_r, pect_radii, 10, 6, [0.58, 0.20, 0.20, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_CHEST, JOINT_RIGHT_HIP, 0.7);

    let pect_myo_mat = make_pbr_material("mat_myo_pectoralis", [0.58, 0.20, 0.20, 1.0], 0.64, 0.0);
    materials.push(pect_myo_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: pect_myo_start,
        index_count: acc.indices.len() as u32 - pect_myo_start,
        material_id: pect_myo_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:pectoralis_supracoracoideus"),
    });

    // 14. Deltoid & Triceps Fossorial (powerful digging/climbing shoulder abductors)
    let delt_start = acc.indices.len() as u32;
    let delt_center_l = [(shoulder_l[0] + elbow_l[0]) * 0.5, (shoulder_l[1] + elbow_l[1]) * 0.5, (shoulder_l[2] + elbow_l[2]) * 0.5];
    let delt_center_r = [(shoulder_r[0] + elbow_r[0]) * 0.5, (shoulder_r[1] + elbow_r[1]) * 0.5, (shoulder_r[2] + elbow_r[2]) * 0.5];
    let delt_radii = [half_width * 0.20, femur_len * 0.30, half_width * 0.20];

    acc.push_ellipsoid(delt_center_l, delt_radii, 8, 6, [0.54, 0.17, 0.17, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_LEFT_SHOULDER, JOINT_LEFT_ELBOW, 0.7);
    acc.push_ellipsoid(delt_center_r, delt_radii, 8, 6, [0.54, 0.17, 0.17, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_LEFT_SHOULDER, JOINT_LEFT_ELBOW, 0.7);

    let delt_mat = make_pbr_material("mat_myo_deltoid", [0.54, 0.17, 0.17, 1.0], 0.63, 0.0);
    materials.push(delt_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: delt_start,
        index_count: acc.indices.len() as u32 - delt_start,
        material_id: delt_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:deltoid_triceps"),
    });

    // 15. Masseter & Temporalis (jaw mastication muscles)
    let mass_start = acc.indices.len() as u32;
    let masseter_l = [-half_width * 0.44, cranium_head_pos[1] - 0.02, cranium_head_pos[2] + head_len * 0.2];
    let masseter_r = [half_width * 0.44, cranium_head_pos[1] - 0.02, cranium_head_pos[2] + head_len * 0.2];
    let masseter_radii = [half_width * 0.15, half_width * 0.24, head_len * 0.22];

    acc.push_ellipsoid(masseter_l, masseter_radii, 8, 5, [0.48, 0.16, 0.16, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_HEAD, JOINT_JAW, 0.6);
    acc.push_ellipsoid(masseter_r, masseter_radii, 8, 5, [0.48, 0.16, 0.16, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_HEAD, JOINT_JAW, 0.6);

    let mass_mat = make_pbr_material("mat_myo_masseter", [0.48, 0.16, 0.16, 1.0], 0.62, 0.0);
    materials.push(mass_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: mass_start,
        index_count: acc.indices.len() as u32 - mass_start,
        material_id: mass_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:masseter_temporalis"),
    });

    // 16. Intercostals & Diaphragm (thoracic respiratory musculature)
    let inter_start = acc.indices.len() as u32;
    let chest_center = [0.0, (chest_pos[1] + sternum_pos[1]) * 0.5, (chest_pos[2] + pelvis_pos[2]) * 0.5];
    let chest_radii = [half_width * 0.72, (chest_pos[1] - sternum_pos[1]) * 0.55, spine_len * 0.36];

    acc.push_ellipsoid(chest_center, chest_radii, 10, 6, [0.46, 0.15, 0.15, 1.0], ZoologicalAnatomicalLayer::MyologyAndTendons, JOINT_CHEST, JOINT_SPINE, 0.7);

    let inter_mat = make_pbr_material("mat_myo_intercostals", [0.46, 0.15, 0.15, 1.0], 0.68, 0.0);
    materials.push(inter_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: inter_start,
        index_count: acc.indices.len() as u32 - inter_start,
        material_id: inter_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:intercostals_diaphragm"),
    });

    let myo_end_v = acc.positions.len();

    // ── LAYER 2: EXTERNAL INTEGUMENT (Pelage / Plumage / Squamation) ────────
    let int_start_v = acc.positions.len();
    let int_start_idx = acc.indices.len() as u32;

    // Archetype pelage / plumage / scale tint
    let integument_color = match archetype {
        FaunaBiomechanicalArchetype::MacropodSaltatorial => [0.48, 0.40, 0.32, 1.0], // Tawny kangaroo fur
        FaunaBiomechanicalArchetype::VombatidFossorial => [0.34, 0.28, 0.22, 1.0],   // Grizzly wombat fur
        FaunaBiomechanicalArchetype::PhascolarctidArboreal => [0.55, 0.52, 0.48, 1.0], // Koala silver wool
        FaunaBiomechanicalArchetype::PetauridGlider => [0.60, 0.58, 0.56, 1.0],     // Glider soft silver pelage
        FaunaBiomechanicalArchetype::DasyuridPredator => [0.42, 0.30, 0.22, 1.0],   // Quoll rufous fur
        FaunaBiomechanicalArchetype::PteropodidChiroptera => [0.22, 0.18, 0.16, 1.0], // Flying-fox black/russet
        FaunaBiomechanicalArchetype::MarinePinnipedCetacean => [0.32, 0.36, 0.42, 1.0], // Seal/dolphin slate skin
        FaunaBiomechanicalArchetype::RatiteBipedal => [0.18, 0.17, 0.18, 1.0],      // Cassowary shaggy plumage
        FaunaBiomechanicalArchetype::RaptorSoaring => [0.28, 0.22, 0.16, 1.0],      // Wedge-tailed eagle umber
        FaunaBiomechanicalArchetype::PasserinePsittacine => [0.38, 0.52, 0.28, 1.0], // Parrot olive plumage
        FaunaBiomechanicalArchetype::SquamateSprawling => [0.44, 0.42, 0.34, 1.0],  // Monitor mottled scales
        FaunaBiomechanicalArchetype::SerpentineUndulatory => [0.20, 0.18, 0.20, 1.0], // Python glossy scales
        FaunaBiomechanicalArchetype::MonotremeTeleostAmphibian => [0.36, 0.26, 0.20, 1.0], // Platypus waterproof pelage
    };

    // Main Torso Envelope
    acc.push_ellipsoid(
        [0.0, hip_height + 0.02, spine_len * 0.45],
        [half_width * 0.95, half_width * 0.90, spine_len * 0.65],
        16,
        10,
        integument_color,
        ZoologicalAnatomicalLayer::ExternalIntegument,
        JOINT_CHEST,
        JOINT_PELVIS_ROOT,
        0.55,
    );

    // Head & Snout Envelope
    acc.push_ellipsoid(
        [0.0, cranium_head_pos[1] - 0.02, cranium_head_pos[2] + head_len * 0.25],
        [half_width * 0.65, half_width * 0.60, head_len * 0.62],
        12,
        8,
        integument_color,
        ZoologicalAnatomicalLayer::ExternalIntegument,
        JOINT_HEAD,
        JOINT_NECK,
        0.80,
    );

    // Limbs outer envelope (Forelimbs L+R)
    acc.push_cylinder(shoulder_l, wrist_l, half_width * 0.25, half_width * 0.16, 8, 3, integument_color, ZoologicalAnatomicalLayer::ExternalIntegument, JOINT_LEFT_SHOULDER, JOINT_LEFT_WRIST_WING, 0.5);
    acc.push_cylinder(shoulder_r, wrist_r, half_width * 0.25, half_width * 0.16, 8, 3, integument_color, ZoologicalAnatomicalLayer::ExternalIntegument, JOINT_LEFT_SHOULDER, JOINT_LEFT_WRIST_WING, 0.5);

    // Hindlimbs outer envelope (Thigh + Leg L+R)
    acc.push_cylinder(hip_l, ankle_l, half_width * 0.38, half_width * 0.20, 10, 4, integument_color, ZoologicalAnatomicalLayer::ExternalIntegument, JOINT_LEFT_HIP, JOINT_LEFT_ANKLE, 0.5);
    acc.push_cylinder(hip_r, ankle_r, half_width * 0.38, half_width * 0.20, 10, 4, integument_color, ZoologicalAnatomicalLayer::ExternalIntegument, JOINT_RIGHT_HIP, JOINT_RIGHT_ANKLE, 0.5);

    // Tail outer envelope
    acc.push_cylinder(tail_base, tail_tip, half_width * 0.34, half_width * 0.10, 8, 4, integument_color, ZoologicalAnatomicalLayer::ExternalIntegument, JOINT_TAIL_BASE, JOINT_TAIL_TIP, 0.5);

    let int_mat = make_pbr_material("mat_integument", integument_color, 0.82, 0.0);
    materials.push(int_mat);
    submesh_ranges.push(SubmeshRange {
        first_index: int_start_idx,
        index_count: acc.indices.len() as u32 - int_start_idx,
        material_id: int_mat.id,
        semantic_id: q_hash("urn:qualia:bio:submesh:external_integument"),
    });

    let int_end_v = acc.positions.len();

    // ── Apply Layer Visibility & Exploded View Displacements ─────────────────
    let total_v = acc.positions.len();
    let mut eval_positions = acc.positions.clone();
    let mut eval_alphas = vec![1.0f32; total_v];

    evaluate_anatomical_layer_peel_zero_heap(
        &acc.positions,
        &acc.normals,
        &acc.layer_tags,
        osteology_vis_01,
        myology_vis_01,
        integument_vis_01,
        explode_layer_01,
        &mut eval_positions,
        &mut eval_alphas,
    );

    // Merge computed alpha into RGBA colors
    for i in 0..total_v {
        acc.colors[i][3] = eval_alphas[i];
    }

    // Compute Mesh AABB
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in &eval_positions {
        for k in 0..3 {
            min[k] = min[k].min(p[k]);
            max[k] = max[k].max(p[k]);
        }
    }

    let mut triangles = Vec::with_capacity(acc.indices.len() / 3);
    for tri in acc.indices.chunks_exact(3) {
        triangles.push([tri[0], tri[1], tri[2]]);
    }

    let mesh = Mesh {
        positions: eval_positions.clone(),
        triangles,
        min,
        max,
    };

    // ── Assemble & Seal .10d Container ───────────────────────────────────────
    let provenance = ProvenanceSidecar::new(
        format!("urn:qualia:fauna:scale5:{}", archetype.as_str()).into_bytes(),
        "application/vnd.ecopunk.fauna-anatomy;version=1",
        "Apache-2.0",
    );

    let mut bytes = compile_mesh_to_10d_with_extras_and_frames(
        &mesh,
        &[],
        Some(&provenance),
        Compile10dExtras::default(),
        Some(&acc.colors),
        Some(&acc.normals),
        None,
    )
    .expect("compile_mesh_to_10d_with_extras_and_frames failed for vertebrate anatomy");

    // MAT4 Material Section (materials must be sorted by id in ascending order)
    let mut sorted_materials = materials.clone();
    sorted_materials.sort_by_key(|m| m.id);
    sorted_materials.dedup_by_key(|m| m.id);
    bytes = attach_material_section(&bytes, &sorted_materials, &submesh_ranges, acc.indices.len() as u32)
        .expect("attach_material_section failed");

    // UV01 Texture Coordinate Section
    bytes = attach_texture_coordinate_section(&bytes, &acc.uvs)
        .expect("attach_texture_coordinate_section failed");

    // PROC1 Procedural Synthesis Section
    let proc_header = ProceduralSynthesisHeader {
        magic: MAGIC_PROC,
        version: PROCEDURAL_SYNTHESIS_SECTION_VERSION,
        flags: (FLAG_CELLULAR_VORONOI
            | FLAG_IMPLICIT_FILLET
            | FLAG_ALLOMETRIC_GROWTH
            | FLAG_PGA_MOTOR_DYNAMICS) as u16,
        seed: q_hash(archetype.as_str()),
        voronoi_frequency: 14.5,
        voronoi_jitter: 0.80,
        fillet_blend_radius: 0.10,
        fillet_threshold: 0.04,
        allometry_exponent_k: 0.67,
        allometry_base_b: 1.12,
        spiral_growth_rate_k: 0.20,
        spiral_initial_radius_a: 0.05,
        motor_frequency_hz: 2.10,
        motor_damping_ratio: 0.06,
        critical_stress_threshold: 32.0,
        reserved_f32: 0.0,
    };
    bytes = attach_procedural_synthesis_section(&bytes, &proc_header)
        .expect("attach_procedural_synthesis_section failed");

    // EONT1 Entity Ontology Section
    let onto_header = EntityOntologyMiniHeader {
        magic: MAGIC_EONT,
        version: ENTITY_ONTOLOGY_SECTION_VERSION,
        stage: 2, // Adult
        reserved_u8: 0,
        entity_did_hash: q_hash(&format!("did:qualia:fauna:{}", archetype.as_str())),
        taxon_iri_hash: q_hash(&format!("urn:qualia:taxon:{}", archetype.as_str())),
        energy_permill: 960,
        damage_permill: 20,
        hydration_permill: 910,
        wear_permill: 40,
        affordance_count: 3,
        reserved_u32: 4,
    };

    let affordances = [
        AffordanceBinaryRecord {
            action_id: 0x0011,
            required_capability_hash: 0xCA00_1011,
            emitted_topic_hash: 0x7000_1011,
            category: 1, // Locomotion
            min_observer_status: 0,
            min_energy_permill: 100,
            max_damage_permill: 900,
            min_hydration_permill: 100,
            max_wear_permill: 950,
            energy_delta: -5,
            damage_delta: 0,
            hydration_delta: -2,
            wear_delta: 1,
            reserved_u16: 0,
            animation_clip_id: 1,
        },
        AffordanceBinaryRecord {
            action_id: 0x0012,
            required_capability_hash: 0xCA00_1012,
            emitted_topic_hash: 0x7000_1012,
            category: 2, // Foraging
            min_observer_status: 0,
            min_energy_permill: 50,
            max_damage_permill: 950,
            min_hydration_permill: 50,
            max_wear_permill: 980,
            energy_delta: 25,
            damage_delta: 0,
            hydration_delta: 10,
            wear_delta: 2,
            reserved_u16: 0,
            animation_clip_id: 2,
        },
        AffordanceBinaryRecord {
            action_id: 0x0013,
            required_capability_hash: 0xCA00_1013,
            emitted_topic_hash: 0x7000_1013,
            category: 3, // Vocalize / Social Display
            min_observer_status: 0,
            min_energy_permill: 80,
            max_damage_permill: 900,
            min_hydration_permill: 80,
            max_wear_permill: 900,
            energy_delta: -8,
            damage_delta: 0,
            hydration_delta: -4,
            wear_delta: 1,
            reserved_u16: 0,
            animation_clip_id: 3,
        },
    ];

    let keyframes = [
        DevelopmentalKeyframeRecord {
            age_days: 60.0,
            stage: 0, // Neonate Pouch Young
            reserved_u8: 0,
            reserved_u16: 0,
            allometric_crown_scale: 0.20,
            allometric_trunk_scale: 0.15,
            lai_multiplier: 0.10,
            reserved_u32: 0,
            mesh_asset_hash: 0,
        },
        DevelopmentalKeyframeRecord {
            age_days: 240.0,
            stage: 1, // Sub-adult Joey
            reserved_u8: 0,
            reserved_u16: 0,
            allometric_crown_scale: 0.65,
            allometric_trunk_scale: 0.58,
            lai_multiplier: 0.50,
            reserved_u32: 0,
            mesh_asset_hash: 0,
        },
        DevelopmentalKeyframeRecord {
            age_days: 1200.0,
            stage: 2, // Mature Adult
            reserved_u8: 0,
            reserved_u16: 0,
            allometric_crown_scale: 1.0,
            allometric_trunk_scale: 1.0,
            lai_multiplier: 1.0,
            reserved_u32: 0,
            mesh_asset_hash: 0,
        },
        DevelopmentalKeyframeRecord {
            age_days: 4500.0,
            stage: 3, // Dominant Elder
            reserved_u8: 0,
            reserved_u16: 0,
            allometric_crown_scale: 1.15,
            allometric_trunk_scale: 1.25,
            lai_multiplier: 0.95,
            reserved_u32: 0,
            mesh_asset_hash: 0,
        },
    ];

    bytes = attach_entity_ontology_section(&bytes, &onto_header, &affordances, &keyframes)
        .expect("attach_entity_ontology_section failed");

    // MRP1 Morph Target Section (Costal Respiration, Gular Vocalization, Achilles Tendon Strain)
    let mut morph_deltas = Vec::with_capacity(total_v * 3 / 8);
    let step = (total_v / 64).max(1);

    for vi in (0..total_v).step_by(step) {
        let p = eval_positions[vi];
        let n = acc.normals[vi];
        let layer = acc.layer_tags[vi];

        // Target 0: Costal respiration (intercostal and ribcage expansion)
        if layer == ZoologicalAnatomicalLayer::Osteology || layer == ZoologicalAnatomicalLayer::MyologyAndTendons {
            let resp_amp = if p[1] > hip_height - 0.1 && p[1] < hip_height + 0.2 { 0.035 } else { 0.008 };
            let d0 = quantize_morph_delta(
                [n[0] * resp_amp, 0.015, n[2] * resp_amp],
                [n[0] * 0.1, 0.05, n[2] * 0.1],
                [0.0; 3],
                0.25,
                0,
                vi as u32,
            );
            morph_deltas.push(d0);
        }

        // Target 1: Gular vocalization (neck / throat / jaw distension)
        if p[2] > spine_len * 0.5 {
            let gular_amp = 0.045;
            let d1 = quantize_morph_delta(
                [n[0] * gular_amp, -0.040, n[2] * gular_amp],
                [0.0, -0.1, 0.0],
                [0.0; 3],
                0.25,
                1,
                vi as u32,
            );
            morph_deltas.push(d1);
        }

        // Target 2: Achilles tendon strain (spring elongation during locomotion)
        if p[1] < hip_height - femur_len * 0.5 && p[2] < 0.05 {
            let strain_amp = 0.055;
            let d2 = quantize_morph_delta(
                [0.0, -strain_amp, 0.012],
                [0.0, -0.05, 0.02],
                [0.0; 3],
                0.25,
                2,
                vi as u32,
            );
            morph_deltas.push(d2);
        }
    }

    let morph_header = MorphSectionHeader {
        magic: MAGIC_MRP1,
        version: MORPH_SECTION_VERSION,
        target_count: 3,
        vertex_count: total_v as u32,
        flags: FLAG_MORPH_SPARSE | FLAG_MORPH_HAS_NORMALS | FLAG_MORPH_ONTOGENETIC_BIND,
        max_displacement: 0.25,
        ontogenetic_bind_mask: 0b0111,
        reserved: [0u8; 8],
    };

    bytes = attach_morph_section(&bytes, &morph_header, &morph_deltas)
        .expect("attach_morph_section failed");

    CompiledVertebrateAnatomy10d {
        archetype,
        rig,
        container_bytes: bytes,
        vertex_count: total_v,
        triangle_count: acc.indices.len() / 3,
        osteology_vertex_range: ost_start_v..ost_end_v,
        myology_vertex_range: myo_start_v..myo_end_v,
        integument_vertex_range: int_start_v..int_end_v,
        positions: eval_positions,
        normals: acc.normals,
        colors: acc.colors,
        uvs: acc.uvs,
        indices: acc.indices,
        materials,
        submesh_ranges,
        skin_joint_indices: acc.joint_indices,
        skin_weights: acc.joint_weights,
        layer_tags: acc.layer_tags,
    }
}

// ── Private Section Attachment Helpers ───────────────────────────────────────

fn attach_raw_section(
    container: &[u8],
    section_type: SectionType,
    alignment_tier: AlignmentTier,
    stride: u32,
    element_count: u32,
    payload: &[u8],
) -> Result<Vec<u8>, String> {
    let header = Container10dHeader::parse(container).map_err(|e| format!("10d header parse: {e:?}"))?;
    let descriptors = parse_section_table(container, &header).map_err(|e| format!("parse_section_table: {e:?}"))?;
    if descriptors.len() >= MAX_SECTION_COUNT as usize {
        return Err("MAX_SECTION_COUNT reached".into());
    }

    let mut inputs = Vec::with_capacity(descriptors.len() + 1);
    for descriptor in descriptors {
        let start = descriptor.byte_offset as usize;
        let end = start
            .checked_add(descriptor.byte_length as usize)
            .filter(|&end| end <= container.len())
            .ok_or_else(|| {
                format!(
                    "SectionTableError::OutOfBounds offset={} length={}",
                    descriptor.byte_offset, descriptor.byte_length
                )
            })?;
        inputs.push(SectionInput {
            section_type: descriptor
                .typ()
                .ok_or_else(|| format!("UndefinedSectionType: {}", descriptor.section_type))?,
            alignment_tier: descriptor
                .tier()
                .ok_or_else(|| format!("UndefinedAlignmentTier: {}", descriptor.alignment_tier))?,
            stride: descriptor.stride,
            element_count: descriptor.element_count,
            payload: &container[start..end],
        });
    }

    inputs.push(SectionInput {
        section_type,
        alignment_tier,
        stride,
        element_count,
        payload,
    });

    let needed = match encode_container(&header, &inputs, &mut []) {
        Err(SectionTableError::OutputBufferTooSmall { needed, .. }) => needed,
        Ok(size) => size,
        Err(error) => return Err(format!("encode_container dry-run error: {error:?}")),
    };
    let mut output = vec![0u8; needed];
    let written = encode_container(&header, &inputs, &mut output)
        .map_err(|e| format!("encode_container error: {e:?}"))?;
    output.truncate(written);
    seal_whole_file_crc32c(&mut output);
    Ok(output)
}

fn attach_procedural_synthesis_section(
    container: &[u8],
    header: &ProceduralSynthesisHeader,
) -> Result<Vec<u8>, String> {
    let mut buf = vec![0u8; PROCEDURAL_SYNTHESIS_HEADER_SIZE];
    encode_procedural_synthesis_section(header, &mut buf)
        .map_err(|e| format!("encode_procedural_synthesis_section: {e:?}"))?;
    attach_raw_section(
        container,
        SectionType::ProceduralSynthesis,
        AlignmentTier::CacheLine,
        0,
        0,
        &buf,
    )
}

fn attach_entity_ontology_section(
    container: &[u8],
    header: &EntityOntologyMiniHeader,
    affordances: &[AffordanceBinaryRecord],
    keyframes: &[DevelopmentalKeyframeRecord],
) -> Result<Vec<u8>, String> {
    let mut hdr = *header;
    hdr.affordance_count = affordances.len() as u32;
    hdr.reserved_u32 = keyframes.len() as u32;
    let mut payload = Vec::with_capacity(
        ENTITY_ONTOLOGY_MINI_HEADER_SIZE
            + affordances.len() * AFFORDANCE_RECORD_SIZE
            + keyframes.len() * DEVELOPMENTAL_KEYFRAME_RECORD_SIZE,
    );
    payload.extend_from_slice(bytemuck::bytes_of(&hdr));
    for a in affordances {
        payload.extend_from_slice(bytemuck::bytes_of(a));
    }
    for k in keyframes {
        payload.extend_from_slice(bytemuck::bytes_of(k));
    }
    attach_raw_section(
        container,
        SectionType::EntityOntology,
        AlignmentTier::CacheLine,
        0,
        0,
        &payload,
    )
}

fn attach_morph_section(
    container: &[u8],
    header: &MorphSectionHeader,
    deltas: &[QuantizedMorphDelta],
) -> Result<Vec<u8>, String> {
    let needed = morph_section_encoded_len(deltas.len())
        .ok_or_else(|| format!("morph_section_encoded_len overflow: deltas={}", deltas.len()))?;
    let mut buf = vec![0u8; needed];
    encode_morph_section(header, deltas, &mut buf)
        .map_err(|e| format!("encode_morph_section: {e:?}"))?;
    attach_raw_section(
        container,
        SectionType::Reconstruction,
        AlignmentTier::CacheLine,
        0,
        0,
        &buf,
    )
}

// ── Unit Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::container_10d::integrity::verify_whole_file_crc32c;

    const ALL_13_ARCHETYPES: [FaunaBiomechanicalArchetype; 13] = [
        FaunaBiomechanicalArchetype::MacropodSaltatorial,
        FaunaBiomechanicalArchetype::VombatidFossorial,
        FaunaBiomechanicalArchetype::PhascolarctidArboreal,
        FaunaBiomechanicalArchetype::PetauridGlider,
        FaunaBiomechanicalArchetype::DasyuridPredator,
        FaunaBiomechanicalArchetype::PteropodidChiroptera,
        FaunaBiomechanicalArchetype::MarinePinnipedCetacean,
        FaunaBiomechanicalArchetype::RatiteBipedal,
        FaunaBiomechanicalArchetype::RaptorSoaring,
        FaunaBiomechanicalArchetype::PasserinePsittacine,
        FaunaBiomechanicalArchetype::SquamateSprawling,
        FaunaBiomechanicalArchetype::SerpentineUndulatory,
        FaunaBiomechanicalArchetype::MonotremeTeleostAmphibian,
    ];

    #[test]
    fn test_all_13_archetypes_build_valid_scale5_anatomy() {
        for arch in ALL_13_ARCHETYPES {
            let compiled = build_scale5_vertebrate_anatomy_10d(arch, 1.0, 1.0, 1.0, 0.0);

            // 1. Verify high-resolution vertex count (>= 1,800)
            assert!(
                compiled.vertex_count >= 1800,
                "Archetype {:?} vertex count {} is below minimum 1,800",
                arch,
                compiled.vertex_count
            );

            // 2. Verify submesh count & material count (17 distinct anatomical parts)
            assert_eq!(
                compiled.submesh_ranges.len(),
                17,
                "Archetype {:?} must have 17 submesh ranges (8 osteo + 8 myo + 1 integument)",
                arch
            );
            assert_eq!(compiled.materials.len(), 17);

            // 3. Verify contiguous index stream without gaps
            let mut expected_first = 0u32;
            for range in &compiled.submesh_ranges {
                assert_eq!(range.first_index, expected_first);
                assert!(range.index_count > 0);
                expected_first += range.index_count;
            }
            assert_eq!(expected_first as usize, compiled.indices.len());

            // 4. Verify 3-layer vertex intervals
            assert!(!compiled.osteology_vertex_range.is_empty());
            assert!(!compiled.myology_vertex_range.is_empty());
            assert!(!compiled.integument_vertex_range.is_empty());
            assert_eq!(compiled.osteology_vertex_range.end, compiled.myology_vertex_range.start);
            assert_eq!(compiled.myology_vertex_range.end, compiled.integument_vertex_range.start);
            assert_eq!(compiled.integument_vertex_range.end, compiled.vertex_count);

            // 5. Verify non-empty .10d container bytes
            assert!(!compiled.container_bytes.is_empty());
        }
    }

    #[test]
    fn test_epipubic_bones_and_achilles_tendon_submeshes() {
        let compiled = build_scale5_vertebrate_anatomy_10d(
            FaunaBiomechanicalArchetype::MacropodSaltatorial,
            1.0,
            1.0,
            1.0,
            0.0,
        );

        // Submesh 5: PelvicGirdleAndEpipubicBones
        let pelvis_range = &compiled.submesh_ranges[5];
        assert!(pelvis_range.index_count >= 36);
        assert_eq!(pelvis_range.semantic_id, q_hash("urn:qualia:bio:submesh:pelvic_epipubic_bones"));

        // Submesh 9: ElasticAchillesTendonSpring
        let achilles_range = &compiled.submesh_ranges[9];
        assert!(achilles_range.index_count >= 36);
        assert_eq!(achilles_range.semantic_id, q_hash("urn:qualia:bio:submesh:achilles_tendon_spring"));

        // Verify tendon material has collagen PBR properties
        let tendon_mat = &compiled.materials[9];
        assert!(tendon_mat.roughness < 0.35, "Tendon should have smooth glistening collagen sheen");
        assert!(tendon_mat.base_color[0] > 0.85, "Tendon should be ivory/white collagen");
    }

    #[test]
    fn test_zero_heap_layer_peeling() {
        let compiled = build_scale5_vertebrate_anatomy_10d(
            FaunaBiomechanicalArchetype::VombatidFossorial,
            1.0,
            1.0,
            1.0,
            0.0,
        );

        let n = compiled.vertex_count;
        let mut out_positions = vec![[0.0f32; 3]; n];
        let mut out_alphas = vec![0.0f32; n];

        // 1. Fully visible without explosion
        let count = evaluate_anatomical_layer_peel_zero_heap(
            &compiled.positions,
            &compiled.normals,
            &compiled.layer_tags,
            1.0,
            1.0,
            1.0,
            0.0,
            &mut out_positions,
            &mut out_alphas,
        );
        assert_eq!(count, n);
        for a in &out_alphas {
            assert_eq!(*a, 1.0);
        }

        // 2. Hide integument (0.0), half myology (0.5), full osteology (1.0), exploded (1.0)
        evaluate_anatomical_layer_peel_zero_heap(
            &compiled.positions,
            &compiled.normals,
            &compiled.layer_tags,
            1.0,
            0.5,
            0.0,
            1.0,
            &mut out_positions,
            &mut out_alphas,
        );

        // Check osteology: alpha = 1.0, displacement = 0
        for vi in compiled.osteology_vertex_range.clone() {
            assert_eq!(out_alphas[vi], 1.0);
            assert_eq!(out_positions[vi], compiled.positions[vi]);
        }

        // Check myology: alpha = 0.5, displaced outward
        for vi in compiled.myology_vertex_range.clone() {
            assert_eq!(out_alphas[vi], 0.5);
            let disp = (
                (out_positions[vi][0] - compiled.positions[vi][0]).powi(2)
                + (out_positions[vi][1] - compiled.positions[vi][1]).powi(2)
                + (out_positions[vi][2] - compiled.positions[vi][2]).powi(2)
            ).sqrt();
            assert!(disp > 0.05, "Myology layer should be exploded outward");
        }

        // Check integument: alpha = 0.0, displaced even further outward
        for vi in compiled.integument_vertex_range.clone() {
            assert_eq!(out_alphas[vi], 0.0);
            let disp = (
                (out_positions[vi][0] - compiled.positions[vi][0]).powi(2)
                + (out_positions[vi][1] - compiled.positions[vi][1]).powi(2)
                + (out_positions[vi][2] - compiled.positions[vi][2]).powi(2)
            ).sqrt();
            assert!(disp > 0.12, "Integument layer should be exploded furthest outward");
        }
    }

    #[test]
    fn test_10d_container_verification() {
        let compiled = build_scale5_vertebrate_anatomy_10d(
            FaunaBiomechanicalArchetype::MacropodSaltatorial,
            1.0,
            1.0,
            1.0,
            0.0,
        );

        let mut bytes = compiled.container_bytes.clone();

        // Verify valid 10d header
        let header = Container10dHeader::parse(&bytes).expect("Header parse must succeed");
        assert_eq!(header.magic, qualia_core_db::container_10d::header::MAGIC_10D);

        // Verify whole-file CRC32C seal
        verify_whole_file_crc32c(&mut bytes).expect("Whole-file CRC-32C verification must pass");

        // Verify section table contents
        let descriptors = parse_section_table(&bytes, &header).expect("Section table parse must succeed");
        assert!(descriptors.len() >= 8, "Expected at least 8 sections, got {}", descriptors.len());

        let section_types: Vec<Option<SectionType>> = descriptors.iter().map(|d| d.typ()).collect();
        assert!(section_types.contains(&Some(SectionType::QuantizedMesh)));
        assert!(section_types.contains(&Some(SectionType::ProvenanceSidecar)));
        assert!(section_types.contains(&Some(SectionType::FieldSidecar)));
        assert!(section_types.contains(&Some(SectionType::SurfaceReading)));
        assert!(section_types.contains(&Some(SectionType::Materials)));
        assert!(section_types.contains(&Some(SectionType::TextureCoordinates)));
        assert!(section_types.contains(&Some(SectionType::Reconstruction))); // Morph targets
        assert!(section_types.contains(&Some(SectionType::EntityOntology)));
        assert!(section_types.contains(&Some(SectionType::ProceduralSynthesis)));
    }
}
