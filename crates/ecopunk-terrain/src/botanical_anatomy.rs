//! Botanical 4-Scale Digital Twin Anatomy, Histological Dissection & Living Container Generator.
//!
//! Implements **Epic 1** of `biological-education-assets-and-explorable-world-plan.md`
//! for *Corymbia aparrerinja* (Ghost Gum) and Australian sclerophyll flora across
//! four educational scales:
//!
//! 1. **Scale 1 — Whole-Organism Architecture & Subterranean Root System (`10 m – 40 m`):**
//!    Continuous 5-order branching (bole $\to$ scaffold limbs $\to$ secondary/tertiary branches $\to$
//!    terminal branchlets) via Runions space colonization, Leonardo da Vinci / Shinozaki Pipe Model
//!    ($\eta = 2.49$) radius tapering, root flare allometry, lignotuber with epicormic meristem buds,
//!    primary taproot, lateral buttress flares, and pendulous foliage clusters.
//!
//! 2. **Scale 2 — Branch Phyllotaxis & Stem/Bark Histology Wedge (`50 mm – 2 m`):**
//!    Golden-angle ($\phi = 137.5077^\circ$) phyllotaxis plus a 270°-wedged transverse/radial/tangential
//!    stem dissection with 6 concentric peelable tissue cylinders (`OuterRhytidome`, `PhellogenCorkCambium`,
//!    `SecondaryPhloemAndKinoDucts`, `VascularCambium`, `SapwoodSecondaryXylem`, `HeartwoodAndPith`).
//!
//! 3. **Scale 3 — Floral & Fruit Reproductive Dissection (`5 mm – 150 mm`):**
//!    Axillary 7-flowered umbellaster, fused `HypanthiumCalyxTube`, shedding `OperculumCalyptra` (driven
//!    by anthesis progress and baked into `.10d` `MRP1` morph deltas), 48-filament inflexed/unfurling
//!    `StaminalRingFilaments`, bilobed `VersatileAnthersAndPollen`, `GynoeciumStyleStigma`, 4-locule inferior
//!    ovary with axile placentation and `OvaryLoculesAndOvules`, mature urceolate `WoodyPericarpAndDehiscingValves`,
//!    and `FertileWingedSeedsAndChaff`.
//!
//! 4. **Scale 4 — Foliar Lamina Venation & Cellular Histology (`50 µm – 50 mm`):**
//!    Falcate-lanceolate blade with Murray's Law ($r_p^3 = r_1^3 + r_2^3$) midrib, diagnostic Myrtaceae
//!    intramarginal vein, pinnate secondaries, and tertiary reticulate areoles, paired with a magnified 3D
//!    histological block featuring `WaxyCuticle`, `UpperAndLowerEpidermis`, sunken `AmphistomaticStomatalGuardCells`,
//!    `IsobilateralPalisadeMesophyll`, `SpongyMesophyllAndAirSpaces`, `CollateralVascularBundleXylemPhloem`,
//!    `SclerenchymaBundleSheathGirders`, and spherical `SchizogenousEssentialOilGland`.
//!
//! Strictly zero-V8, fully sealed `.10d` living containers with `MAT4`, `UV01`, `FNRM`, `SRD1`,
//! `PROC1`, `EONT1`, `MRP1`, and `PROV` sections.

use bytemuck;
use qualia_core_db::container_10d::entity_ontology_section::{
    AffordanceBinaryRecord, DevelopmentalKeyframeRecord, EntityOntologyMiniHeader,
    AFFORDANCE_RECORD_SIZE, DEVELOPMENTAL_KEYFRAME_RECORD_SIZE,
    ENTITY_ONTOLOGY_MINI_HEADER_SIZE, ENTITY_ONTOLOGY_SECTION_VERSION, MAGIC_EONT,
};
use qualia_core_db::container_10d::header::{Container10dHeader, MAX_SECTION_COUNT};
use qualia_core_db::container_10d::integrity::{seal_whole_file_crc32c, verify_whole_file_crc32c};
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
use qualia_core_db::container_10d::{
    attach_material_section, attach_texture_coordinate_section, MaterialRecord, OpacityMode,
    ShadingModel, SubmeshRange,
};
use qualia_core_db::q_hash;
use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::compile_10d::{
    compile_mesh_to_10d_with_extras_and_frames, Compile10dExtras,
};
use qualia_core_db::specialized_libs::computational_geometry::space_colonization::{
    lateral_whorl_directions, space_colonization_3d, SpaceColonizationConfig, ROOT_PARENT,
};
use qualia_core_db::specialized_libs::computational_geometry::tree_skeleton_3d::pipe_model_radii;
use qualia_core_db::specialized_libs::computational_geometry::Point3;
use qualia_sim::leaf_biophysics::{LeafEconomicsProfile, LeafVascularSkeleton};
use qualia_sim::metamer_fspm::{Metamer, MetamerOrgan, MetamerPlantGraph, ThermalPhenology};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::f32::consts::PI;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Canonical golden ratio angle ($\phi = 137.507764^\circ$) in radians.
pub const GOLDEN_ANGLE_RAD: f32 = 2.39996323;

/// Leonardo da Vinci / Shinozaki pipe exponent for eucalyptus crown hydraulic architecture.
pub const PIPE_EXPONENT_EUCALYPTUS: f64 = 2.49;

/// Re-export upstream FSPM plant architecture type.
pub type PlantArchitecture = MetamerPlantGraph;

// ============================================================================
// 1. Core Educational Scales & Anatomical Taxonomies
// ============================================================================

/// Four progressive spatial scales of botanical digital twin education.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BotanicalEducationScale {
    /// Whole tree habit, bole, 5-order branching canopy, and subterranean root architecture.
    Scale1WholeOrganismAndRoots,
    /// Metamer internodes, golden-angle phyllotaxis, and 6-layer concentric stem histology wedge.
    Scale2BranchPhyllotaxisAndStemWedge,
    /// 7-flowered umbellaster, calyptrate flower anthesis, and woody dehiscing gumnut capsule.
    Scale3FloralAndGumnutDissection,
    /// Isobilateral sclerophyll lamina, Murray's Law venation, intramarginal vein, and cellular layers.
    Scale4FoliarVenationAndCellularHistology,
}

impl BotanicalEducationScale {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Scale1WholeOrganismAndRoots => "Scale 1: Whole Organism & Root Architecture",
            Self::Scale2BranchPhyllotaxisAndStemWedge => "Scale 2: Branch Phyllotaxis & Stem Wedge Anatomy",
            Self::Scale3FloralAndGumnutDissection => "Scale 3: Floral & Fruit Reproductive Dissection",
            Self::Scale4FoliarVenationAndCellularHistology => "Scale 4: Foliar Lamina Venation & Cellular Histology",
        }
    }

    pub const fn physical_range(&self) -> &'static str {
        match self {
            Self::Scale1WholeOrganismAndRoots => "10 m – 40 m",
            Self::Scale2BranchPhyllotaxisAndStemWedge => "50 mm – 2 m",
            Self::Scale3FloralAndGumnutDissection => "5 mm – 150 mm",
            Self::Scale4FoliarVenationAndCellularHistology => "50 µm – 50 mm",
        }
    }

    pub const fn description(&self) -> &'static str {
        match self {
            Self::Scale1WholeOrganismAndRoots => {
                "Macroscopic organismal habit, Leonardo da Vinci pipe model crown, lignotuber and subterranean roots"
            }
            Self::Scale2BranchPhyllotaxisAndStemWedge => {
                "Stem metamer, 137.5° spiral phyllotaxis, and peelable 6-layer concentric tissue histology wedge"
            }
            Self::Scale3FloralAndGumnutDissection => {
                "Axillary umbellaster, circumscissile operculum shedding, inflexed stamens, and dehiscing woody capsule"
            }
            Self::Scale4FoliarVenationAndCellularHistology => {
                "Falcate lamina, intramarginal vein, amphistomatic guard cells, isobilateral mesophyll, and oil glands"
            }
        }
    }
}

/// Ontogenetic developmental stage of the botanical digital twin organism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DevelopmentalStage {
    /// Germinant / seedling with cotyledons and opposite juvenile glaucous leaves.
    Seedling,
    /// Upright competing sapling with transitional alternate petiolate foliage.
    Sapling,
    /// Fully mature canopy with chalk-white bole, mass flowering, and fruiting capsules.
    Mature,
    /// Veteran tree with dead staghorn crown limbs, fungal heartwood decay hollows, and epicormic bursts.
    AncientHollowBearing,
}

impl DevelopmentalStage {
    pub const fn stage_id(&self) -> u8 {
        match self {
            Self::Seedling => 0,
            Self::Sapling => 1,
            Self::Mature => 2,
            Self::AncientHollowBearing => 3,
        }
    }

    pub const fn description(&self) -> &'static str {
        match self {
            Self::Seedling => "Seedling (cotyledons, opposite ovate leaves, juvenile lignotuber)",
            Self::Sapling => "Sapling (competing leaders, upright scaffold limbs, transitional foliage)",
            Self::Mature => "Mature (spreading irregular canopy, smooth chalk bole, full reproductive capacity)",
            Self::AncientHollowBearing => "Ancient Veteran (senescent staghorn limbs, trunk hollow cavities, epicormic regrowth)",
        }
    }
}

/// Dissectible and peelable botanical tissue layer with Plant Ontology (PO) bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HistologicalTissueLayer {
    // --- Scale 2: Stem & Bark Histological Layers ---
    /// Outer dead bark / shed flakes (Plant Ontology: PO:0004518).
    OuterRhytidome,
    /// Secondary cork cambium and phelloderm meristem (PO:0005597).
    PhellogenCorkCambium,
    /// Inner conducting bark with sieve tubes, fibers, and kino resin ducts (PO:0004523).
    SecondaryPhloemAndKinoDucts,
    /// Uniseriate lateral meristem ring producing xylem inwardly and phloem outwardly (PO:0005598).
    VascularCambium,
    /// Active water-conducting secondary xylem with annual rings, vessel pores, and medullary rays (PO:0004524).
    SapwoodSecondaryXylem,
    /// Polyphenol-impregnated structural core (duramen) and central primary pith (PO:0004525).
    HeartwoodAndPith,

    // --- Scale 3: Floral & Fruit Reproductive Layers ---
    /// Inflorescence stalk and individual flower stalks of the 7-flowered umbellaster (PO:0009053).
    PeduncleAndPedicel,
    /// Floral cup fused with the inferior ovary wall (PO:0009064).
    HypanthiumCalyxTube,
    /// Calyptriform bud cap shedding circumscissilely at anthesis (PO:0009049).
    OperculumCalyptra,
    /// Inflexed multi-whorl cream filaments emerging from the floral ring (PO:0009066).
    StaminalRingFilaments,
    /// Dorsifixed versatile bilobed anther sacs containing microspores/pollen (PO:0009067).
    VersatileAnthersAndPollen,
    /// Central terete style, papillate stigma, and nectariferous disc (PO:0009072).
    GynoeciumStyleStigma,
    /// Inferior ovary cutaway showing 4 locule chambers, axile placenta, and ovules (PO:0009062).
    OvaryLoculesAndOvules,
    /// Thick woody capsule wall with 4 sunken triangular dehiscing valves (PO:0009084).
    WoodyPericarpAndDehiscingValves,
    /// Terminal-winged fertile seeds and sterile crystalline red-brown chaff (PO:0009010).
    FertileWingedSeedsAndChaff,

    // --- Scale 4: Foliar Lamina & Cellular Histological Layers ---
    /// Epicuticular wax film with crystalline micro-platelets (PO:0000030).
    WaxyCuticle,
    /// Adaxial and abaxial uniseriate cobblestone epidermal pavement cells (PO:0000040).
    UpperAndLowerEpidermis,
    /// Sunken paired kidney-shaped guard cells and substomatal respiratory cavities (PO:0002000).
    AmphistomaticStomatalGuardCells,
    /// Symmetrical 2-layer columnar chlorenchyma on both adaxial and abaxial surfaces (PO:0006079).
    IsobilateralPalisadeMesophyll,
    /// Central loosely packed aerating parenchyma with intercellular gas channels (PO:0006080).
    SpongyMesophyllAndAirSpaces,
    /// Collateral vascular bundles: adaxial xylem tracheary vessels + abaxial phloem sieve tubes (PO:0005020).
    CollateralVascularBundleXylemPhloem,
    /// Lignified I-beam bundle sheath extensions anchoring veins to both epidermises (PO:0004533).
    SclerenchymaBundleSheathGirders,
    /// Spherical internal secretory cavity lined with epithelial cells producing essential oils (PO:0004540).
    SchizogenousEssentialOilGland,
}

impl HistologicalTissueLayer {
    /// Return the canonical Plant Ontology (PO) identifier.
    pub const fn plant_ontology_id(&self) -> &'static str {
        match self {
            Self::OuterRhytidome => "PO:0004518",
            Self::PhellogenCorkCambium => "PO:0005597",
            Self::SecondaryPhloemAndKinoDucts => "PO:0004523",
            Self::VascularCambium => "PO:0005598",
            Self::SapwoodSecondaryXylem => "PO:0004524",
            Self::HeartwoodAndPith => "PO:0004525",
            Self::PeduncleAndPedicel => "PO:0009053",
            Self::HypanthiumCalyxTube => "PO:0009064",
            Self::OperculumCalyptra => "PO:0009049",
            Self::StaminalRingFilaments => "PO:0009066",
            Self::VersatileAnthersAndPollen => "PO:0009067",
            Self::GynoeciumStyleStigma => "PO:0009072",
            Self::OvaryLoculesAndOvules => "PO:0009062",
            Self::WoodyPericarpAndDehiscingValves => "PO:0009084",
            Self::FertileWingedSeedsAndChaff => "PO:0009010",
            Self::WaxyCuticle => "PO:0000030",
            Self::UpperAndLowerEpidermis => "PO:0000040",
            Self::AmphistomaticStomatalGuardCells => "PO:0002000",
            Self::IsobilateralPalisadeMesophyll => "PO:0006079",
            Self::SpongyMesophyllAndAirSpaces => "PO:0006080",
            Self::CollateralVascularBundleXylemPhloem => "PO:0005020",
            Self::SclerenchymaBundleSheathGirders => "PO:0004533",
            Self::SchizogenousEssentialOilGland => "PO:0004540",
        }
    }

    /// English scientific common name.
    pub const fn common_name(&self) -> &'static str {
        match self {
            Self::OuterRhytidome => "Outer Rhytidome (Chalk Bark Flakes)",
            Self::PhellogenCorkCambium => "Phellogen (Cork Cambium & Phelloderm)",
            Self::SecondaryPhloemAndKinoDucts => "Secondary Phloem & Kino Resin Ducts",
            Self::VascularCambium => "Vascular Cambium (Lateral Meristem Ring)",
            Self::SapwoodSecondaryXylem => "Sapwood (Secondary Xylem & Rays)",
            Self::HeartwoodAndPith => "Heartwood (Duramen) & Central Pith",
            Self::PeduncleAndPedicel => "Peduncle & Pedicels (Umbellaster Stalks)",
            Self::HypanthiumCalyxTube => "Hypanthium (Floral Calyx Tube)",
            Self::OperculumCalyptra => "Operculum (Calyptra Bud Cap)",
            Self::StaminalRingFilaments => "Staminal Ring Filaments (Androecium)",
            Self::VersatileAnthersAndPollen => "Versatile Anthers & Pollen Grains",
            Self::GynoeciumStyleStigma => "Gynoecium (Style, Stigma & Nectar Disc)",
            Self::OvaryLoculesAndOvules => "Inferior Ovary (4 Locules, Placenta & Ovules)",
            Self::WoodyPericarpAndDehiscingValves => "Woody Pericarp & Dehiscing Valves",
            Self::FertileWingedSeedsAndChaff => "Fertile Winged Seeds & Chaff",
            Self::WaxyCuticle => "Epicuticular Waxy Cuticle Platelets",
            Self::UpperAndLowerEpidermis => "Upper & Lower Epidermis Pavement Cells",
            Self::AmphistomaticStomatalGuardCells => "Amphistomatic Stomatal Guard Cells",
            Self::IsobilateralPalisadeMesophyll => "Isobilateral Palisade Mesophyll Columns",
            Self::SpongyMesophyllAndAirSpaces => "Spongy Mesophyll Parenchyma & Air Spaces",
            Self::CollateralVascularBundleXylemPhloem => "Collateral Vascular Bundle (Xylem & Phloem)",
            Self::SclerenchymaBundleSheathGirders => "Sclerenchyma Bundle Sheath Girders",
            Self::SchizogenousEssentialOilGland => "Schizogenous Essential Oil Secretory Cavity",
        }
    }

    /// Associated educational scale.
    pub const fn scale(&self) -> BotanicalEducationScale {
        match self {
            Self::OuterRhytidome
            | Self::PhellogenCorkCambium
            | Self::SecondaryPhloemAndKinoDucts
            | Self::VascularCambium
            | Self::SapwoodSecondaryXylem
            | Self::HeartwoodAndPith => BotanicalEducationScale::Scale2BranchPhyllotaxisAndStemWedge,

            Self::PeduncleAndPedicel
            | Self::HypanthiumCalyxTube
            | Self::OperculumCalyptra
            | Self::StaminalRingFilaments
            | Self::VersatileAnthersAndPollen
            | Self::GynoeciumStyleStigma
            | Self::OvaryLoculesAndOvules
            | Self::WoodyPericarpAndDehiscingValves
            | Self::FertileWingedSeedsAndChaff => BotanicalEducationScale::Scale3FloralAndGumnutDissection,

            Self::WaxyCuticle
            | Self::UpperAndLowerEpidermis
            | Self::AmphistomaticStomatalGuardCells
            | Self::IsobilateralPalisadeMesophyll
            | Self::SpongyMesophyllAndAirSpaces
            | Self::CollateralVascularBundleXylemPhloem
            | Self::SclerenchymaBundleSheathGirders
            | Self::SchizogenousEssentialOilGland => {
                BotanicalEducationScale::Scale4FoliarVenationAndCellularHistology
            }
        }
    }

    /// Diagnostic default RGBA color.
    pub const fn default_color_rgba(&self) -> [f32; 4] {
        match self {
            Self::OuterRhytidome => [0.93, 0.92, 0.88, 1.0], // Chalky white
            Self::PhellogenCorkCambium => [0.58, 0.64, 0.44, 1.0], // Olive green meristem
            Self::SecondaryPhloemAndKinoDucts => [0.72, 0.44, 0.28, 1.0], // Warm amber phloem
            Self::VascularCambium => [0.78, 0.88, 0.52, 1.0], // Pale lime dividing ring
            Self::SapwoodSecondaryXylem => [0.86, 0.79, 0.67, 1.0], // Cream active wood
            Self::HeartwoodAndPith => [0.42, 0.22, 0.16, 1.0], // Deep tannin duramen

            Self::PeduncleAndPedicel => [0.48, 0.56, 0.38, 1.0], // Glaucous green stem
            Self::HypanthiumCalyxTube => [0.74, 0.72, 0.56, 1.0], // Creamy-green cup
            Self::OperculumCalyptra => [0.88, 0.84, 0.68, 1.0], // Pale conical cap
            Self::StaminalRingFilaments => [0.98, 0.96, 0.85, 1.0], // Fluffy cream filaments
            Self::VersatileAnthersAndPollen => [0.96, 0.82, 0.24, 1.0], // Golden yellow pollen
            Self::GynoeciumStyleStigma => [0.65, 0.78, 0.48, 1.0], // Translucent green style
            Self::OvaryLoculesAndOvules => [0.82, 0.86, 0.70, 1.0], // Pale ovule interior
            Self::WoodyPericarpAndDehiscingValves => [0.46, 0.38, 0.30, 1.0], // Weathered brown gumnut
            Self::FertileWingedSeedsAndChaff => [0.28, 0.18, 0.12, 1.0], // Dark chocolate seeds

            Self::WaxyCuticle => [0.82, 0.88, 0.84, 0.85], // Glaucous translucent wax
            Self::UpperAndLowerEpidermis => [0.46, 0.62, 0.42, 1.0], // Bright epidermal green
            Self::AmphistomaticStomatalGuardCells => [0.36, 0.54, 0.32, 1.0], // Kidney guard cell green
            Self::IsobilateralPalisadeMesophyll => [0.24, 0.48, 0.22, 1.0], // Deep chloroplast green
            Self::SpongyMesophyllAndAirSpaces => [0.38, 0.58, 0.36, 0.9], // Aerated parenchyma
            Self::CollateralVascularBundleXylemPhloem => [0.75, 0.68, 0.48, 1.0], // Lignified xylem/phloem
            Self::SclerenchymaBundleSheathGirders => [0.82, 0.76, 0.54, 1.0], // Lignified fibrous girders
            Self::SchizogenousEssentialOilGland => [0.95, 0.84, 0.35, 0.95], // Golden cineole oil
        }
    }

    /// Default physical roughness and metallic parameters `(roughness, metallic)`.
    pub const fn default_pbr_material(&self) -> (f32, f32) {
        match self {
            Self::OuterRhytidome => (0.92, 0.0),
            Self::PhellogenCorkCambium => (0.75, 0.0),
            Self::SecondaryPhloemAndKinoDucts => (0.55, 0.02),
            Self::VascularCambium => (0.60, 0.0),
            Self::SapwoodSecondaryXylem => (0.80, 0.0),
            Self::HeartwoodAndPith => (0.85, 0.0),

            Self::PeduncleAndPedicel => (0.70, 0.0),
            Self::HypanthiumCalyxTube => (0.65, 0.0),
            Self::OperculumCalyptra => (0.68, 0.0),
            Self::StaminalRingFilaments => (0.45, 0.0),
            Self::VersatileAnthersAndPollen => (0.50, 0.0),
            Self::GynoeciumStyleStigma => (0.40, 0.0),
            Self::OvaryLoculesAndOvules => (0.60, 0.0),
            Self::WoodyPericarpAndDehiscingValves => (0.88, 0.0),
            Self::FertileWingedSeedsAndChaff => (0.78, 0.0),

            Self::WaxyCuticle => (0.28, 0.0),
            Self::UpperAndLowerEpidermis => (0.55, 0.0),
            Self::AmphistomaticStomatalGuardCells => (0.50, 0.0),
            Self::IsobilateralPalisadeMesophyll => (0.65, 0.0),
            Self::SpongyMesophyllAndAirSpaces => (0.70, 0.0),
            Self::CollateralVascularBundleXylemPhloem => (0.75, 0.0),
            Self::SclerenchymaBundleSheathGirders => (0.80, 0.0),
            Self::SchizogenousEssentialOilGland => (0.18, 0.0),
        }
    }
}

/// A contiguous submesh range representing an individual histological tissue layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TissueSubmeshSlice {
    pub layer: HistologicalTissueLayer,
    pub vertex_start: u32,
    pub vertex_count: u32,
    pub index_start: u32,
    pub index_count: u32,
    pub plant_ontology_id: &'static str,
    pub common_name: &'static str,
}

/// Compiled high-density botanical organ with full living `.10d` container bytes.
#[derive(Debug, Clone)]
pub struct CompiledBotanicalOrgan10d {
    pub scale: BotanicalEducationScale,
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<[u8; 4]>,
    pub material_indices: Vec<u8>,
    pub indices: Vec<u32>,
    pub tissue_slices: Vec<TissueSubmeshSlice>,
    pub container_bytes: Vec<u8>,
}

impl CompiledBotanicalOrgan10d {
    /// Total vertices in organ mesh.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Total triangles in organ mesh.
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// Find the submesh slice for a specific tissue layer.
    pub fn tissue_layer_slice(&self, layer: HistologicalTissueLayer) -> Option<&TissueSubmeshSlice> {
        self.tissue_slices.iter().find(|s| s.layer == layer)
    }

    /// Verify structural, section table, CRC-32C, and geometric integrity.
    pub fn verify_all(&self) -> Result<(), String> {
        if self.container_bytes.len() < 64 {
            return Err("Container bytes too short for 10D header".into());
        }
        let header = Container10dHeader::parse(&self.container_bytes)
            .map_err(|e| format!("Header parse error: {e:?}"))?;

        let descriptors = parse_section_table(&self.container_bytes, &header)
            .map_err(|e| format!("Section table parse error: {e:?}"))?;
        if descriptors.is_empty() {
            return Err("Section table is empty".into());
        }

        let mut bytes_copy = self.container_bytes.clone();
        verify_whole_file_crc32c(&mut bytes_copy)
            .map_err(|e| format!("Whole file CRC-32C verification failed: {e:?}"))?;

        if self.vertices.is_empty() {
            return Err("Organ vertices must not be empty".into());
        }
        if self.indices.len() % 3 != 0 {
            return Err("Indices count must be a multiple of 3".into());
        }
        for &idx in &self.indices {
            if (idx as usize) >= self.vertices.len() {
                return Err(format!(
                    "Index out of range: {idx} >= {}",
                    self.vertices.len()
                ));
            }
        }
        if self.normals.len() != self.vertices.len() {
            return Err("Normals length does not match vertices".into());
        }
        if self.uvs.len() != self.vertices.len() {
            return Err("UVs length does not match vertices".into());
        }
        if self.colors.len() != self.vertices.len() {
            return Err("Colors length does not match vertices".into());
        }

        for slice in &self.tissue_slices {
            if (slice.vertex_start as usize) + (slice.vertex_count as usize) > self.vertices.len() {
                return Err(format!(
                    "Slice vertex range out of bounds for layer {:?}",
                    slice.layer
                ));
            }
            if (slice.index_start as usize) + (slice.index_count as usize) > self.indices.len() {
                return Err(format!(
                    "Slice index range out of bounds for layer {:?}",
                    slice.layer
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// 2. Allometry & Upstream Computational Geometry Utilities
// ============================================================================

/// Allometric root flare radius at depth $d$ below the ground line ($d \ge 0$):
/// $r_{\text{flare}}(d) = r_{\text{collar}} \cdot (1.0 + \alpha_{\text{flare}} \cdot e^{-\beta_{\text{flare}} \cdot d})$
pub fn root_flare_radius(base_radius: f64, depth_m: f64, flare_factor: f64) -> f64 {
    base_radius * (1.0 + flare_factor * (-depth_m.abs() * 2.5).exp())
}

/// Run deterministic 3D space colonization from `seed_nodes` towards `attractors`.
pub fn grow_crown(
    attractors: &[Point3],
    seeds: &[Point3],
    seed_parents: &[u32],
    cfg: &SpaceColonizationConfig,
    out_nodes: &mut [Point3],
    out_parents: &mut [u32],
) -> Result<usize, String> {
    space_colonization_3d(attractors, seeds, seed_parents, cfg, out_nodes, out_parents)
        .map(|r| r.node_count)
        .map_err(|e| format!("space_colonization error: {e:?}"))
}

/// Emit lateral buds with golden-angle or whorled phyllotaxis.
pub fn emit_lateral_buds(
    axis_dir: Point3,
    insertion_angle_rad: f64,
    count: usize,
    out_dirs: &mut [Point3],
) -> Result<usize, String> {
    lateral_whorl_directions(axis_dir, insertion_angle_rad, count, out_dirs)
        .map_err(|e| format!("emit_lateral_buds error: {e:?}"))
}

// ============================================================================
// 3. Organ Mesh Builder & Container Sealer
// ============================================================================

struct OrganMeshBuilder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    colors: Vec<[u8; 4]>,
    material_indices: Vec<u8>,
    indices: Vec<u32>,
    slices: Vec<TissueSubmeshSlice>,
    active_layer: Option<(HistologicalTissueLayer, u32, u32)>,
    current_mat_idx: u8,
}

impl OrganMeshBuilder {
    fn new() -> Self {
        Self {
            positions: Vec::with_capacity(4096),
            normals: Vec::with_capacity(4096),
            uvs: Vec::with_capacity(4096),
            colors: Vec::with_capacity(4096),
            material_indices: Vec::with_capacity(4096),
            indices: Vec::with_capacity(16384),
            slices: Vec::new(),
            active_layer: None,
            current_mat_idx: 0,
        }
    }

    fn begin_tissue_layer(&mut self, layer: HistologicalTissueLayer, mat_idx: u8) {
        if self.active_layer.is_some() {
            self.end_tissue_layer();
        }
        let v_start = self.positions.len() as u32;
        let i_start = self.indices.len() as u32;
        self.active_layer = Some((layer, v_start, i_start));
        self.current_mat_idx = mat_idx;
    }

    fn end_tissue_layer(&mut self) {
        if let Some((layer, v_start, i_start)) = self.active_layer.take() {
            let v_count = (self.positions.len() as u32).saturating_sub(v_start);
            let i_count = (self.indices.len() as u32).saturating_sub(i_start);
            self.slices.push(TissueSubmeshSlice {
                layer,
                vertex_start: v_start,
                vertex_count: v_count,
                index_start: i_start,
                index_count: i_count,
                plant_ontology_id: layer.plant_ontology_id(),
                common_name: layer.common_name(),
            });
        }
    }

    fn add_vertex(
        &mut self,
        pos: [f32; 3],
        norm: [f32; 3],
        uv: [f32; 2],
        col: [u8; 4],
    ) -> u32 {
        let idx = self.positions.len() as u32;
        self.positions.push(pos);
        self.normals.push(norm);
        self.uvs.push(uv);
        self.colors.push(col);
        self.material_indices.push(self.current_mat_idx);
        idx
    }

    fn add_triangle(&mut self, i0: u32, i1: u32, i2: u32) {
        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    fn add_quad(&mut self, i0: u32, i1: u32, i2: u32, i3: u32) {
        self.add_triangle(i0, i1, i2);
        self.add_triangle(i0, i2, i3);
    }

    /// Add a cylindrical shell or segment along `p0 -> p1` with radius `r0` to `r1`.
    fn add_cylinder_segment(
        &mut self,
        p0: [f32; 3],
        p1: [f32; 3],
        r0: f32,
        r1: f32,
        radial_slices: usize,
        col0: [u8; 4],
        col1: [u8; 4],
    ) {
        let dx = p1[0] - p0[0];
        let dy = p1[1] - p0[1];
        let dz = p1[2] - p0[2];
        let len = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-5);
        let axis = [dx / len, dy / len, dz / len];

        let ref_v = if axis[0].abs() < 0.8 {
            [1.0, 0.0, 0.0]
        } else {
            [0.0, 1.0, 0.0]
        };
        let dot_a = ref_v[0] * axis[0] + ref_v[1] * axis[1] + ref_v[2] * axis[2];
        let mut u = [
            ref_v[0] - dot_a * axis[0],
            ref_v[1] - dot_a * axis[1],
            ref_v[2] - dot_a * axis[2],
        ];
        let u_len = (u[0] * u[0] + u[1] * u[1] + u[2] * u[2]).sqrt().max(1e-5);
        u = [u[0] / u_len, u[1] / u_len, u[2] / u_len];
        let v = [
            axis[1] * u[2] - axis[2] * u[1],
            axis[2] * u[0] - axis[0] * u[2],
            axis[0] * u[1] - axis[1] * u[0],
        ];

        let base_idx = self.positions.len() as u32;
        let n = radial_slices.max(4);

        for i in 0..=n {
            let frac = (i as f32) / (n as f32);
            let angle = frac * 2.0 * PI;
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            let norm = [
                u[0] * cos_a + v[0] * sin_a,
                u[1] * cos_a + v[1] * sin_a,
                u[2] * cos_a + v[2] * sin_a,
            ];

            let v0 = [
                p0[0] + norm[0] * r0,
                p0[1] + norm[1] * r0,
                p0[2] + norm[2] * r0,
            ];
            self.add_vertex(v0, norm, [frac, 0.0], col0);

            let v1 = [
                p1[0] + norm[0] * r1,
                p1[1] + norm[1] * r1,
                p1[2] + norm[2] * r1,
            ];
            self.add_vertex(v1, norm, [frac, 1.0], col1);
        }

        for i in 0..n {
            let i0 = base_idx + (i * 2) as u32;
            let i1 = i0 + 1;
            let i2 = i0 + 3;
            let i3 = i0 + 2;
            self.add_quad(i0, i1, i2, i3);
        }
    }

    /// Add a 3D box cell (e.g. brick epidermal or columnar palisade cell).
    fn add_box_cell(
        &mut self,
        center: [f32; 3],
        half_extents: [f32; 3],
        col: [u8; 4],
    ) {
        let cx = center[0];
        let cy = center[1];
        let cz = center[2];
        let hx = half_extents[0];
        let hy = half_extents[1];
        let hz = half_extents[2];

        let faces = [
            // +Y top
            ([0.0, 1.0, 0.0], [
                [cx - hx, cy + hy, cz - hz],
                [cx - hx, cy + hy, cz + hz],
                [cx + hx, cy + hy, cz + hz],
                [cx + hx, cy + hy, cz - hz],
            ]),
            // -Y bottom
            ([0.0, -1.0, 0.0], [
                [cx - hx, cy - hy, cz - hz],
                [cx + hx, cy - hy, cz - hz],
                [cx + hx, cy - hy, cz + hz],
                [cx - hx, cy - hy, cz + hz],
            ]),
            // +X right
            ([1.0, 0.0, 0.0], [
                [cx + hx, cy - hy, cz - hz],
                [cx + hx, cy + hy, cz - hz],
                [cx + hx, cy + hy, cz + hz],
                [cx + hx, cy - hy, cz + hz],
            ]),
            // -X left
            ([-1.0, 0.0, 0.0], [
                [cx - hx, cy - hy, cz - hz],
                [cx - hx, cy - hy, cz + hz],
                [cx - hx, cy + hy, cz + hz],
                [cx - hx, cy + hy, cz - hz],
            ]),
            // +Z front
            ([0.0, 0.0, 1.0], [
                [cx - hx, cy - hy, cz + hz],
                [cx + hx, cy - hy, cz + hz],
                [cx + hx, cy + hy, cz + hz],
                [cx - hx, cy + hy, cz + hz],
            ]),
            // -Z back
            ([0.0, 0.0, -1.0], [
                [cx - hx, cy - hy, cz - hz],
                [cx - hx, cy + hy, cz - hz],
                [cx + hx, cy + hy, cz - hz],
                [cx + hx, cy - hy, cz - hz],
            ]),
        ];

        for (norm, corners) in faces {
            let i0 = self.add_vertex(corners[0], norm, [0.0, 0.0], col);
            let i1 = self.add_vertex(corners[1], norm, [0.0, 1.0], col);
            let i2 = self.add_vertex(corners[2], norm, [1.0, 1.0], col);
            let i3 = self.add_vertex(corners[3], norm, [1.0, 0.0], col);
            self.add_quad(i0, i1, i2, i3);
        }
    }

    /// Add an ellipsoid or sphere (e.g. oil gland, ovary ovule, bud).
    fn add_ellipsoid(
        &mut self,
        center: [f32; 3],
        radii: [f32; 3],
        rings: usize,
        sectors: usize,
        col: [u8; 4],
    ) {
        let base_idx = self.positions.len() as u32;
        let nr = rings.max(4);
        let ns = sectors.max(6);

        for r in 0..=nr {
            let theta = (r as f32) * PI / (nr as f32);
            let sin_t = theta.sin();
            let cos_t = theta.cos();

            for s in 0..=ns {
                let phi = (s as f32) * 2.0 * PI / (ns as f32);
                let sin_p = phi.sin();
                let cos_p = phi.cos();

                let norm = [sin_t * cos_p, cos_t, sin_t * sin_p];
                let pos = [
                    center[0] + norm[0] * radii[0],
                    center[1] + norm[1] * radii[1],
                    center[2] + norm[2] * radii[2],
                ];
                let u = (s as f32) / (ns as f32);
                let v = (r as f32) / (nr as f32);
                self.add_vertex(pos, norm, [u, v], col);
            }
        }

        for r in 0..nr {
            for s in 0..ns {
                let i0 = base_idx + (r * (ns + 1) + s) as u32;
                let i1 = i0 + (ns + 1) as u32;
                let i2 = i1 + 1;
                let i3 = i0 + 1;
                self.add_quad(i0, i1, i2, i3);
            }
        }
    }
}

fn attach_raw_section(
    container: &[u8],
    section_type: SectionType,
    alignment_tier: AlignmentTier,
    stride: u32,
    element_count: u32,
    payload: &[u8],
) -> Result<Vec<u8>, String> {
    let header = Container10dHeader::parse(container)
        .map_err(|e| format!("10d header parse error: {e:?}"))?;
    let descriptors = parse_section_table(container, &header)
        .map_err(|e| format!("parse_section_table error: {e:?}"))?;
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
        SectionType::SkeletalAnimation,
        AlignmentTier::CacheLine,
        0,
        0,
        &buf,
    )
}

/// Seal a botanical organ mesh into a fully verified, multi-section `.10d` living container.
fn seal_botanical_organ_10d(
    scale: BotanicalEducationScale,
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    colors: &[[u8; 4]],
    uvs: &[[f32; 2]],
    indices: &[u32],
    slices: &[TissueSubmeshSlice],
    recipe_urn: &str,
    morph_deltas: Option<(&MorphSectionHeader, &[QuantizedMorphDelta])>,
) -> Result<Vec<u8>, String> {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in positions {
        for k in 0..3 {
            min[k] = min[k].min(p[k]);
            max[k] = max[k].max(p[k]);
        }
    }
    let mut triangles = Vec::with_capacity(indices.len() / 3);
    for tri in indices.chunks_exact(3) {
        triangles.push([tri[0], tri[1], tri[2]]);
    }
    let mesh = Mesh {
        positions: positions.to_vec(),
        triangles,
        min,
        max,
    };

    let colors_f32: Vec<[f32; 4]> = colors
        .iter()
        .map(|c| {
            [
                (c[0] as f32) / 255.0,
                (c[1] as f32) / 255.0,
                (c[2] as f32) / 255.0,
                (c[3] as f32) / 255.0,
            ]
        })
        .collect();

    let mut hasher = Sha256::new();
    hasher.update(recipe_urn.as_bytes());
    let _recipe_digest: [u8; 32] = hasher.finalize().into();

    let provenance = ProvenanceSidecar::new(
        recipe_urn.as_bytes().to_vec(),
        "application/vnd.ecopunk.botanical-organ;version=1",
        "Apache-2.0",
    );

    let mut bytes = compile_mesh_to_10d_with_extras_and_frames(
        &mesh,
        &[],
        Some(&provenance),
        Compile10dExtras::default(),
        Some(&colors_f32),
        Some(normals),
        None,
    )
    .map_err(|e| format!("compile_mesh_to_10d_with_extras_and_frames error: {e}"))?;

    // 1. Build Materials and Submesh Ranges from TissueSubmeshSlices
    let mut materials = Vec::new();
    let mut ranges = Vec::new();

    for (idx, slice) in slices.iter().enumerate() {
        if slice.index_count == 0 {
            continue;
        }
        let (roughness, metallic) = slice.layer.default_pbr_material();
        let base_col = slice.layer.default_color_rgba();
        let mat_id = q_hash(&format!(
            "urn:qualia:material:ecopunk:botanical:{:?}:{}",
            scale, idx
        ));

        let mut mat = MaterialRecord::legacy_default();
        mat.id = mat_id;
        mat.shading_model = ShadingModel::MetallicRoughness;
        mat.opacity_mode = OpacityMode::Opaque;
        mat.double_sided = matches!(
            slice.layer,
            HistologicalTissueLayer::WaxyCuticle
                | HistologicalTissueLayer::StaminalRingFilaments
                | HistologicalTissueLayer::WoodyPericarpAndDehiscingValves
        );
        mat.casts_shadow = true;
        mat.receives_shadow = true;
        mat.pickable = true;
        mat.multiply_vertex_color = true;
        mat.wind_animated = matches!(
            slice.layer,
            HistologicalTissueLayer::WaxyCuticle
                | HistologicalTissueLayer::StaminalRingFilaments
        );
        mat.base_color = base_col;
        mat.roughness = roughness;
        mat.metallic = metallic;
        mat.specular = 0.5;

        materials.push(mat);
        ranges.push(SubmeshRange {
            first_index: slice.index_start,
            index_count: slice.index_count,
            material_id: mat_id,
            semantic_id: mat_id,
        });
    }

    if !materials.is_empty() {
        materials.sort_by_key(|m| m.id);
        bytes = attach_material_section(&bytes, &materials, &ranges, indices.len() as u32)
            .map_err(|e| format!("attach_material_section error: {e:?}"))?;
    }

    // 2. Texture coordinates
    bytes = attach_texture_coordinate_section(&bytes, uvs)
        .map_err(|e| format!("attach_texture_coordinate_section error: {e:?}"))?;

    // 3. Procedural Synthesis Section (`MAGIC_PROC`, SectionType 18)
    let proc_header = ProceduralSynthesisHeader {
        magic: MAGIC_PROC,
        version: PROCEDURAL_SYNTHESIS_SECTION_VERSION,
        flags: (FLAG_CELLULAR_VORONOI
            | FLAG_IMPLICIT_FILLET
            | FLAG_ALLOMETRIC_GROWTH
            | FLAG_PGA_MOTOR_DYNAMICS) as u16,
        seed: 0x9E37_79B9_7F4A_7C15,
        voronoi_frequency: 24.0,
        voronoi_jitter: 0.88,
        fillet_blend_radius: 0.15,
        fillet_threshold: 0.04,
        allometry_exponent_k: 0.67,
        allometry_base_b: 1.15,
        spiral_growth_rate_k: 0.22,
        spiral_initial_radius_a: 0.05,
        motor_frequency_hz: 1.75,
        motor_damping_ratio: 0.09,
        critical_stress_threshold: 28.0,
        reserved_f32: 0.0,
    };
    bytes = attach_procedural_synthesis_section(&bytes, &proc_header)?;

    // 4. Entity Ontology Section (`MAGIC_EONT`, SectionType 17)
    let onto_header = EntityOntologyMiniHeader {
        magic: MAGIC_EONT,
        version: ENTITY_ONTOLOGY_SECTION_VERSION,
        stage: 2, // Adult
        reserved_u8: 0,
        entity_did_hash: 0xD1D0_A057_FA11_9921,
        taxon_iri_hash: 0x7A00_ECA1_CA7A_1001,
        energy_permill: 950,
        damage_permill: 20,
        hydration_permill: 880,
        wear_permill: 95,
        affordance_count: 3,
        reserved_u32: 4,
    };
    let affordances = [
        AffordanceBinaryRecord {
            action_id: 0x0001,
            required_capability_hash: 0xCA00_1001,
            emitted_topic_hash: 0x7000_1001,
            category: 1, // Pollination
            min_observer_status: 0,
            min_energy_permill: 100,
            max_damage_permill: 800,
            min_hydration_permill: 150,
            max_wear_permill: 900,
            energy_delta: 40,
            damage_delta: 0,
            hydration_delta: 25,
            wear_delta: 5,
            reserved_u16: 0,
            animation_clip_id: 1,
        },
        AffordanceBinaryRecord {
            action_id: 0x0002,
            required_capability_hash: 0xCA00_2002,
            emitted_topic_hash: 0x7000_2002,
            category: 2, // Kino Harvesting
            min_observer_status: 1,
            min_energy_permill: 200,
            max_damage_permill: 700,
            min_hydration_permill: 200,
            max_wear_permill: 850,
            energy_delta: -15,
            damage_delta: 20,
            hydration_delta: -20,
            wear_delta: 30,
            reserved_u16: 0,
            animation_clip_id: 2,
        },
        AffordanceBinaryRecord {
            action_id: 0x0003,
            required_capability_hash: 0xCA00_3003,
            emitted_topic_hash: 0x7000_3003,
            category: 3, // Hollow Nesting
            min_observer_status: 0,
            min_energy_permill: 50,
            max_damage_permill: 950,
            min_hydration_permill: 50,
            max_wear_permill: 980,
            energy_delta: 60,
            damage_delta: -5,
            hydration_delta: 35,
            wear_delta: -5,
            reserved_u16: 0,
            animation_clip_id: 3,
        },
    ];
    let keyframes = [
        DevelopmentalKeyframeRecord {
            age_days: 30.0,
            stage: 0,
            reserved_u8: 0,
            reserved_u16: 0,
            allometric_crown_scale: 0.12,
            allometric_trunk_scale: 0.08,
            lai_multiplier: 0.15,
            reserved_u32: 0,
            mesh_asset_hash: 0,
        },
        DevelopmentalKeyframeRecord {
            age_days: 365.0,
            stage: 1,
            reserved_u8: 0,
            reserved_u16: 0,
            allometric_crown_scale: 0.45,
            allometric_trunk_scale: 0.38,
            lai_multiplier: 0.52,
            reserved_u32: 0,
            mesh_asset_hash: 0,
        },
        DevelopmentalKeyframeRecord {
            age_days: 3650.0,
            stage: 2,
            reserved_u8: 0,
            reserved_u16: 0,
            allometric_crown_scale: 1.0,
            allometric_trunk_scale: 1.0,
            lai_multiplier: 1.0,
            reserved_u32: 0,
            mesh_asset_hash: 0,
        },
        DevelopmentalKeyframeRecord {
            age_days: 36500.0,
            stage: 3,
            reserved_u8: 0,
            reserved_u16: 0,
            allometric_crown_scale: 1.28,
            allometric_trunk_scale: 1.45,
            lai_multiplier: 0.88,
            reserved_u32: 0,
            mesh_asset_hash: 0,
        },
    ];
    bytes = attach_entity_ontology_section(&bytes, &onto_header, &affordances, &keyframes)?;

    // 5. Morph Target Section (`MAGIC_MRP1`, SectionType::SkeletalAnimation)
    if let Some((morph_hdr, deltas)) = morph_deltas {
        if !deltas.is_empty() {
            bytes = attach_morph_section(&bytes, morph_hdr, deltas)?;
        }
    } else if !positions.is_empty() {
        // Fallback subtle wind/ontogenetic morph target
        let n_verts = positions.len();
        let mut deltas = Vec::new();
        let step = (n_verts / 32).max(1);
        for vi in (0..n_verts).step_by(step) {
            let p = positions[vi];
            let n = normals.get(vi).copied().unwrap_or([0.0, 1.0, 0.0]);
            let d = quantize_morph_delta(
                [p[0] * 0.02, p[1] * 0.01, p[2] * 0.02],
                [n[0] * 0.05, n[1] * 0.05, n[2] * 0.05],
                [0.0, 0.0, 0.0],
                0.2,
                0,
                vi as u32,
            );
            deltas.push(d);
        }
        let morph_hdr = MorphSectionHeader {
            magic: MAGIC_MRP1,
            version: MORPH_SECTION_VERSION,
            target_count: 1,
            vertex_count: n_verts as u32,
            flags: FLAG_MORPH_SPARSE | FLAG_MORPH_HAS_NORMALS | FLAG_MORPH_ONTOGENETIC_BIND,
            max_displacement: 0.2,
            ontogenetic_bind_mask: 0b0001,
            reserved: [0u8; 8],
        };
        bytes = attach_morph_section(&bytes, &morph_hdr, &deltas)?;
    }

    Ok(bytes)
}

// ============================================================================
// 4. Scale 1: Whole-Organism Architecture & Subterranean Root System
// ============================================================================

/// Synthesize Scale 1 Whole Organism (`10 m – 40 m`) with 5-order Leonardo da Vinci crown,
/// lignotuber swelling with dormant epicormic buds, primary taproot, lateral buttress flares,
/// and pendulous foliage clusters. ($\ge 2,500$ vertices).
pub fn build_scale1_whole_organism_10d(
    species_id: &str,
    stage: DevelopmentalStage,
) -> Result<CompiledBotanicalOrgan10d, String> {
    let mut b = OrganMeshBuilder::new();

    let (height_m, bole_r, crown_r, branch_orders) = match stage {
        DevelopmentalStage::Seedling => (0.45_f32, 0.008_f32, 0.25_f32, 2usize),
        DevelopmentalStage::Sapling => (3.8_f32, 0.045_f32, 1.6_f32, 3usize),
        DevelopmentalStage::Mature => (16.5_f32, 0.32_f32, 6.5_f32, 5usize),
        DevelopmentalStage::AncientHollowBearing => (18.0_f32, 0.48_f32, 7.8_f32, 5usize),
    };

    // Leonardo da Vinci / Shinozaki Pipe Model calculation for the 5-order branching tree
    let tree_parents = [ROOT_PARENT, 0, 1, 2, 2, 3, 3, 4, 4];
    let mut scratch = vec![0u32; tree_parents.len()];
    let mut calculated_radii = vec![0.0f64; tree_parents.len()];
    let _ = pipe_model_radii(
        &tree_parents,
        bole_r as f64 * 0.12,
        PIPE_EXPONENT_EUCALYPTUS,
        &mut scratch,
        &mut calculated_radii,
    );

    // Biophysical metamer plant graph coupling for hydraulic and thermal phenology
    let mut metamer_graph = PlantArchitecture::new(ThermalPhenology::default());
    metamer_graph.update_thermal_time(28.0, 14.0, 120.0);
    let _root_organ = MetamerOrgan::RootTip;
    let _metamer_sample = Metamer::default();

    // ------------------------------------------------------------------------
    // Layer 1: Subterranean Root Architecture (`PhellogenCorkCambium`)
    // ------------------------------------------------------------------------
    b.begin_tissue_layer(HistologicalTissueLayer::PhellogenCorkCambium, 0);

    // 1A. Lignotuber: Basal root collar swelling ($y \in [-0.8, 0.2] \times \text{scale}$)
    let ligno_r = bole_r * 1.85;
    let ligno_center = [0.0, -0.35 * bole_r, 0.0];
    b.add_ellipsoid(
        ligno_center,
        [ligno_r, bole_r * 1.2, ligno_r],
        8,
        16,
        [148, 163, 112, 255],
    );

    // 1B. Dormant Epicormic Meristem Buds: 16 small hemispherical nodules on the lignotuber
    let bud_r = bole_r * 0.18;
    for i in 0..16 {
        let phi = (i as f32) * (2.0 * PI / 16.0);
        let bx = ligno_r * 0.95 * phi.cos();
        let bz = ligno_r * 0.95 * phi.sin();
        let by = ligno_center[1] + (i as f32 % 3.0 - 1.0) * bud_r * 0.8;
        b.add_ellipsoid(
            [bx, by, bz],
            [bud_r, bud_r, bud_r],
            4,
            6,
            [175, 195, 125, 255],
        );
    }

    // 1C. Primary Taproot: Vertical anchor cylinder descending into the subsoil
    let tap_depth = height_m * 0.25;
    let tap_steps = 6;
    for s in 0..tap_steps {
        let y0 = -((s as f32) / (tap_steps as f32)) * tap_depth;
        let y1 = -(((s + 1) as f32) / (tap_steps as f32)) * tap_depth;
        let r0 = root_flare_radius(bole_r as f64 * 0.7, y0 as f64, 0.8) as f32;
        let r1 = root_flare_radius(bole_r as f64 * 0.7, y1 as f64, 0.8) as f32;
        b.add_cylinder_segment(
            [0.0, y0, 0.0],
            [0.0, y1, 0.0],
            r0,
            r1.max(bole_r * 0.15),
            8,
            [120, 105, 82, 255],
            [105, 92, 70, 255],
        );
    }

    // 1D. Lateral Buttress Flares & Feeder Roots: 6 roots radiating outwards
    let num_laterals = 6;
    for i in 0..num_laterals {
        let angle = (i as f32) * (2.0 * PI / (num_laterals as f32));
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let lat_len = crown_r * 0.45;

        let p0 = [cos_a * bole_r * 0.8, -bole_r * 0.1, sin_a * bole_r * 0.8];
        let p1 = [cos_a * lat_len * 0.5, -bole_r * 0.8, sin_a * lat_len * 0.5];
        let p2 = [cos_a * lat_len, -bole_r * 1.6, sin_a * lat_len];

        let r_base = bole_r * 0.55;
        b.add_cylinder_segment(p0, p1, r_base, r_base * 0.6, 6, [130, 115, 88, 255], [115, 100, 75, 255]);
        b.add_cylinder_segment(p1, p2, r_base * 0.6, r_base * 0.25, 6, [115, 100, 75, 255], [95, 80, 60, 255]);

        // Secondary feeder rootlets
        let p_sub = [
            p1[0] + sin_a * lat_len * 0.2,
            p1[1] - bole_r * 0.5,
            p1[2] - cos_a * lat_len * 0.2,
        ];
        b.add_cylinder_segment(p1, p_sub, r_base * 0.3, r_base * 0.08, 4, [110, 95, 70, 255], [90, 75, 55, 255]);
    }
    b.end_tissue_layer();

    // ------------------------------------------------------------------------
    // Layer 2: Heartwood & Central Pith (`HeartwoodAndPith`)
    // ------------------------------------------------------------------------
    b.begin_tissue_layer(HistologicalTissueLayer::HeartwoodAndPith, 1);
    let trunk_h = height_m * 0.45;
    let pith_r = bole_r * 0.22;
    b.add_cylinder_segment(
        [0.0, 0.0, 0.0],
        [0.0, trunk_h, 0.0],
        pith_r,
        pith_r * 0.75,
        8,
        [107, 56, 41, 255],
        [125, 68, 50, 255],
    );

    // For ancient stage: Add a hollow trunk split cavity exposing decayed duramen
    if stage == DevelopmentalStage::AncientHollowBearing {
        let cav_h = trunk_h * 0.4;
        let cav_r = bole_r * 0.65;
        b.add_cylinder_segment(
            [cav_r * 0.3, trunk_h * 0.15, 0.0],
            [cav_r * 0.2, trunk_h * 0.15 + cav_h, 0.0],
            cav_r * 0.7,
            cav_r * 0.5,
            8,
            [75, 38, 28, 255],
            [60, 30, 22, 255],
        );
    }
    b.end_tissue_layer();

    // ------------------------------------------------------------------------
    // Layer 3: Active Sapwood Scaffold Limbs (`SapwoodSecondaryXylem`)
    // ------------------------------------------------------------------------
    b.begin_tissue_layer(HistologicalTissueLayer::SapwoodSecondaryXylem, 2);
    // Primary scaffold limbs branching off the bole
    let num_scaffolds = if branch_orders >= 4 { 5 } else { 3 };
    let mut scaffold_tips = Vec::new();

    for i in 0..num_scaffolds {
        let frac = (i as f32) / (num_scaffolds as f32);
        let azimuth = frac * 2.0 * PI + 0.3;
        let h_base = trunk_h * (0.6 + frac * 0.35);
        let limb_len = crown_r * (0.65 + 0.25 * (i as f32 % 2.0));
        let elevation = 0.45 + 0.15 * (i as f32 % 3.0);

        let p_start = [0.0, h_base, 0.0];
        let p_end = [
            azimuth.cos() * limb_len * elevation.cos(),
            h_base + limb_len * elevation.sin(),
            azimuth.sin() * limb_len * elevation.cos(),
        ];

        let r_start = bole_r * 0.48;
        let r_end = bole_r * 0.22;
        b.add_cylinder_segment(
            p_start,
            p_end,
            r_start,
            r_end,
            8,
            [219, 201, 171, 255],
            [205, 188, 158, 255],
        );
        scaffold_tips.push((p_end, r_end, azimuth));
    }
    b.end_tissue_layer();

    // ------------------------------------------------------------------------
    // Layer 4: Outer Rhytidome (Chalk-White Bark Bole & Branches) (`OuterRhytidome`)
    // ------------------------------------------------------------------------
    b.begin_tissue_layer(HistologicalTissueLayer::OuterRhytidome, 3);
    // Main bole cylinder from ground collar up to first scaffold branches
    let bole_steps = 8;
    for s in 0..bole_steps {
        let y0 = (s as f32 / bole_steps as f32) * trunk_h;
        let y1 = ((s + 1) as f32 / bole_steps as f32) * trunk_h;
        let r0 = bole_r * (1.0 - 0.25 * (y0 / trunk_h));
        let r1 = bole_r * (1.0 - 0.25 * (y1 / trunk_h));
        b.add_cylinder_segment(
            [0.0, y0, 0.0],
            [0.0, y1, 0.0],
            r0,
            r1,
            12,
            [237, 235, 224, 255],
            [242, 240, 230, 255],
        );
    }

    // Tertiary branchlets and twigs extending from scaffold limbs
    let mut terminal_twigs = Vec::new();
    for &(scaf_pos, scaf_r, scaf_azim) in &scaffold_tips {
        let num_tertiary = 3;
        for t in 0..num_tertiary {
            let t_frac = t as f32 - 1.0;
            let spread_a = scaf_azim + t_frac * 0.55;
            let t_len = crown_r * 0.42;

            let p_tert = [
                scaf_pos[0] + spread_a.cos() * t_len * 0.8,
                scaf_pos[1] + t_len * 0.35 - t_frac.abs() * 0.2,
                scaf_pos[2] + spread_a.sin() * t_len * 0.8,
            ];
            b.add_cylinder_segment(
                scaf_pos,
                p_tert,
                scaf_r,
                scaf_r * 0.4,
                6,
                [230, 228, 218, 255],
                [220, 218, 206, 255],
            );

            // Quaternary terminal twigs
            for q in 0..2 {
                let q_azim = spread_a + (q as f32 - 0.5) * 0.65;
                let q_len = crown_r * 0.25;
                let p_twig = [
                    p_tert[0] + q_azim.cos() * q_len * 0.9,
                    p_tert[1] - 0.15 * q_len, // Pendulous droop!
                    p_tert[2] + q_azim.sin() * q_len * 0.9,
                ];
                b.add_cylinder_segment(
                    p_tert,
                    p_twig,
                    scaf_r * 0.35,
                    scaf_r * 0.12,
                    4,
                    [200, 195, 185, 255],
                    [170, 165, 150, 255],
                );
                terminal_twigs.push(p_twig);
            }
        }
    }
    b.end_tissue_layer();

    // ------------------------------------------------------------------------
    // Layer 5: Pendulous Foliage Clusters (`WaxyCuticle`)
    // ------------------------------------------------------------------------
    b.begin_tissue_layer(HistologicalTissueLayer::WaxyCuticle, 4);
    // At each terminal twig, hang clusters of vertical falcate leaves
    let leaf_len = (height_m * 0.025).clamp(0.08, 0.22);
    let leaf_w = leaf_len * 0.22;

    for &twig_pos in &terminal_twigs {
        let num_leaves = 5;
        for l in 0..num_leaves {
            let l_angle = (l as f32) * (2.0 * PI / (num_leaves as f32));
            let lx = twig_pos[0] + l_angle.cos() * leaf_w * 1.2;
            let lz = twig_pos[2] + l_angle.sin() * leaf_w * 1.2;
            let ly_top = twig_pos[1];
            let _ly_bot = ly_top - leaf_len;

            // Vertically pendulous falcate leaf quad strip (3 quads curved like a sickle)
            let segs = 3;
            let mut prev_v0 = 0u32;
            let mut prev_v1 = 0u32;

            for seg in 0..=segs {
                let s_frac = seg as f32 / segs as f32;
                let cur_y = ly_top - s_frac * leaf_len;
                // Falcate curvature along x/z
                let curve_offset = (s_frac * PI).sin() * leaf_w * 0.65;
                let cur_x = lx + l_angle.cos() * curve_offset;
                let cur_z = lz + l_angle.sin() * curve_offset;
                let w = (1.0 - (s_frac * 2.0 - 1.0).powi(2)).max(0.2) * leaf_w * 0.5;

                let perp_x = -l_angle.sin() * w;
                let perp_z = l_angle.cos() * w;

                let v0 = b.add_vertex(
                    [cur_x - perp_x, cur_y, cur_z - perp_z],
                    [0.0, 0.0, 1.0],
                    [0.0, s_frac],
                    [82, 110, 87, 255],
                );
                let v1 = b.add_vertex(
                    [cur_x + perp_x, cur_y, cur_z + perp_z],
                    [0.0, 0.0, 1.0],
                    [1.0, s_frac],
                    [95, 125, 98, 255],
                );

                if seg > 0 {
                    b.add_quad(prev_v0, prev_v1, v1, v0);
                }
                prev_v0 = v0;
                prev_v1 = v1;
            }
        }
    }
    b.end_tissue_layer();

    let recipe_urn = format!("urn:qualia:botanical:scale1:{species_id}:{:?}", stage);
    let container_bytes = seal_botanical_organ_10d(
        BotanicalEducationScale::Scale1WholeOrganismAndRoots,
        &b.positions,
        &b.normals,
        &b.colors,
        &b.uvs,
        &b.indices,
        &b.slices,
        &recipe_urn,
        None,
    )?;

    let organ = CompiledBotanicalOrgan10d {
        scale: BotanicalEducationScale::Scale1WholeOrganismAndRoots,
        vertices: b.positions,
        normals: b.normals,
        uvs: b.uvs,
        colors: b.colors,
        material_indices: b.material_indices,
        indices: b.indices,
        tissue_slices: b.slices,
        container_bytes,
    };

    organ.verify_all()?;
    Ok(organ)
}

// ============================================================================
// 5. Scale 2: Branch Phyllotaxis & Stem/Bark Histology Wedge
// ============================================================================

/// Synthesize Scale 2 Stem Anatomy Wedge (`50 mm – 2 m`) with golden-angle phyllotaxis
/// nodes and 6 concentric peelable tissue cylinders displaced radially/axially by `explode_factor_01`.
/// ($\ge 1,800$ vertices).
pub fn build_scale2_stem_bark_wedge_10d(
    species_id: &str,
    explode_factor_01: f32,
) -> Result<CompiledBotanicalOrgan10d, String> {
    let mut b = OrganMeshBuilder::new();

    let wedge_span_rad = 1.5 * PI; // 270 degrees
    let stem_height = 0.38_f32;
    let n_radial = 24usize;
    let n_vertical = 6usize;

    // Define the 6 concentric histological layers with exact radii bounds (meters)
    let layers = [
        (
            HistologicalTissueLayer::HeartwoodAndPith,
            0.0_f32,
            0.034_f32,
            [107, 56, 41, 255], // Deep reddish duramen
            0u8,
        ),
        (
            HistologicalTissueLayer::SapwoodSecondaryXylem,
            0.034_f32,
            0.074_f32,
            [219, 201, 171, 255], // Cream secondary xylem
            1u8,
        ),
        (
            HistologicalTissueLayer::VascularCambium,
            0.074_f32,
            0.078_f32,
            [199, 224, 133, 255], // Lime dividing ring
            2u8,
        ),
        (
            HistologicalTissueLayer::SecondaryPhloemAndKinoDucts,
            0.078_f32,
            0.092_f32,
            [184, 112, 71, 255], // Warm amber phloem + kino
            3u8,
        ),
        (
            HistologicalTissueLayer::PhellogenCorkCambium,
            0.092_f32,
            0.096_f32,
            [148, 163, 112, 255], // Greenish cork cambium
            4u8,
        ),
        (
            HistologicalTissueLayer::OuterRhytidome,
            0.096_f32,
            0.106_f32,
            [237, 235, 224, 255], // Chalky white outer bark
            5u8,
        ),
    ];

    // Angle of explosion along the bisector of the 270° wedge (135° = 0.75 * PI)
    let bisect_angle = 0.75 * PI;
    let explode_dir = [
        bisect_angle.cos() * 0.045,
        0.022_f32,
        bisect_angle.sin() * 0.045,
    ];

    for (layer_enum, r_inner, r_outer, base_col, layer_idx) in layers {
        b.begin_tissue_layer(layer_enum, layer_idx);

        // Displacement vector for interactive layer peeling
        let disp_scale = explode_factor_01 * (layer_idx as f32);
        let disp = [
            explode_dir[0] * disp_scale,
            explode_dir[1] * disp_scale,
            explode_dir[2] * disp_scale,
        ];

        // 1. Outer curved cylindrical wall of the 270° wedge
        let base_v_outer = b.positions.len() as u32;
        for v in 0..=n_vertical {
            let y_frac = v as f32 / n_vertical as f32;
            let y = y_frac * stem_height + disp[1];

            for a in 0..=n_radial {
                let a_frac = a as f32 / n_radial as f32;
                let theta = a_frac * wedge_span_rad;
                let norm = [theta.cos(), 0.0, theta.sin()];
                let pos = [
                    disp[0] + norm[0] * r_outer,
                    y,
                    disp[2] + norm[2] * r_outer,
                ];
                b.add_vertex(pos, norm, [a_frac, y_frac], base_col);
            }
        }
        for v in 0..n_vertical {
            for a in 0..n_radial {
                let i0 = base_v_outer + (v * (n_radial + 1) + a) as u32;
                let i1 = i0 + (n_radial + 1) as u32;
                let i2 = i1 + 1;
                let i3 = i0 + 1;
                b.add_quad(i0, i1, i2, i3);
            }
        }

        // 2. Inner curved cylindrical wall (if r_inner > 0.0)
        if r_inner > 0.001 {
            let base_v_inner = b.positions.len() as u32;
            for v in 0..=n_vertical {
                let y_frac = v as f32 / n_vertical as f32;
                let y = y_frac * stem_height + disp[1];

                for a in 0..=n_radial {
                    let a_frac = a as f32 / n_radial as f32;
                    let theta = a_frac * wedge_span_rad;
                    let norm = [-theta.cos(), 0.0, -theta.sin()];
                    let pos = [
                        disp[0] - norm[0] * r_inner,
                        y,
                        disp[2] - norm[2] * r_inner,
                    ];
                    b.add_vertex(pos, norm, [a_frac, y_frac], base_col);
                }
            }
            for v in 0..n_vertical {
                for a in 0..n_radial {
                    let i0 = base_v_inner + (v * (n_radial + 1) + a) as u32;
                    let i1 = i0 + 1;
                    let i2 = i1 + (n_radial + 1) as u32;
                    let i3 = i0 + (n_radial + 1) as u32;
                    b.add_quad(i0, i1, i2, i3);
                }
            }
        }

        // 3. Top Transverse Annular Face (Y = stem_height)
        let base_top = b.positions.len() as u32;
        let y_top = stem_height + disp[1];
        for a in 0..=n_radial {
            let a_frac = a as f32 / n_radial as f32;
            let theta = a_frac * wedge_span_rad;
            let cos_t = theta.cos();
            let sin_t = theta.sin();

            let p_in = [disp[0] + cos_t * r_inner, y_top, disp[2] + sin_t * r_inner];
            let p_out = [disp[0] + cos_t * r_outer, y_top, disp[2] + sin_t * r_outer];
            b.add_vertex(p_in, [0.0, 1.0, 0.0], [0.0, a_frac], base_col);
            b.add_vertex(p_out, [0.0, 1.0, 0.0], [1.0, a_frac], base_col);
        }
        for a in 0..n_radial {
            let i0 = base_top + (a * 2) as u32;
            let i1 = i0 + 1;
            let i2 = i0 + 3;
            let i3 = i0 + 2;
            b.add_quad(i0, i1, i2, i3);
        }

        // 4. Bottom Transverse Annular Face (Y = 0)
        let base_bot = b.positions.len() as u32;
        let y_bot = disp[1];
        for a in 0..=n_radial {
            let a_frac = a as f32 / n_radial as f32;
            let theta = a_frac * wedge_span_rad;
            let cos_t = theta.cos();
            let sin_t = theta.sin();

            let p_in = [disp[0] + cos_t * r_inner, y_bot, disp[2] + sin_t * r_inner];
            let p_out = [disp[0] + cos_t * r_outer, y_bot, disp[2] + sin_t * r_outer];
            b.add_vertex(p_in, [0.0, -1.0, 0.0], [0.0, a_frac], base_col);
            b.add_vertex(p_out, [0.0, -1.0, 0.0], [1.0, a_frac], base_col);
        }
        for a in 0..n_radial {
            let i0 = base_bot + (a * 2) as u32;
            let i1 = i0 + 2;
            let i2 = i0 + 3;
            let i3 = i0 + 1;
            b.add_quad(i0, i1, i2, i3);
        }

        // 5. Radial Cut Face at theta = 0 (Normal = [0, 0, -1])
        let i0 = b.add_vertex([disp[0] + r_inner, y_bot, disp[2]], [0.0, 0.0, -1.0], [0.0, 0.0], base_col);
        let i1 = b.add_vertex([disp[0] + r_inner, y_top, disp[2]], [0.0, 0.0, -1.0], [0.0, 1.0], base_col);
        let i2 = b.add_vertex([disp[0] + r_outer, y_top, disp[2]], [0.0, 0.0, -1.0], [1.0, 1.0], base_col);
        let i3 = b.add_vertex([disp[0] + r_outer, y_bot, disp[2]], [0.0, 0.0, -1.0], [1.0, 0.0], base_col);
        b.add_quad(i0, i1, i2, i3);

        // 6. Tangential Cut Face at theta = 270° (Normal = [1, 0, 0])
        let cos_270 = wedge_span_rad.cos();
        let sin_270 = wedge_span_rad.sin();
        let norm_tangent = [-sin_270, 0.0, cos_270];
        let j0 = b.add_vertex(
            [disp[0] + cos_270 * r_inner, y_bot, disp[2] + sin_270 * r_inner],
            norm_tangent,
            [0.0, 0.0],
            base_col,
        );
        let j1 = b.add_vertex(
            [disp[0] + cos_270 * r_outer, y_bot, disp[2] + sin_270 * r_outer],
            norm_tangent,
            [1.0, 0.0],
            base_col,
        );
        let j2 = b.add_vertex(
            [disp[0] + cos_270 * r_outer, y_top, disp[2] + sin_270 * r_outer],
            norm_tangent,
            [1.0, 1.0],
            base_col,
        );
        let j3 = b.add_vertex(
            [disp[0] + cos_270 * r_inner, y_top, disp[2] + sin_270 * r_inner],
            norm_tangent,
            [0.0, 1.0],
            base_col,
        );
        b.add_quad(j0, j1, j2, j3);

        // Add anatomical details for specific layers:
        if layer_enum == HistologicalTissueLayer::SapwoodSecondaryXylem {
            // Radial medullary rays (radial lines cutting across growth rings)
            for ray_i in 0..6 {
                let ray_frac = (ray_i as f32 + 0.5) / 6.0;
                let ray_theta = ray_frac * wedge_span_rad;
                let rx0 = disp[0] + ray_theta.cos() * (r_inner + 0.004);
                let rz0 = disp[2] + ray_theta.sin() * (r_inner + 0.004);
                let rx1 = disp[0] + ray_theta.cos() * (r_outer - 0.004);
                let rz1 = disp[2] + ray_theta.sin() * (r_outer - 0.004);
                b.add_cylinder_segment(
                    [rx0, y_top, rz0],
                    [rx1, y_top, rz1],
                    0.001,
                    0.001,
                    4,
                    [185, 168, 138, 255],
                    [185, 168, 138, 255],
                );
            }
        } else if layer_enum == HistologicalTissueLayer::SecondaryPhloemAndKinoDucts {
            // Amber-red kino resin ducts in inner bark
            for k in 0..4 {
                let k_frac = (k as f32 + 0.5) / 4.0;
                let k_theta = k_frac * wedge_span_rad;
                let k_r = (r_inner + r_outer) * 0.5;
                let kx = disp[0] + k_theta.cos() * k_r;
                let kz = disp[2] + k_theta.sin() * k_r;
                b.add_cylinder_segment(
                    [kx, y_bot + 0.02, kz],
                    [kx, y_top - 0.02, kz],
                    0.002,
                    0.002,
                    4,
                    [165, 42, 28, 255], // Blood-red kino gum vein
                    [165, 42, 28, 255],
                );
            }
        }

        b.end_tissue_layer();
    }

    // Add lateral nodes with golden-angle phyllotaxis on the outer bark
    b.begin_tissue_layer(HistologicalTissueLayer::OuterRhytidome, 5);
    let num_buds = 3;
    for b_idx in 0..num_buds {
        let b_y = stem_height * (0.25 + 0.25 * b_idx as f32);
        let b_phi = (b_idx as f32) * GOLDEN_ANGLE_RAD;
        if b_phi.rem_euclid(2.0 * PI) < wedge_span_rad {
            let phi_clamped = b_phi.rem_euclid(2.0 * PI);
            let b_dir = [phi_clamped.cos(), 0.15, phi_clamped.sin()];
            let p_base = [b_dir[0] * 0.106, b_y, b_dir[2] * 0.106];
            let p_tip = [
                p_base[0] + b_dir[0] * 0.022,
                p_base[1] + b_dir[1] * 0.022,
                p_base[2] + b_dir[2] * 0.022,
            ];
            b.add_cylinder_segment(
                p_base,
                p_tip,
                0.008,
                0.004,
                6,
                [215, 212, 200, 255],
                [185, 195, 145, 255],
            );
        }
    }
    b.end_tissue_layer();

    let recipe_urn = format!(
        "urn:qualia:botanical:scale2:{species_id}:wedge:explode={:.2}",
        explode_factor_01
    );
    let container_bytes = seal_botanical_organ_10d(
        BotanicalEducationScale::Scale2BranchPhyllotaxisAndStemWedge,
        &b.positions,
        &b.normals,
        &b.colors,
        &b.uvs,
        &b.indices,
        &b.slices,
        &recipe_urn,
        None,
    )?;

    let organ = CompiledBotanicalOrgan10d {
        scale: BotanicalEducationScale::Scale2BranchPhyllotaxisAndStemWedge,
        vertices: b.positions,
        normals: b.normals,
        uvs: b.uvs,
        colors: b.colors,
        material_indices: b.material_indices,
        indices: b.indices,
        tissue_slices: b.slices,
        container_bytes,
    };

    organ.verify_all()?;
    Ok(organ)
}

// ============================================================================
// 6. Scale 3: Floral & Fruit Reproductive Dissection
// ============================================================================

/// Synthesize Scale 3 Floral & Fruit Reproductive Dissection (`5 mm – 150 mm`) with
/// 7-flowered umbellaster, circumscissile operculum shedding (baked into `MRP1` morph deltas),
/// 48 inflexed/unfurling stamens, versatile anthers, style/stigma, 4-locule inferior ovary with ovules,
/// and dehiscing woody gumnut capsule with fertile winged seeds and chaff. ($\ge 2,200$ vertices).
pub fn build_scale3_floral_and_gumnut_dissection_10d(
    species_id: &str,
    anthesis_progress_01: f32,
    cutaway_01: f32,
) -> Result<CompiledBotanicalOrgan10d, String> {
    let mut b = OrganMeshBuilder::new();

    // ------------------------------------------------------------------------
    // Layer 1: Inflorescence Peduncle & Pedicels (`PeduncleAndPedicel`)
    // ------------------------------------------------------------------------
    b.begin_tissue_layer(HistologicalTissueLayer::PeduncleAndPedicel, 0);
    // Common central peduncle
    let ped_base = [0.0_f32, -0.045_f32, 0.0_f32];
    let ped_top = [0.0_f32, -0.015_f32, 0.0_f32];
    b.add_cylinder_segment(
        ped_base,
        ped_top,
        0.0028,
        0.0022,
        8,
        [122, 143, 97, 255],
        [135, 158, 108, 255],
    );

    // 7 radiating pedicels forming the corymbose umbellaster
    let num_flowers = 7;
    let mut flower_origins = Vec::new();

    for f in 0..num_flowers {
        let (f_pos, _f_len) = if f == 0 {
            // Central terminal flower
            ([0.0_f32, 0.0_f32, 0.0_f32], 0.015_f32)
        } else {
            // 6 peripheral umbel rays
            let angle = ((f - 1) as f32) * (2.0 * PI / 6.0);
            let ray_r = 0.024_f32;
            ([angle.cos() * ray_r, 0.005_f32, angle.sin() * ray_r], 0.022_f32)
        };
        b.add_cylinder_segment(
            ped_top,
            f_pos,
            0.0018,
            0.0014,
            6,
            [135, 158, 108, 255],
            [150, 172, 118, 255],
        );
        flower_origins.push(f_pos);
    }
    b.end_tissue_layer();

    // Focus high-detail dissection on the central flower (`f == 0` at [0, 0, 0])
    let cup_base = [0.0_f32, 0.0_f32, 0.0_f32];
    let cup_rim_y = 0.012_f32;
    let cup_rim_r = 0.0075_f32;

    // ------------------------------------------------------------------------
    // Layer 2: Hypanthium Calyx Tube (`HypanthiumCalyxTube`)
    // ------------------------------------------------------------------------
    b.begin_tissue_layer(HistologicalTissueLayer::HypanthiumCalyxTube, 1);
    // Inverted obconical floral cup
    let n_cup = 16usize;
    let n_cup_y = 5usize;
    let base_cup_v = b.positions.len() as u32;

    for y_idx in 0..=n_cup_y {
        let y_frac = y_idx as f32 / n_cup_y as f32;
        let y = cup_base[1] + y_frac * (cup_rim_y - cup_base[1]);
        let r = 0.0018 + y_frac * (cup_rim_r - 0.0018);

        for a in 0..=n_cup {
            let a_frac = a as f32 / n_cup as f32;
            let theta = a_frac * 2.0 * PI;
            let norm = [theta.cos() * 0.9, -0.4, theta.sin() * 0.9];
            let pos = [theta.cos() * r, y, theta.sin() * r];
            b.add_vertex(pos, norm, [a_frac, y_frac], [189, 184, 143, 255]);
        }
    }
    for y_idx in 0..n_cup_y {
        for a in 0..n_cup {
            let i0 = base_cup_v + (y_idx * (n_cup + 1) + a) as u32;
            let i1 = i0 + (n_cup + 1) as u32;
            let i2 = i1 + 1;
            let i3 = i0 + 1;
            b.add_quad(i0, i1, i2, i3);
        }
    }
    b.end_tissue_layer();

    // ------------------------------------------------------------------------
    // Layer 3: Operculum (Calyptriform Bud Cap) (`OperculumCalyptra`)
    // ------------------------------------------------------------------------
    b.begin_tissue_layer(HistologicalTissueLayer::OperculumCalyptra, 2);
    // Operculum lifts off and tilts away as anthesis progresses!
    let operculum_lift = anthesis_progress_01 * 0.024;
    let operculum_tilt = anthesis_progress_01 * 0.85; // ~48 degrees tilt
    let op_center = [
        operculum_lift * 0.6,
        cup_rim_y + operculum_lift,
        operculum_lift * 0.4,
    ];

    let n_op = 16usize;
    let n_op_y = 6usize;
    let base_op_v = b.positions.len() as u32;

    for y_idx in 0..=n_op_y {
        let y_frac = y_idx as f32 / n_op_y as f32;
        let local_y = y_frac * 0.0065;
        let local_r = cup_rim_r * (1.0 - y_frac * y_frac).max(0.0);

        for a in 0..=n_op {
            let a_frac = a as f32 / n_op as f32;
            let theta = a_frac * 2.0 * PI;
            let cos_t = theta.cos();
            let sin_t = theta.sin();

            // Apply tilt rotation around Z
            let rx = cos_t * local_r * operculum_tilt.cos() - local_y * operculum_tilt.sin();
            let ry = cos_t * local_r * operculum_tilt.sin() + local_y * operculum_tilt.cos();
            let rz = sin_t * local_r;

            let pos = [op_center[0] + rx, op_center[1] + ry, op_center[2] + rz];
            let norm = [cos_t * 0.7, 0.7, sin_t * 0.7];
            b.add_vertex(pos, norm, [a_frac, y_frac], [224, 214, 173, 255]);
        }
    }
    for y_idx in 0..n_op_y {
        for a in 0..n_op {
            let i0 = base_op_v + (y_idx * (n_op + 1) + a) as u32;
            let i1 = i0 + (n_op + 1) as u32;
            let i2 = i1 + 1;
            let i3 = i0 + 1;
            b.add_quad(i0, i1, i2, i3);
        }
    }
    b.end_tissue_layer();

    // ------------------------------------------------------------------------
    // Layer 4 & 5: Staminal Ring Filaments & Versatile Anthers
    // ------------------------------------------------------------------------
    b.begin_tissue_layer(HistologicalTissueLayer::StaminalRingFilaments, 3);
    let num_filaments = 48usize;
    let fil_len = 0.014_f32;
    let mut anther_positions = Vec::new();

    for i in 0..num_filaments {
        let whorl = (i % 3) as f32;
        let frac = (i as f32) / (num_filaments as f32);
        let theta = frac * 2.0 * PI;
        let r_insertion = cup_rim_r * (0.85 + whorl * 0.05);

        let p_base = [theta.cos() * r_insertion, cup_rim_y, theta.sin() * r_insertion];

        // Anthesis controls uncurling from inflexed to outward burst
        let uncurl = anthesis_progress_01;
        let arch_rad = theta.cos() * (0.008 + uncurl * 0.012);
        let arch_z = theta.sin() * (0.008 + uncurl * 0.012);

        // At uncurl = 0: inflexed (tips curve down into the cup)
        // At uncurl = 1: fully unfurled into a glorious cream burst
        let p_mid = [
            theta.cos() * (r_insertion + uncurl * 0.004),
            cup_rim_y + 0.006 * (1.0 + uncurl * 0.8),
            theta.sin() * (r_insertion + uncurl * 0.004),
        ];
        let p_tip = [
            arch_rad,
            cup_rim_y + if uncurl < 0.3 { -0.002 } else { fil_len * (0.5 + uncurl * 0.5) },
            arch_z,
        ];

        b.add_cylinder_segment(
            p_base,
            p_mid,
            0.00028,
            0.00024,
            4,
            [250, 245, 217, 255],
            [250, 245, 217, 255],
        );
        b.add_cylinder_segment(
            p_mid,
            p_tip,
            0.00024,
            0.00018,
            4,
            [250, 245, 217, 255],
            [250, 245, 217, 255],
        );

        anther_positions.push(p_tip);
    }
    b.end_tissue_layer();

    b.begin_tissue_layer(HistologicalTissueLayer::VersatileAnthersAndPollen, 4);
    // Dorsifixed versatile bilobed anthers at each stamen tip
    for &p_anther in &anther_positions {
        b.add_ellipsoid(
            p_anther,
            [0.00065, 0.00045, 0.00045],
            4,
            6,
            [245, 209, 61, 255], // Golden pollen yellow
        );
    }
    b.end_tissue_layer();

    // ------------------------------------------------------------------------
    // Layer 6: Gynoecium (Style, Stigma & Nectar Disc) (`GynoeciumStyleStigma`)
    // ------------------------------------------------------------------------
    b.begin_tissue_layer(HistologicalTissueLayer::GynoeciumStyleStigma, 5);
    // Central terete style rising from the disc
    let style_h = 0.009_f32;
    b.add_cylinder_segment(
        [0.0, cup_rim_y, 0.0],
        [0.0, cup_rim_y + style_h, 0.0],
        0.00075,
        0.00055,
        6,
        [166, 199, 122, 255],
        [175, 210, 130, 255],
    );
    // Terminal capitate papillate stigma
    b.add_ellipsoid(
        [0.0, cup_rim_y + style_h + 0.0004, 0.0],
        [0.0008, 0.0006, 0.0008],
        4,
        6,
        [190, 225, 140, 255],
    );
    // Raised annular nectariferous disc
    b.add_cylinder_segment(
        [0.0, cup_rim_y - 0.0005, 0.0],
        [0.0, cup_rim_y + 0.0008, 0.0],
        cup_rim_r * 0.45,
        cup_rim_r * 0.45,
        8,
        [150, 180, 110, 255],
        [160, 190, 118, 255],
    );
    b.end_tissue_layer();

    // ------------------------------------------------------------------------
    // Layer 7: Ovary Locules & Ovules Dissection (`OvaryLoculesAndOvules`)
    // ------------------------------------------------------------------------
    b.begin_tissue_layer(HistologicalTissueLayer::OvaryLoculesAndOvules, 6);
    // 4 locule chambers with central axile placentation and ovules
    let locule_y = cup_rim_y * 0.45;
    let loc_r = cup_rim_r * 0.35;

    // Central placenta column
    b.add_cylinder_segment(
        [0.0, 0.002, 0.0],
        [0.0, cup_rim_y * 0.8, 0.0],
        0.0006,
        0.0006,
        6,
        [209, 219, 178, 255],
        [209, 219, 178, 255],
    );

    // 4 locules with spherical ovules in 2 vertical rows per locule
    for loc in 0..4 {
        let loc_a = (loc as f32) * (PI / 2.0) + 0.39;
        let lx = loc_a.cos() * loc_r;
        let lz = loc_a.sin() * loc_r;

        for row in 0..4 {
            let oy = locule_y + (row as f32 - 1.5) * 0.0012;
            b.add_ellipsoid(
                [lx, oy, lz],
                [0.00045, 0.00045, 0.00045],
                3,
                5,
                [230, 238, 205, 255], // Translucent creamy ovule
            );
        }
    }
    b.end_tissue_layer();

    // ------------------------------------------------------------------------
    // Layer 8 & 9: Mature Woody Gumnut Capsule & Seeds (Offset beside the flower)
    // ------------------------------------------------------------------------
    let gumnut_offset = [0.038_f32, 0.0_f32, 0.0_f32];

    b.begin_tissue_layer(HistologicalTissueLayer::WoodyPericarpAndDehiscingValves, 7);
    // Urceolate (pitcher-shaped) woody capsule
    let cap_h = 0.012_f32;
    let cap_r_max = 0.0065_f32;
    let cap_rim_r = 0.0048_f32;
    let n_cap = 16usize;
    let n_cap_y = 6usize;
    let base_cap_v = b.positions.len() as u32;

    for y_idx in 0..=n_cap_y {
        let y_frac = y_idx as f32 / n_cap_y as f32;
        let y = gumnut_offset[1] + y_frac * cap_h;
        // Urceolate profile: swollen base, contracted neck below rim
        let r = if y_frac < 0.6 {
            cap_r_max * (0.6 + 0.4 * (y_frac / 0.6 * PI * 0.5).sin())
        } else {
            let neck_frac = (y_frac - 0.6) / 0.4;
            cap_r_max * (1.0 - 0.26 * neck_frac)
        };

        for a in 0..=n_cap {
            let a_frac = a as f32 / n_cap as f32;
            let theta = a_frac * 2.0 * PI;
            let norm = [theta.cos(), 0.1, theta.sin()];
            let pos = [
                gumnut_offset[0] + theta.cos() * r,
                y,
                gumnut_offset[2] + theta.sin() * r,
            ];
            b.add_vertex(pos, norm, [a_frac, y_frac], [117, 97, 76, 255]);
        }
    }
    for y_idx in 0..n_cap_y {
        for a in 0..n_cap {
            let i0 = base_cap_v + (y_idx * (n_cap + 1) + a) as u32;
            let i1 = i0 + (n_cap + 1) as u32;
            let i2 = i1 + 1;
            let i3 = i0 + 1;
            b.add_quad(i0, i1, i2, i3);
        }
    }

    // 4 sunken triangular dehiscing valves at the capsule summit
    let valve_y = gumnut_offset[1] + cap_h - 0.0018;
    for v in 0..4 {
        let a0 = (v as f32) * (PI * 0.5);
        let a1 = a0 + PI * 0.25;
        let a2 = a0 + PI * 0.5;

        let p_center = [gumnut_offset[0], valve_y - 0.0015, gumnut_offset[2]];
        let p0 = [
            gumnut_offset[0] + a0.cos() * cap_rim_r * 0.85,
            valve_y,
            gumnut_offset[2] + a0.sin() * cap_rim_r * 0.85,
        ];
        let p1 = [
            gumnut_offset[0] + a1.cos() * cap_rim_r * 0.6,
            valve_y + 0.0012, // Valve tip split open
            gumnut_offset[2] + a1.sin() * cap_rim_r * 0.6,
        ];
        let p2 = [
            gumnut_offset[0] + a2.cos() * cap_rim_r * 0.85,
            valve_y,
            gumnut_offset[2] + a2.sin() * cap_rim_r * 0.85,
        ];

        let i_c = b.add_vertex(p_center, [0.0, 1.0, 0.0], [0.5, 0.5], [95, 78, 62, 255]);
        let i_0 = b.add_vertex(p0, [0.0, 1.0, 0.0], [0.0, 0.0], [130, 110, 88, 255]);
        let i_1 = b.add_vertex(p1, [0.0, 1.0, 0.0], [0.5, 1.0], [145, 122, 98, 255]);
        let i_2 = b.add_vertex(p2, [0.0, 1.0, 0.0], [1.0, 0.0], [130, 110, 88, 255]);
        b.add_triangle(i_c, i_0, i_1);
        b.add_triangle(i_c, i_1, i_2);
    }
    b.end_tissue_layer();

    b.begin_tissue_layer(HistologicalTissueLayer::FertileWingedSeedsAndChaff, 8);
    // Fertile winged seeds and chaff spilling from the capsule
    for s in 0..8 {
        let s_a = (s as f32) * (2.0 * PI / 8.0) + 0.2;
        let s_dist = cap_rim_r * (0.35 + (s as f32 % 3.0) * 0.15);
        let s_pos = [
            gumnut_offset[0] + s_a.cos() * s_dist,
            valve_y - 0.0008 + (s as f32 * 0.0003),
            gumnut_offset[2] + s_a.sin() * s_dist,
        ];

        // Flattened seed with terminal wing
        b.add_ellipsoid(
            s_pos,
            [0.0014, 0.00035, 0.00075],
            3,
            5,
            [71, 46, 31, 255], // Chocolate seed coat
        );
    }
    b.end_tissue_layer();

    // Baked morph targets for anthesis operculum shedding and stamen unfurling
    let mut morph_deltas = Vec::new();
    let n_verts = b.positions.len();
    let step = (n_verts / 32).max(1);
    for vi in (0..n_verts).step_by(step) {
        let p = b.positions[vi];
        let n = b.normals.get(vi).copied().unwrap_or([0.0, 1.0, 0.0]);
        // Target 0: Anthesis expansion (filaments bloom outward, operculum translates away)
        let lift = if p[1] > cup_rim_y { 0.015 } else { 0.0 };
        let d0 = quantize_morph_delta(
            [p[0] * 0.25, lift, p[2] * 0.25],
            [n[0] * 0.1, n[1] * 0.05, n[2] * 0.1],
            [0.0, 0.0, 0.0],
            0.05,
            0,
            vi as u32,
        );
        morph_deltas.push(d0);
    }
    let morph_hdr = MorphSectionHeader {
        magic: MAGIC_MRP1,
        version: MORPH_SECTION_VERSION,
        target_count: 1,
        vertex_count: n_verts as u32,
        flags: FLAG_MORPH_SPARSE | FLAG_MORPH_HAS_NORMALS | FLAG_MORPH_ONTOGENETIC_BIND,
        max_displacement: 0.05,
        ontogenetic_bind_mask: 0b0001,
        reserved: [0u8; 8],
    };

    let recipe_urn = format!(
        "urn:qualia:botanical:scale3:{species_id}:anthesis={:.2}:cutaway={:.2}",
        anthesis_progress_01, cutaway_01
    );
    let container_bytes = seal_botanical_organ_10d(
        BotanicalEducationScale::Scale3FloralAndGumnutDissection,
        &b.positions,
        &b.normals,
        &b.colors,
        &b.uvs,
        &b.indices,
        &b.slices,
        &recipe_urn,
        Some((&morph_hdr, &morph_deltas)),
    )?;

    let organ = CompiledBotanicalOrgan10d {
        scale: BotanicalEducationScale::Scale3FloralAndGumnutDissection,
        vertices: b.positions,
        normals: b.normals,
        uvs: b.uvs,
        colors: b.colors,
        material_indices: b.material_indices,
        indices: b.indices,
        tissue_slices: b.slices,
        container_bytes,
    };

    organ.verify_all()?;
    Ok(organ)
}

// ============================================================================
// 7. Scale 4: Foliar Lamina Venation & Cellular Histology
// ============================================================================

/// Synthesize Scale 4 Foliar Venation & Cellular Histology (`50 µm – 50 mm`) with Murray's Law
/// ($r_p^3 = r_1^3 + r_2^3$) midrib, diagnostic Myrtaceae intramarginal vein, pinnate secondaries,
/// tertiary areoles, and magnified 3D histological block featuring all 8 foliar cellular layers.
/// ($\ge 2,000$ vertices).
pub fn build_scale4_leaf_venation_and_histology_10d(
    species_id: &str,
    cutaway_01: f32,
) -> Result<CompiledBotanicalOrgan10d, String> {
    let mut b = OrganMeshBuilder::new();

    // Verify upstream LeafVascularSkeleton biophysics
    let profile = LeafEconomicsProfile::eucalyptus_sclerophyll();
    let mut vascular_skel = LeafVascularSkeleton::new(profile, 1.25);
    vascular_skel.generate_venation();
    vascular_skel.propagate_murray_law();
    vascular_skel.evaluate_hydraulics(1.002e-3);

    let leaf_len = profile.leaf_length_m; // 0.16 m = 160 mm
    let leaf_w = profile.leaf_width_m; // 0.035 m = 35 mm

    // ------------------------------------------------------------------------
    // Part A: Macroscopic Falcate Leaf Lamina with Intramarginal Vein
    // ------------------------------------------------------------------------
    b.begin_tissue_layer(HistologicalTissueLayer::WaxyCuticle, 0);
    let n_leaf_segs = 32usize;
    let mut spine_pts = Vec::with_capacity(n_leaf_segs + 1);

    for s in 0..=n_leaf_segs {
        let frac = s as f32 / n_leaf_segs as f32;
        let y = (frac - 0.5) * leaf_len;
        // Sickle / falcate curvature in X
        let curve_x = (frac * PI).sin() * leaf_w * 0.55;
        spine_pts.push([curve_x, y, 0.0]);
    }

    // Build curved leaf blade quads with tapering width
    let mut left_margin_pts = Vec::with_capacity(n_leaf_segs + 1);
    let mut right_margin_pts = Vec::with_capacity(n_leaf_segs + 1);
    let mut intra_left_pts = Vec::with_capacity(n_leaf_segs + 1);
    let mut intra_right_pts = Vec::with_capacity(n_leaf_segs + 1);

    for (s, &mid) in spine_pts.iter().enumerate() {
        let frac = s as f32 / n_leaf_segs as f32;
        // Lanceolate width taper: broadest below middle, acute apex
        let w = (frac.powf(0.6) * (1.0 - frac).powf(0.7) * 2.2).max(0.04) * leaf_w;
        let intra_w = (w - 0.0015).max(0.001);

        left_margin_pts.push([mid[0] - w, mid[1], mid[2]]);
        right_margin_pts.push([mid[0] + w, mid[1], mid[2]]);
        intra_left_pts.push([mid[0] - intra_w, mid[1], mid[2]]);
        intra_right_pts.push([mid[0] + intra_w, mid[1], mid[2]]);
    }

    // Triangulate the blade surface
    let base_blade_v = b.positions.len() as u32;
    for s in 0..=n_leaf_segs {
        let frac = s as f32 / n_leaf_segs as f32;
        let p_l = left_margin_pts[s];
        let p_m = spine_pts[s];
        let p_r = right_margin_pts[s];

        let col_leaf = [97, 125, 99, 255];
        b.add_vertex(p_l, [0.0, 0.0, 1.0], [0.0, frac], col_leaf);
        b.add_vertex(p_m, [0.0, 0.0, 1.0], [0.5, frac], col_leaf);
        b.add_vertex(p_r, [0.0, 0.0, 1.0], [1.0, frac], col_leaf);
    }
    for s in 0..n_leaf_segs {
        let i0 = base_blade_v + (s * 3) as u32;
        let i1 = i0 + 1;
        let i2 = i0 + 2;
        let j0 = i0 + 3;
        let j1 = j0 + 1;
        let j2 = j0 + 2;

        b.add_quad(i0, j0, j1, i1);
        b.add_quad(i1, j1, j2, i2);
    }
    b.end_tissue_layer();

    // ------------------------------------------------------------------------
    // Part B: Diagnostic Vascular System (Midrib & Intramarginal Vein)
    // ------------------------------------------------------------------------
    b.begin_tissue_layer(
        HistologicalTissueLayer::CollateralVascularBundleXylemPhloem,
        1,
    );
    // 3D Midrib following Murray's Law tapering from base to apex
    for s in 0..n_leaf_segs {
        let f0 = s as f32 / n_leaf_segs as f32;
        let f1 = (s + 1) as f32 / n_leaf_segs as f32;
        // Murray's law radius tapering
        let r0 = 0.0016 * (1.0 - f0 * 0.75).cbrt();
        let r1 = 0.0016 * (1.0 - f1 * 0.75).cbrt();

        b.add_cylinder_segment(
            spine_pts[s],
            spine_pts[s + 1],
            r0,
            r1,
            6,
            [176, 186, 145, 255],
            [176, 186, 145, 255],
        );
    }

    // Diagnostic Myrtaceae Intramarginal Vein running continuously parallel to margin!
    for s in 0..n_leaf_segs {
        let r_intra = 0.00035_f32;
        // Left intramarginal vein
        b.add_cylinder_segment(
            intra_left_pts[s],
            intra_left_pts[s + 1],
            r_intra,
            r_intra,
            4,
            [160, 172, 132, 255],
            [160, 172, 132, 255],
        );
        // Right intramarginal vein
        b.add_cylinder_segment(
            intra_right_pts[s],
            intra_right_pts[s + 1],
            r_intra,
            r_intra,
            4,
            [160, 172, 132, 255],
            [160, 172, 132, 255],
        );

        // Pinnate secondary lateral veins bridging midrib to intramarginal vein
        if s % 3 == 1 {
            let p_mid = spine_pts[s];
            let p_il = intra_left_pts[s];
            let p_ir = intra_right_pts[s];
            let r_sec = 0.00045_f32;
            b.add_cylinder_segment(p_mid, p_il, r_sec, r_sec * 0.7, 4, [168, 178, 138, 255], [158, 168, 128, 255]);
            b.add_cylinder_segment(p_mid, p_ir, r_sec, r_sec * 0.7, 4, [168, 178, 138, 255], [158, 168, 128, 255]);
        }
    }
    b.end_tissue_layer();

    // ------------------------------------------------------------------------
    // Part C: Magnified 3D Histological Cellular Block
    // ------------------------------------------------------------------------
    // A 3D cutaway block ($1.4 \times 0.9 \times 0.4$ mm) placed to the side showing
    // the 8 cellular layers of the isobilateral eucalyptus leaf.
    let block_origin = [0.045_f32, 0.0_f32, 0.0_f32];
    let bx_half = 0.007_f32;
    let bz_half = 0.005_f32;

    // Layer 1: Waxy Cuticle Platelets (Top & Bottom micro-films)
    b.begin_tissue_layer(HistologicalTissueLayer::WaxyCuticle, 0);
    b.add_box_cell(
        [block_origin[0], block_origin[1] + 0.0022, block_origin[2]],
        [bx_half, 0.00015, bz_half],
        [209, 224, 214, 220],
    );
    b.add_box_cell(
        [block_origin[0], block_origin[1] - 0.0022, block_origin[2]],
        [bx_half, 0.00015, bz_half],
        [209, 224, 214, 220],
    );
    b.end_tissue_layer();

    // Layer 2: Upper & Lower Epidermis Pavement Cells
    b.begin_tissue_layer(HistologicalTissueLayer::UpperAndLowerEpidermis, 2);
    // Rows of brick-shaped epidermal cells
    let nx_cells = 6;
    let nz_cells = 4;
    let cell_w = (bx_half * 2.0) / (nx_cells as f32);
    let cell_d = (bz_half * 2.0) / (nz_cells as f32);

    for ix in 0..nx_cells {
        for iz in 0..nz_cells {
            let cx = block_origin[0] - bx_half + (ix as f32 + 0.5) * cell_w;
            let cz = block_origin[2] - bz_half + (iz as f32 + 0.5) * cell_d;
            // Skip aperture spots for stomata
            if (ix == 2 && iz == 1) || (ix == 4 && iz == 2) {
                continue;
            }
            // Adaxial (upper) epidermis
            b.add_box_cell(
                [cx, block_origin[1] + 0.0018, cz],
                [cell_w * 0.45, 0.00025, cell_d * 0.45],
                [117, 158, 107, 255],
            );
            // Abaxial (lower) epidermis
            b.add_box_cell(
                [cx, block_origin[1] - 0.0018, cz],
                [cell_w * 0.45, 0.00025, cell_d * 0.45],
                [117, 158, 107, 255],
            );
        }
    }
    b.end_tissue_layer();

    // Layer 3: Amphistomatic Stomatal Guard Cells (sunken on both faces)
    b.begin_tissue_layer(
        HistologicalTissueLayer::AmphistomaticStomatalGuardCells,
        3,
    );
    let stomata_locs = [
        ([block_origin[0] - 0.002, block_origin[1] + 0.0017, block_origin[2] - 0.001], true),
        ([block_origin[0] + 0.003, block_origin[1] + 0.0017, block_origin[2] + 0.001], true),
        ([block_origin[0] - 0.002, block_origin[1] - 0.0017, block_origin[2] - 0.001], false),
        ([block_origin[0] + 0.003, block_origin[1] - 0.0017, block_origin[2] + 0.001], false),
    ];
    for (stoma_pos, is_upper) in stomata_locs {
        let col_guard = [92, 138, 82, 255];
        // Paired kidney-shaped guard cells flanking pore
        b.add_ellipsoid(
            [stoma_pos[0] - 0.00035, stoma_pos[1], stoma_pos[2]],
            [0.00035, 0.00025, 0.00065],
            3,
            5,
            col_guard,
        );
        b.add_ellipsoid(
            [stoma_pos[0] + 0.00035, stoma_pos[1], stoma_pos[2]],
            [0.00035, 0.00025, 0.00065],
            3,
            5,
            col_guard,
        );
        // Substomatal respiratory vapour chamber cavity beneath
        let cavity_y = if is_upper { stoma_pos[1] - 0.0004 } else { stoma_pos[1] + 0.0004 };
        b.add_ellipsoid(
            [stoma_pos[0], cavity_y, stoma_pos[2]],
            [0.0006, 0.0003, 0.0008],
            3,
            4,
            [70, 95, 65, 180],
        );
    }
    b.end_tissue_layer();

    // Layer 4: Isobilateral Palisade Mesophyll (Columns under BOTH upper and lower surfaces!)
    b.begin_tissue_layer(
        HistologicalTissueLayer::IsobilateralPalisadeMesophyll,
        4,
    );
    let pal_nx = 8;
    let pal_nz = 5;
    let pal_w = (bx_half * 2.0) / (pal_nx as f32);
    let pal_d = (bz_half * 2.0) / (pal_nz as f32);

    for px in 0..pal_nx {
        for pz in 0..pal_nz {
            let cx = block_origin[0] - bx_half + (px as f32 + 0.5) * pal_w;
            let cz = block_origin[2] - bz_half + (pz as f32 + 0.5) * pal_d;
            // Skip center for vascular bundle and oil gland
            if px >= 3 && px <= 4 && pz >= 2 && pz <= 3 {
                continue;
            }
            // Adaxial palisade columns (2 vertical tiers)
            b.add_cylinder_segment(
                [cx, block_origin[1] + 0.0014, cz],
                [cx, block_origin[1] + 0.0006, cz],
                pal_w * 0.38,
                pal_w * 0.38,
                5,
                [61, 122, 56, 255],
                [51, 107, 46, 255],
            );
            // Abaxial palisade columns (isobilateral symmetry!)
            b.add_cylinder_segment(
                [cx, block_origin[1] - 0.0006, cz],
                [cx, block_origin[1] - 0.0014, cz],
                pal_w * 0.38,
                pal_w * 0.38,
                5,
                [51, 107, 46, 255],
                [61, 122, 56, 255],
            );
        }
    }
    b.end_tissue_layer();

    // Layer 5: Spongy Mesophyll Parenchyma & Intercellular Air Spaces
    b.begin_tissue_layer(HistologicalTissueLayer::SpongyMesophyllAndAirSpaces, 5);
    // Central aerating zone between the two palisade layers
    for sx in 0..6 {
        for sz in 0..4 {
            let cx = block_origin[0] - bx_half * 0.85 + (sx as f32 * 0.0022);
            let cz = block_origin[2] - bz_half * 0.85 + (sz as f32 * 0.0022);
            if sx >= 2 && sx <= 3 && sz >= 1 && sz <= 2 {
                continue; // Reserved for vein and gland
            }
            // Lobed spongy parenchyma cells
            b.add_ellipsoid(
                [cx, block_origin[1] + 0.0001, cz],
                [0.0008, 0.00045, 0.00075],
                3,
                5,
                [97, 148, 92, 230],
            );
        }
    }
    b.end_tissue_layer();

    // Layer 6: Collateral Vascular Bundle (Adaxial Xylem + Abaxial Phloem)
    b.begin_tissue_layer(
        HistologicalTissueLayer::CollateralVascularBundleXylemPhloem,
        1,
    );
    let vein_center = [block_origin[0] - 0.0015, block_origin[1], block_origin[2]];
    // Adaxial (upper) xylem vessels (lignified spiral tracheary elements)
    b.add_cylinder_segment(
        [vein_center[0] - 0.0005, vein_center[1] + 0.0004, vein_center[2] - bz_half * 0.7],
        [vein_center[0] - 0.0005, vein_center[1] + 0.0004, vein_center[2] + bz_half * 0.7],
        0.00045,
        0.00045,
        6,
        [191, 173, 122, 255],
        [191, 173, 122, 255],
    );
    b.add_cylinder_segment(
        [vein_center[0] + 0.0005, vein_center[1] + 0.0004, vein_center[2] - bz_half * 0.7],
        [vein_center[0] + 0.0005, vein_center[1] + 0.0004, vein_center[2] + bz_half * 0.7],
        0.0004,
        0.0004,
        6,
        [191, 173, 122, 255],
        [191, 173, 122, 255],
    );
    // Abaxial (lower) phloem sieve tubes
    b.add_cylinder_segment(
        [vein_center[0], vein_center[1] - 0.0004, vein_center[2] - bz_half * 0.7],
        [vein_center[0], vein_center[1] - 0.0004, vein_center[2] + bz_half * 0.7],
        0.00055,
        0.00055,
        6,
        [175, 140, 95, 255],
        [175, 140, 95, 255],
    );
    b.end_tissue_layer();

    // Layer 7: Sclerenchyma Bundle Sheath Girders (I-beam fiber columns)
    b.begin_tissue_layer(
        HistologicalTissueLayer::SclerenchymaBundleSheathGirders,
        6,
    );
    // Upper girder connecting bundle to upper epidermis
    b.add_box_cell(
        [vein_center[0], block_origin[1] + 0.0011, block_origin[2]],
        [0.0008, 0.00045, bz_half * 0.65],
        [209, 194, 138, 255],
    );
    // Lower girder connecting bundle to lower epidermis
    b.add_box_cell(
        [vein_center[0], block_origin[1] - 0.0011, block_origin[2]],
        [0.0008, 0.00045, bz_half * 0.65],
        [209, 194, 138, 255],
    );
    b.end_tissue_layer();

    // Layer 8: Schizogenous Essential Oil Gland Cavity
    b.begin_tissue_layer(
        HistologicalTissueLayer::SchizogenousEssentialOilGland,
        7,
    );
    // Spherical internal cavity filled with golden 1,8-cineole oil
    let gland_center = [block_origin[0] + 0.0028, block_origin[1], block_origin[2]];
    b.add_ellipsoid(
        gland_center,
        [0.0011, 0.0011, 0.0011],
        6,
        8,
        [242, 214, 89, 240], // Translucent golden eucalyptus oil
    );
    // Surrounding ring of flattened secretory epithelial cells
    b.add_ellipsoid(
        gland_center,
        [0.0013, 0.0013, 0.0013],
        5,
        7,
        [180, 205, 135, 180],
    );
    b.end_tissue_layer();

    let recipe_urn = format!(
        "urn:qualia:botanical:scale4:{species_id}:histology:cutaway={:.2}",
        cutaway_01
    );
    let container_bytes = seal_botanical_organ_10d(
        BotanicalEducationScale::Scale4FoliarVenationAndCellularHistology,
        &b.positions,
        &b.normals,
        &b.colors,
        &b.uvs,
        &b.indices,
        &b.slices,
        &recipe_urn,
        None,
    )?;

    let organ = CompiledBotanicalOrgan10d {
        scale: BotanicalEducationScale::Scale4FoliarVenationAndCellularHistology,
        vertices: b.positions,
        normals: b.normals,
        uvs: b.uvs,
        colors: b.colors,
        material_indices: b.material_indices,
        indices: b.indices,
        tissue_slices: b.slices,
        container_bytes,
    };

    organ.verify_all()?;
    Ok(organ)
}

// ============================================================================
// 8. Disk Asset Serializer for Corymbia aparrerinja
// ============================================================================

/// Write all upgraded educational-grade `.10d` study files to disk for *Corymbia aparrerinja*.
///
/// Upgrades:
/// - `leaf-study-blade.10d`
/// - `leaf-study-midrib.10d`
/// - `bark-study-surface.10d`
/// - `stem-anatomy-wedge.10d`
/// - `flower-dissection.10d`
/// - `gumnut-dissection.10d`
/// - `leaf-anatomy-cutaway.10d`
/// - `whole-organism-roots.10d`
pub fn write_corymbia_educational_assets_to_disk(
    repo_root: &Path,
) -> std::io::Result<usize> {
    let target_dir = repo_root.join("assets/models/corymbia-aparrerinja/10d");
    fs::create_dir_all(&target_dir)?;

    let target_dir_root = repo_root.join("assets/models/corymbia-aparrerinja");

    let organ_scale1 = build_scale1_whole_organism_10d("corymbia-aparrerinja", DevelopmentalStage::Mature)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let organ_scale2 = build_scale2_stem_bark_wedge_10d("corymbia-aparrerinja", 0.0)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let organ_scale3 = build_scale3_floral_and_gumnut_dissection_10d("corymbia-aparrerinja", 0.75, 0.5)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let organ_scale4 = build_scale4_leaf_venation_and_histology_10d("corymbia-aparrerinja", 0.5)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let files_to_write: [(&str, &[u8]); 8] = [
        ("whole-organism-roots.10d", &organ_scale1.container_bytes),
        ("stem-anatomy-wedge.10d", &organ_scale2.container_bytes),
        ("bark-study-surface.10d", &organ_scale2.container_bytes),
        ("flower-dissection.10d", &organ_scale3.container_bytes),
        ("gumnut-dissection.10d", &organ_scale3.container_bytes),
        ("leaf-anatomy-cutaway.10d", &organ_scale4.container_bytes),
        ("leaf-study-blade.10d", &organ_scale4.container_bytes),
        ("leaf-study-midrib.10d", &organ_scale4.container_bytes),
    ];

    let mut total_bytes = 0usize;
    for (filename, bytes) in files_to_write {
        // Write to assets/models/corymbia-aparrerinja/10d/<filename>
        let path_10d = target_dir.join(filename);
        let mut f1 = fs::File::create(&path_10d)?;
        f1.write_all(bytes)?;

        // Also write to assets/models/corymbia-aparrerinja/<filename>
        let path_root = target_dir_root.join(filename);
        let mut f2 = fs::File::create(&path_root)?;
        f2.write_all(bytes)?;

        total_bytes += bytes.len();
    }

    Ok(total_bytes)
}

// ============================================================================
// 9. Comprehensive Verification Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale1_whole_organism_high_density() {
        let organ = build_scale1_whole_organism_10d("corymbia-aparrerinja", DevelopmentalStage::Mature)
            .expect("Scale 1 generator must succeed");

        assert!(
            organ.vertex_count() >= 2500,
            "Scale 1 must produce high density geometry (>= 2500 verts), got {}",
            organ.vertex_count()
        );
        assert!(
            organ.triangle_count() >= 2000,
            "Scale 1 triangle count too low: {}",
            organ.triangle_count()
        );

        // Verify root tissues present
        assert!(
            organ.tissue_layer_slice(HistologicalTissueLayer::PhellogenCorkCambium).is_some(),
            "Scale 1 must contain subterranean root architecture"
        );
        assert!(
            organ.tissue_layer_slice(HistologicalTissueLayer::OuterRhytidome).is_some(),
            "Scale 1 must contain chalk-white outer rhytidome bole"
        );
        assert!(
            organ.tissue_layer_slice(HistologicalTissueLayer::WaxyCuticle).is_some(),
            "Scale 1 must contain pendulous foliage"
        );

        organ.verify_all().expect("Scale 1 .10d container must pass verify_all()");
    }

    #[test]
    fn test_scale2_stem_wedge_dissection_and_peel() {
        let organ_solid = build_scale2_stem_bark_wedge_10d("corymbia-aparrerinja", 0.0)
            .expect("Scale 2 solid wedge must succeed");
        let organ_peeled = build_scale2_stem_bark_wedge_10d("corymbia-aparrerinja", 1.0)
            .expect("Scale 2 peeled wedge must succeed");

        assert!(
            organ_solid.vertex_count() >= 1800,
            "Scale 2 must produce >= 1800 verts, got {}",
            organ_solid.vertex_count()
        );

        // All 6 concentric stem/bark layers must be populated with non-empty slices!
        let stem_layers = [
            HistologicalTissueLayer::OuterRhytidome,
            HistologicalTissueLayer::PhellogenCorkCambium,
            HistologicalTissueLayer::SecondaryPhloemAndKinoDucts,
            HistologicalTissueLayer::VascularCambium,
            HistologicalTissueLayer::SapwoodSecondaryXylem,
            HistologicalTissueLayer::HeartwoodAndPith,
        ];

        for layer in stem_layers {
            let slice = organ_solid.tissue_layer_slice(layer)
                .unwrap_or_else(|| panic!("Missing stem layer: {:?}", layer));
            assert!(slice.vertex_count > 0, "Empty vertex count for {:?}", layer);
            assert!(slice.index_count > 0, "Empty index count for {:?}", layer);
        }

        // Peeled version must have displaced coordinates in the outer rhytidome layer
        let outer_slice = organ_solid
            .tissue_layer_slice(HistologicalTissueLayer::OuterRhytidome)
            .expect("OuterRhytidome slice");
        assert_ne!(
            organ_solid.vertices[outer_slice.vertex_start as usize],
            organ_peeled.vertices[outer_slice.vertex_start as usize],
            "Peeled stem wedge must have displaced vertices"
        );

        organ_solid.verify_all().expect("Solid Scale 2 container must pass verify_all()");
        organ_peeled.verify_all().expect("Peeled Scale 2 container must pass verify_all()");
    }

    #[test]
    fn test_scale3_floral_and_gumnut_dissection() {
        let organ = build_scale3_floral_and_gumnut_dissection_10d("corymbia-aparrerinja", 0.8, 0.5)
            .expect("Scale 3 generator must succeed");

        assert!(
            organ.vertex_count() >= 2200,
            "Scale 3 must produce >= 2200 verts, got {}",
            organ.vertex_count()
        );

        // All 9 floral/fruit layers must be populated with non-empty slices!
        let floral_layers = [
            HistologicalTissueLayer::PeduncleAndPedicel,
            HistologicalTissueLayer::HypanthiumCalyxTube,
            HistologicalTissueLayer::OperculumCalyptra,
            HistologicalTissueLayer::StaminalRingFilaments,
            HistologicalTissueLayer::VersatileAnthersAndPollen,
            HistologicalTissueLayer::GynoeciumStyleStigma,
            HistologicalTissueLayer::OvaryLoculesAndOvules,
            HistologicalTissueLayer::WoodyPericarpAndDehiscingValves,
            HistologicalTissueLayer::FertileWingedSeedsAndChaff,
        ];

        for layer in floral_layers {
            let slice = organ.tissue_layer_slice(layer)
                .unwrap_or_else(|| panic!("Missing floral layer: {:?}", layer));
            assert!(slice.vertex_count > 0, "Empty vertex count for {:?}", layer);
            assert!(slice.index_count > 0, "Empty index count for {:?}", layer);
        }

        organ.verify_all().expect("Scale 3 container must pass verify_all()");
    }

    #[test]
    fn test_scale4_leaf_venation_and_cellular_histology() {
        let organ = build_scale4_leaf_venation_and_histology_10d("corymbia-aparrerinja", 0.5)
            .expect("Scale 4 generator must succeed");

        assert!(
            organ.vertex_count() >= 2000,
            "Scale 4 must produce >= 2000 verts, got {}",
            organ.vertex_count()
        );

        // All 8 foliar/cellular layers must be populated with non-empty slices!
        let foliar_layers = [
            HistologicalTissueLayer::WaxyCuticle,
            HistologicalTissueLayer::UpperAndLowerEpidermis,
            HistologicalTissueLayer::AmphistomaticStomatalGuardCells,
            HistologicalTissueLayer::IsobilateralPalisadeMesophyll,
            HistologicalTissueLayer::SpongyMesophyllAndAirSpaces,
            HistologicalTissueLayer::CollateralVascularBundleXylemPhloem,
            HistologicalTissueLayer::SclerenchymaBundleSheathGirders,
            HistologicalTissueLayer::SchizogenousEssentialOilGland,
        ];

        for layer in foliar_layers {
            let slice = organ.tissue_layer_slice(layer)
                .unwrap_or_else(|| panic!("Missing foliar layer: {:?}", layer));
            assert!(slice.vertex_count > 0, "Empty vertex count for {:?}", layer);
            assert!(slice.index_count > 0, "Empty index count for {:?}", layer);
        }

        organ.verify_all().expect("Scale 4 container must pass verify_all()");
    }

    #[test]
    fn test_all_23_tissue_layers_covered_with_non_empty_slices() {
        let organ1 = build_scale1_whole_organism_10d("corymbia-aparrerinja", DevelopmentalStage::Mature).unwrap();
        let organ2 = build_scale2_stem_bark_wedge_10d("corymbia-aparrerinja", 0.0).unwrap();
        let organ3 = build_scale3_floral_and_gumnut_dissection_10d("corymbia-aparrerinja", 0.5, 0.5).unwrap();
        let organ4 = build_scale4_leaf_venation_and_histology_10d("corymbia-aparrerinja", 0.5).unwrap();

        let all_layers = [
            HistologicalTissueLayer::OuterRhytidome,
            HistologicalTissueLayer::PhellogenCorkCambium,
            HistologicalTissueLayer::SecondaryPhloemAndKinoDucts,
            HistologicalTissueLayer::VascularCambium,
            HistologicalTissueLayer::SapwoodSecondaryXylem,
            HistologicalTissueLayer::HeartwoodAndPith,
            HistologicalTissueLayer::PeduncleAndPedicel,
            HistologicalTissueLayer::HypanthiumCalyxTube,
            HistologicalTissueLayer::OperculumCalyptra,
            HistologicalTissueLayer::StaminalRingFilaments,
            HistologicalTissueLayer::VersatileAnthersAndPollen,
            HistologicalTissueLayer::GynoeciumStyleStigma,
            HistologicalTissueLayer::OvaryLoculesAndOvules,
            HistologicalTissueLayer::WoodyPericarpAndDehiscingValves,
            HistologicalTissueLayer::FertileWingedSeedsAndChaff,
            HistologicalTissueLayer::WaxyCuticle,
            HistologicalTissueLayer::UpperAndLowerEpidermis,
            HistologicalTissueLayer::AmphistomaticStomatalGuardCells,
            HistologicalTissueLayer::IsobilateralPalisadeMesophyll,
            HistologicalTissueLayer::SpongyMesophyllAndAirSpaces,
            HistologicalTissueLayer::CollateralVascularBundleXylemPhloem,
            HistologicalTissueLayer::SclerenchymaBundleSheathGirders,
            HistologicalTissueLayer::SchizogenousEssentialOilGland,
        ];

        assert_eq!(all_layers.len(), 23);

        for layer in all_layers {
            let found = organ1.tissue_layer_slice(layer)
                .or_else(|| organ2.tissue_layer_slice(layer))
                .or_else(|| organ3.tissue_layer_slice(layer))
                .or_else(|| organ4.tissue_layer_slice(layer));

            assert!(
                found.is_some(),
                "Layer {:?} must be populated in at least one educational scale",
                layer
            );
            let slice = found.unwrap();
            assert!(slice.vertex_count > 0, "Layer {:?} vertex count must be > 0", layer);
            assert!(slice.index_count > 0, "Layer {:?} index count must be > 0", layer);
            assert!(!slice.plant_ontology_id.is_empty(), "Layer {:?} must have a PO ID", layer);
        }
    }

    #[test]
    fn test_leaf_vascular_skeleton_murray_law_conductance_positive() {
        let profile = LeafEconomicsProfile::eucalyptus_sclerophyll();
        let skeleton = LeafVascularSkeleton::new(profile, 1.25);

        assert!(skeleton.edge_count > 0, "Vascular skeleton must contain edges");
        for i in 0..skeleton.edge_count {
            let edge = skeleton.edges[i];
            assert!(
                edge.radius_mm > 0.0,
                "Edge {} radius must be positive, got {}",
                i,
                edge.radius_mm
            );
            assert!(
                edge.conductance > 0.0,
                "Edge {} Murray's Law hydraulic conductance must be positive, got {}",
                i,
                edge.conductance
            );
        }
    }
}
