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

/// Refuse a reused revision before any artifact phases can overwrite files.
/// This is an ordinary preflight, not a lock against concurrent external writers.
fn require_empty_output_directory(path: &std::path::Path) -> io::Result<()> {
    fs::create_dir_all(path)?;
    if fs::read_dir(path)?.next().transpose()?.is_some() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!(
            "Output revision directory is not empty: {}. Select a new --terrain-output-dir and --pack-output-dir; existing work will not be deleted.", path.display())));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    println!("============================================================");
    println!("🌿 EcoPunk Native Qualia Substrate & WGSL Forge Compiler");
    println!("============================================================");

    let args: Vec<String> = std::env::args().collect();
    let allow_synthetic_fixture = args.iter().any(|arg| arg == "--allow-synthetic-fixture");
    let with_terrain_surface = args.iter().any(|arg| arg == "--terrain-surface");
    let dem_m64_path = if let Some(index) = args.iter().position(|arg| arg == "--dem-m64") {
        let path = args.get(index + 1).filter(|arg| !arg.starts_with("--"))
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "--dem-m64 requires a canonical v3 M64 file path"))?;
        Some(PathBuf::from(path))
    } else { None };
    let dem_source_licence = if let Some(index) = args.iter().position(|arg| arg == "--dem-source-license") {
        args.get(index + 1).filter(|arg| !arg.starts_with("--") && !arg.trim().is_empty() && arg.len() <= 1024)
            .map(String::as_str)
    } else { None };
    if dem_m64_path.is_some() && dem_source_licence.is_none() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput,
            "--dem-m64 also requires --dem-source-license <source licence statement>; source rights are not inferred"));
    }
    if dem_m64_path.is_none() && !allow_synthetic_fixture {
        return Err(io::Error::new(io::ErrorKind::InvalidInput,
            "Supply --dem-m64 <source.m64> with explicit raster/frame metadata; --allow-synthetic-fixture is only for unaccepted legacy diagnostics"));
    }
    if with_terrain_surface && dem_m64_path.is_none() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput,"--terrain-surface requires an explicitly framed --dem-m64 source"));
    }

    let root = find_workspace_root();
    let data_dir = root.join("data");
    let ontologies_dir = data_dir.join("ontologies");
    // Select a new revision directory without deleting prior source/output.
    // The same path is passed to the downstream compiler phases and packer.
    let assets_dir = if let Some(index) = args.iter().position(|arg| arg == "--terrain-output-dir") {
        let value = args.get(index + 1).filter(|arg| !arg.starts_with("--") && !arg.is_empty() && arg.len() <= 4096)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "--terrain-output-dir requires a bounded directory path"))?;
        PathBuf::from(value)
    } else { root.join("assets").join("terrain") };
    let packs_dir = if let Some(index) = args.iter().position(|arg| arg == "--pack-output-dir") {
        let value = args.get(index + 1).filter(|arg| !arg.starts_with("--") && !arg.is_empty() && arg.len() <= 4096)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "--pack-output-dir requires a bounded directory path"))?;
        PathBuf::from(value)
    } else { root.join("assets").join("packs") };
    require_empty_output_directory(&assets_dir)?;
    require_empty_output_directory(&packs_dir)?;

    // Admit the terrain source before writing ontology/LUT phase outputs.
    // ------------------------------------------------------------------------
    // Phase 4: Fail-Closed DEM Elevation Admission & Runtime Strata (.m64 & .10d)
    // ------------------------------------------------------------------------
    println!("\n--- Phase 4: Fail-Closed DEM Elevation Admission (.m64 & .10d) ---");
    let compiled_terrain = if let Some(path) = dem_m64_path {
        substrate_phases::dem_admission::compile_terrain_from_m64(&path, &assets_dir,
            dem_source_licence.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Source licence missing"))?, with_terrain_surface)?
    } else {
        // Explicitly synthetic mode never upgrades a raw integer file into a
        // measured/georeferenced source merely because one happens to exist.
        let (samples, receipt) = substrate_phases::dem_admission::synthetic_dem_fixture(
            qualia_geospatial::CHUNK_CELL_COUNT)?;
        substrate_phases::dem_admission::compile_terrain_strata_and_10d(&samples, &assets_dir, receipt)?
    };
    println!(
        "  * Admitted DEM elevation: {} cells, status={}, min={} mm, max={} mm",
        compiled_terrain.receipt.cell_count,
        compiled_terrain.receipt.acceptance_status,
        compiled_terrain.receipt.min_elevation_mm,
        compiled_terrain.receipt.max_elevation_mm
    );
    println!(
        "[OK] Compiled .m64 Strata & .10d Heightfield: {} ({} bytes)",
        compiled_terrain.m64_path.display(),
        compiled_terrain.m64_len
    );

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
        &compiled_terrain,
    )?;

    println!("\n--- Phase 7B: .hmc Verification & Zero-V8 PortalGpu Rendering ---");
    substrate_phases::hmc_verification::execute_hmc_verification_and_rendering(
        &assets_dir.join("renders"),
        &packed,
        &vibe_phase,
    )?;

    println!("\n============================================================");
    println!("Compiler run completed for the selected output revision.");
    println!("Asset quality, source rights/frame accuracy, delivery profiles and human acceptance remain separate review requirements.");
    println!("Generated illustrative scene sectors do not adopt the admitted terrain source.");
    println!("Terrain revision: {}", assets_dir.display());
    println!("Pack revision: {}", packs_dir.display());
    println!("============================================================");

    Ok(())
}

#[cfg(test)]
mod output_revision_tests {
    use super::*;

    #[test]
    fn existing_revision_is_refused_without_modifying_prior_receipt() {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!("ecopunk-output-revision-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        fs::create_dir(&dir).unwrap();
        let receipt = dir.join("compilation_receipt.json");
        fs::write(&receipt, b"previous revision").unwrap();
        assert_eq!(require_empty_output_directory(&dir).unwrap_err().kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(fs::read(&receipt).unwrap(), b"previous revision");
        fs::remove_file(receipt).unwrap();
        require_empty_output_directory(&dir).unwrap();
        fs::remove_dir(dir).unwrap();
    }
}
