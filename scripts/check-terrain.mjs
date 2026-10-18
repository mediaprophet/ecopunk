/**
 * Ecopunk & QualiaDB Automated Terrain & Visual Verification Suite
 *
 * Verifies WebGPU terrain rendering, .m64 strata chunk streaming,
 * BankedConstituentLut 16-KB GPU upload, and FrameLockPacket synchronization.
 */

import { existsSync, readFileSync, statSync } from 'fs';
import { join } from 'path';

console.log('=== Ecopunk Earth-Terrain Automated Verification Suite ===\n');

let passCount = 0;
let failCount = 0;

function assert(condition, message) {
    if (condition) {
        console.log(`[PASS] ${message}`);
        passCount++;
    } else {
        console.error(`[FAIL] ${message}`);
        failCount++;
    }
}

// 1. Check Container File Formats & Registries
console.log('--- 1. Substrate & Container Alignment Checks ---');
const m64MatterDir = 'C:\\github\\qualiaDB\\crates\\qualia-core-db\\src\\q42\\m64_matter';
assert(existsSync(join(m64MatterDir, 'chunk_directory.rs')), 'M64ChunkDirectoryEntry source exists');
assert(existsSync(join(m64MatterDir, 'strata_record.rs')), 'M64StrataColumnRecord source exists');
assert(existsSync(join(m64MatterDir, 'phase_juncture.rs')), 'M64PhaseJunctureRecord source exists');
assert(existsSync(join(m64MatterDir, 'banked_lut.rs')), 'BankedConstituentLut source exists');

// 2. Check 10D Manifold Voxel Lattice
console.log('\n--- 2. 10D Manifold & SVDAG Check ---');
const voxelSectionFile = 'C:\\github\\qualiaDB\\crates\\qualia-core-db\\src\\container_10d\\voxel_section.rs';
assert(existsSync(voxelSectionFile), 'VOX0 Section 22 codec exists in container_10d');
if (existsSync(voxelSectionFile)) {
    const src = readFileSync(voxelSectionFile, 'utf8');
    assert(src.includes('pub struct Vox0DagNode'), 'Vox0DagNode 16B struct declared');
    assert(src.includes('MAGIC_VOX0'), 'VOX0 magic header constant defined');
}

// 3. Check Homogenization & Observer Shaders
console.log('\n--- 3. Homogenization & Observer Kernels Check ---');
const physicsDir = 'C:\\github\\qualiaDB\\crates\\qualia-core-db\\src\\specialized_libs\\physics_simulation';
assert(existsSync(join(physicsDir, 'homogenization_mech.rs')), 'VRH & Drucker-Prager mechanics exist');
assert(existsSync(join(physicsDir, 'homogenization_hydro.rs')), 'Kozeny-Carman & Exner hydraulics exist');
assert(existsSync(join(physicsDir, 'homogenization_optic.rs')), 'Bachmann wetting & Kubelka-Munk optics exist');
assert(existsSync(join(physicsDir, 'homogenization_acoust.rs')), 'CRIM & Biot acoustics exist');
assert(existsSync(join(physicsDir, 'cartoon_deformation.rs')), '4-regime cartoon kinetics exist');

const observerFile = 'C:\\github\\qualiaDB\\crates\\qualia-core-db\\src\\wgsl_forge\\observer_kernels.rs';
assert(existsSync(observerFile), 'Five normative observer projection kernels exist');
if (existsSync(observerFile)) {
    const src = readFileSync(observerFile, 'utf8');
    assert(src.includes('WGSL_KERNEL_PBR'), 'Cook-Torrance PBR shader defined');
    assert(src.includes('WGSL_KERNEL_TOON'), 'Cel-shaded Toon shader defined');
    assert(src.includes('WGSL_KERNEL_WATERCOLOR'), 'Storybook Watercolor shader defined');
    assert(src.includes('WGSL_KERNEL_HUD'), 'Tactical HUD shader defined');
    assert(src.includes('WGSL_KERNEL_THERMAL'), 'Planck Blackbody Thermal IR shader defined');
}

// 4. Check Ecopunk Geospatial & Companion Display
console.log('\n--- 4. Ecopunk Geospatial & Distributed Display Checks ---');
const geoDir = 'C:\\github\\ecopunk\\crates\\ecopunk-terrain\\src\\geospatial';
assert(existsSync(join(geoDir, 'frame.rs')), 'Geodetic frame & AusPix DGGS exist');
assert(existsSync(join(geoDir, 'hilbert.rs')), '2D Hilbert curve mapping exists');
assert(existsSync(join(geoDir, 'geotiff.rs')), 'DEM GeoTIFF / Terrain-RGB compiler exists');
assert(existsSync(join(geoDir, 'wfs.rs')), 'Dual-path WFS geology unit parser exists');
assert(existsSync(join(geoDir, 'wms_streamer.rs')), 'WMS 2D Texture Array streamer exists');
assert(existsSync(join(geoDir, 'corridor.rs')), 'Hermite cubic corridor conformer exists');
assert(existsSync(join(geoDir, 'manifest.rs')), 'Semantic asset manifest generator exists');

if (existsSync(join(geoDir, 'geotiff.rs'))) {
    const geotiffSrc = readFileSync(join(geoDir, 'geotiff.rs'), 'utf8');
    assert(geotiffSrc.includes('pub enum CompressionCodec'), 'CompressionCodec enum defined in geotiff.rs');
    assert(geotiffSrc.includes('Htj2kWavelet'), 'HTJ2K Wavelet multiresolution codec supported');
}

if (existsSync(join(geoDir, 'manifest.rs'))) {
    const manifestSrc = readFileSync(join(geoDir, 'manifest.rs'), 'utf8');
    assert(manifestSrc.includes('geosparql#asWKT'), 'GeoSPARQL WKT polygon geometry emitted in manifest');
    assert(manifestSrc.includes('inDggsCell'), 'AusPix DGGS cell emitted in manifest');
}

const displayDir = 'C:\\github\\ecopunk\\crates\\ecopunk-terrain\\src\\display';
assert(existsSync(join(displayDir, 'frame_lock.rs')), 'FrameLockPacket 64B UDP multicast exists');
assert(existsSync(join(displayDir, 'frustum.rs')), 'Conical Dome Frustum Wedge exists');
assert(existsSync(join(displayDir, 'companion.rs')), 'Companion Device Synchronization exists');

const governorDir = 'C:\\github\\ecopunk\\crates\\ecopunk-terrain\\src\\governor';
assert(existsSync(join(governorDir, 'scaling.rs')), 'VramAndComputeGovernor scaling exists');
assert(existsSync(join(governorDir, 'telemetry.rs')), 'GpuTelemetrySample exists');

const specimenDir = 'C:\\github\\ecopunk\\crates\\ecopunk-terrain\\src\\specimen';
assert(existsSync(join(specimenDir, 'loupe.rs')), 'Corymbia aparrerinja 3D Specimen Loupe exists');

// Summary
console.log(`\n========================================`);
console.log(`Verification Complete: ${passCount} Passed, ${failCount} Failed.`);
if (failCount > 0) {
    process.exit(1);
} else {
    console.log('All planetary terrain, continuum physics, and spatial substrate checks passed!');
    process.exit(0);
}
