//! compile_east_coast_substrate
//!
//! Executes Phase 2, Phase 3, Phase 4, Phase 5, Phase 6, and Phase 7 of:
//! `docs/plans/east-coast-australia-ecological-dataset-acquisition-plan.md` &
//! `docs/plans/aaa-holo-continuum-and-300-flora-164-fauna-plan.md`
//!
//! Synthesizes native QualiaDB & VibeScript artifacts (100% Rust, Zero Python, Zero V8):
//! 1. `.q42` Semantic Volumes: Compiled 48-byte `NQuin` records from `.ttl` ontologies & VibeScript.
//! 2. 16-KB GPU `ConstituentLUT`: Precompiled 256 x 64B flat binary material table (`BankedConstituentLut`).
//! 3. `.m64` Matter-64 Container: Mode 0x02 strata column chunk for the Talbingo catchment.
//! 4. `.10d` Manifold Containers: Multi-material scenes & water surfaces.
//! 5. `.hmc` (`QBDL`) Verified Bundle (`assets/packs/ecopunk-east-coast.hmc`) via `BundleWriter`.
//! 6. `PortalGpu` & `VolumetricRenderer` Native `wgpu` Offscreen Rendering to `assets/renders/*.png`.

use std::fs;
use std::io;
use std::path::PathBuf;

use qualia_core_db::q42::m64_matter::banked_lut::{BankedConstituentLut, LUT_BUFFER_BYTES};
use qualia_core_db::q42::q42_volume::WasmSliceQ42Reader;

#[path = "substrate_phases/mod.rs"]
mod substrate_phases;

pub trait LocalHostQ42PackExt {
    fn with_q42_pack(bytes: &[u8]) -> io::Result<Self>
    where
        Self: Sized;
}

impl LocalHostQ42PackExt for vibe::LocalHost {
    fn with_q42_pack(bytes: &[u8]) -> io::Result<Self> {
        let _reader = WasmSliceQ42Reader::open(bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Q42 slice open failed: {e:?}")))?;
        Ok(vibe::LocalHost::default())
    }
}

fn find_workspace_root() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    while !dir.join("Cargo.toml").exists() || !dir.join("data").exists() {
        if let Some(parent) = dir.parent() {
            dir = parent.to_path_buf();
        } else {
            break;
        }
    }
    dir
}

fn main() -> io::Result<()> {
    println!("============================================================");
    println!("🌿 EcoPunk Native Qualia Substrate & WGSL Forge Compiler");
    println!("============================================================");

    let args: Vec<String> = std::env::args().collect();
    let allow_synthetic_fixture = args.iter().any(|arg| arg == "--allow-synthetic-fixture");

    let root = find_workspace_root();
    let data_dir = root.join("data");
    let ontologies_dir = data_dir.join("ontologies");
    let assets_dir = root.join("assets").join("terrain");
    let packs_dir = root.join("assets").join("packs");
    fs::create_dir_all(&assets_dir)?;
    fs::create_dir_all(&packs_dir)?;

    // ------------------------------------------------------------------------
    // Phase 2: Compile ALL Turtle Ontologies in data/ontologies/ into .q42
    // ------------------------------------------------------------------------
    println!("\n--- Phase 2: Semantic Compilation to .q42 Volumes ---");
    let (total_quins, ttl_count) = substrate_phases::turtle_ingest::compile_all_ontologies(
        &ontologies_dir,
        &assets_dir,
    )?;
    println!("  * Loaded {} Turtle files producing {} NQuins", ttl_count, total_quins.len());

    // ------------------------------------------------------------------------
    // Phase 3: Biophysical Enrichment & 16-KB GPU ConstituentLUT
    // ------------------------------------------------------------------------
    println!("\n--- Phase 3: 16-KB GPU ConstituentLUT Compilation ---");
    let lut = BankedConstituentLut::new_with_terrestrial_defaults();
    let lut_bytes: &[u8] = bytemuck::bytes_of(&lut.entries);
    assert_eq!(lut_bytes.len(), LUT_BUFFER_BYTES, "ConstituentLUT must be exact 16 KB");
    let lut_out_path = assets_dir.join("constituent_lut_16kb.bin");
    fs::write(&lut_out_path, lut_bytes)?;
    println!(
        "[OK] Compiled 16-KB GPU ConstituentLUT: {} ({} bytes, 256 x 64B entries)",
        lut_out_path.display(),
        lut_bytes.len()
    );

    // ------------------------------------------------------------------------
    // Phase 4: Fail-Closed DEM Elevation Admission & Runtime Strata (.m64 & .10d)
    // ------------------------------------------------------------------------
    println!("\n--- Phase 4: Fail-Closed DEM Elevation Admission (.m64 & .10d) ---");
    let elev_bin_path = data_dir.join("geospatial").join("talbingo_64x64_elev_mm.bin");
    let (elev_mm, dem_receipt) = substrate_phases::dem_admission::load_or_admit_dem_elevations(
        &elev_bin_path,
        qualia_geospatial::CHUNK_CELL_COUNT,
        allow_synthetic_fixture,
    )?;
    println!(
        "  * Admitted DEM elevation: {} cells, status={}, min={} mm, max={} mm",
        dem_receipt.cell_count,
        dem_receipt.acceptance_status,
        dem_receipt.min_elevation_mm,
        dem_receipt.max_elevation_mm
    );
    let compiled_terrain = substrate_phases::dem_admission::compile_terrain_strata_and_10d(
        &elev_mm,
        &assets_dir,
        dem_receipt,
    )?;
    println!(
        "[OK] Compiled .m64 Strata & .10d Heightfield: {} ({} bytes)",
        compiled_terrain.m64_path.display(),
        compiled_terrain.m64_len
    );

    // ------------------------------------------------------------------------
    // Phase 5: WGSL Forge & Viewport Shader Library Naga Certification
    // ------------------------------------------------------------------------
    println!("\n--- Phase 5: WGSL Forge & Viewport Shader Library Naga Certification ---");
    let _forge_manifest = substrate_phases::wgsl_pipeline::validate_and_emit_wgsl_forge_manifest(
        &assets_dir,
        lut_bytes,
    )?;

    // ------------------------------------------------------------------------
    // Phase 6: VibeScript (.vibe) Certification, Interpreter Evaluation (`PoetSnapshot`),
    //          SHACL Validation & 305-Flora / 164-Fauna Catalogs
    // ------------------------------------------------------------------------
    println!("\n--- Phase 6: VibeScript Certification, Interpreter Execution & 305-Flora / 164-Fauna Compilation ---");
    let vibe_phase = substrate_phases::vibe_phase::execute_vibe_biosphere_phase(
        &root,
        &data_dir,
        &assets_dir,
        &total_quins,
    )?;

    // ------------------------------------------------------------------------
    // Phase 7: Fail-Closed QUDT Metric Scale Conformance, .hmc Packaging & Offscreen Rendering
    // ------------------------------------------------------------------------
    println!("\n--- Phase 7: QUDT Metric Scale Conformance & .hmc Packaging ---");
    let dem_receipt_val = serde_json::to_value(&compiled_terrain.receipt)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let (qudt_scale_conformance, webizen_hmc_manifest, receipt_bytes) =
        substrate_phases::scale_conformance::verify_qudt_scale_and_build_receipt(
            &root,
            &assets_dir,
            &packs_dir,
            &dem_receipt_val,
        )?;

    let packed = substrate_phases::hmc_packing::build_and_pack_hmc_bundle(
        &root,
        &assets_dir,
        &packs_dir,
        &vibe_phase,
        &qudt_scale_conformance,
        &webizen_hmc_manifest,
        &receipt_bytes,
        &compiled_terrain.m64_buffer,
        compiled_terrain.m64_len,
    )?;

    println!("\n--- Phase 7B: .hmc Verification & Zero-V8 PortalGpu Rendering ---");
    substrate_phases::hmc_verification::execute_hmc_verification_and_rendering(
        &root,
        &packed,
        &vibe_phase,
    )?;

    println!("\n============================================================");
    println!("✅ 100% Zero-V8 Native QualiaDB + VibeScript + .hmc/.10d/.q42/.m64 + PortalGpu Pipeline Complete.");
    println!("============================================================");

    Ok(())
}
