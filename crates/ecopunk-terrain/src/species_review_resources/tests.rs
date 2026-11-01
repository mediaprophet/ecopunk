use super::*;
use crate::species_review_reopen;
use qualia_core_db::{
    q42_volume::{OpfsSliceRangeSource, Q42RangeVolume, UnifiedVolumeBuilder},
    q_hash, NQuin,
};
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
fn source() -> &'static str {
    include_str!("../../../../tools/species-production/white-gum-review-r1/white-gum-r2.vibe")
}
fn graph_resource(bad_parity: bool) -> ResourceData {
    let terms = [
        "https://example.invalid/test-source/entity",
        "http://purl.org/dc/terms/source",
        "https://example.invalid/test-source/item",
    ];
    let mut lex = HashMap::new();
    for term in terms {
        lex.insert(q_hash(term), term.to_string());
    }
    let mut q = NQuin {
        subject: q_hash(terms[0]),
        predicate: q_hash(terms[1]),
        object: q_hash(terms[2]),
        context: 0,
        metadata: 0,
        parity: 0,
    };
    q.parity = NQuin::calculate_parity(q.subject, q.predicate, q.object, q.context, q.metadata);
    if bad_parity {
        q.parity ^= 1;
    }
    let mut builder = UnifiedVolumeBuilder::with_lex_map(&lex).unwrap();
    builder.push_block(0, &[q]).unwrap();
    ResourceData {
        key: "resources/knowledge/test-corpus.q42".into(),
        kind: "q42".into(),
        bytes: builder.finish_to_bytes(),
        meta: Some(vec![0xa0]),
    }
}
#[test]
fn dependent_admission_failure_preserves_source_and_native_resource_inventory() {
    let mut session = SpeciesReviewPackageSession::new(0, ReviewResourceLimits::default()).unwrap();
    session.submit_with_resources(source(), 0, vec![graph_resource(false)]).unwrap();
    let before = session.resources().entries()[0].bytes.clone();
    let before_sources = session.sources().to_vec();
    let failure = session.submit_with_resources_prepared(
        "= species_asset", 1, Vec::new(), crate::species_authored_review::build,
        |_, _| Err::<(), _>("dependent runtime refused".into()),
    );
    assert!(failure.is_err());
    assert_eq!(session.revision(), 1);
    assert_eq!(session.sources(), before_sources);
    assert_eq!(session.resources().entries().len(), 1);
    assert_eq!(session.resources().entries()[0].bytes, before);
    // The same operation can then succeed and visibly replace the inventory.
    session.submit_with_resources_prepared("= species_asset", 1, Vec::new(),
        crate::species_authored_review::build, |_, _| Ok(())).unwrap();
    assert_eq!(session.revision(), 2);
    assert!(session.resources().entries().is_empty());
}
#[test]
fn generated_native_graph_binds_same_revision_and_survives_fresh_reopen() {
    let mut session = SpeciesReviewPackageSession::new(0, ReviewResourceLimits::default()).unwrap();
    let failed = session.submit_with_generated_resources_prepared(source(), 0,
        crate::species_authored_review::build, |_| Ok(vec![graph_resource(false)]),
        |_, _| Err::<(), _>("runtime admission refused".into()));
    assert!(failed.is_err());
    assert_eq!(session.revision(), 0);
    assert!(session.resources().entries().is_empty());
    let (asset, ()) = session.submit_with_generated_resources_prepared(source(), 0,
        crate::species_authored_review::build, |candidate| {
            assert_eq!(candidate.render.metadata["source_revision"], 1);
            Ok(vec![graph_resource(false)])
        }, |authored, asset| {
            assert_eq!(authored.revision, 1);
            let reader = BundleReader::parse(&asset.bundle).unwrap();
            assert!(reader.entry("resources/knowledge/test-corpus.q42").is_some());
            Ok(())
        }).unwrap();
    let reopened = species_review_reopen::reopen(0, &asset.bundle).unwrap();
    assert_eq!(reopened.session.revision(), 1);
    assert_eq!(reopened.asset.bundle, asset.bundle);
    assert_eq!(reopened.session.resources().inventory(), session.resources().inventory());
    assert!(session.submit_with_generated_resources_prepared("= species_asset", 0,
        crate::species_authored_review::build, |_| panic!("Stale factory must not execute"),
        |_, _| Ok(())).is_err());
    assert_eq!(session.revision(), 1);
}
fn matter_resource() -> ResourceData {
    use qualia_core_db::q42::m64_matter::{M64RasterEdges, M64RasterRows};
    use qualia_geospatial::{
        ElevationRaster, GeodeticFrame,
    };
    use qualia_geospatial::raster_m64::{compile_raster_to_m64_v3, RasterM64Limits, RasterM64Policy};
    let samples = [1000, 2000, 3000, 4000];
    let raster = ElevationRaster {
        width: 2,
        height: 2,
        cell_size_mm: 1000,
        elevations_mm: &samples,
    };
    let frame = GeodeticFrame {
        srid: qualia_geospatial::SRID_GDA2020,
        vertical_datum: qualia_geospatial::VERTICAL_DATUM_AHD,
        epoch_year: 2020.0,
        origin_lat_deg: -35.0,
        origin_lon_deg: 148.0,
        origin_elevation_mm: 1000,
    };
    let mut bytes = vec![0; 300_000];
    let len = compile_raster_to_m64_v3(
        &raster,
        &frame,
        1,
        RasterM64Policy {
            rows: M64RasterRows::South,
            edges: M64RasterEdges::RepeatNearest,
            nodata_sentinel_mm: None,
        },
        RasterM64Limits {
            max_output_bytes: 300_000,
            max_chunks: 1,
            max_records: 4096,
        },
        &AtomicBool::new(false),
        &mut bytes,
    )
    .unwrap();
    bytes.truncate(len);
    ResourceData {
        key: "resources/matter/test-grid.m64".into(),
        kind: "m64".into(),
        bytes,
        meta: Some(vec![0x81, 0x01]),
    }
}
#[test]
fn exact_native_resources_survive_source_edit_save_reopen_and_reedit() {
    let limits = ReviewResourceLimits::default();
    let graph = graph_resource(false);
    let matter = matter_resource();
    let mut session = SpeciesReviewPackageSession::new(0, limits).unwrap();
    let first = session
        .submit_with_resources(source(), 0, vec![graph.clone(), matter.clone()])
        .unwrap();
    let r = BundleReader::parse(&first.bundle).unwrap();
    assert_eq!(r.get(&graph.key), Some(graph.bytes.as_slice()));
    assert_eq!(r.entry(&matter.key).unwrap().meta, matter.meta);
    let mut reopened = species_review_reopen::reopen(0, &first.bundle).unwrap();
    assert_eq!(
        reopened.session.resources().inventory(),
        session.resources().inventory()
    );
    let edit = source()
        .replace("let mut species_asset =", "species_asset =")
        .replace("ring_slices: 12", "ring_slices: 16");
    assert!(edit.contains("ring_slices: 16"));
    let next = reopened.session.submit(&edit, 1).unwrap();
    assert_ne!(first.render.bytes, next.render.bytes);
    let mut again = species_review_reopen::reopen(0, &next.bundle).unwrap();
    assert_eq!(again.session.revision(), 2);
    let saved = again.session.submit("= species_asset", 2).unwrap();
    let final_open = species_review_reopen::reopen(0, &saved.bundle).unwrap();
    assert_eq!(final_open.session.revision(), 3);
    let reader = BundleReader::parse(&saved.bundle).unwrap();
    for resource in [graph, matter] {
        assert_eq!(reader.get(&resource.key), Some(resource.bytes.as_slice()));
        assert_eq!(reader.entry(&resource.key).unwrap().meta, resource.meta);
    }
    let bytes = reader.get("resources/knowledge/test-corpus.q42").unwrap();
    let volume = Q42RangeVolume::open(OpfsSliceRangeSource::new(bytes)).unwrap();
    let h = volume.header();
    let lex = qualia_core_db::q42_lex::Q42LexMmap::from_bytes(
        &bytes[h.lex_offset as usize..(h.lex_offset + h.lex_length) as usize],
    )
    .unwrap();
    assert_eq!(
        lex.lookup_owned(q_hash("https://example.invalid/test-source/item")),
        Some("https://example.invalid/test-source/item".into())
    );
}
#[test]
fn malformed_native_resource_or_stale_edit_does_not_promote_either_owner() {
    let mut session = SpeciesReviewPackageSession::new(0, ReviewResourceLimits::default()).unwrap();
    let graph = graph_resource(false);
    session
        .submit_with_resources(source(), 0, vec![graph.clone()])
        .unwrap();
    let inventory = session.resources().inventory();
    assert!(session
        .submit_with_resources("= species_asset", 0, vec![])
        .is_err());
    assert!(session
        .submit_with_resources("= species_asset", 1, vec![graph_resource(true)])
        .is_err());
    let mut bad = matter_resource();
    bad.bytes[0] ^= 1;
    assert!(session
        .submit_with_resources("= species_asset", 1, vec![bad])
        .is_err());
    assert_eq!(session.revision(), 1);
    assert_eq!(session.resources().inventory(), inventory);
    assert!(session.submit_with_resources("= null", 1, vec![]).is_err());
    assert_eq!(session.revision(), 1);
    assert_eq!(session.resources().inventory(), inventory);
}
#[test]
fn quotas_unsupported_keys_and_raw_quins_are_explicitly_refused() {
    let limits = ReviewResourceLimits::default();
    let graph = graph_resource(false);
    let mut tiny = limits;
    tiny.total_bytes = 1;
    assert!(ReviewResources::admit(vec![graph.clone()], tiny).is_err());
    let mut tiny = limits;
    tiny.q42_blocks = 0;
    assert!(ReviewResources::admit(vec![graph.clone()], tiny).is_err());
    let mut tiny = limits;
    tiny.metadata_bytes = 0;
    assert!(ReviewResources::admit(vec![graph.clone()], tiny).is_err());
    let mut unsupported = graph.clone();
    unsupported.key = "resources/../escape.q42".into();
    assert!(ReviewResources::admit(vec![unsupported], limits).is_err());
    let mut unsupported = graph.clone();
    unsupported.kind = "json".into();
    assert!(ReviewResources::admit(vec![unsupported], limits).is_err());
    let mut raw = graph.clone();
    raw.bytes = vec![0; 48];
    assert!(ReviewResources::admit(vec![raw], limits).is_err());
    assert!(ReviewResources::admit(vec![graph.clone(), graph], limits).is_err());
}
#[test]
fn caller_package_caps_apply_to_resource_free_source_and_overlapping_sections_refuse() {
    for cold in [false, true] {
        let mut limits = ReviewResourceLimits::default();
        if cold {
            limits.cold_work_bytes = 0;
        } else {
            limits.package_bytes = 0;
        }
        let mut session = SpeciesReviewPackageSession::new(0, limits).unwrap();
        assert!(session.submit(source(), 0).is_err());
        assert_eq!(session.revision(), 0);
    }
    let mut resource = graph_resource(false);
    let mut header = {
        let volume =
            Q42RangeVolume::open(OpfsSliceRangeSource::new(resource.bytes.as_slice())).unwrap();
        *volume.header()
    };
    header.temporal_index_offset = header.lex_offset;
    header.temporal_index_length = header.lex_length;
    resource.bytes[..qualia_core_db::q42_volume::HEADER_SIZE]
        .copy_from_slice(&qualia_core_db::q42_volume::header_to_bytes(&header));
    assert!(ReviewResources::admit(vec![resource], ReviewResourceLimits::default()).is_err());
}
#[test]
fn custom_metadata_quota_cannot_export_an_index_the_same_profile_cannot_reopen() {
    let mut limits = ReviewResourceLimits::default();
    limits.entries = 33;
    limits.metadata_bytes = 8192;
    let original = graph_resource(false);
    let resources = (0..33)
        .map(|index| {
            let mut r = original.clone();
            r.key = format!("resources/knowledge/part-{index}.q42");
            r.meta = Some(vec![0; 8192]);
            r
        })
        .collect();
    let mut session = SpeciesReviewPackageSession::new(0, limits).unwrap();
    let error = match session.submit_with_resources(source(), 0, resources) {
        Ok(_) => panic!("unreopenable index exported"),
        Err(error) => error,
    };
    assert!(error.contains("index profile"));
    assert_eq!(session.revision(), 0);
    assert!(session.resources().entries().is_empty());
}
#[test]
fn repacked_corruption_is_refused_and_opaque_metadata_is_never_dropped() {
    let mut session = SpeciesReviewPackageSession::new(0, ReviewResourceLimits::default()).unwrap();
    let asset = session
        .submit_with_resources(source(), 0, vec![matter_resource()])
        .unwrap();
    let reader = BundleReader::parse(&asset.bundle).unwrap();
    let mut writer = BundleWriter::new();
    for entry in reader.entries() {
        let mut bytes = reader.get(&entry.key).unwrap().to_vec();
        if entry.key.ends_with(".m64") {
            bytes[0] ^= 1;
        }
        writer
            .add_file(&entry.key, &entry.kind, bytes, entry.meta.clone())
            .unwrap();
    }
    assert!(species_review_reopen::reopen(0, &writer.build().unwrap()).is_err());
    let mut opened = species_review_reopen::reopen(0, &asset.bundle).unwrap();
    let saved = opened.session.submit("= species_asset", 1).unwrap();
    let reader = BundleReader::parse(&saved.bundle).unwrap();
    assert_eq!(
        reader.entry("resources/matter/test-grid.m64").unwrap().meta,
        Some(vec![0x81, 0x01])
    );
}
