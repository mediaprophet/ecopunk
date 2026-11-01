//! HMC Bundle Packaging Module
//!
//! Assembles multi-material .10d sectors, water surfaces, procedural textures,
//! .q42 semantic volumes, compiled .qbc VibeScript bytecode, and writes
//! `assets/packs/ecopunk-east-coast.hmc` (`QBDL`).

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
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
    terrain: &super::dem_admission::CompiledTerrainResult,
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

    add_terrain_resources(&mut bundle_writer, terrain)?;

    let hmc_bytes = bundle_writer.build()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleWriter build: {e}")))?;
    let hmc_out_path = packs_dir.join("ecopunk-east-coast.hmc");
    publish_final_hmc_no_clobber(&hmc_out_path, &hmc_bytes)?;
    println!("[OK] Built & Wrote .hmc Bundle: {} ({} bytes)", hmc_out_path.display(), hmc_bytes.len());

    Ok(PackHmcResult {
        hmc_bytes,
        compiled_sectors,
        textures,
        snow_gum_specimen,
        kangaroo_specimen,
    })
}

/// Preserve any prior final bundle, including a file created since preflight.
/// Only a newly created destination belongs to this call's error cleanup.
fn publish_final_hmc_no_clobber(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut output = fs::OpenOptions::new().write(true).create_new(true).open(path)?;
    let result = output.write_all(bytes).and_then(|()| output.sync_all());
    drop(output);
    if let Err(error) = result {
        return match fs::remove_file(path) {
            Ok(()) => Err(error),
            Err(cleanup) => Err(io::Error::new(error.kind(), format!(
                "Final HMC publication failed: {error}; newly created partial output {} could not be removed: {cleanup}", path.display()))),
        };
    }
    Ok(())
}

/// Bind exact compiler products, not the historical hardcoded Talbingo frame.
/// The pre-existing JSON member remains a metadata projection, with an explicit
/// schema because its old anonymous fixed-frame shape could mislabel inputs.
fn add_terrain_resources(writer: &mut BundleWriter, terrain: &super::dem_admission::CompiledTerrainResult) -> io::Result<()> {
    let bad = |message: &str| io::Error::new(io::ErrorKind::InvalidData, message.to_owned());
    let source = terrain.m64_buffer.get(..terrain.m64_len).ok_or_else(|| bad("Terrain source length mismatch"))?;
    let output = terrain.receipt.outputs.as_ref().ok_or_else(|| bad("Terrain output receipt missing"))?;
    let source_sha = format!("{:x}", Sha256::digest(source));
    let ten_d_sha = format!("{:x}", Sha256::digest(&terrain.ten_d_buffer));
    if source_sha != output.m64_sha256 || ten_d_sha != output.ten_d_sha256 {
        return Err(bad("Terrain resource bytes no longer match compiler receipt"));
    }
    if !terrain.receipt.is_synthetic_fixture && terrain.receipt.source_sha256.as_ref() != Some(&source_sha) {
        return Err(bad("Terrain source revision disagrees with admitted M64"));
    }
    qualia_core_db::container_10d::integrity::check_whole_file_crc32c(&terrain.ten_d_buffer)
        .map_err(|e| bad(&format!("Terrain .10d CRC refused: {e:?}")))?;
    let header = qualia_core_db::container_10d::Container10dHeader::parse(&terrain.ten_d_buffer)
        .map_err(|e| bad(&format!("Terrain .10d header refused: {e:?}")))?;
    let sections = qualia_core_db::container_10d::parse_section_table(&terrain.ten_d_buffer, &header)
        .map_err(|e| bad(&format!("Terrain .10d sections refused: {e:?}")))?;
    if !terrain.receipt.is_synthetic_fixture {
        use qualia_core_db::container_10d::{SectionType, provenance_section::{decode_provenance_section, validate_provenance}};
        let section = sections.iter().find(|s| s.typ() == Some(SectionType::ProvenanceSidecar))
            .ok_or_else(|| bad("Terrain .10d lacks in-envelope source M64"))?;
        let payload = &terrain.ten_d_buffer[section.byte_offset as usize..(section.byte_offset + section.byte_length) as usize];
        let provenance = decode_provenance_section(payload).map_err(|e| bad(&format!("Terrain provenance refused: {e:?}")))?;
        validate_provenance(&provenance).map_err(|e| bad(&format!("Terrain source provenance refused: {e:?}")))?;
        let expected: [u8; 32] = Sha256::digest(source).into();
        if provenance.source_bytes() != source || provenance.version_hash() != &expected {
            return Err(bad("Terrain .10d provenance does not bind the exact packed M64"));
        }
    }
    match (&terrain.surface,&terrain.receipt.surface) {
        (None,None) => {},
        (Some(surface),Some(receipt)) => {
            if format!("{:x}",Sha256::digest(&surface.bytes)) != receipt.sha256 || receipt.source_sha256 != source_sha {
                return Err(bad("Terrain surface bytes/source no longer match derivative receipt"));
            }
            use qualia_core_db::container_10d::{Container10dHeader,SectionType,parse_section_table,
                provenance_section::{decode_provenance_section,validate_provenance}};
            qualia_core_db::render::compile_10d::decode_10d_mesh(&surface.bytes)
                .map_err(|e| bad(&format!("Terrain renderable surface refused: {e:?}")))?;
            let header = Container10dHeader::parse(&surface.bytes).map_err(|e| bad(&format!("Surface header refused: {e:?}")))?;
            let sections = parse_section_table(&surface.bytes,&header).map_err(|e| bad(&format!("Surface sections refused: {e:?}")))?;
            let s = sections.iter().find(|s|s.typ()==Some(SectionType::ProvenanceSidecar))
                .ok_or_else(||bad("Terrain surface source provenance missing"))?;
            let payload = &surface.bytes[s.byte_offset as usize..(s.byte_offset+s.byte_length) as usize];
            let provenance = decode_provenance_section(payload).map_err(|e|bad(&format!("Surface provenance refused: {e:?}")))?;
            validate_provenance(&provenance).map_err(|e|bad(&format!("Surface source refused: {e:?}")))?;
            let expected: [u8;32] = Sha256::digest(source).into();
            if provenance.source_bytes()!=source || provenance.version_hash()!=&expected {
                return Err(bad("Surface provenance differs from the exact packed M64"));
            }
        },
        _ => return Err(bad("Terrain surface output and receipt inventories disagree")),
    }
    let metadata = json!({
        "schema": "ecopunk.terrain-source-derivative/1",
        "source_resource": "talbingo_catchment.m64", "source_sha256": source_sha,
        "derivative_resource": "talbingo_heightfield.10d", "derivative_sha256": ten_d_sha,
        "declared_frame": terrain.receipt.frame,
        "surface_resource":terrain.surface.as_ref().map(|_|"terrain/surface.10d"),
        "source_receipt": terrain.receipt,
        "generated_scene_sectors_adopt_this_source": false,
        "scope": "Elevation-field derivative; source declarations are not scientific or rights acceptance"
    });
    writer.add_file("talbingo_catchment.m64", "m64", source.to_vec(), None)
        .map_err(|e| bad(&format!("M64 bundle resource refused: {e:?}")))?;
    writer.add_file("talbingo_heightfield.10d", "10d", terrain.ten_d_buffer.clone(), None)
        .map_err(|e| bad(&format!("Terrain .10d bundle resource refused: {e:?}")))?;
    if let Some(surface) = terrain.surface.as_ref() {
        writer.add_file("terrain/surface.10d","10d",surface.bytes.clone(),None)
            .map_err(|e|bad(&format!("Terrain surface bundle resource refused: {e:?}")))?;
    }
    writer.add_file("m64/continuum_metadata.json", "application/json", serde_json::to_vec_pretty(&metadata)?, None)
        .map_err(|e| bad(&format!("Terrain metadata bundle resource refused: {e:?}")))?;
    Ok(())
}

#[cfg(test)]
mod terrain_pack_tests {
    use super::*;
    use qualia_core_db::bundle::BundleReader;
    #[test]
    fn final_hmc_publication_preserves_existing_bundle() {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!("ecopunk-final-hmc-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        publish_final_hmc_no_clobber(&path, b"original").unwrap();
        assert_eq!(publish_final_hmc_no_clobber(&path, b"replacement").unwrap_err().kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(fs::read(&path).unwrap(), b"original");
        fs::remove_file(path).unwrap();
    }
    #[test]
    fn packed_field_and_m64_match_compiler_receipt_without_fabricated_frame() {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!("ecopunk-terrain-pack-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        fs::create_dir(&dir).unwrap();
        struct Owned(std::path::PathBuf);
        impl Drop for Owned { fn drop(&mut self) {
            let _ = fs::remove_file(self.0.join("talbingo_catchment.m64"));
            let _ = fs::remove_file(self.0.join("talbingo_heightfield.10d"));
            let _ = fs::remove_dir(&self.0);
        } }
        let _owned = Owned(dir.clone());
        let (samples, receipt) = super::super::dem_admission::synthetic_dem_fixture(4096).unwrap();
        let mut compiled = super::super::dem_admission::compile_terrain_strata_and_10d(&samples, &dir, receipt).unwrap();
        let mut writer = BundleWriter::new();
        add_terrain_resources(&mut writer, &compiled).unwrap();
        let pack = writer.build().unwrap();
        let reader = BundleReader::parse(&pack).unwrap();
        assert_eq!(reader.get("talbingo_heightfield.10d"), Some(compiled.ten_d_buffer.as_slice()));
        assert_eq!(reader.get("talbingo_catchment.m64"), Some(&compiled.m64_buffer[..compiled.m64_len]));
        assert!(reader.entries().iter().all(|e| reader.verify_entry(&e.key)));
        let metadata: serde_json::Value = serde_json::from_slice(reader.get("m64/continuum_metadata.json").unwrap()).unwrap();
        assert!(metadata["declared_frame"].is_null());
        assert_eq!(metadata["source_receipt"]["is_synthetic_fixture"], true);
        assert_eq!(metadata["generated_scene_sectors_adopt_this_source"], false);
        compiled.m64_buffer[0] ^= 1;
        assert!(add_terrain_resources(&mut BundleWriter::new(), &compiled).is_err());
    }
}
