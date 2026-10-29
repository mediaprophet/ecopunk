//! HMC Bundle Packaging Module
//!
//! Assembles multi-material .10d sectors, water surfaces, procedural textures,
//! .q42 semantic volumes, compiled .qbc VibeScript bytecode, and writes
//! `assets/packs/ecopunk-east-coast.hmc` (`QBDL`).

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

use ecopunk_terrain::{
    build_closeup_specimen_10d, build_procedural_textures, build_sector_10d, CompiledSector10d,
    ProceduralTextureEntry,
};
use qualia_core_db::bundle::BundleWriter;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub struct PackHmcResult {
    pub hmc_bytes: Vec<u8>,
    pub compiled_sectors: BTreeMap<&'static str, CompiledSector10d>,
    pub textures: Vec<ProceduralTextureEntry>,
    pub snow_gum_specimen: CompiledSector10d,
    pub kangaroo_specimen: CompiledSector10d,
}

pub fn build_and_pack_hmc_bundle(
    root: &Path,
    assets_dir: &Path,
    packs_dir: &Path,
    vibe_phase: &super::vibe_phase::VibePhaseResult,
    qudt_scale_conformance: &Value,
    webizen_hmc_manifest: &Value,
    receipt_bytes: &[u8],
    m64_buffer: &[u8],
    m64_len: usize,
) -> io::Result<PackHmcResult> {
    let textures = build_procedural_textures();
    let sector_specs = [
        ("talbingo", "Talbingo — Tumut River Gorge & Bogong Foothills", false),
        ("yarrangobilly", "Yarrangobilly — Silurian Karst Gorge & 6-Layer SLGA Strata Cutaway", true),
        ("three_mile_dam", "Three Mile Dam — Subalpine Frost Hollow & Snow Gum Basin", false),
        ("sydney_sandstone", "Sydney Basin — Hawkesbury Sandstone Escarpment & Gully", false),
        ("noosa_headland", "Noosa Headland — Subtropical Wallum Heath & Coastal Estuary", false),
        ("jounama_karst", "Jounama Creek — Bogong Peaks & Subterranean Karst", false),
    ];

    let mut compiled_sectors = BTreeMap::new();
    for (sec_id, sec_title, cutaway) in sector_specs {
        let sector = build_sector_10d(sec_id, sec_title, &textures, cutaway)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let sec_10d_path = assets_dir.join(format!("sector_{sec_id}.10d"));
        let wat_10d_path = assets_dir.join(format!("water_{sec_id}.10d"));
        fs::write(&sec_10d_path, &sector.scene_10d_bytes)?;
        fs::write(&wat_10d_path, &sector.water_10d_bytes)?;
        println!(
            "  * Sealed .10d Sector `{}`: {} verts, {} tris, {} B (CRC32C: {:#010x}), AG-12 veg instances: {}",
            sec_id,
            sector.positions.len(),
            sector.indices.len() / 3,
            sector.scene_10d_bytes.len(),
            sector.scene_crc32c,
            sector.vegetation_instances_evaluated
        );
        compiled_sectors.insert(sec_id, sector);
    }

    let snow_gum_specimen = build_closeup_specimen_10d(
        "snow_gum_loupe",
        "Botanical Loupe — Eucalyptus pauciflora & Corymbia aparrerinja 3D Falcate Foliage",
        &textures,
        false,
    ).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let kangaroo_specimen = build_closeup_specimen_10d(
        "kangaroo_tracker",
        "Zoological Tracker — Macropus giganteus & Submerged Riverbed Pebbles",
        &textures,
        true,
    ).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let vibe_dir = root.join("vibe");
    let additional_vibe_names = [
        "australia_east_coast_world",
        "talbingo_ecosystem",
        "corymbia_aparrerinja_specimen",
        "yarrangobilly_karst",
        "three_mile_dam_subalpine",
    ];

    let mut compiled_additional_vibe = Vec::new();
    for name in additional_vibe_names {
        let v_path = vibe_dir.join(format!("{name}.vibe"));
        let src = fs::read_to_string(&v_path)?;
        let prog = vibe::load_program(&src)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{name} .vibe load: {e:?}")))?;
        let chunk = vibe::bytecode::compile(&prog)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{name} .qbc compile: {e:?}")))?;
        let qbc = vibe::bytecode::encode_chunk(&chunk);
        let qbc_path = vibe_dir.join(format!("{name}.qbc"));
        fs::write(&qbc_path, &qbc)?;
        println!(
            "  * Compiled VibeScript module `{name}`: src={} B, qbc={} B",
            src.len(),
            qbc.len()
        );
        compiled_additional_vibe.push((name, src, qbc));
    }

    let drive_vibe_src = fs::read_to_string(vibe_dir.join("ecopunk_continuum_drive.vibe"))?;
    let lab_vibe_src = fs::read_to_string(vibe_dir.join("ecopunk_biological_lab.vibe"))?;
    let drive_prog = vibe::load_program(&drive_vibe_src)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("drive .vibe load: {e:?}")))?;
    let drive_chunk = vibe::bytecode::compile(&drive_prog)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("drive .qbc compile: {e:?}")))?;
    let drive_qbc = vibe::bytecode::encode_chunk(&drive_chunk);
    fs::write(vibe_dir.join("ecopunk_continuum_drive.qbc"), &drive_qbc)?;

    let lab_prog = vibe::load_program(&lab_vibe_src)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("lab .vibe load: {e:?}")))?;
    let lab_chunk = vibe::bytecode::compile(&lab_prog)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("lab .qbc compile: {e:?}")))?;
    let lab_qbc = vibe::bytecode::encode_chunk(&lab_chunk);
    fs::write(vibe_dir.join("ecopunk_biological_lab.qbc"), &lab_qbc)?;

    let mut manifest_assets = Vec::new();
    for sector in compiled_sectors.values() {
        let mut sh = Sha256::new();
        sh.update(&sector.scene_10d_bytes);
        let scene_sha = format!("{:x}", sh.finalize());
        manifest_assets.push(json!({
            "id": format!("https://ns.webizen.org/ecopunk/sector/{}", sector.sector_id),
            "file": sector.scene_hmc_key,
            "waterFile": sector.water_hmc_key,
            "bytes": sector.scene_10d_bytes.len(),
            "crc32c": sector.scene_crc32c,
            "sha256": scene_sha,
            "vertices": sector.positions.len(),
            "triangles": sector.indices.len() / 3,
        }));
    }

    let manifest_textures: Vec<Value> = textures.iter().map(|t| {
        json!({
            "name": t.name,
            "file": t.hmc_key,
            "sha256": t.sha256_hex,
            "width": t.width,
            "height": t.height,
            "bytes": t.png_bytes.len(),
        })
    }).collect();

    let hmc_manifest = json!({
        "packVersion": 1,
        "canonicalNamespace": "https://webizen.org/",
        "ontologyNamespace": "https://ns.webizen.org/",
        "packer": "crates/ecopunk-terrain (Zero-V8 Native QualiaDB + VibeScript)",
        "totalFloraSpecies": vibe_phase.flora_total,
        "totalFaunaSpecies": vibe_phase.fauna_total,
        "assets": manifest_assets,
        "textures": manifest_textures,
        "taxonBindings": vibe_phase.taxon_bindings_json,
        "semantics": [
            { "key": "east_coast_australia.q42", "quins": vibe_phase.unified_australia_quins.len(), "bytes": vibe_phase.q42_unified_bytes.len() },
            { "key": "sem/east_coast_biosphere.q42", "quins": vibe_phase.vibe_quins.len(), "bytes": vibe_phase.q42_v4_bytes.len() }
        ],
        "qudt_metric_scale_conformance": qudt_scale_conformance,
        "webizen_hmc_manifest": webizen_hmc_manifest
    });
    let manifest_bytes = serde_json::to_vec_pretty(&hmc_manifest)?;

    let mut bundle_writer = BundleWriter::new();
    bundle_writer.add_file("manifest.json", "manifest", manifest_bytes, None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter manifest: {e:?}")))?;
    bundle_writer.add_file("compilation_receipt.json", "application/json", receipt_bytes.to_vec(), None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter receipt: {e:?}")))?;

    for sector in compiled_sectors.values() {
        bundle_writer.add_file(&sector.scene_hmc_key, "10d", sector.scene_10d_bytes.clone(), None)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter 10d: {e:?}")))?;
        bundle_writer.add_file(&sector.water_hmc_key, "10d", sector.water_10d_bytes.clone(), None)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter water 10d: {e:?}")))?;
    }

    let corymbia_whole_10d = fs::read(root.join("assets").join("models").join("corymbia-aparrerinja").join("whole-organism-roots.10d"))?;
    bundle_writer.add_file("corymbia_closeup.10d", "10d", corymbia_whole_10d, None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter corymbia_closeup.10d: {e:?}")))?;
    bundle_writer.add_file("vertebrate_anatomy_macropod.10d", "10d", vibe_phase.macropod_anat.container_bytes.clone(), None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter macropod anatomy: {e:?}")))?;

    for tex in &textures {
        bundle_writer.add_file(&tex.hmc_key, "image/png", tex.png_bytes.clone(), None)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter tex: {e:?}")))?;
    }

    bundle_writer.add_file("east_coast_australia.q42", "q42", vibe_phase.q42_unified_bytes.clone(), None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter east_coast_australia.q42: {e:?}")))?;
    bundle_writer.add_file("sem/east_coast_australia.q42", "q42", vibe_phase.q42_unified_bytes.clone(), None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter sem/east_coast_australia.q42: {e:?}")))?;
    bundle_writer.add_file("sem/east_coast_biosphere.q42", "q42", vibe_phase.q42_v4_bytes.clone(), None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter q42: {e:?}")))?;

    for (name, src, qbc) in &compiled_additional_vibe {
        bundle_writer.add_file(format!("vibe/{name}.vibe"), "vibe", src.as_bytes().to_vec(), None)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter {name}.vibe: {e:?}")))?;
        bundle_writer.add_file(format!("vibe/{name}.qbc"), "qbc", qbc.clone(), None)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter {name}.qbc: {e:?}")))?;
    }

    bundle_writer.add_file("vibe/ecopunk_continuum_drive.vibe", "vibe", drive_vibe_src.into_bytes(), None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter drive.vibe: {e:?}")))?;
    bundle_writer.add_file("vibe/ecopunk_continuum_drive.qbc", "qbc", drive_qbc.clone(), None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter drive.qbc: {e:?}")))?;
    bundle_writer.add_file("vibe/ecopunk_biological_lab.vibe", "vibe", lab_vibe_src.into_bytes(), None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter lab.vibe: {e:?}")))?;
    bundle_writer.add_file("vibe/ecopunk_biological_lab.qbc", "qbc", lab_qbc.clone(), None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter lab.qbc: {e:?}")))?;

    let wasm_stub = b"\0asm\x01\0\0\0".to_vec();
    for (wasm_json_key, wasm_val) in &[
        ("wasm/terrain_cog_hydrator.wasm.json", json!({"$schema": "https://ns.webizen.org/cap/v1", "capability_iri": "https://ns.webizen.org/cap#TerrainCogHydrator"})),
        ("wasm/sdf_marching_cubes_kernel.wasm.json", json!({"$schema": "https://ns.webizen.org/cap/v1", "capability_iri": "https://ns.webizen.org/cap#SdfMarchingCubesKernel"})),
        ("wasm/holosonic_wavefield_dsp.wasm.json", json!({"$schema": "https://ns.webizen.org/cap/v1", "capability_iri": "https://ns.webizen.org/cap#HolosonicWavefieldDsp"}))
    ] {
        bundle_writer.add_file(*wasm_json_key, "application/json", serde_json::to_vec_pretty(wasm_val)?, None)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter {wasm_json_key}: {e:?}")))?;
        bundle_writer.add_file(wasm_json_key.trim_end_matches(".json"), "application/wasm", wasm_stub.clone(), None)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter wasm: {e:?}")))?;
    }

    let m64_meta = json!({
        "srid": qualia_geospatial::SRID_GDA2020, "geodetic_datum": "EPSG:7844 GDA2020", "vertical_datum": "EPSG:5711 AHD",
        "cell_size_mm": 30_000, "chunk_width": 64, "chunk_height": 64, "chunk_count": 1, "total_records": 4096,
        "origin_lat_deg": -35.5854, "origin_lon_deg": 148.3240, "origin_elevation_mm": 385_000
    });
    bundle_writer.add_file("talbingo_catchment.m64", "m64", m64_buffer[..m64_len].to_vec(), None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter talbingo_catchment.m64: {e:?}")))?;
    bundle_writer.add_file("m64/continuum_metadata.json", "application/json", serde_json::to_vec_pretty(&m64_meta)?, None)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter m64/continuum_metadata.json: {e:?}")))?;

    let hmc_bytes = bundle_writer.build()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter build: {e}")))?;
    let hmc_out_path = packs_dir.join("ecopunk-east-coast.hmc");
    fs::write(&hmc_out_path, &hmc_bytes)?;
    println!("[OK] Built & Wrote .hmc Bundle: {} ({} bytes)", hmc_out_path.display(), hmc_bytes.len());

    Ok(PackHmcResult {
        hmc_bytes,
        compiled_sectors,
        textures,
        snow_gum_specimen,
        kangaroo_specimen,
    })
}
