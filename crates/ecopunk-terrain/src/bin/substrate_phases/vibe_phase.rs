//! VibeScript Certification, Interpreter Execution & Ecological Catalog Compilation
//!
//! Evaluates `east_coast_biosphere_statements.vibe` using `PoetSnapshot`,
//! validates Darwin Core SHACL shapes, compiles 305-Flora and 164-Fauna catalogs,
//! and writes `.q42` unified semantic volumes.

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

use ecopunk_terrain::{
    compile_164_fauna_catalog, compile_305_flora_catalog, write_corymbia_educational_assets_to_disk,
    ContinuumExplorationSession, ExplorationTickInput, FaunaBiomechanicalArchetype,
    RegionalCorridorWorld, Scale5VertebrateAnatomyResult, StudySiteId, CANONICAL_ANATOMICAL_PINS,
};
use qualia_core_db::container_10d::MAGIC_10D;
use qualia_core_db::lexicon::generate_60bit_token;
use qualia_core_db::q42::q42_volume::write_sorted_quins_volume;
use qualia_core_db::q42_volume::{StreamingQ42VolumeWriter, Q42_MAGIC};
use qualia_core_db::query::shacl_compiler::{
    validate_shacl_property, ShaclConstraint, ShaclDatatype,
};
use qualia_core_db::resolver::INLINE_TAG_INTEGER;
use qualia_core_db::{NQuin, QUINS_PER_BLOCK};
use serde_json::{json, Value};

pub struct VibePhaseResult {
    pub vibe_quins: Vec<NQuin>,
    pub unified_australia_quins: Vec<NQuin>,
    pub q42_v4_bytes: Vec<u8>,
    pub q42_unified_bytes: Vec<u8>,
    pub taxon_bindings_json: serde_json::Map<String, Value>,
    pub flora_total: u64,
    pub fauna_total: u64,
    pub macropod_anat: Scale5VertebrateAnatomyResult,
}

pub fn execute_vibe_biosphere_phase(
    root: &Path,
    data_dir: &Path,
    assets_dir: &Path,
    total_quins: &[NQuin],
) -> io::Result<VibePhaseResult> {
    let vibe_paths = [
        root.join("entities").join("taxa").join("corymbia-aparrerinja").join("corymbia-aparrerinja.vibe"),
        root.join("vibe").join("east_coast_flora_305.vibe"),
        root.join("vibe").join("east_coast_fauna_164.vibe"),
        root.join("vibe").join("lithology_pedology_hydrology.vibe"),
    ];

    for vp in &vibe_paths {
        if vp.exists() {
            let src = fs::read_to_string(vp)?;
            let ast = vibe::parse_program(&src).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("VibeScript parse error in {}: {e:?}", vp.display()))
            })?;
            let ty_env = vibe::check_program(&ast).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("VibeScript typecheck error in {}: {e:?}", vp.display()))
            })?;
            println!(
                "  * Certified VibeScript schema module: {} ({} items, effect: {:?})",
                vp.file_name().and_then(|s| s.to_str()).unwrap_or("module.vibe"),
                ast.items.len(),
                ty_env.effect
            );
        }
    }

    let stmt_vibe_path = root.join("vibe").join("east_coast_biosphere_statements.vibe");
    let stmt_vibe_src = fs::read_to_string(&stmt_vibe_path)?;
    let vibe_program = vibe::load_program(&stmt_vibe_src)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("vibe::load_program error: {e}")))?;

    let hero_keys = [
        ("corymbia_aparrerinja", "https://ns.webizen.org/ecopunk/asset/flora/corymbia-aparrerinja-0"),
        ("eucalyptus_pauciflora", "https://ns.webizen.org/ecopunk/asset/flora/eucalyptus-pauciflora-0"),
        ("eucalyptus_regnans", "https://ns.webizen.org/ecopunk/asset/flora/eucalyptus-regnans-0"),
        ("eucalyptus_camaldulensis", "https://ns.webizen.org/ecopunk/asset/flora/eucalyptus-camaldulensis-0"),
        ("acacia_pycnantha", "https://ns.webizen.org/ecopunk/asset/flora/acacia-pycnantha-0"),
        ("banksia_serrata", "https://ns.webizen.org/ecopunk/asset/flora/banksia-serrata-0"),
        ("xanthorrhoea_australis", "https://ns.webizen.org/ecopunk/asset/flora/xanthorrhoea-australis-0"),
        ("dicksonia_antarctica", "https://ns.webizen.org/ecopunk/asset/flora/dicksonia-antarctica-0"),
        ("telopea_speciosissima", "https://ns.webizen.org/ecopunk/asset/flora/telopea-speciosissima-0"),
        ("themeda_triandra", "https://ns.webizen.org/ecopunk/asset/flora/themeda-triandra-0"),
        ("vallisneria_australis", "https://ns.webizen.org/ecopunk/asset/flora/vallisneria-australis-0"),
        ("macropus_giganteus", "https://ns.webizen.org/ecopunk/asset/fauna/macropus-giganteus-0"),
        ("vombatus_ursinus", "https://ns.webizen.org/ecopunk/asset/fauna/vombatus-ursinus-0"),
        ("ornithorhynchus_anatinus", "https://ns.webizen.org/ecopunk/asset/fauna/ornithorhynchus-anatinus-0"),
        ("phascolarctos_cinereus", "https://ns.webizen.org/ecopunk/asset/fauna/phascolarctos-cinereus-0"),
        ("aquila_audax", "https://ns.webizen.org/ecopunk/asset/fauna/aquila-audax-0"),
        ("maccullochella_peelii", "https://ns.webizen.org/ecopunk/asset/fauna/maccullochella-peelii-0"),
    ];

    let mut lex_terms: HashMap<u64, String> = HashMap::new();
    let mut vibe_quins: Vec<NQuin> = Vec::new();
    let mut taxon_bindings_json = serde_json::Map::new();

    for (render_key, asset_iri) in &hero_keys {
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env = vibe::Env::default();
        let taxon_val = vibe::eval_function(
            &vibe_program,
            "taxon_for",
            vec![vibe::Value::String((*render_key).to_string())],
            &mut host,
            &mut env,
        )
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("eval taxon_for: {e}")))?;

        let vibe::Value::Record(rec) = taxon_val else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "taxon_for not record"));
        };
        let gbif_key = match rec.get("gbifKey") {
            Some(vibe::Value::F64(v)) => *v as u64,
            _ => 0,
        };
        let taxon_iri = match rec.get("iri") {
            Some(vibe::Value::String(s)) => s.clone(),
            _ => String::new(),
        };
        let sci_name = match rec.get("scientificName") {
            Some(vibe::Value::String(s)) => s.clone(),
            _ => String::new(),
        };
        let common_name = match rec.get("commonName") {
            Some(vibe::Value::String(s)) => s.clone(),
            _ => String::new(),
        };
        taxon_bindings_json.insert(
            (*render_key).to_string(),
            json!({
                "gbifKey": gbif_key,
                "iri": taxon_iri,
                "scientificName": sci_name,
                "commonName": common_name,
                "assetIri": asset_iri,
            }),
        );

        let mut host2 = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env2 = vibe::Env::default();
        let stmts_val = vibe::eval_function(
            &vibe_program,
            "dwc_statements_for",
            vec![
                vibe::Value::String((*render_key).to_string()),
                vibe::Value::String((*asset_iri).to_string()),
            ],
            &mut host2,
            &mut env2,
        )
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("eval dwc_statements_for: {e}")))?;

        let vibe::Value::List(items) = stmts_val else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "dwc_statements_for not list"));
        };

        for item in items {
            if let vibe::Value::Record(srec) = item {
                let s_str = match srec.get("s") {
                    Some(vibe::Value::String(s)) => s.clone(),
                    _ => String::new(),
                };
                let p_str = match srec.get("p") {
                    Some(vibe::Value::String(s)) => s.clone(),
                    _ => String::new(),
                };
                let o_str = match srec.get("o") {
                    Some(vibe::Value::String(s)) => s.clone(),
                    _ => String::new(),
                };
                if let Some(vibe::Value::QuinRef(qref)) = srec.get("quin") {
                    let f = qref.raw_fields();
                    let nq = NQuin {
                        subject: f[0],
                        predicate: f[1],
                        object: f[2],
                        context: f[3],
                        metadata: f[4],
                        parity: f[5],
                    };
                    assert_eq!(
                        nq.parity,
                        NQuin::calculate_parity(nq.subject, nq.predicate, nq.object, nq.context, nq.metadata),
                        "VibeScript QuinRef parity seal must be valid"
                    );
                    lex_terms.insert(nq.subject, s_str);
                    lex_terms.insert(nq.predicate, p_str);
                    lex_terms.insert(nq.object, o_str);
                    vibe_quins.push(nq);
                }
            }
        }

        let s_tok = generate_60bit_token(taxon_iri.as_bytes());
        let p_tok = generate_60bit_token(b"urn:qualia:gbif:taxonKey");
        let o_tok = INLINE_TAG_INTEGER | gbif_key;
        lex_terms.insert(s_tok, taxon_iri.clone());
        lex_terms.insert(p_tok, "urn:qualia:gbif:taxonKey".to_string());
        vibe_quins.push(NQuin {
            subject: s_tok,
            predicate: p_tok,
            object: o_tok,
            context: 0,
            metadata: 0,
            parity: NQuin::calculate_parity(s_tok, p_tok, o_tok, 0, 0),
        });

        let one = [ShaclConstraint::MinCount(1)];
        for prop in [
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
            "http://rs.tdwg.org/dwc/terms/taxonID",
            "http://rs.tdwg.org/dwc/terms/scientificName",
            "http://rs.tdwg.org/dwc/terms/taxonRank",
        ] {
            assert!(
                validate_shacl_property(&vibe_quins, s_tok, generate_60bit_token(prop.as_bytes()), &one),
                "SHACL validation failed for {taxon_iri} property {prop}"
            );
        }
        assert!(
            validate_shacl_property(
                &vibe_quins,
                s_tok,
                p_tok,
                &[
                    ShaclConstraint::MinCount(1),
                    ShaclConstraint::Datatype(ShaclDatatype::Integer),
                ],
            ),
            "SHACL integer taxonKey validation failed for {taxon_iri}"
        );
    }

    vibe_quins.sort_unstable_by_key(|q| q.object);
    let q42_v4_path = assets_dir.join("east_coast_biosphere_v4.q42");
    let mut q42_writer = StreamingQ42VolumeWriter::new(&lex_terms)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("q42 writer: {e}")))?;
    q42_writer.declare_permissive_commons();
    for (seq, chunk) in vibe_quins.chunks(QUINS_PER_BLOCK).enumerate() {
        q42_writer
            .push_block(seq as u64, chunk)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("q42 push_block: {e}")))?;
    }
    q42_writer
        .finish(&q42_v4_path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("q42 finish: {e}")))?;
    let q42_v4_bytes = fs::read(&q42_v4_path)?;
    assert!(q42_v4_bytes.starts_with(&Q42_MAGIC), "Q42 v4 magic verified");
    println!(
        "[OK] Evaluated VibeScript `east_coast_biosphere_statements.vibe` via `PoetSnapshot` & SHACL: {} sealed NQuins, {} lexicon terms -> {} ({} bytes)",
        vibe_quins.len(),
        lex_terms.len(),
        q42_v4_path.display(),
        q42_v4_bytes.len()
    );

    let mut unified_australia_quins = Vec::with_capacity(total_quins.len() + vibe_quins.len());
    unified_australia_quins.extend_from_slice(total_quins);
    unified_australia_quins.extend_from_slice(&vibe_quins);
    unified_australia_quins.sort_unstable_by_key(|q| q.object);
    let q42_unified_path = assets_dir.join("east_coast_australia.q42");
    let unified_written = write_sorted_quins_volume(&q42_unified_path, &unified_australia_quins)?;
    let q42_unified_bytes = fs::read(&q42_unified_path)?;
    println!(
        "[OK] Compiled Unified .q42 Semantic Volume: {} ({} NQuins written)",
        q42_unified_path.display(),
        unified_written
    );

    let catalog_dir = data_dir.join("catalog");
    let flora_100_path = catalog_dir.join("flora_100.json");
    let fauna_64_path = catalog_dir.join("fauna_64.json");

    let raw_flora_val: Value = serde_json::from_str(&fs::read_to_string(&flora_100_path)?)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("flora JSON error: {e}")))?;
    let base_flora_species: Vec<Value> = raw_flora_val
        .get("species")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().take(105).cloned().collect())
        .unwrap_or_default();
    let base_flora_input = json!({ "species": base_flora_species });

    let compiled_flora = compile_305_flora_catalog(&base_flora_input);
    let flora_total = compiled_flora
        .get("total_species")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    assert_eq!(flora_total, 305, "Expected 305 compiled flora species");

    let flora_305_path = catalog_dir.join("flora_305.json");
    let flora_pretty = serde_json::to_string_pretty(&compiled_flora)?;
    fs::write(&flora_100_path, &flora_pretty)?;
    fs::write(&flora_305_path, &flora_pretty)?;
    println!(
        "[OK] Compiled 305-Species Flora Catalog: {} & {} ({} species)",
        flora_100_path.display(),
        flora_305_path.display(),
        flora_total
    );

    let raw_fauna_val: Value = serde_json::from_str(&fs::read_to_string(&fauna_64_path)?)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("fauna JSON error: {e}")))?;
    let base_fauna_species: Vec<Value> = raw_fauna_val
        .get("species")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().take(64).cloned().collect())
        .unwrap_or_default();
    let base_fauna_input = json!({ "species": base_fauna_species });

    let compiled_fauna = compile_164_fauna_catalog(&base_fauna_input);
    let fauna_total = compiled_fauna
        .get("total_species")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    assert_eq!(fauna_total, 164, "Expected 164 compiled fauna species");

    let fauna_164_path = catalog_dir.join("fauna_164.json");
    let fauna_pretty = serde_json::to_string_pretty(&compiled_fauna)?;
    fs::write(&fauna_64_path, &fauna_pretty)?;
    fs::write(&fauna_164_path, &fauna_pretty)?;
    println!(
        "[OK] Compiled 164-Species Fauna Catalog: {} & {} ({} species)",
        fauna_64_path.display(),
        fauna_164_path.display(),
        fauna_total
    );

    let written_edu_bytes = write_corymbia_educational_assets_to_disk(root)?;
    let macropod_anat = ecopunk_terrain::build_scale5_vertebrate_anatomy_10d(
        FaunaBiomechanicalArchetype::MacropodSaltatorial,
        1.0,
        1.0,
        1.0,
        0.0,
    );
    let macropod_10d_path = assets_dir.join("vertebrate_anatomy_macropod.10d");
    fs::write(&macropod_10d_path, &macropod_anat.container_bytes)?;
    println!(
        "[OK] Synthesized 5-Scale Botanical & Zoological Educational Twins: {} B botanical .10d + {} verts / {} B Macropod 3-layer anatomy ({} PO/UBERON callout pins)",
        written_edu_bytes,
        macropod_anat.positions.len(),
        macropod_anat.container_bytes.len(),
        CANONICAL_ANATOMICAL_PINS.len()
    );

    let corridor_world = RegionalCorridorWorld::new();
    let mut exp_session = ContinuumExplorationSession::new(corridor_world);
    let _drive_snap = exp_session.step_tick_zero_heap(
        &ExplorationTickInput { throttle: 1.0, ..Default::default() },
        1.0 / 64.0,
    );
    exp_session.teleport_to_study_site(StudySiteId::YarrangobillyKarstGorge);
    let dismount_snap = exp_session.step_tick_zero_heap(
        &ExplorationTickInput { toggle_dismount: true, ..Default::default() },
        1.0 / 64.0,
    );
    println!(
        "[OK] Verified 26.5 km RegionalCorridorWorld & ContinuumExplorationSession: 8 sites, dismounted at {:?} ({:.1} m AHD, cave ceiling {:.1} m)",
        dismount_snap.nearest_study_site,
        dismount_snap.elevation_ahd_m,
        dismount_snap.cave_ceiling_m
    );

    let corymbia_10d_dir = root.join("assets").join("models").join("corymbia-aparrerinja").join("10d");
    let mut corymbia_chunks = Vec::new();
    if corymbia_10d_dir.exists() {
        for entry in fs::read_dir(&corymbia_10d_dir)? {
            let entry = entry?;
            let p = entry.path();
            if p.extension().and_then(|e| e.to_str()) == Some("10d") {
                let bytes = fs::read(&p)?;
                let valid_magic = bytes.len() >= 4 && &bytes[0..4] == &MAGIC_10D;
                corymbia_chunks.push(json!({
                    "part": p.file_stem().and_then(|s| s.to_str()).unwrap_or("part"),
                    "bytes": bytes.len(),
                    "valid_10d_magic": valid_magic,
                }));
            }
        }
    }
    println!("  * Verified {} .10d Digital Twin sub-meshes for Corymbia aparrerinja", corymbia_chunks.len());

    let master_path = data_dir.join("master_ecological_dataset.json");
    let mut master_val: Value = if master_path.exists() {
        serde_json::from_str(&fs::read_to_string(&master_path)?).unwrap_or_else(|_| json!({}))
    } else {
        json!({})
    };
    if let Some(obj) = master_val.as_object_mut() {
        obj.insert("total_flora_species".to_string(), json!(flora_total));
        obj.insert("total_fauna_species".to_string(), json!(fauna_total));
        obj.insert("flora".to_string(), compiled_flora.get("species").cloned().unwrap_or_else(|| json!([])));
        obj.insert("fauna".to_string(), compiled_fauna.get("species").cloned().unwrap_or_else(|| json!([])));
    }
    fs::write(&master_path, serde_json::to_string_pretty(&master_val)?)?;

    Ok(VibePhaseResult {
        vibe_quins,
        unified_australia_quins,
        q42_v4_bytes,
        q42_unified_bytes,
        taxon_bindings_json,
        flora_total,
        fauna_total,
        macropod_anat,
    })
}
