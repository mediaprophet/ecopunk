//! 16-KB GPU ConstituentLUT Entry (.q42 -> GPU Uniform/Storage Buffer)
//! Represents a precompiled constituent material or species property entry.
//! 256 entries x 64 bytes = 16 KB total, fitting in GPU L1 cache for O(1) lookups.

use bytemuck::{Pod, Zeroable};

/// Fixed 64-Byte Material / Species Property Entry compiled from .q42 NQuins.
#[repr(C, align(64))]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Q42ConstituentLutEntry {
    /// 0x00..0x08: FNV-1a hash of the .q42 entity IRI (e.g. `q_hash("geo:QuartzSand")`)
    pub nquin_subject_hash: u64,
    /// 0x08..0x0A: Specific gravity relative to water (half-precision float bits, e.g. 2.65 for quartz)
    pub specific_gravity_half: u16,
    /// 0x0A..0x0C: [Porosity φ_p, Darcy log-permeability κ]
    pub porosity_permeability: [u8; 2],
    /// 0x0C..0x0E: Mohr-Coulomb cohesion c in kPa (half-precision float bits)
    pub cohesion_kpa_half: u16,
    /// 0x0E..0x10: Internal friction angle / Angle of repose in radians (half-precision float bits)
    pub friction_repose_rad_half: u16,
    /// 0x10..0x14: Herschel-Bulkley fluid/slurry [Viscosity η, Yield stress τ_y] (half-precision float bits)
    pub viscosity_yield_half: [u16; 2],
    /// 0x14..0x18: Krumbein phi moments [mean_phi, std_phi, sphericity, imbrication]
    pub grain_phi_stats: [i8; 4],
    /// 0x18..0x1C: Elastic moduli [Young's Modulus E, Poisson's Ratio ν] (half-precision float bits)
    pub elastic_moduli_half: [u16; 2],
    /// 0x1C..0x1E: Specific acoustic impedance Z in Rayls / 100 (half-precision float bits)
    pub acoustic_impedance_half: u16,
    /// 0x1E..0x1F: Nutrient index for flora morphogenesis (0..255)
    pub biotic_fertility: u8,
    /// 0x1F..0x20: Epistemic10D density threshold for liquefaction
    pub phase_change_thresh: u8,
    /// 0x20..0x30: 16-band EMF reflectance spectrum (400nm - 1000nm, σ-axis)
    pub spectral_spd_curve: [u8; 16],
    /// 0x30..0x40: Reserved padding to exact 64 bytes
    pub _reserved: [u8; 16],
}

impl Default for Q42ConstituentLutEntry {
    fn default() -> Self {
        Self {
            nquin_subject_hash: 0,
            specific_gravity_half: 0x414D, // ~2.65 in half-precision
            porosity_permeability: [76, 128], // ~30% porosity
            cohesion_kpa_half: 0,
            friction_repose_rad_half: 0x3914, // ~0.57 rad (33 degrees)
            viscosity_yield_half: [0, 0],
            grain_phi_stats: [1, 1, 80, 0],
            elastic_moduli_half: [0x5000, 0x3400], // E ~ 30 GPa, ν ~ 0.25
            acoustic_impedance_half: 0x4800,
            biotic_fertility: 10,
            phase_change_thresh: 200,
            spectral_spd_curve: [180; 16],
            _reserved: [0; 16],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lut_entry_size_and_alignment() {
        assert_eq!(std::mem::size_of::<Q42ConstituentLutEntry>(), 64);
        assert_eq!(std::mem::align_of::<Q42ConstituentLutEntry>(), 64);
    }
}
