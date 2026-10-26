//! Retained resource safety uses real native compiled assets and native HMC.
use super::*;
use qualia_core_db::bundle::{BundleReader, BundleWriter};

fn fixture(root: &Path, id: &str) -> (CheckpointService, StoreStamp) {
    let (_, stamp) = studio_store::create_artifact_if_absent(StudioArtifact::new(
        id,
        "Geometry",
        "resource preservation fixture",
    ))
    .unwrap();
    let service = CheckpointService::new(
        root,
        TrustedBinding {
            project_id: "fixture-project".into(),
            document_id: id.into(),
            source_id: id.into(),
            domain: "Geometry".into(),
        },
        Default::default(),
    )
    .unwrap();
    (service, stamp)
}
fn actual_asset() -> CompiledAsset {
    qualia_core_db::render::compile_10d::compile_asset(
        b"v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n",
        Some("obj"),
        "urn:fixture:retained-derived",
        "obj",
    )
    .unwrap()
}

#[test]
fn replacement_cannot_drop_real_geometry_or_native_q42_even_with_supplied_assets() {
    let root = tempfile::tempdir().unwrap();
    let (service, stamp) = fixture(root.path(), "urn:poet:checkpoint-preserve-derived");
    let saved = service
        .save(
            &stamp,
            "derived.hmc",
            ExpectedFile::Absent,
            None,
            vec![("derived/geometry.10d".into(), actual_asset())],
        )
        .unwrap();
    let before = std::fs::read(&saved.receipt.path).unwrap();
    let bundle = BundleReader::parse(&before).unwrap();
    assert!(bundle.entries().iter().any(|e| e.kind == "10d"));
    assert!(bundle.entries().iter().any(|e| e.kind == "q42"));
    for assets in [
        vec![],
        vec![("derived/geometry.10d".into(), actual_asset())],
    ] {
        let error = service
            .save(
                &stamp,
                "derived.hmc",
                ExpectedFile::Sha256(saved.receipt.digest),
                Some(saved.receipt.digest),
                assets,
            )
            .err()
            .expect("resource-dropping replacement must fail");
        assert!(error.contains("complete resource preservation"));
        assert_eq!(std::fs::read(&saved.receipt.path).unwrap(), before);
    }
}

#[test]
fn replacement_cannot_drop_unknown_inert_hmc_entries() {
    let root = tempfile::tempdir().unwrap();
    let (service, stamp) = fixture(root.path(), "urn:poet:checkpoint-preserve-unknown");
    let saved = service
        .save(&stamp, "source.hmc", ExpectedFile::Absent, None, vec![])
        .unwrap();
    let source = std::fs::read(&saved.receipt.path).unwrap();
    let reader = BundleReader::parse(&source).unwrap();
    let mut writer = BundleWriter::new();
    for entry in reader.entries() {
        writer
            .add_file(
                &entry.key,
                &entry.kind,
                reader.get(&entry.key).unwrap().to_vec(),
                entry.meta.clone(),
            )
            .unwrap();
    }
    writer
        .add_file(
            "evidence/retained.txt",
            "text/plain",
            b"inert retained source evidence".to_vec(),
            None,
        )
        .unwrap();
    let with_resource = writer.build().unwrap();
    let decoded = core::decode(&with_resource, Default::default()).unwrap();
    let digest = decoded.file_digest().unwrap();
    std::fs::write(&saved.receipt.path, &with_resource).unwrap();
    let error = service
        .save(
            &stamp,
            "source.hmc",
            ExpectedFile::Sha256(digest),
            Some(digest),
            vec![],
        )
        .err()
        .expect("unknown resources may not disappear");
    assert!(error.contains("complete resource preservation"));
    assert_eq!(std::fs::read(&saved.receipt.path).unwrap(), with_resource);
}
