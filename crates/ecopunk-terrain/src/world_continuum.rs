//! Multi-Kilometer Contiguous `.m64` Terrain Substrate & `HermiteCorridorSegment` Road Network (`world_continuum.rs`).
//!
//! Epic 4 of the Ecopunk Biological Education Assets and Explorable World Plan.
//! Replaces toy diorama boxes and 1D road rails with a real metric, geodetically anchored
//! regional world corridor:
//! - Metric world-space coordinate system (ENU meters relative to Talbingo Dam origin
//!   `35.5880°S, 148.3180°E`, elevation `400 m – 1,800 m AHD`).
//! - Continuous 26.5 km `HermiteCorridorSegment` road, fire-trail & river network connecting
//!   8 high-detail ecological study sites across Kosciuszko National Park and regional portals.
//! - Hilbert-indexed $64 \times 64$ `.m64` DEM chunk directory entries with 4-tier hybrid LOD streaming.
//! - Zero-heap terrain elevation, surface normal, and material classification queries.
//! - Hydro-enforced water surfaces (Blowering Reservoir, Jounama Creek, Yarrangobilly Thermal Pool,
//!   Three Mile Dam, Sydney Creek, Noosa Pacific swell).
//! - Silurian limestone karst cave ceiling overhang queries.

use qualia_core_db::q42::m64_matter::{M64ChunkDirectoryEntry, M64_MODE_STRATA};
use qualia_geospatial::corridor::{CorridorKind, HermiteCorridorSegment, SplineControlPoint};
use qualia_geospatial::hilbert::xy_to_hilbert;
use qualia_sim::geomechanics::SoilMechanicsProfile;

// ============================================================================
// 1. Study Sites & Geological Descriptors
// ============================================================================

/// Canonical identifier for the 8 interconnected high-detail ecological study sites
/// along the metric world corridor.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StudySiteId {
    /// km 0.0: Talbingo Dam wall & The Pines foreshore (410.0 m AHD).
    ThePinesForeshore = 0,
    /// km 1.8: Blowering foreshore alluvial flats & river gravel bars (405.0 m AHD).
    HumesCrossing = 1,
    /// km 4.2: Jounama Creek montane wet sclerophyll & Bogong moth granite tors (460.0 m AHD).
    JounamaCreekValley = 2,
    /// km 9.5: Yarrangobilly karst gorge, cave arch & 27°C thermal pool (940.0 m AHD).
    YarrangobillyKarstGorge = 3,
    /// km 13.0: O'Hares rest area tall *Eucalyptus delegatensis* montane forest (1180.0 m AHD).
    OHaresAlpineAshRestArea = 4,
    /// km 17.5: Three Mile Dam subalpine *Eucalyptus pauciflora* frost hollow & bog (1460.0 m AHD).
    ThreeMileDamSnowGumBog = 5,
    /// km 22.0 portal: Triassic Hawkesbury sandstone ledges & *Angophora costata* (185.0 m AHD).
    SydneyHawkesburySandstoneGully = 6,
    /// km 26.5 portal: Coastal basalt headland & *Pandanus tectorius* (32.0 m AHD).
    NoosaCoastalPandanusHeadland = 7,
}

impl StudySiteId {
    /// All 8 canonical study sites.
    pub const ALL: [Self; 8] = [
        Self::ThePinesForeshore,
        Self::HumesCrossing,
        Self::JounamaCreekValley,
        Self::YarrangobillyKarstGorge,
        Self::OHaresAlpineAshRestArea,
        Self::ThreeMileDamSnowGumBog,
        Self::SydneyHawkesburySandstoneGully,
        Self::NoosaCoastalPandanusHeadland,
    ];

    /// Number of study sites in the continuum.
    pub const COUNT: usize = 8;

    /// Canonical kebab-case identifier string.
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::ThePinesForeshore => "the-pines-foreshore",
            Self::HumesCrossing => "humes-crossing",
            Self::JounamaCreekValley => "jounama-creek-valley",
            Self::YarrangobillyKarstGorge => "yarrangobilly-karst-gorge",
            Self::OHaresAlpineAshRestArea => "ohares-alpine-ash-rest-area",
            Self::ThreeMileDamSnowGumBog => "three-mile-dam-snow-gum-bog",
            Self::SydneyHawkesburySandstoneGully => "sydney-hawkesbury-sandstone-gully",
            Self::NoosaCoastalPandanusHeadland => "noosa-coastal-pandanus-headland",
        }
    }

    /// Site index in 0..8.
    #[inline]
    pub const fn site_index(&self) -> usize {
        *self as usize
    }

    /// Resolves site from index.
    #[inline]
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::ThePinesForeshore),
            1 => Some(Self::HumesCrossing),
            2 => Some(Self::JounamaCreekValley),
            3 => Some(Self::YarrangobillyKarstGorge),
            4 => Some(Self::OHaresAlpineAshRestArea),
            5 => Some(Self::ThreeMileDamSnowGumBog),
            6 => Some(Self::SydneyHawkesburySandstoneGully),
            7 => Some(Self::NoosaCoastalPandanusHeadland),
            _ => None,
        }
    }
}

/// Comprehensive ecological, geological, and spatial descriptor for a study site.
#[derive(Debug, Clone, PartialEq)]
pub struct StudySiteDescriptor {
    /// Canonical site identifier.
    pub id: StudySiteId,
    /// Display name.
    pub name: &'static str,
    /// Corridor odometer marker in kilometers from Talbingo origin.
    pub corridor_km: f32,
    /// Local East-North-Up (ENU) metric coordinates `[x, z]` in meters relative to Talbingo Dam.
    pub world_pos_xz: [f32; 2],
    /// Geodetic elevation in meters Australian Height Datum (AHD).
    pub elevation_ahd_m: f32,
    /// Dominant vegetation community.
    pub vegetation_community: &'static str,
    /// Geological substrate description.
    pub geology_description: &'static str,
    /// Static water surface elevation in meters AHD if a water body is present.
    pub water_surface_m: Option<f32>,
    /// Dominant Soil and Landscape Grid of Australia (SLGA) geotechnical profile.
    pub soil_profile: SoilMechanicsProfile,
    /// True if subterranean karst cave overhangs exist at this site.
    pub has_karst_caves: bool,
    /// Primary botanical or zoological education specimen target.
    pub primary_study_target: &'static str,
    /// Canonical ontology CURIE associated with the primary target.
    pub target_ontology_curie: &'static str,
}

// ============================================================================
// 2. Surface Materials, Soil Stratigraphy & LOD Tiers
// ============================================================================

/// Surface material classification for traction, soundscape acoustic scattering,
/// and botanical seed germination.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SurfaceMaterialKind {
    /// Sealed bitumen road surface (Snowy Mountains Highway).
    RoadTarmac = 0,
    /// Compacted unsealed gravel fire-trail / road shoulder.
    FireTrailGravel = 1,
    /// Friable sandy loam alluvial deposit.
    AlluvialLoam = 2,
    /// Water-worn fluvial cobbles and riverbed pebbles.
    RiverPebble = 3,
    /// Crystalline granodiorite corestone bedrock (Bogong tors).
    GraniteBedrock = 4,
    /// Dissolved Silurian limestone karst bluff and speleothem rock.
    LimestoneKarst = 5,
    /// Rich humic forest soil under montane eucalypt canopy.
    ForestHumusLoam = 6,
    /// Subalpine *Sphagnum* moss peat bog over periglacial silt.
    SubalpinePeatBog = 7,
    /// Triassic cross-bedded quartzose Hawkesbury sandstone ledge.
    SandstoneLedge = 8,
    /// Tertiary columnar olivine basalt coastal headland rock.
    CoastalBasalt = 9,
}

impl SurfaceMaterialKind {
    /// Friction coefficient $\mu$ against rubber vehicle tires.
    #[inline]
    pub const fn tire_friction(&self) -> f32 {
        match self {
            Self::RoadTarmac => 0.95,
            Self::FireTrailGravel => 0.75,
            Self::AlluvialLoam => 0.65,
            Self::RiverPebble => 0.60,
            Self::GraniteBedrock => 0.88,
            Self::LimestoneKarst => 0.82,
            Self::ForestHumusLoam => 0.70,
            Self::SubalpinePeatBog => 0.45,
            Self::SandstoneLedge => 0.85,
            Self::CoastalBasalt => 0.80,
        }
    }

    /// Rolling resistance coefficient $C_{rr}$.
    #[inline]
    pub const fn rolling_resistance(&self) -> f32 {
        match self {
            Self::RoadTarmac => 0.015,
            Self::FireTrailGravel => 0.028,
            Self::AlluvialLoam => 0.038,
            Self::RiverPebble => 0.045,
            Self::GraniteBedrock => 0.018,
            Self::LimestoneKarst => 0.020,
            Self::ForestHumusLoam => 0.035,
            Self::SubalpinePeatBog => 0.075,
            Self::SandstoneLedge => 0.022,
            Self::CoastalBasalt => 0.025,
        }
    }
}

/// 6-layer Soil and Landscape Grid of Australia (SLGA) subsurface stratigraphic stack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SoilProfile {
    /// 6 discrete depth intervals:
    /// - Layer 0: 0–5 cm (humic root-mat epipedon)
    /// - Layer 1: 5–15 cm (friable A-horizon topsoil)
    /// - Layer 2: 15–30 cm (transitional loam/clay)
    /// - Layer 3: 30–60 cm (dense B-horizon illuvial clay)
    /// - Layer 4: 60–100 cm (gravelly saprolite / weathered clasts)
    /// - Layer 5: 100–200 cm (lithic refusal / solid bedrock)
    pub layers: [SoilMechanicsProfile; 6],
}

impl Default for SoilProfile {
    fn default() -> Self {
        Self::standard_slga(SoilMechanicsProfile::loam())
    }
}

impl SoilProfile {
    /// Constructs an SLGA 6-layer profile based on a dominant surface soil profile.
    pub fn standard_slga(dominant: SoilMechanicsProfile) -> Self {
        Self {
            layers: [
                SoilMechanicsProfile::humic_root_mat(),
                dominant,
                SoilMechanicsProfile::loam(),
                SoilMechanicsProfile::illuvial_clay(),
                SoilMechanicsProfile::saprolite(),
                SoilMechanicsProfile::bedrock(),
            ],
        }
    }

    /// Surface epipedon mechanics profile.
    #[inline]
    pub fn surface(&self) -> &SoilMechanicsProfile {
        &self.layers[0]
    }
}

/// 4-tier hybrid Level-of-Detail assignment for terrain and botanical streaming.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TerrainLod {
    /// `< 30 m`: Full biological organ B-Rep + SDF analytical geometry.
    LOD0 = 0,
    /// `30 m – 90 m`: SGGX microflake canopy + high-density terrain mesh.
    LOD1 = 1,
    /// `90 m – 250 m`: Surfel point cloud + simplified terrain mesh.
    LOD2 = 2,
    /// `> 250 m`: Octahedral billboard impostors + low-frequency DEM tile.
    LOD3 = 3,
}

/// Active streaming chunk slot in the Hilbert spatial cache.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StreamingChunkSlot {
    /// 64-bit 2D Hilbert space-filling curve coordinate.
    pub hilbert_index: u64,
    /// Discrete chunk grid coordinate `[cx, cz]`.
    pub chunk_coord: [i32; 2],
    /// World center position `[x, z]` in meters.
    pub world_center_xz: [f32; 2],
    /// Euclidean distance from viewer in meters.
    pub distance_m: f32,
    /// Assigned Level of Detail tier.
    pub lod: TerrainLod,
    /// Pre-indexed `.m64` chunk directory entry header.
    pub dir_entry: M64ChunkDirectoryEntry,
}

// ============================================================================
// 3. Regional Corridor World Continuum
// ============================================================================

/// Multi-kilometer contiguous terrain and corridor network uniting all 8 study sites.
pub struct RegionalCorridorWorld {
    /// 8 high-detail ecological study sites.
    pub sites: [StudySiteDescriptor; 8],
    /// Road and fire-trail Hermite cubic spline segments connecting the entire corridor.
    pub road_segments: Vec<HermiteCorridorSegment>,
    /// River channel Hermite cubic spline segments (Tumut, Jounama, Yarrangobilly).
    pub creek_segments: Vec<HermiteCorridorSegment>,
    /// Pre-indexed `.m64` terrain chunk directory entries along the 26.5 km corridor.
    pub chunk_dir_entries: Vec<M64ChunkDirectoryEntry>,
    /// 6-layer SLGA soil profiles per site.
    pub site_soil_profiles: [SoilProfile; 8],
}

impl Default for RegionalCorridorWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl RegionalCorridorWorld {
    /// Width of the sealed 2-lane highway roadbed in millimeters (9.0 meters).
    pub const ROAD_WIDTH_MM: u32 = 9000;
    /// Crown height at road centerline in millimeters (120 mm camber for storm drainage).
    pub const ROAD_CROWN_HEIGHT_MM: i16 = 120;
    /// Roadbed cut-and-fill batter slope blending width in meters.
    pub const ROAD_BATTER_WIDTH_M: f32 = 6.0;

    /// Creates and initializes the complete multi-kilometer regional world continuum.
    pub fn new() -> Self {
        let sites = Self::build_study_sites();
        let road_segments = Self::build_road_network(&sites);
        let creek_segments = Self::build_creek_network(&sites);
        let site_soil_profiles = Self::build_soil_profiles(&sites);
        let chunk_dir_entries = Self::build_chunk_directory_entries(&sites);

        Self {
            sites,
            road_segments,
            creek_segments,
            chunk_dir_entries,
            site_soil_profiles,
        }
    }

    /// Initializes descriptors for all 8 study sites along the 26.5 km corridor.
    fn build_study_sites() -> [StudySiteDescriptor; 8] {
        [
            // Site 0: The Pines Foreshore
            StudySiteDescriptor {
                id: StudySiteId::ThePinesForeshore,
                name: "The Pines & Talbingo Dam Foreshore",
                corridor_km: 0.0,
                world_pos_xz: [0.0, 0.0],
                elevation_ahd_m: 410.0,
                vegetation_community: "Corymbia aparrerinja / Eucalyptus viminalis Riparian Gallery",
                geology_description: "Silurian Talbingo granodiorite & lake alluvium",
                water_surface_m: Some(402.5),
                soil_profile: SoilMechanicsProfile::loam(),
                has_karst_caves: false,
                primary_study_target: "Corymbia aparrerinja / Eucalyptus viminalis Ribbon Gum",
                target_ontology_curie: "po:0009010",
            },
            // Site 1: Humes Crossing
            StudySiteDescriptor {
                id: StudySiteId::HumesCrossing,
                name: "Humes Crossing & Blowering Foreshore",
                corridor_km: 1.8,
                world_pos_xz: [1450.0, -980.0],
                elevation_ahd_m: 405.0,
                vegetation_community: "Blowering foreshore alluvial flats & river gravel bars",
                geology_description: "Quaternary river gravel, quartz pebbles & silt alluvium",
                water_surface_m: Some(402.5),
                soil_profile: SoilMechanicsProfile::humic_root_mat(),
                has_karst_caves: false,
                primary_study_target: "Macropus giganteus (Eastern Grey Kangaroo) grazing flats",
                target_ontology_curie: "uberon:0002415",
            },
            // Site 2: Jounama Creek Valley
            StudySiteDescriptor {
                id: StudySiteId::JounamaCreekValley,
                name: "Jounama Creek Valley & Granite Tors",
                corridor_km: 4.2,
                world_pos_xz: [3400.0, -1850.0],
                elevation_ahd_m: 460.0,
                vegetation_community: "Montane wet sclerophyll & Bogong moth granite tors",
                geology_description: "Devonian Jounama granodiorite corestones & exfoliation slabs",
                water_surface_m: Some(458.0),
                soil_profile: SoilMechanicsProfile::saprolite(),
                has_karst_caves: false,
                primary_study_target: "Agrotis infusa (Bogong Moth) aestivation granite tor",
                target_ontology_curie: "uberon:0003661",
            },
            // Site 3: Yarrangobilly Karst Gorge
            StudySiteDescriptor {
                id: StudySiteId::YarrangobillyKarstGorge,
                name: "Yarrangobilly Karst Gorge & Thermal Pool",
                corridor_km: 9.5,
                world_pos_xz: [7800.0, -4200.0],
                elevation_ahd_m: 940.0,
                vegetation_community: "Silurian limestone karst bluff, cave arch, 27°C thermal pool",
                geology_description: "Massive Silurian Yarrangobilly limestone karst with speleothems",
                water_surface_m: Some(938.5),
                soil_profile: SoilMechanicsProfile::illuvial_clay(),
                has_karst_caves: true,
                primary_study_target: "Silurian karst cave arch & calcite speleothem formation",
                target_ontology_curie: "po:0004708",
            },
            // Site 4: O'Hares Alpine Ash Rest Area
            StudySiteDescriptor {
                id: StudySiteId::OHaresAlpineAshRestArea,
                name: "O'Hares Alpine Ash Montane Forest",
                corridor_km: 13.0,
                world_pos_xz: [10600.0, -5900.0],
                elevation_ahd_m: 1180.0,
                vegetation_community: "Tall Eucalyptus delegatensis montane forest",
                geology_description: "Deep mountain krasnozem soils over weathered siltstone",
                water_surface_m: None,
                soil_profile: SoilMechanicsProfile::loam(),
                has_karst_caves: false,
                primary_study_target: "Eucalyptus delegatensis (Alpine Ash) towering canopy",
                target_ontology_curie: "po:0005352",
            },
            // Site 5: Three Mile Dam Snow Gum Bog
            StudySiteDescriptor {
                id: StudySiteId::ThreeMileDamSnowGumBog,
                name: "Three Mile Dam Subalpine Snow Gum Bog",
                corridor_km: 17.5,
                world_pos_xz: [14200.0, -8100.0],
                elevation_ahd_m: 1460.0,
                vegetation_community: "Subalpine Eucalyptus pauciflora frost hollow & Sphagnum bog",
                geology_description: "Subalpine fibrous peat, organosols & glacial frost hollow",
                water_surface_m: Some(1458.0),
                soil_profile: SoilMechanicsProfile::humic_root_mat(),
                has_karst_caves: false,
                primary_study_target: "Pseudophryne corroboree (Southern Corroboree Frog) in Sphagnum",
                target_ontology_curie: "uberon:0001004",
            },
            // Site 6: Sydney Hawkesbury Sandstone Gully
            StudySiteDescriptor {
                id: StudySiteId::SydneyHawkesburySandstoneGully,
                name: "Sydney Hawkesbury Sandstone Gully (Portal Corridor)",
                corridor_km: 22.0,
                world_pos_xz: [17800.0, -4500.0],
                elevation_ahd_m: 185.0,
                vegetation_community: "Triassic Hawkesbury sandstone ledges & Angophora costata",
                geology_description: "Cross-bedded quartzose Hawkesbury sandstone honeycombed caverns",
                water_surface_m: Some(182.0),
                soil_profile: SoilMechanicsProfile::saprolite(),
                has_karst_caves: false,
                primary_study_target: "Angophora costata (Smooth-barked Apple) rock-clinging roots",
                target_ontology_curie: "po:0000032",
            },
            // Site 7: Noosa Coastal Pandanus Headland
            StudySiteDescriptor {
                id: StudySiteId::NoosaCoastalPandanusHeadland,
                name: "Noosa Coastal Pandanus Headland (Portal Corridor)",
                corridor_km: 26.5,
                world_pos_xz: [21500.0, -1200.0],
                elevation_ahd_m: 32.0,
                vegetation_community: "Coastal basalt headland & Pandanus tectorius",
                geology_description: "Tertiary olivine basalt wave-cut platforms & littoral sands",
                water_surface_m: Some(0.8),
                soil_profile: SoilMechanicsProfile::saprolite(),
                has_karst_caves: false,
                primary_study_target: "Pandanus tectorius (Screw Pine) prop roots & littoral zone",
                target_ontology_curie: "po:0009084",
            },
        ]
    }

    /// Constructs the contiguous Hermite cubic spline road network chaining all 8 sites.
    fn build_road_network(sites: &[StudySiteDescriptor; 8]) -> Vec<HermiteCorridorSegment> {
        let mut segments = Vec::with_capacity(14);

        for i in 0..7 {
            let s0 = &sites[i];
            let s1 = &sites[i + 1];

            let x0 = s0.world_pos_xz[0] as f64 * 1000.0;
            let y0 = s0.world_pos_xz[1] as f64 * 1000.0;
            let elev0 = (s0.elevation_ahd_m * 1000.0).round() as i32;

            let x1 = s1.world_pos_xz[0] as f64 * 1000.0;
            let y1 = s1.world_pos_xz[1] as f64 * 1000.0;
            let elev1 = (s1.elevation_ahd_m * 1000.0).round() as i32;

            let dx = x1 - x0;
            let dy = y1 - y0;
            let dist = (dx * dx + dy * dy).sqrt();

            // If the segment is long (> 3 km), insert a midpoint to provide realistic mountain bends
            if dist > 3_000_000.0 {
                // Midpoint with curve deviation perpendicular to chord
                let perp_x = -dy / dist;
                let perp_y = dx / dist;
                let curve_offset = 350_000.0; // 350 m curve outward

                let mid_x = (x0 + x1) * 0.5 + perp_x * curve_offset;
                let mid_y = (y0 + y1) * 0.5 + perp_y * curve_offset;
                let mid_elev = ((elev0 + elev1) as f64 * 0.5) as i32;

                let tan0_x = (mid_x - x0) * 1.0;
                let tan0_y = (mid_y - y0) * 1.0;
                let tan1_x = (x1 - mid_x) * 1.0;
                let tan1_y = (y1 - mid_y) * 1.0;

                // Sub-segment 1
                segments.push(HermiteCorridorSegment {
                    p0: SplineControlPoint {
                        pos_xy_mm: [x0, y0],
                        elevation_mm: elev0,
                        tangent_xy: [tan0_x, tan0_y],
                    },
                    p1: SplineControlPoint {
                        pos_xy_mm: [mid_x, mid_y],
                        elevation_mm: mid_elev,
                        tangent_xy: [tan0_x, tan0_y],
                    },
                    kind: CorridorKind::Road {
                        width_mm: Self::ROAD_WIDTH_MM,
                        crown_height_mm: Self::ROAD_CROWN_HEIGHT_MM,
                    },
                });

                // Sub-segment 2
                segments.push(HermiteCorridorSegment {
                    p0: SplineControlPoint {
                        pos_xy_mm: [mid_x, mid_y],
                        elevation_mm: mid_elev,
                        tangent_xy: [tan1_x, tan1_y],
                    },
                    p1: SplineControlPoint {
                        pos_xy_mm: [x1, y1],
                        elevation_mm: elev1,
                        tangent_xy: [tan1_x, tan1_y],
                    },
                    kind: CorridorKind::Road {
                        width_mm: Self::ROAD_WIDTH_MM,
                        crown_height_mm: Self::ROAD_CROWN_HEIGHT_MM,
                    },
                });
            } else {
                segments.push(HermiteCorridorSegment {
                    p0: SplineControlPoint {
                        pos_xy_mm: [x0, y0],
                        elevation_mm: elev0,
                        tangent_xy: [dx, dy],
                    },
                    p1: SplineControlPoint {
                        pos_xy_mm: [x1, y1],
                        elevation_mm: elev1,
                        tangent_xy: [dx, dy],
                    },
                    kind: CorridorKind::Road {
                        width_mm: Self::ROAD_WIDTH_MM,
                        crown_height_mm: Self::ROAD_CROWN_HEIGHT_MM,
                    },
                });
            }
        }

        segments
    }

    /// Constructs the river and creek Hermite spline corridors.
    fn build_creek_network(sites: &[StudySiteDescriptor; 8]) -> Vec<HermiteCorridorSegment> {
        vec![
            // Tumut River / Blowering reservoir foreshore creek (Site 0 -> Site 1)
            HermiteCorridorSegment {
                p0: SplineControlPoint {
                    pos_xy_mm: [
                        (sites[0].world_pos_xz[0] - 80.0) as f64 * 1000.0,
                        (sites[0].world_pos_xz[1] - 40.0) as f64 * 1000.0,
                    ],
                    elevation_mm: 403_000,
                    tangent_xy: [1200_000.0, -800_000.0],
                },
                p1: SplineControlPoint {
                    pos_xy_mm: [
                        (sites[1].world_pos_xz[0] - 60.0) as f64 * 1000.0,
                        (sites[1].world_pos_xz[1] - 30.0) as f64 * 1000.0,
                    ],
                    elevation_mm: 402_000,
                    tangent_xy: [1200_000.0, -800_000.0],
                },
                kind: CorridorKind::Creek {
                    width_mm: 14000,
                    depth_mm: 2200,
                    flow_velocity_xy: [350, -180],
                },
            },
            // Jounama Creek meandering reach (Site 1 -> Site 2)
            HermiteCorridorSegment {
                p0: SplineControlPoint {
                    pos_xy_mm: [
                        (sites[1].world_pos_xz[0] + 50.0) as f64 * 1000.0,
                        (sites[1].world_pos_xz[1] - 120.0) as f64 * 1000.0,
                    ],
                    elevation_mm: 412_000,
                    tangent_xy: [1800_000.0, -850_000.0],
                },
                p1: SplineControlPoint {
                    pos_xy_mm: [
                        (sites[2].world_pos_xz[0] - 40.0) as f64 * 1000.0,
                        (sites[2].world_pos_xz[1] + 20.0) as f64 * 1000.0,
                    ],
                    elevation_mm: 457_000,
                    tangent_xy: [1800_000.0, -850_000.0],
                },
                kind: CorridorKind::Creek {
                    width_mm: 11000,
                    depth_mm: 1600,
                    flow_velocity_xy: [620, -290],
                },
            },
            // Yarrangobilly River Gorge channel (near Site 3)
            HermiteCorridorSegment {
                p0: SplineControlPoint {
                    pos_xy_mm: [7720_000.0, -4250_000.0],
                    elevation_mm: 942_000,
                    tangent_xy: [300_000.0, 180_000.0],
                },
                p1: SplineControlPoint {
                    pos_xy_mm: [7950_000.0, -4110_000.0],
                    elevation_mm: 934_000,
                    tangent_xy: [300_000.0, 180_000.0],
                },
                kind: CorridorKind::Creek {
                    width_mm: 16000,
                    depth_mm: 2400,
                    flow_velocity_xy: [850, 420],
                },
            },
        ]
    }

    /// Initializes 6-layer SLGA soil profiles for each study site.
    fn build_soil_profiles(sites: &[StudySiteDescriptor; 8]) -> [SoilProfile; 8] {
        [
            SoilProfile::standard_slga(sites[0].soil_profile),
            SoilProfile::standard_slga(sites[1].soil_profile),
            SoilProfile::standard_slga(sites[2].soil_profile),
            SoilProfile::standard_slga(sites[3].soil_profile),
            SoilProfile::standard_slga(sites[4].soil_profile),
            SoilProfile::standard_slga(sites[5].soil_profile),
            SoilProfile::standard_slga(sites[6].soil_profile),
            SoilProfile::standard_slga(sites[7].soil_profile),
        ]
    }

    /// Initializes pre-indexed Hilbert `.m64` chunk directory entries along the corridor.
    fn build_chunk_directory_entries(sites: &[StudySiteDescriptor; 8]) -> Vec<M64ChunkDirectoryEntry> {
        let mut entries = Vec::with_capacity(64);

        for (site_idx, site) in sites.iter().enumerate() {
            // Place 4 chunks clustered around each site (2x2 grid)
            let base_cx = (site.world_pos_xz[0] / 64.0).floor() as i32;
            let base_cz = (site.world_pos_xz[1] / 64.0).floor() as i32;

            for dy in 0..2 {
                for dx in 0..2 {
                    let cx = (base_cx + dx).max(0) as u32;
                    let cy = (base_cz + dy).max(0) as u32;
                    let h_idx = xy_to_hilbert(cx, cy, 16);

                    let mut entry = M64ChunkDirectoryEntry {
                        chunk_uuid: [0; 16],
                        spatial_hilbert_index: h_idx,
                        byte_offset: 4096 + (entries.len() as u64) * 262_144,
                        byte_length: 262_144, // 4096 * 64 bytes
                        record_count: 4096,
                        srid_geodetic_code: 7844, // GDA2020
                        lut_bank_epoch_id: 1,
                        chunk_record_mode: M64_MODE_STRATA,
                        compression_flags: 0, // Uncompressed mmap
                        bao_root_hash_prefix: [0; 12],
                        entry_crc32c: 0,
                    };

                    let h_bytes = h_idx.to_le_bytes();
                    entry.chunk_uuid[..8].copy_from_slice(&h_bytes);
                    entry.chunk_uuid[8..16].copy_from_slice(&h_bytes);
                    entry.chunk_uuid[0] = site_idx as u8;
                    entry.update_crc();

                    entries.push(entry);
                }
            }
        }

        entries
    }

    // ========================================================================
    // 4. Zero-Heap World Queries
    // ========================================================================

    /// Evaluates base natural mountain topography without road excavation.
    pub fn evaluate_natural_elevation(&self, wx_m: f32, wz_m: f32) -> (f32, SurfaceMaterialKind) {
        // Find distance to each study site for smooth multi-site mountain hypsometry blending
        let mut total_weight = 0.0001;
        let mut blended_elev = 0.0;
        let mut nearest_site_idx = 0;
        let mut min_dist = f32::MAX;

        for (i, site) in self.sites.iter().enumerate() {
            let dx = wx_m - site.world_pos_xz[0];
            let dz = wz_m - site.world_pos_xz[1];
            let dist = (dx * dx + dz * dz).sqrt().max(1.0);
            if dist < min_dist {
                min_dist = dist;
                nearest_site_idx = i;
            }

            let weight = 1.0 / (dist * dist);
            blended_elev += site.elevation_ahd_m * weight;
            total_weight += weight;
        }

        let base_hypsometry = blended_elev / total_weight;

        // Layered analytical ridge-and-valley harmonics
        let ridge1 = (wx_m * 0.0018 + wz_m * 0.0011).sin() * 24.0;
        let ridge2 = (wx_m * 0.0055 - wz_m * 0.0042).cos() * 8.5;
        let micro_relief = (wx_m * 0.032 + wz_m * 0.027).sin() * 1.8;

        let natural_elev = base_hypsometry + ridge1 + ridge2 + micro_relief;

        let material = match self.sites[nearest_site_idx].id {
            StudySiteId::ThePinesForeshore => SurfaceMaterialKind::AlluvialLoam,
            StudySiteId::HumesCrossing => SurfaceMaterialKind::AlluvialLoam,
            StudySiteId::JounamaCreekValley => {
                if (wx_m * 0.05).sin() > 0.6 {
                    SurfaceMaterialKind::GraniteBedrock
                } else {
                    SurfaceMaterialKind::ForestHumusLoam
                }
            }
            StudySiteId::YarrangobillyKarstGorge => SurfaceMaterialKind::LimestoneKarst,
            StudySiteId::OHaresAlpineAshRestArea => SurfaceMaterialKind::ForestHumusLoam,
            StudySiteId::ThreeMileDamSnowGumBog => SurfaceMaterialKind::SubalpinePeatBog,
            StudySiteId::SydneyHawkesburySandstoneGully => SurfaceMaterialKind::SandstoneLedge,
            StudySiteId::NoosaCoastalPandanusHeadland => SurfaceMaterialKind::CoastalBasalt,
        };

        (natural_elev, material)
    }

    /// Evaluates continuous terrain elevation, surface normal vector, and material kind
    /// conforming to the 26.5 km `HermiteCorridorSegment` road network with zero heap allocations.
    pub fn sample_world_elevation_and_normal_zero_heap(
        &self,
        wx_m: f32,
        wz_m: f32,
    ) -> (f32, [f32; 3], SurfaceMaterialKind) {
        let (elev, mat) = self.sample_world_elevation_zero_heap(wx_m, wz_m);

        // Finite differences with delta = 0.25 m for normal evaluation
        const EPS: f32 = 0.25;
        let (e_xp, _) = self.sample_world_elevation_zero_heap(wx_m + EPS, wz_m);
        let (e_xm, _) = self.sample_world_elevation_zero_heap(wx_m - EPS, wz_m);
        let (e_zp, _) = self.sample_world_elevation_zero_heap(wx_m, wz_m + EPS);
        let (e_zm, _) = self.sample_world_elevation_zero_heap(wx_m, wz_m - EPS);

        let dx = (e_xp - e_xm) / (2.0 * EPS);
        let dz = (e_zp - e_zm) / (2.0 * EPS);

        let nx = -dx;
        let ny = 1.0;
        let nz = -dz;
        let len = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-4);

        (elev, [nx / len, ny / len, nz / len], mat)
    }

    /// Evaluates terrain elevation and material kind at `(wx_m, wz_m)`.
    pub fn sample_world_elevation_zero_heap(
        &self,
        wx_m: f32,
        wz_m: f32,
    ) -> (f32, SurfaceMaterialKind) {
        let (natural_elev, mut mat) = self.evaluate_natural_elevation(wx_m, wz_m);

        let wx_mm = wx_m as f64 * 1000.0;
        let wz_mm = wz_m as f64 * 1000.0;

        let mut final_elev = natural_elev;
        let mut min_road_dist_mm = f64::MAX;

        // 1. Check road network cut-and-fill conformation
        for seg in &self.road_segments {
            // Broadphase bounding box check
            let min_x = seg.p0.pos_xy_mm[0].min(seg.p1.pos_xy_mm[0]) - 50_000.0;
            let max_x = seg.p0.pos_xy_mm[0].max(seg.p1.pos_xy_mm[0]) + 50_000.0;
            let min_y = seg.p0.pos_xy_mm[1].min(seg.p1.pos_xy_mm[1]) - 50_000.0;
            let max_y = seg.p0.pos_xy_mm[1].max(seg.p1.pos_xy_mm[1]) + 50_000.0;

            if wx_mm < min_x || wx_mm > max_x || wz_mm < min_y || wz_mm > max_y {
                continue;
            }

            let (dist_mm, t) = seg.distance_to_point(wx_mm, wz_mm);
            if dist_mm < min_road_dist_mm {
                min_road_dist_mm = dist_mm;
            }

            if let CorridorKind::Road {
                width_mm,
                crown_height_mm,
            } = seg.kind
            {
                let half_width_mm = width_mm as f64 * 0.5;
                let batter_width_mm = Self::ROAD_BATTER_WIDTH_M as f64 * 1000.0;

                if dist_mm <= half_width_mm + batter_width_mm {
                    let (_, centerline_elev_mm) = seg.sample_centerline(t);
                    let target_center_elev = centerline_elev_mm as f32 / 1000.0;

                    if dist_mm <= half_width_mm {
                        // On the tarmac roadbed: apply parabolic crown
                        let rel = (dist_mm / half_width_mm) as f32;
                        let crown_m = (1.0 - rel * rel) * (crown_height_mm as f32 / 1000.0);
                        final_elev = target_center_elev + crown_m;
                        mat = SurfaceMaterialKind::RoadTarmac;
                        break;
                    } else {
                        // On the cut-and-fill batter slope: smooth Hermite blend into natural terrain
                        let rel_batter = ((dist_mm - half_width_mm) / batter_width_mm) as f32;
                        let s = rel_batter * rel_batter * (3.0 - 2.0 * rel_batter);
                        final_elev = target_center_elev * (1.0 - s) + natural_elev * s;
                        mat = SurfaceMaterialKind::FireTrailGravel;
                        break;
                    }
                }
            }
        }

        // 2. Check creek network riverbed excavation if not on road
        if mat != SurfaceMaterialKind::RoadTarmac {
            for creek in &self.creek_segments {
                let min_x = creek.p0.pos_xy_mm[0].min(creek.p1.pos_xy_mm[0]) - 30_000.0;
                let max_x = creek.p0.pos_xy_mm[0].max(creek.p1.pos_xy_mm[0]) + 30_000.0;
                let min_y = creek.p0.pos_xy_mm[1].min(creek.p1.pos_xy_mm[1]) - 30_000.0;
                let max_y = creek.p0.pos_xy_mm[1].max(creek.p1.pos_xy_mm[1]) + 30_000.0;

                if wx_mm < min_x || wx_mm > max_x || wz_mm < min_y || wz_mm > max_y {
                    continue;
                }

                let (dist_mm, t) = creek.distance_to_point(wx_mm, wz_mm);
                if let CorridorKind::Creek { width_mm, depth_mm, .. } = creek.kind {
                    let half_width_mm = width_mm as f64 * 0.5;
                    if dist_mm <= half_width_mm {
                        let (_, creek_center_elev_mm) = creek.sample_centerline(t);
                        let center_elev = creek_center_elev_mm as f32 / 1000.0;
                        let rel = (dist_mm / half_width_mm) as f32;
                        let bed_depth = (1.0 - rel * rel) * (depth_mm as f32 / 1000.0);
                        final_elev = center_elev - bed_depth;
                        mat = SurfaceMaterialKind::RiverPebble;
                        break;
                    }
                }
            }
        }

        (final_elev, mat)
    }

    /// Evaluates water surface elevation in meters AHD with zero heap allocations.
    ///
    /// Returns `Some(water_elevation_m)` if `(wx_m, wz_m)` is within a lake, creek,
    /// thermal pool, dam, or ocean body, and `None` on dry land.
    pub fn sample_water_surface_zero_heap(&self, wx_m: f32, wz_m: f32) -> Option<f32> {
        // 1. Talbingo / Blowering Reservoir Basin (Site 0 & Site 1)
        if wx_m < 2200.0 && wz_m > -1600.0 {
            let lake_level = 402.5;
            let (ground_y, _) = self.sample_world_elevation_zero_heap(wx_m, wz_m);
            let near_foreshore_0 = wx_m * wx_m + wz_m * wz_m < 140.0 * 140.0;
            let dx1 = wx_m - 1450.0;
            let dz1 = wz_m - (-980.0);
            let near_foreshore_1 = dx1 * dx1 + dz1 * dz1 < 140.0 * 140.0;
            if ground_y < lake_level + 0.5 || near_foreshore_0 || near_foreshore_1 {
                return Some(lake_level);
            }
        }

        // 2. Jounama Creek Corridor (Site 2)
        let dx_j = wx_m - 3400.0;
        let dz_j = wz_m - (-1850.0);
        if dx_j * dx_j + dz_j * dz_j < 60.0 * 60.0 {
            return Some(458.0);
        }

        // 3. Yarrangobilly Karst Thermal Pool & River (Site 3)
        let dx_k = wx_m - 7800.0;
        let dz_k = wz_m - (-4200.0);
        if dx_k * dx_k + dz_k * dz_k < 90.0 * 90.0 {
            // Thermal spring pool basin
            let dx_pool = wx_m - 7785.0;
            let dz_pool = wz_m - (-4185.0);
            if dx_pool * dx_pool + dz_pool * dz_pool < 25.0 * 25.0 {
                return Some(938.5); // 27°C thermal spring pool
            }
            return Some(936.0); // Yarrangobilly River gorge
        }

        // 4. Three Mile Dam Subalpine Pond (Site 5)
        let dx_tmd = wx_m - 14200.0;
        let dz_tmd = wz_m - (-8100.0);
        if dx_tmd * dx_tmd + dz_tmd * dz_tmd < 180.0 * 180.0 {
            return Some(1458.0);
        }

        // 5. Sydney Hawkesbury Gully Stream (Site 6)
        let dx_syd = wx_m - 17800.0;
        let dz_syd = wz_m - (-4500.0);
        if dx_syd * dx_syd + dz_syd * dz_syd < 50.0 * 50.0 {
            return Some(182.0);
        }

        // 6. Noosa Coastal Pacific Swell (Site 7)
        if wx_m > 21400.0 {
            let swell = (wx_m * 0.02 + wz_m * 0.015).sin() * 0.45;
            return Some(0.8 + swell);
        }

        None
    }

    /// Evaluates karst cave ceiling overhang height in meters AHD with zero heap allocations.
    ///
    /// In the Yarrangobilly Karst Gorge cave arch zone, returns the vault ceiling elevation.
    /// Returns `f32::INFINITY` for all outdoor open terrain.
    pub fn sample_ceiling_overhang_zero_heap(&self, wx_m: f32, wz_m: f32) -> f32 {
        // Yarrangobilly Karst Cave Arch zone (Site 3: centered at [7820.0, -4210.0])
        let dx = wx_m - 7820.0;
        let dz = wz_m - (-4210.0);
        let dist_sq = dx * dx + dz * dz;

        const CAVE_RADIUS_M: f32 = 45.0;
        if dist_sq <= CAVE_RADIUS_M * CAVE_RADIUS_M {
            // Parabolic limestone arch ceiling vault peaking at 946.5 m AHD
            let rel_dist_sq = dist_sq / (CAVE_RADIUS_M * CAVE_RADIUS_M);
            let vault_height = 946.5 - rel_dist_sq * 6.5;
            return vault_height;
        }

        f32::INFINITY
    }

    /// Identifies the nearest study site and distance in meters with zero heap allocations.
    pub fn locate_nearest_study_site_zero_heap(&self, wx_m: f32, wz_m: f32) -> (StudySiteId, f32) {
        let mut best_id = self.sites[0].id;
        let mut min_dist = f32::MAX;

        for site in &self.sites {
            let dx = wx_m - site.world_pos_xz[0];
            let dz = wz_m - site.world_pos_xz[1];
            let dist = (dx * dx + dz * dz).sqrt();
            if dist < min_dist {
                min_dist = dist;
                best_id = site.id;
            }
        }

        (best_id, min_dist)
    }

    /// Computes streaming Hilbert `.m64` chunk directory entries and 4-tier hybrid LOD
    /// assignments with zero heap allocations.
    ///
    /// Writes up to `out_chunks.len()` active slots into `out_chunks` and returns count.
    pub fn compute_streaming_hilbert_chunks_zero_heap(
        &self,
        wx_m: f32,
        wz_m: f32,
        view_radius_m: f32,
        out_chunks: &mut [StreamingChunkSlot],
    ) -> usize {
        const CHUNK_SIZE_M: f32 = 64.0;
        let mut count = 0;

        let center_cx = (wx_m / CHUNK_SIZE_M).floor() as i32;
        let center_cz = (wz_m / CHUNK_SIZE_M).floor() as i32;
        let radius_chunks = (view_radius_m / CHUNK_SIZE_M).ceil() as i32;

        let r_min_x = center_cx - radius_chunks;
        let r_max_x = center_cx + radius_chunks;
        let r_min_z = center_cz - radius_chunks;
        let r_max_z = center_cz + radius_chunks;

        for cz in r_min_z..=r_max_z {
            for cx in r_min_x..=r_max_x {
                if count >= out_chunks.len() {
                    return count;
                }

                let chunk_wx = cx as f32 * CHUNK_SIZE_M + CHUNK_SIZE_M * 0.5;
                let chunk_wz = cz as f32 * CHUNK_SIZE_M + CHUNK_SIZE_M * 0.5;

                let dx = chunk_wx - wx_m;
                let dz = chunk_wz - wz_m;
                let dist = (dx * dx + dz * dz).sqrt();

                if dist <= view_radius_m {
                    let lod = if dist < 50.0 {
                        TerrainLod::LOD0
                    } else if dist < 110.0 {
                        TerrainLod::LOD1
                    } else if dist < 250.0 {
                        TerrainLod::LOD2
                    } else {
                        TerrainLod::LOD3
                    };

                    let grid_x = (cx + 512).max(0) as u32;
                    let grid_y = (cz + 512).max(0) as u32;
                    let hilbert_idx = xy_to_hilbert(grid_x, grid_y, 16);

                    let mut dir_entry = M64ChunkDirectoryEntry {
                        chunk_uuid: [0; 16],
                        spatial_hilbert_index: hilbert_idx,
                        byte_offset: 4096 + (hilbert_idx % 256) * 262_144,
                        byte_length: 262_144,
                        record_count: 4096,
                        srid_geodetic_code: 7844,
                        lut_bank_epoch_id: 1,
                        chunk_record_mode: M64_MODE_STRATA,
                        compression_flags: 0,
                        bao_root_hash_prefix: [0; 12],
                        entry_crc32c: 0,
                    };
                    let h_bytes = hilbert_idx.to_le_bytes();
                    dir_entry.chunk_uuid[..8].copy_from_slice(&h_bytes);
                    dir_entry.chunk_uuid[8..16].copy_from_slice(&h_bytes);
                    dir_entry.update_crc();

                    out_chunks[count] = StreamingChunkSlot {
                        hilbert_index: hilbert_idx,
                        chunk_coord: [cx, cz],
                        world_center_xz: [chunk_wx, chunk_wz],
                        distance_m: dist,
                        lod,
                        dir_entry,
                    };

                    count += 1;
                }
            }
        }

        count
    }

    /// Samples 6-layer SLGA soil profile at `(wx_m, wz_m)`.
    pub fn sample_soil_profile(&self, wx_m: f32, wz_m: f32) -> SoilProfile {
        let (site_id, _) = self.locate_nearest_study_site_zero_heap(wx_m, wz_m);
        self.site_soil_profiles[site_id.site_index()]
    }
}

// ============================================================================
// 5. Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_study_sites_initialization_and_descriptors() {
        let world = RegionalCorridorWorld::new();
        assert_eq!(world.sites.len(), 8);

        // Verify Site 0 (The Pines)
        assert_eq!(world.sites[0].id, StudySiteId::ThePinesForeshore);
        assert_eq!(world.sites[0].corridor_km, 0.0);
        assert_eq!(world.sites[0].elevation_ahd_m, 410.0);

        // Verify Site 3 (Yarrangobilly)
        assert_eq!(world.sites[3].id, StudySiteId::YarrangobillyKarstGorge);
        assert_eq!(world.sites[3].corridor_km, 9.5);
        assert_eq!(world.sites[3].elevation_ahd_m, 940.0);
        assert!(world.sites[3].has_karst_caves);

        // Verify Site 5 (Three Mile Dam)
        assert_eq!(world.sites[5].id, StudySiteId::ThreeMileDamSnowGumBog);
        assert_eq!(world.sites[5].corridor_km, 17.5);
        assert_eq!(world.sites[5].elevation_ahd_m, 1460.0);

        // Verify Site 7 (Noosa)
        assert_eq!(world.sites[7].id, StudySiteId::NoosaCoastalPandanusHeadland);
        assert_eq!(world.sites[7].corridor_km, 26.5);
        assert_eq!(world.sites[7].elevation_ahd_m, 32.0);
    }

    #[test]
    fn test_continuous_road_elevation_and_corridor_conformation() {
        let world = RegionalCorridorWorld::new();

        // Sample directly on road at Site 0
        let (elev0, norm0, mat0) = world.sample_world_elevation_and_normal_zero_heap(0.0, 0.0);
        assert_eq!(mat0, SurfaceMaterialKind::RoadTarmac);
        assert!((elev0 - 410.0).abs() < 1.0, "Site 0 road elevation mismatch: {elev0}");
        assert!(norm0[1] > 0.9, "Road normal must point mostly upward: {norm0:?}");

        // Sample on road midway between Site 0 and Site 1
        let mid_x = 725.0;
        let mid_z = -490.0;
        let (elev_mid, norm_mid, mat_mid) =
            world.sample_world_elevation_and_normal_zero_heap(mid_x, mid_z);
        assert_eq!(mat_mid, SurfaceMaterialKind::RoadTarmac);
        assert!(
            elev_mid >= 404.0 && elev_mid <= 411.0,
            "Mid-corridor road elevation out of bounds: {elev_mid}"
        );
        assert!(norm_mid[1] > 0.95);

        // Sample 30m off the road to verify natural terrain transition
        let (elev_off, _, mat_off) =
            world.sample_world_elevation_and_normal_zero_heap(mid_x + 30.0, mid_z);
        assert_ne!(mat_off, SurfaceMaterialKind::RoadTarmac);
        assert_ne!(elev_off, elev_mid);
    }

    #[test]
    fn test_water_surface_queries_across_sites() {
        let world = RegionalCorridorWorld::new();

        // Site 0 / Blowering foreshore water
        let water0 = world.sample_water_surface_zero_heap(10.0, 10.0);
        assert_eq!(water0, Some(402.5));

        // Yarrangobilly thermal pool
        let water_thermal = world.sample_water_surface_zero_heap(7785.0, -4185.0);
        assert_eq!(water_thermal, Some(938.5));

        // Three Mile Dam pond
        let water_tmd = world.sample_water_surface_zero_heap(14200.0, -8100.0);
        assert_eq!(water_tmd, Some(1458.0));

        // High dry mountain ridge (O'Hares Rest Area)
        let water_dry = world.sample_water_surface_zero_heap(10600.0, -5900.0);
        assert_eq!(water_dry, None);

        // Noosa Pacific swell
        let water_ocean = world.sample_water_surface_zero_heap(21550.0, -1200.0);
        assert!(water_ocean.is_some());
    }

    #[test]
    fn test_karst_cave_ceiling_overhang_detection() {
        let world = RegionalCorridorWorld::new();

        // Inside Yarrangobilly karst cave arch
        let ceil_inside = world.sample_ceiling_overhang_zero_heap(7820.0, -4210.0);
        assert!(ceil_inside.is_finite());
        assert!((ceil_inside - 946.5).abs() < 1.0);

        // Outside in open air
        let ceil_outside = world.sample_ceiling_overhang_zero_heap(0.0, 0.0);
        assert_eq!(ceil_outside, f32::INFINITY);
    }

    #[test]
    fn test_streaming_hilbert_chunks_lod_assignment() {
        let world = RegionalCorridorWorld::new();
        let mut slots = [StreamingChunkSlot {
            hilbert_index: 0,
            chunk_coord: [0, 0],
            world_center_xz: [0.0, 0.0],
            distance_m: 0.0,
            lod: TerrainLod::LOD3,
            dir_entry: M64ChunkDirectoryEntry {
                chunk_uuid: [0; 16],
                spatial_hilbert_index: 0,
                byte_offset: 0,
                byte_length: 0,
                record_count: 0,
                srid_geodetic_code: 0,
                lut_bank_epoch_id: 0,
                chunk_record_mode: 0,
                compression_flags: 0,
                bao_root_hash_prefix: [0; 12],
                entry_crc32c: 0,
            },
        }; 32];

        let count = world.compute_streaming_hilbert_chunks_zero_heap(0.0, 0.0, 100.0, &mut slots);
        assert!(count > 0);

        // First chunk nearest to viewer must be LOD0
        let has_lod0 = slots[..count].iter().any(|s| s.lod == TerrainLod::LOD0);
        assert!(has_lod0, "Viewer center must contain LOD0 chunks");

        // Verify CRC on directory entry
        for s in &slots[..count] {
            assert!(s.dir_entry.validate().is_ok());
        }
    }

    #[test]
    fn test_nearest_study_site_queries() {
        let world = RegionalCorridorWorld::new();

        let (site0, d0) = world.locate_nearest_study_site_zero_heap(10.0, 5.0);
        assert_eq!(site0, StudySiteId::ThePinesForeshore);
        assert!(d0 < 15.0);

        let (site4, d4) = world.locate_nearest_study_site_zero_heap(10590.0, -5910.0);
        assert_eq!(site4, StudySiteId::OHaresAlpineAshRestArea);
        assert!(d4 < 25.0);
    }
}
