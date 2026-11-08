//! Export the same native generators used by the demo; no parallel asset engine.
use std::{collections::HashMap, fs, path::Path};
use ecopunk_terrain::{build_procedural_textures, build_sector_10d};
use ecopunk_wasm::{build_dissection_scale_10d, sector_key_for_study_site};
use qualia_core_db::{bundle::{BundleReader, BundleWriter}, render::compile_10d::decode_10d_mesh};

fn admit(bundle: &mut BundleWriter, root: &Path, name: &str, kind: &str, bytes: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let path = root.join(name);
    if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
    fs::write(&path, &bytes)?;
    bundle.add_file(name, kind, bytes, None).map_err(|e| format!("{e:?}"))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argument = std::env::args().nth(1).ok_or("output directory required")?;
    let root = Path::new(&argument);
    fs::create_dir_all(root)?;
    if std::env::args().any(|arg| arg == "--refresh-sources") {
        let repository = std::env::args().nth(2).ok_or("repository directory required")?;
        let previous = fs::read(root.join("ecopunk-demo.hmc"))?;
        let reader = BundleReader::parse(&previous).map_err(|e| format!("{e:?}"))?;
        let mut refreshed = BundleWriter::new();
        for entry in reader.entries() {
            if !reader.verify_entry(&entry.key) { return Err(format!("invalid prior entry {}", entry.key).into()); }
            if entry.key.starts_with("dossier/") { continue; }
            let bytes = if let Some(source) = entry.key.strip_prefix("sources/") {
                let bytes = fs::read(Path::new(&repository).join(source))?;
                fs::write(root.join(&entry.key), &bytes)?;
                bytes
            } else { reader.get(&entry.key).ok_or("missing entry")?.to_vec() };
            refreshed.add_file(&entry.key, &entry.kind, bytes, None).map_err(|e| format!("{e:?}"))?;
        }
        refreshed.add_file("dossier/README.md", "document", fs::read(root.join("dossier/README.md"))?, None).map_err(|e| format!("{e:?}"))?;
        for case in fs::read_dir(root.join("dossier/cases"))? {
            let case = case?;
            if !case.file_type()?.is_dir() { continue; }
            for name in ["README.md", "claims.json"] {
                let key = format!("dossier/cases/{}/{name}", case.file_name().to_string_lossy());
                refreshed.add_file(&key, "claim-projection", fs::read(case.path().join(name))?, None).map_err(|e| format!("{e:?}"))?;
            }
        }
        let bytes = refreshed.build().map_err(|e| format!("{e:?}"))?;
        let verified = BundleReader::parse(&bytes).map_err(|e| format!("{e:?}"))?;
        for entry in verified.entries() { if !verified.verify_entry(&entry.key) { return Err("refreshed entry failed verification".into()); } }
        println!("refreshed sources and dossiers; verified {} entries; {} bytes", verified.entries().len(), bytes.len());
        fs::write(root.join("ecopunk-demo.hmc"), bytes)?;
        return Ok(());
    }
    let textures = build_procedural_textures();
    let mut bundle = BundleWriter::new();
    let mut inventory = String::from("resource\tvertices\ttriangles\tstatus\n");
    let mut resources = Vec::new();
    let mut sector_cache = HashMap::new();
    let mut preview_cache = HashMap::new();
    for texture in &textures {
        let name = format!("textures/{}.png", texture.name);
        admit(&mut bundle, root, &name, "png", texture.png_bytes.clone())?;
        resources.push(name);
    }
    for site in 0..8 {
        let (key, title) = sector_key_for_study_site(site);
        if !sector_cache.contains_key(key) {
            sector_cache.insert(key, build_sector_10d(key, title, &textures, false)?);
        }
        let sector = &sector_cache[key];
        for (role, bytes) in [("scene", sector.scene_10d_bytes.clone()), ("water", sector.water_10d_bytes.clone())] {
            let name = format!("sites/{site:02}-{key}-{role}.10d");
            let mesh = decode_10d_mesh(&bytes)?;
            inventory.push_str(&format!("{name}\t{}\t{}\tdecoded\n", mesh.positions.len(), mesh.triangles.len()));
            admit(&mut bundle, root, &name, "10d", bytes)?;
            resources.push(name);
        }
        // Native reference frames use the same authored scene, material and texture data.
        if !preview_cache.contains_key(key) {
            preview_cache.insert(key, ecopunk_terrain::render_sector_via_portal_gpu_png(sector, &textures, 1024, 768, None)?);
        }
        let png = preview_cache[key].clone();
        let name = format!("previews/site-{site:02}-{key}.png");
        admit(&mut bundle, root, &name, "png", png)?;
        resources.push(name);
        println!("rebuilt site {site}: {key}");
    }
    // Continuous sliders cannot be exhaustively enumerated. Export neutral,
    // middle and endpoint states, with all five fauna archetypes at scale five.
    for scale in 1..=5 {
        for archetype in 0..if scale == 5 { 5 } else { 1 } {
            for state in 0..if scale == 1 { 1 } else { 3 } {
                let value = state as f32 * 0.5;
                let (bytes, _) = build_dissection_scale_10d(scale, value, value, value, value, archetype)?;
                let mesh = decode_10d_mesh(&bytes)?;
                if mesh.positions.is_empty() || mesh.triangles.is_empty() { return Err("empty lab asset".into()); }
                let name = format!("lab/scale-{scale}-archetype-{archetype}-state-{state}.10d");
                inventory.push_str(&format!("{name}\t{}\t{}\tdecoded\n", mesh.positions.len(), mesh.triangles.len()));
                admit(&mut bundle, root, &name, "10d", bytes)?;
                resources.push(name);
            }
        }
        println!("rebuilt lab scale {scale}");
    }
    admit(&mut bundle, root, "inventory.tsv", "inventory", inventory.into_bytes())?;
    resources.push("inventory.tsv".into());
    let repository = std::env::args().nth(2).unwrap_or_else(|| ".".into());
    for source in ["vibe/ecopunk_continuum_drive.vibe", "vibe/ecopunk_biological_lab.vibe",
        "crates/ecopunk-terrain/src/scene_builder_10d.rs", "crates/ecopunk-terrain/src/botanical_anatomy.rs",
        "crates/ecopunk-terrain/src/zoological_anatomy.rs", "crates/ecopunk-wasm/src/lib.rs"] {
        let name = format!("sources/{source}");
        admit(&mut bundle, root, &name, "source", fs::read(Path::new(&repository).join(source))?)?;
        resources.push(name);
    }
    let mut lexicon = HashMap::new();
    let mut quins = Vec::new();
    for resource in &resources {
        let subject = format!("urn:ecopunk:demo:rebuild-2026-10-29:{resource}");
        for (predicate, object) in [("http://www.w3.org/1999/02/22-rdf-syntax-ns#type", "urn:ecopunk:IllustrativeDemoResource"),
            ("http://www.w3.org/ns/prov#wasGeneratedBy", "urn:ecopunk:demo:native-rebuild-2026-10-29")] {
            for term in [subject.as_str(), predicate, object] { lexicon.insert(qualia_core_db::q_hash(term), term.to_string()); }
            let mut quin = qualia_core_db::NQuin { subject: qualia_core_db::q_hash(&subject), predicate: qualia_core_db::q_hash(predicate),
                object: qualia_core_db::q_hash(object), context: 0, metadata: 0, parity: 0 };
            quin.recalculate_parity(); quins.push(quin);
        }
    }
    quins.sort_by_key(|q| q.object);
    let mut graph = qualia_core_db::q42_volume::UnifiedVolumeBuilder::with_lex_map(&lexicon).map_err(|e| format!("{e:?}"))?;
    graph.push_block(0, &quins).map_err(|e| format!("{e:?}"))?;
    admit(&mut bundle, root, "knowledge.q42", "q42", graph.finish_to_bytes())?;
    resources.push("knowledge.q42".into());
    let bytes = bundle.build().map_err(|e| format!("{e:?}"))?;
    let reader = BundleReader::parse(&bytes).map_err(|e| format!("{e:?}"))?;
    for resource in &resources {
        if !reader.verify_entry(resource) { return Err(format!("invalid bundle entry {resource}").into()); }
    }
    fs::write(root.join("ecopunk-demo.hmc"), &bytes)?;
    println!("verified {} bundle entries; {} bytes", resources.len(), bytes.len());
    Ok(())
}
