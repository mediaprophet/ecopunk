//! Public consumer tests of inert carrier continuity; no activation or grants.
use super::super::{create_artifact_if_absent, mutate_artifact_stamped, StudioArtifact};
use super::*;
use crate::bundle::{BundleReader, BundleWriter};

fn package(id: &str, domain: &str, meta: Option<Vec<u8>>) -> (Vec<u8>, super::super::StoreStamp) {
    let (_, stamp) =
        create_artifact_if_absent(StudioArtifact::new(id, domain, "carrier fixture")).unwrap();
    let source = capture(id, &stamp, Default::default()).unwrap();
    (with_source_meta(&source, meta), stamp)
}

fn with_source_meta(source: &Checkpoint, meta: Option<Vec<u8>>) -> Vec<u8> {
    let encoded = encode(source, vec![], Default::default()).unwrap();
    let reader = BundleReader::parse(&encoded).unwrap();
    let mut writer = BundleWriter::new();
    for entry in reader.entries() {
        writer
            .add_file(
                &entry.key,
                &entry.kind,
                reader.get(&entry.key).unwrap().to_vec(),
                if entry.key == SOURCE_RESOURCE_KEY {
                    meta.clone()
                } else {
                    entry.meta.clone()
                },
            )
            .unwrap();
    }
    writer.build().unwrap()
}

fn resource() -> ResourceData {
    ResourceData {
        key: "future/input".into(),
        kind: "application/vnd.unknown+v9".into(),
        bytes: vec![0, 255, 42],
        meta: Some(vec![0xa1, 0x61, b'v', 9]),
    }
}

fn add(input: &Checkpoint) -> Checkpoint {
    input
        .apply_resource_edits(vec![ResourceEdit::Add(resource())], Default::default())
        .unwrap()
}

#[test]
fn current_source_and_opaque_metadata_union_is_exact_and_inert() {
    let id = "urn:carrier:current-source";
    let meta = Some(vec![0xa1, 0x61, b'x', 9]);
    let (original, stamp) = package(id, "Geometry", meta.clone());
    let input = add(&decode(&original, Default::default()).unwrap());
    let (_, stamp) = mutate_artifact_stamped(id, &stamp, |a| {
        a.nodes
            .entry("poet.native.submission/00000000000000000001".into())
            .or_default()
            .insert("text".into(), "let accepted_current = 42;".into());
        Ok(())
    })
    .unwrap();
    let current = capture(id, &stamp, Default::default()).unwrap();
    let expected_artifact = current.artifact().clone();
    let merged = current
        .with_parent_digest([3; 32])
        .with_retained_inputs_from(&input, Default::default())
        .unwrap();
    assert_eq!(merged.artifact(), &expected_artifact);
    assert_ne!(merged.artifact(), input.artifact());
    assert_eq!(merged.parent_digest(), Some([3; 32]));
    assert_eq!(merged.file_digest(), None);
    assert_eq!(merged.resources(), &[resource()]);
    let bytes = encode(&merged, vec![], Default::default()).unwrap();
    let reader = BundleReader::parse(&bytes).unwrap();
    assert_eq!(reader.entry(SOURCE_RESOURCE_KEY).unwrap().meta, meta);
    assert_eq!(reader.entry("future/input").unwrap().meta, resource().meta);
    assert_eq!(
        decode(&bytes, Default::default()).unwrap().artifact(),
        &expected_artifact
    );
    // Retention never imported an old source body or touched the stamped owner.
    assert!(super::super::artifact_stamp_is_current(id, &stamp));
}

#[test]
fn source_metadata_conflict_and_absence_are_explicit() {
    let (bytes, _) = package("urn:carrier:metadata", "Geometry", Some(vec![0xa0]));
    let input = decode(&bytes, Default::default()).unwrap();
    let different = decode(
        &with_source_meta(&input, Some(vec![0x80])),
        Default::default(),
    )
    .unwrap();
    assert!(input
        .clone()
        .clone()
        .with_retained_inputs_from(&different, Default::default())
        .is_err());
    let absent = decode(&with_source_meta(&input, None), Default::default()).unwrap();
    for merged in [
        input
            .clone()
            .with_retained_inputs_from(&absent, Default::default())
            .unwrap(),
        absent
            .with_retained_inputs_from(&input, Default::default())
            .unwrap(),
    ] {
        let bytes = encode(&merged, vec![], Default::default()).unwrap();
        assert_eq!(
            BundleReader::parse(&bytes)
                .unwrap()
                .entry(SOURCE_RESOURCE_KEY)
                .unwrap()
                .meta,
            Some(vec![0xa0])
        );
    }
}

#[test]
fn exact_overlap_is_idempotent_but_payload_kind_and_metadata_conflicts_refuse() {
    let (bytes, _) = package("urn:carrier:resource-conflicts", "Geometry", None);
    let input = add(&decode(&bytes, Default::default()).unwrap());
    let merged = input
        .clone()
        .with_retained_inputs_from(&input, Default::default())
        .unwrap();
    assert_eq!(merged.resources(), input.resources());
    let before = encode(&input, vec![], Default::default()).unwrap();
    for field in ["payload", "kind", "metadata", "metadata-absence"] {
        let mut replacement = resource();
        match field {
            "payload" => replacement.bytes.push(1),
            "kind" => replacement.kind.push_str("-other"),
            "metadata" => replacement.meta = Some(vec![0xa0]),
            _ => replacement.meta = None,
        }
        let conflict = input
            .apply_resource_edits(
                vec![ResourceEdit::Replace {
                    expected: input.inventory()[0].digest,
                    resource: replacement,
                }],
                Default::default(),
            )
            .unwrap();
        assert!(input
            .clone()
            .with_retained_inputs_from(&conflict, Default::default())
            .is_err());
        assert_eq!(encode(&input, vec![], Default::default()).unwrap(), before);
    }
}

#[test]
fn foreign_source_or_domain_and_union_budgets_refuse() {
    let (bytes, _) = package("urn:carrier:bounds", "Geometry", Some(vec![0xa0]));
    let input = add(&decode(&bytes, Default::default()).unwrap());
    let (foreign, _) = package("urn:carrier:foreign", "Geometry", None);
    let foreign = decode(&foreign, Default::default()).unwrap();
    assert!(input
        .clone()
        .with_retained_inputs_from(&foreign, Default::default())
        .is_err());
    // Construct another valid same-key source-domain through the canonical codec.
    let reader = BundleReader::parse(&bytes).unwrap();
    let mut record: serde_json::Value =
        serde_json::from_slice(reader.get(SOURCE_RESOURCE_KEY).unwrap()).unwrap();
    record["artifact"]["domain"] = "Material".into();
    let mut writer = BundleWriter::new();
    let entry = reader.entry(SOURCE_RESOURCE_KEY).unwrap();
    writer
        .add_file(
            SOURCE_RESOURCE_KEY,
            &entry.kind,
            serde_json::to_vec(&record).unwrap(),
            entry.meta.clone(),
        )
        .unwrap();
    let domain = decode(&writer.build().unwrap(), Default::default()).unwrap();
    assert!(input
        .clone()
        .with_retained_inputs_from(&domain, Default::default())
        .is_err());
    for limits in [
        CheckpointLimits {
            entries: 1,
            ..Default::default()
        },
        CheckpointLimits {
            source_bytes: 1,
            ..Default::default()
        },
        CheckpointLimits {
            file_bytes: 128,
            ..Default::default()
        },
    ] {
        assert!(input
            .clone()
            .with_retained_inputs_from(&input, limits)
            .is_err());
    }
    // Both individual sets fit; their distinct-resource union does not.
    let mut other = resource();
    other.key = "future/second".into();
    let base = decode(&bytes, Default::default()).unwrap();
    let second = base
        .apply_resource_edits(vec![ResourceEdit::Add(other)], Default::default())
        .unwrap();
    assert!(input
        .with_retained_inputs_from(
            &second,
            CheckpointLimits {
                entries: 2,
                ..Default::default()
            }
        )
        .is_err());
    let entry = reader.entry(SOURCE_RESOURCE_KEY).unwrap();
    let retained = resource();
    let one_resource_budget = 128
        + reader.get(SOURCE_RESOURCE_KEY).unwrap().len()
        + SOURCE_RESOURCE_KEY.len()
        + entry.kind.len()
        + entry.meta.as_ref().map_or(0, Vec::len)
        + 512
        + retained.key.len()
        + retained.kind.len()
        + retained.bytes.len()
        + retained.meta.as_ref().map_or(0, Vec::len)
        + 512;
    let limits = CheckpointLimits {
        file_bytes: one_resource_budget,
        ..Default::default()
    };
    assert!(input
        .clone()
        .with_retained_inputs_from(&input, limits)
        .is_ok());
    assert!(second
        .clone()
        .with_retained_inputs_from(&second, limits)
        .is_ok());
    assert!(input.with_retained_inputs_from(&second, limits).is_err());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_fresh_save_reopen_and_save_as_preserve_carrier_without_stale_source() {
    let (bytes, stamp) = package("urn:carrier:native-reopen", "Geometry", Some(vec![0xa0]));
    let input = add(&decode(&bytes, Default::default()).unwrap());
    let (_, stamp) = mutate_artifact_stamped(input.artifact().id.as_str(), &stamp, |a| {
        a.properties
            .insert("accepted.current".into(), "new-source".into());
        Ok(())
    })
    .unwrap();
    let current = capture(input.artifact().id.as_str(), &stamp, Default::default()).unwrap();
    let artifact = current.artifact().clone();
    let merged = current
        .with_retained_inputs_from(&input, Default::default())
        .unwrap();
    let output = encode(&merged, vec![], Default::default()).unwrap();
    let dir = tempfile::TempDir::new().unwrap();
    let first_path = dir.path().join("fresh.hmc");
    let receipt = save(
        &first_path,
        &output,
        ExpectedFile::Absent,
        Default::default(),
    )
    .unwrap();
    drop(merged);
    let reopened = load(&first_path, Default::default()).unwrap();
    assert_eq!(reopened.artifact(), &artifact);
    let current = capture(artifact.id.as_str(), &stamp, Default::default()).unwrap();
    let copy = current
        .with_retained_inputs_from(&reopened, Default::default())
        .unwrap()
        .with_parent_digest(receipt.digest);
    let copied = encode(&copy, vec![], Default::default()).unwrap();
    let copy_path = dir.path().join("copy.hmc");
    save(
        &copy_path,
        &copied,
        ExpectedFile::Absent,
        Default::default(),
    )
    .unwrap();
    let reopened_copy = load(&copy_path, Default::default()).unwrap();
    assert_eq!(reopened_copy.parent_digest(), Some(receipt.digest));
    assert_eq!(reopened_copy.resources(), input.resources());
    assert_eq!(reopened_copy.artifact(), &artifact);
    assert_eq!(
        BundleReader::parse(&copied)
            .unwrap()
            .entry(SOURCE_RESOURCE_KEY)
            .unwrap()
            .meta,
        Some(vec![0xa0])
    );
    // Digest refusal and corrupt source refuse without changing installed files.
    assert!(save(
        &first_path,
        &copied,
        ExpectedFile::Sha256([0; 32]),
        Default::default()
    )
    .is_err());
    let mut corrupt = output.clone();
    let offset = BundleReader::parse(&output)
        .unwrap()
        .entry(SOURCE_RESOURCE_KEY)
        .unwrap()
        .offset as usize;
    corrupt[offset] ^= 1;
    assert!(decode(&corrupt, Default::default()).is_err());
    assert_eq!(std::fs::read(&first_path).unwrap(), output);
    assert!(super::super::artifact_stamp_is_current(
        artifact.id.as_str(),
        &stamp
    ));
    let cleanup_path = dir.path().to_path_buf();
    drop(dir);
    assert!(!cleanup_path.exists());
}
