//! Interactive 3D Specimen Loupe Model for Corymbia aparrerinja (Ghost Gum / Aparrerinja)
//! Provides allometric growth scrubbing, Jakob-Hanika spectral albedo reflectance,
//! seasonal phenology curves, biocultural Arrernte custodianship metadata,
//! .10d anatomical organ submesh definitions, and EONT1 living ontology binding.

use serde::{Deserialize, Serialize};

use qualia_core_db::container_10d::entity_ontology_section::{
    AffordanceBinaryRecord, DevelopmentalKeyframeRecord, EntityOntologyMiniHeader, MAGIC_EONT,
};
pub use qualia_core_db::render::scene_graph::ontology_binding::DevelopmentalStage;
use qualia_core_db::render::scene_graph::ontology_binding::{
    dispatch_affordance, AffordanceDispatchOutcome, LivingOntologyState,
};

/// Seasonal phenology profile for the Ghost Gum across the Central Australian arid cycle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GhostGumPhenology {
    pub month: u8,
    pub phase_name: &'static str,
    pub flowering_intensity: f32,
    pub foliar_growth_rate: f32,
    pub nectar_yield_g_per_day: f32,
    pub bark_albedo_visible: f32,
    pub leaf_transpiration_permill: f32,
    pub avian_foragers: &'static str,
}

/// Catalog of `.10d` anatomical sub-meshes composing the multi-organ digital twin of *Corymbia aparrerinja*.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum GhostGumOrganSubmesh {
    /// Smooth chalk-white trunk and primary boles (`chalk-bark`).
    ChalkBark = 0,
    /// Structural scaffold of primary, secondary, and tertiary limbs (`branchwork`).
    Branchwork = 1,
    /// Spreading, open, irregular crown profile (`irregular-canopy`).
    IrregularCanopy = 2,
    /// Alternating, pendulous falcate adult leaf blades (`falcate-foliage`).
    FalcateFoliage = 3,
    /// Cream-coloured summer clavate flower buds (`cream-buds`).
    CreamBuds = 4,
    /// Fire-resilient epicormic leaf clusters sprouting from bole dormant buds (`epicormic-shoots`).
    EpicormicShoots = 5,
    /// High-resolution microscopic anatomical leaf lamina `.10d` study (`leaf-study-blade.10d`).
    LeafStudyBlade = 6,
    /// Primary longitudinal vascular midrib and intramarginal vein study (`leaf-study-midrib.10d`).
    LeafStudyMidrib = 7,
    /// Numerous exserted white/cream stamen clusters (`flower-study-stamens.10d`).
    FlowerStudyStamens = 8,
    /// Urceolate / cup-shaped woody fruit capsule study (`gumnut-study-capsule.10d`).
    GumnutStudyCapsule = 9,
}

impl GhostGumOrganSubmesh {
    /// All 10 anatomical organ sub-meshes.
    pub const ALL: [Self; 10] = [
        Self::ChalkBark,
        Self::Branchwork,
        Self::IrregularCanopy,
        Self::FalcateFoliage,
        Self::CreamBuds,
        Self::EpicormicShoots,
        Self::LeafStudyBlade,
        Self::LeafStudyMidrib,
        Self::FlowerStudyStamens,
        Self::GumnutStudyCapsule,
    ];

    /// The exact canonical asset name string matching `.10d` section headers.
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::ChalkBark => "chalk-bark",
            Self::Branchwork => "branchwork",
            Self::IrregularCanopy => "irregular-canopy",
            Self::FalcateFoliage => "falcate-foliage",
            Self::CreamBuds => "cream-buds",
            Self::EpicormicShoots => "epicormic-shoots",
            Self::LeafStudyBlade => "leaf-study-blade.10d",
            Self::LeafStudyMidrib => "leaf-study-midrib.10d",
            Self::FlowerStudyStamens => "flower-study-stamens.10d",
            Self::GumnutStudyCapsule => "gumnut-study-capsule.10d",
        }
    }

    /// Returns `true` if this organ is a high-resolution close-up loupe study mesh.
    #[inline]
    pub const fn is_high_detail_study(&self) -> bool {
        matches!(
            self,
            Self::LeafStudyBlade
                | Self::LeafStudyMidrib
                | Self::FlowerStudyStamens
                | Self::GumnutStudyCapsule
        )
    }

    /// Resolves an organ sub-mesh from its canonical or file name.
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "chalk-bark" => Some(Self::ChalkBark),
            "branchwork" => Some(Self::Branchwork),
            "irregular-canopy" => Some(Self::IrregularCanopy),
            "falcate-foliage" => Some(Self::FalcateFoliage),
            "cream-buds" => Some(Self::CreamBuds),
            "epicormic-shoots" => Some(Self::EpicormicShoots),
            "leaf-study-blade.10d" | "leaf-study-blade" => Some(Self::LeafStudyBlade),
            "leaf-study-midrib.10d" | "leaf-study-midrib" => Some(Self::LeafStudyMidrib),
            "flower-study-stamens.10d" | "flower-study-stamens" => Some(Self::FlowerStudyStamens),
            "gumnut-study-capsule.10d" | "gumnut-study-capsule" => Some(Self::GumnutStudyCapsule),
            _ => None,
        }
    }
}

/// Detailed 5-stage botanical ontogenetic classification for eucalypts and corymbias.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BotanicalDevelopmentalStage {
    /// Cotyledonous seedling / early lignotuber establishment.
    Seedling = 0,
    /// Vigorous vertical sapling with juvenile opposite leaves.
    Sapling = 1,
    /// Morphologically mature adult canopy with alternate falcate leaves and regular flowering.
    MatureCanopy = 2,
    /// Mature post-reproductive tree experiencing crown retraction and branch shed.
    Senescent = 3,
    /// Veteran ancient hollow-bearing tree providing critical ecological arboreal fauna habitat.
    AncientHollowBearing = 4,
}

impl From<BotanicalDevelopmentalStage> for DevelopmentalStage {
    #[inline]
    fn from(stage: BotanicalDevelopmentalStage) -> Self {
        match stage {
            BotanicalDevelopmentalStage::Seedling => Self::SeedOrSpore,
            BotanicalDevelopmentalStage::Sapling => Self::Juvenile,
            BotanicalDevelopmentalStage::MatureCanopy => Self::Adult,
            BotanicalDevelopmentalStage::Senescent => Self::Senescent,
            BotanicalDevelopmentalStage::AncientHollowBearing => Self::Adult,
        }
    }
}

/// Action predicate hashes for biocultural Arrernte custodianship affordances.
pub const ACTION_CULTURAL_COOL_BURN: u64 = 0xAA01_BEEF_C001_B001;
pub const ACTION_SEED_HARVEST: u64 = 0xAA02_BEEF_C002_B002;
pub const ACTION_CUSTODIAL_GREETING: u64 = 0xAA03_BEEF_C003_B003;
pub const ACTION_HOLLOW_INSPECTION: u64 = 0xAA04_BEEF_C004_B004;

/// Actor capability masks required for custodial affordances.
pub const CAPABILITY_CUSTODIAL_FIRE_CARE: u32 = 1 << 0;
pub const CAPABILITY_BOTANICAL_HARVEST: u32 = 1 << 1;
pub const CAPABILITY_CEREMONIAL_PRESENCE: u32 = 1 << 2;
pub const CAPABILITY_CANOPY_SURVEY: u32 = 1 << 3;

/// The Specimen Loupe Data Provider for Corymbia aparrerinja.
pub struct GhostGumSpecimenLoupe;

impl GhostGumSpecimenLoupe {
    /// Taxon IRI
    pub const TAXON_IRI: &'static str = "urn:qualia:taxon:corymbia_aparrerinja";
    /// Arrernte Name
    pub const ARRERNTE_NAME: &'static str = "Aparrerinja";
    /// Scientific Name
    pub const SCIENTIFIC_NAME: &'static str = "Corymbia aparrerinja K.D.Hill & L.A.S.Johnson";

    /// Maximum biological limits from AusTraits profile
    pub const MAX_TREE_HEIGHT_M: f32 = 20.0;
    pub const MAX_STEM_DIAMETER_M: f32 = 1.10;
    pub const MAX_ROOT_DEPTH_M: f32 = 18.0;

    /// Jakob-Hanika spectral reflectance coefficients for trunk bark:
    /// High-albedo thermal protection reflecting 74.2% of visible light,
    /// keeping living cambium <= 36.8°C at 45°C ambient desert air.
    pub const BARK_ALBEDO_C0: f32 = 0.742;
    pub const BARK_ALBEDO_C1: f32 = 0.0185;
    pub const BARK_ALBEDO_C2: f32 = -0.0092;

    /// Evaluates seasonal phenology for a given month (1 = Jan, 12 = Dec).
    pub fn phenology_for_month(month: u8) -> GhostGumPhenology {
        match month {
            12 | 1 | 2 => GhostGumPhenology {
                month,
                phase_name: "SummerFlowering",
                flowering_intensity: 0.95,
                foliar_growth_rate: 0.40,
                nectar_yield_g_per_day: 48.0,
                bark_albedo_visible: 0.76,
                leaf_transpiration_permill: 18.0,
                avian_foragers: "Honeyeaters, Budgerigars, Red-tailed Black Cockatoos",
            },
            3..=5 => GhostGumPhenology {
                month,
                phase_name: "FruitingAndSeedShed",
                flowering_intensity: 0.05,
                foliar_growth_rate: 0.15,
                nectar_yield_g_per_day: 2.0,
                bark_albedo_visible: 0.72,
                leaf_transpiration_permill: 12.0,
                avian_foragers: "Seed-eating finches, Galahs",
            },
            6..=8 => GhostGumPhenology {
                month,
                phase_name: "WinterQuiescence",
                flowering_intensity: 0.0,
                foliar_growth_rate: 0.02,
                nectar_yield_g_per_day: 0.0,
                bark_albedo_visible: 0.78,
                leaf_transpiration_permill: 5.0,
                avian_foragers: "Resident insectivores sheltering in hollows",
            },
            _ => GhostGumPhenology {
                month,
                phase_name: "SpringBudInitiation",
                flowering_intensity: 0.25,
                foliar_growth_rate: 0.75,
                nectar_yield_g_per_day: 12.0,
                bark_albedo_visible: 0.74,
                leaf_transpiration_permill: 15.0,
                avian_foragers: "Desert woodswallows, Honeyeaters",
            },
        }
    }

    /// Evaluates allometric tree metrics given a normalized growth slider [0.0..1.0].
    pub fn allometric_growth(slider: f32) -> (f32, f32, f32) {
        let t = slider.clamp(0.0, 1.0);
        // Non-linear allometric curve: height, diameter, taproot depth
        let height_m = 0.5 + t * (Self::MAX_TREE_HEIGHT_M - 0.5);
        let diameter_m = 0.02 + t.powf(1.5) * (Self::MAX_STEM_DIAMETER_M - 0.02);
        let root_depth_m = 1.0 + t.powf(0.8) * (Self::MAX_ROOT_DEPTH_M - 1.0);
        (height_m, diameter_m, root_depth_m)
    }

    /// Evaluates live EONT1 runtime metabolic and ontogenetic state for *Corymbia aparrerinja*
    /// given a seasonal calendar month (1..=12) and normalized growth slider [0.0..1.0].
    pub fn evaluate_living_ontology_state(month: u8, growth_slider: f32) -> LivingOntologyState {
        let pheno = Self::phenology_for_month(month);
        let progress = growth_slider.clamp(0.0, 1.0);
        let (height_m, diam_m, _root_m) = Self::allometric_growth(progress);

        let stage = if progress < 0.15 {
            DevelopmentalStage::SeedOrSpore
        } else if progress < 0.50 {
            DevelopmentalStage::Juvenile
        } else if progress < 0.85 {
            DevelopmentalStage::Adult
        } else {
            DevelopmentalStage::Senescent
        };

        // Arid hydration: modulated by winter rains vs summer heat
        let hydration = ((850.0 - pheno.leaf_transpiration_permill * 18.0)
            .clamp(250.0, 1000.0)) as u16;

        // Energy level: driven by foliar growth rate and flowering
        let energy = ((600.0 + (pheno.foliar_growth_rate + pheno.flowering_intensity) * 200.0)
            .clamp(100.0, 1000.0)) as u16;

        // Structural damage: hollow formation occurs in senescent / ancient trees
        let damage = if progress >= 0.85 {
            ((progress - 0.85) * 1200.0).clamp(0.0, 350.0) as u16
        } else {
            0
        };

        // Mechanical wear: accumulates monotonically with age
        let wear = (progress * 350.0) as u16;

        // Ghost gum allometry: Huxley power law y = b * x^k
        // Base trunk diameter b calibrated against maximum stem diameter
        let b = (diam_m / height_m.max(0.1)).clamp(0.05, 1.5);
        let k = 1.35; // Allometric scaling exponent for eucalypt wood volume

        LivingOntologyState {
            stage,
            growth_progress_01: progress,
            age_days: progress * 365.0 * 200.0, // Lifespan ~200 years
            energy_permill: energy,
            damage_permill: damage,
            hydration_permill: hydration,
            wear_permill: wear,
            active_state_id: stage as u32,
            allometry_base_b: b,
            allometry_exponent_k: k,
        }
    }

    /// Dispatches a biocultural custodial affordance against *Corymbia aparrerinja*.
    pub fn dispatch_custodial_affordance(
        state: &mut LivingOntologyState,
        affordances: &[AffordanceBinaryRecord],
        action_hash: u64,
        actor_capabilities: u32,
        deontic_permitted: bool,
    ) -> Option<AffordanceDispatchOutcome> {
        dispatch_affordance(
            state,
            affordances,
            action_hash,
            actor_capabilities,
            deontic_permitted,
        )
    }

    /// Default custodial affordance records for *Corymbia aparrerinja*.
    pub fn default_custodial_affordances() -> [AffordanceBinaryRecord; 4] {
        [
            // 1. Cultural Cool Burn: smoke cleansing, fuel load reduction, canopy protection
            AffordanceBinaryRecord {
                action_id: ACTION_CULTURAL_COOL_BURN,
                required_capability_hash: CAPABILITY_CUSTODIAL_FIRE_CARE as u64,
                emitted_topic_hash: 0xC001_B001_E001_0001,
                category: 1, // Cultural / Ecological Care
                min_observer_status: 1,
                min_energy_permill: 200,
                max_damage_permill: 700,
                min_hydration_permill: 200,
                max_wear_permill: 900,
                energy_delta: 50,
                damage_delta: -30, // Removes parasite load
                hydration_delta: 0,
                wear_delta: 10,
                reserved_u16: 0,
                animation_clip_id: 101,
            },
            // 2. Seed Harvest: gathering viable seed capsules for propagation
            AffordanceBinaryRecord {
                action_id: ACTION_SEED_HARVEST,
                required_capability_hash: CAPABILITY_BOTANICAL_HARVEST as u64,
                emitted_topic_hash: 0xC002_B002_E002_0002,
                category: 2, // Botanical Harvest
                min_observer_status: 0,
                min_energy_permill: 400,
                max_damage_permill: 800,
                min_hydration_permill: 150,
                max_wear_permill: 950,
                energy_delta: -20,
                damage_delta: 5,
                hydration_delta: 0,
                wear_delta: 15,
                reserved_u16: 0,
                animation_clip_id: 102,
            },
            // 3. Custodial Greeting: respectful acknowledgement in Arrernte country
            AffordanceBinaryRecord {
                action_id: ACTION_CUSTODIAL_GREETING,
                required_capability_hash: CAPABILITY_CEREMONIAL_PRESENCE as u64,
                emitted_topic_hash: 0xC003_B003_E003_0003,
                category: 3, // Ceremonial Presence
                min_observer_status: 0,
                min_energy_permill: 0,
                max_damage_permill: 1000,
                min_hydration_permill: 0,
                max_wear_permill: 1000,
                energy_delta: 10,
                damage_delta: 0,
                hydration_delta: 0,
                wear_delta: 0,
                reserved_u16: 0,
                animation_clip_id: 103,
            },
            // 4. Hollow Inspection: non-invasive inspection of arboreal fauna nest hollows
            AffordanceBinaryRecord {
                action_id: ACTION_HOLLOW_INSPECTION,
                required_capability_hash: CAPABILITY_CANOPY_SURVEY as u64,
                emitted_topic_hash: 0xC004_B004_E004_0004,
                category: 4, // Fauna Monitoring
                min_observer_status: 0,
                min_energy_permill: 100,
                max_damage_permill: 950,
                min_hydration_permill: 100,
                max_wear_permill: 950,
                energy_delta: 0,
                damage_delta: 0,
                hydration_delta: 0,
                wear_delta: 5,
                reserved_u16: 0,
                animation_clip_id: 104,
            },
        ]
    }

    /// Default ontogenetic keyframe records across the life cycle of *Corymbia aparrerinja*.
    pub fn default_developmental_keyframes() -> [DevelopmentalKeyframeRecord; 4] {
        [
            // Keyframe 0: Seedling (age 90 days)
            DevelopmentalKeyframeRecord {
                age_days: 90.0,
                stage: DevelopmentalStage::SeedOrSpore as u8,
                reserved_u8: 0,
                reserved_u16: 0,
                allometric_crown_scale: 0.12,
                allometric_trunk_scale: 0.08,
                lai_multiplier: 0.25,
                reserved_u32: 0,
                mesh_asset_hash: 0x0001,
            },
            // Keyframe 1: Sapling (age 1,800 days ~ 5 years)
            DevelopmentalKeyframeRecord {
                age_days: 1800.0,
                stage: DevelopmentalStage::Juvenile as u8,
                reserved_u8: 0,
                reserved_u16: 0,
                allometric_crown_scale: 0.45,
                allometric_trunk_scale: 0.35,
                lai_multiplier: 0.70,
                reserved_u32: 0,
                mesh_asset_hash: 0x0002,
            },
            // Keyframe 2: Mature Canopy (age 18,000 days ~ 50 years)
            DevelopmentalKeyframeRecord {
                age_days: 18000.0,
                stage: DevelopmentalStage::Adult as u8,
                reserved_u8: 0,
                reserved_u16: 0,
                allometric_crown_scale: 1.0,
                allometric_trunk_scale: 1.0,
                lai_multiplier: 1.0,
                reserved_u32: 0,
                mesh_asset_hash: 0x0003,
            },
            // Keyframe 3: Ancient Hollow-Bearing (age 55,000 days ~ 150 years)
            DevelopmentalKeyframeRecord {
                age_days: 55000.0,
                stage: DevelopmentalStage::Senescent as u8,
                reserved_u8: 0,
                reserved_u16: 0,
                allometric_crown_scale: 0.88,
                allometric_trunk_scale: 1.25,
                lai_multiplier: 0.65,
                reserved_u32: 0,
                mesh_asset_hash: 0x0004,
            },
        ]
    }

    /// Default EntityOntologyMiniHeader for *Corymbia aparrerinja*.
    pub fn default_ontology_mini_header() -> EntityOntologyMiniHeader {
        EntityOntologyMiniHeader {
            magic: MAGIC_EONT,
            version: 1,
            stage: DevelopmentalStage::Adult as u8,
            reserved_u8: 0,
            entity_did_hash: 0xCAFE_BEEF_C001_0001,
            taxon_iri_hash: 0x9988_7766_5544_3322,
            energy_permill: 850,
            damage_permill: 20,
            hydration_permill: 780,
            wear_permill: 120,
            affordance_count: 4,
            reserved_u32: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ghost_gum_phenology() {
        let summer = GhostGumSpecimenLoupe::phenology_for_month(1);
        assert_eq!(summer.phase_name, "SummerFlowering");
        assert!(summer.flowering_intensity > 0.9);

        let winter = GhostGumSpecimenLoupe::phenology_for_month(7);
        assert_eq!(winter.phase_name, "WinterQuiescence");
        assert_eq!(winter.flowering_intensity, 0.0);
    }

    #[test]
    fn test_allometric_growth() {
        let (h_seedling, _, _) = GhostGumSpecimenLoupe::allometric_growth(0.0);
        assert!((h_seedling - 0.5).abs() < 1e-4);

        let (h_mature, d_mature, r_mature) = GhostGumSpecimenLoupe::allometric_growth(1.0);
        assert!((h_mature - 20.0).abs() < 1e-4);
        assert!((d_mature - 1.10).abs() < 1e-4);
        assert!((r_mature - 18.0).abs() < 1e-4);
    }

    #[test]
    fn test_ghost_gum_organ_submeshes() {
        assert_eq!(GhostGumOrganSubmesh::ALL.len(), 10);
        assert_eq!(GhostGumOrganSubmesh::ChalkBark.as_str(), "chalk-bark");
        assert_eq!(GhostGumOrganSubmesh::Branchwork.as_str(), "branchwork");
        assert_eq!(GhostGumOrganSubmesh::IrregularCanopy.as_str(), "irregular-canopy");
        assert_eq!(GhostGumOrganSubmesh::FalcateFoliage.as_str(), "falcate-foliage");
        assert_eq!(GhostGumOrganSubmesh::CreamBuds.as_str(), "cream-buds");
        assert_eq!(GhostGumOrganSubmesh::EpicormicShoots.as_str(), "epicormic-shoots");
        assert_eq!(GhostGumOrganSubmesh::LeafStudyBlade.as_str(), "leaf-study-blade.10d");
        assert_eq!(GhostGumOrganSubmesh::LeafStudyMidrib.as_str(), "leaf-study-midrib.10d");
        assert_eq!(GhostGumOrganSubmesh::FlowerStudyStamens.as_str(), "flower-study-stamens.10d");
        assert_eq!(GhostGumOrganSubmesh::GumnutStudyCapsule.as_str(), "gumnut-study-capsule.10d");

        assert!(GhostGumOrganSubmesh::LeafStudyBlade.is_high_detail_study());
        assert!(!GhostGumOrganSubmesh::ChalkBark.is_high_detail_study());

        assert_eq!(
            GhostGumOrganSubmesh::from_str("chalk-bark"),
            Some(GhostGumOrganSubmesh::ChalkBark)
        );
        assert_eq!(
            GhostGumOrganSubmesh::from_str("flower-study-stamens.10d"),
            Some(GhostGumOrganSubmesh::FlowerStudyStamens)
        );
    }

    #[test]
    fn test_evaluate_living_ontology_state() {
        let state_young = GhostGumSpecimenLoupe::evaluate_living_ontology_state(1, 0.05);
        assert_eq!(state_young.stage, DevelopmentalStage::SeedOrSpore);
        assert!(state_young.growth_progress_01 < 0.1);
        assert_eq!(state_young.damage_permill, 0);

        let state_mature = GhostGumSpecimenLoupe::evaluate_living_ontology_state(1, 0.75);
        assert_eq!(state_mature.stage, DevelopmentalStage::Adult);
        assert!(state_mature.energy_permill > 500);

        let state_ancient = GhostGumSpecimenLoupe::evaluate_living_ontology_state(7, 0.95);
        assert_eq!(state_ancient.stage, DevelopmentalStage::Senescent);
        assert!(state_ancient.damage_permill > 0); // Hollow formation
    }

    #[test]
    fn test_dispatch_custodial_affordance() {
        let mut state = GhostGumSpecimenLoupe::evaluate_living_ontology_state(1, 0.70);
        let affordances = GhostGumSpecimenLoupe::default_custodial_affordances();

        // 1. Without deontic permission -> denied
        let outcome_denied = GhostGumSpecimenLoupe::dispatch_custodial_affordance(
            &mut state,
            &affordances,
            ACTION_CULTURAL_COOL_BURN,
            CAPABILITY_CUSTODIAL_FIRE_CARE,
            false,
        );
        assert!(outcome_denied.is_none());

        // 2. Without required capability -> denied
        let outcome_nocap = GhostGumSpecimenLoupe::dispatch_custodial_affordance(
            &mut state,
            &affordances,
            ACTION_CULTURAL_COOL_BURN,
            0, // Missing fire care capability
            true,
        );
        assert!(outcome_nocap.is_none());

        // 3. With capability and permission -> success
        let initial_energy = state.energy_permill;
        let outcome = GhostGumSpecimenLoupe::dispatch_custodial_affordance(
            &mut state,
            &affordances,
            ACTION_CULTURAL_COOL_BURN,
            CAPABILITY_CUSTODIAL_FIRE_CARE,
            true,
        );
        assert!(outcome.is_some());
        let res = outcome.unwrap();
        assert_eq!(res.action_id, ACTION_CULTURAL_COOL_BURN);
        assert_eq!(res.animation_clip_id, 101);
        assert_eq!(state.energy_permill, initial_energy + 50);
    }
}
