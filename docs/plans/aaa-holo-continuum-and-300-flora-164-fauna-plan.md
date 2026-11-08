# EcoPunk AAA Holographic Continuum & Ecological Digital Twin Master Implementation Plan

## 1. Executive Summary & Root-Cause Audit

A full technical audit of `C:\github\ecopunk` and `C:\github\qualiaDB` (commits `fc870ae0`, `ade92ed4`, `f9958c5e`, `1b9dcb08`, `0d324dbb`, `b7c40458`, `7bfc84d4`, `5395fb7b`, `237ad881`, `bc9a8683`, `c3a870be`) identified why previous work failed to appear in the rendered output:

1. **Legacy Python Scripts (`scripts/*.py`) Bypassed `qualiaDB` & `qualia-sim`**:
   - Five legacy Python scripts (~270 KB) generated low-poly placeholder JSONs and caused GitHub Linguist to classify the repository as Python instead of Rust + VibeScript.
2. **Unused Engine Capabilities in `qualia-core-db`, `qualia-sim`, and `qualia-geospatial`**:
   - **Leaf & Plant Biophysics (`qualia-sim::leaf_biophysics`, `metamer_fspm`, `flora_archetype`, `qualia-core-db::modalities::botany`)**: Dual-graph vascular venation (`LeafVascularSkeleton`, Murray's law $r_p^3 = \sum r_c^3$, Hagen-Poiseuille conductance, Leaf Economics Spectrum `LeafEconomicsProfile`, turgor droop), Shinozaki Pipe Model (`MetamerPlantGraph`), Raunkiær life-form mechanics (`FloraBiomechanicsEvaluator`), and PlantGL computational geometry (`space_colonization_3d`, `pipe_model_radii`, `turtle_3d`, `compute_bishop_frames`, `asymmetric_hull_into`) were built in `qualiaDB` but never wired into `ecopunk`'s mesh generator.
   - **Existing High-Resolution `.10d` Botanical Assets Ignored**: `entities/taxa/corymbia-aparrerinja/` and `assets/models/corymbia-aparrerinja/10d/` already contain `corymbia-aparrerinja.vibe` and **115 multi-tier `.10d` meshes** (including `chalk-bark`, `branchwork`, `irregular-canopy`, `falcate-foliage`, `cream-buds`, `epicormic-shoots`, `leaf-study-blade.10d`, `leaf-study-midrib.10d`, `flower-study-stamens.10d`, `gumnut-study-capsule.10d`), which were completely unreferenced by the viewer.
   - **Ground Cover, Grasses & SGGX Microflakes (`FOLIAGE_LOD_WGSL`, `SGGX_MICROFLAKE_WGSL`, `qualia-sim::ground_cover::ToroidalGroundCoverClipmap`)**: The 4-tier continuous ground cover system (LOD 0 Cosserat cubic Bézier blades $0\text{--}8\text{m}$, LOD 1 clump ribbons $8\text{--}25\text{m}$, LOD 2 volumetric shells $25\text{--}80\text{m}$, LOD 3 SGGX anisotropic microflake BRDF $>80\text{m}$) with turgor droop and trample deformation was unused.
   - **Rocks, Boulders, Pebbles, Cobbles, Sand & 6-Layer Soil (`BankedConstituentLut`, `M64StrataColumnRecord`, `qualia-sim::geomechanics`)**: The 16-KB `BankedConstituentLut` (Bank 0 canonical materials + Bank 1 WFS soil recipes) and Krumbein $\phi$ grain-size statistics (`grain_phi_stats`) for granite tors, columnar basalt, karst rillenkarren, river cobbles, pebbles, quartz sand, and 6 soil horizons ($O, A, E, B, C, R$) were not synthesized into visible 3D rocks, scree, gravel bars, and underwater riverbed pebbles.
   - **Opaque Water Instead of Refractive Bio-Geo-Optical Water (`HYDROLOGIC_OPTICS_WGSL`, `SWE_SOLVER_WGSL`, `qualia-sim::optics::BioGeoOpticalConstituents`)**: Water was rendered as an opaque surface rather than a transparent refractive volume ($n = 1.333$ Snell refraction, Cauchy dispersion, Beer-Lambert spectral transmittance $\exp(-c(\lambda)d)$, solar caustics projected onto the submerged riverbed pebbles/sand, and Ångström/Bachmann shoreline capillary wetting darkening).
   - **Small `120m × 120m` Patch Instead of Google-Earth-Scale GIS/DGGS Continuum (`qualia-geospatial`)**: `GeodeticFrame` (`SRID_GDA2020 = 7844`, `VERTICAL_DATUM_AHD = 5711`), AusPix DGGS, Hilbert `.m64` terrain compilation (`compile_raster_to_m64`), `HermiteCorridorSegment` (roads, bush tracks, incised creek beds), and multi-kilometer CDLOD clipmaps + $60\text{km}$ radial Hermite mountain horizons were not exposed as a continuous playable landscape.

---

## 2. Phased Implementation Roadmap

### Phase 1: Zero-Python Purge & Native Rust + VibeScript (`.vibe`) Pipeline
- Remove all Python scripts (`scripts/*.py`) so the repository is 100% Rust, VibeScript, WGSL, and WebGPU JS.
- Wire `qualia-sim` and `qualia-geospatial` into `ecopunk-terrain`.
- Author comprehensive `.vibe` modules (`vibe/east_coast_flora_305.vibe`, `vibe/east_coast_fauna_164.vibe`, `vibe/lithology_pedology_hydrology.vibe`) alongside `entities/taxa/corymbia-aparrerinja/corymbia-aparrerinja.vibe`.

### Phase 2: Google-Earth-Scale Playable GIS/DGGS Continuum
- Anchor all 8 East Coast Australian regions in `GDA2020` (`EPSG:7844`), `AHD` (`EPSG:5711`), and `AusPix DGGS` using `qualia-geospatial`.
- Expand the playable terrain continuum from `120m × 120m` to a multi-kilometer continuous basin (`2,400m × 2,400m` high-resolution playable basin + `24 km` surrounding mountain range skirt) with multi-resolution CDLOD rings, `HermiteCorridorSegment` incised gorges/tracks, and seamless orbital-to-millimeter camera zoom.

### Phase 3: Lithological Rocks, Boulders, Pebbles, Cobbles, Sand & 6-Layer Soil Horizons
- Generate 3D granite tors, columnar basalt cliffs, Hawkesbury sandstone ledges, Silurian karst limestone outcrops (*rillenkarren*), talus scree, and exposed gravel bars.
- Generate hydrodynamically imbricated riverbed cobbles, variegated river pebbles (Krumbein $\phi \in [-2, -6]$), and alluvial quartz sand ripples ($\phi \in [1, 3]$) driven by the 16-KB `BankedConstituentLut`.
- Render distinct 6-layer soil horizons ($O$ humic root mat, $A$ biomantle loam, $E$ eluvial sand, $B$ illuvial ferric clay, $C$ saprolite, $R$ bedrock) both on surface exposures and in geological cutaways.

### Phase 4: Transparent Refractive Water & Submerged Benthic Ecosystem
- Implement dual-pass transparent refractive water using `HYDROLOGIC_OPTICS_WGSL` and `qualia_sim::optics::BioGeoOpticalConstituents`:
  - Snell's law refraction ($n = 1.333$) and wave-normal perturbation (`SimdGerstnerSpectrum`).
  - Beer-Lambert depth-dependent spectral transmittance $T(\lambda, d) = \exp(-c(\lambda)d)$ with CDOM/tannin and turbidity profiles.
  - Animated underwater solar caustics projected onto the submerged riverbed pebbles, cobbles, and sand ripples.
  - Ångström/Bachmann shoreline capillary wetting darkening along riverbanks and emergent boulders.
  - Visible underwater benthic flora (*Vallisneria*, *Myriophyllum*) and fauna (Platypus, Murray Cod, Long-Necked Turtle, Murray Crayfish).

### Phase 5: Botanical & Leaf Biophysics Overhaul — 305+ Flora Species (+200 New + Upgraded Existing)
- Integrate the 115 `.10d` anatomical sub-meshes of `Corymbia aparrerinja` (`chalk-bark`, `branchwork`, `irregular-canopy`, `falcate-foliage`, `cream-buds`, `epicormic-shoots`, `leaf-study-blade.10d`, `leaf-study-midrib.10d`, `flower-study-stamens.10d`, `gumnut-study-capsule.10d`).
- Upgrade all 105 existing flora and add 200 new East Coast flora species (**305+ total**) across 7 vertical strata using `LeafVascularSkeleton` venation, `MetamerPlantGraph` Shinozaki pipe model, and PlantGL `computational_geometry` (`space_colonization_3d`, `compute_bishop_frames`, `turtle_3d`).
- Populate dense understory shrubs, tree ferns, *Xanthorrhoea* grass trees, sedges, rushes, and Cosserat cubic Bézier tussock grasses (`FOLIAGE_LOD_WGSL` + `SGGX_MICROFLAKE_WGSL`).

### Phase 6: 164+ Fauna Species (+100 New + Upgraded Existing) with Articulated Anatomy & Ethograms
- Upgrade all 64 existing fauna and add 100 new Australian fauna species (**164+ total** across mammals, birds, reptiles, amphibians, freshwater fish, and macro-invertebrates) with multi-part $C^1$ smooth-minimum SDF anatomy, Bishop-frame limbs/wings/tails, `SKL1` skeletal joints, PGA motor animation curves, and 16-state ethograms.

### Phase 7: End-to-End Automated & Visual Verification + GitHub Pages Deployment
- Verify all gates via `cargo test` and `node scripts/check-terrain.mjs`, capture visual proof screenshots across macro/meso/micro/underwater views, and deploy to `https://mediaprophet.github.io/ecopunk/`.
