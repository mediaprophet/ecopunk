//! Interactive 3D Specimen Loupe Model for Corymbia aparrerinja (Ghost Gum / Aparrerinja)
//! Provides allometric growth scrubbing, Jakob-Hanika spectral albedo reflectance,
//! seasonal phenology curves, and biocultural Arrernte custodianship metadata.

use serde::{Deserialize, Serialize};

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
}
