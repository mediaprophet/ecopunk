//! Public SDK regression: independent frame/morph/scientific payloads coexist.
use qualia_core_db::container_10d::{self as c, field_section as science, *};
use qualia_core_db::render::compile_10d::{decode_10d_mesh_frames, Compile10dError};

fn identity() -> ManifoldIdentityV2 {
    ManifoldIdentityV2 { entity_id: StableEntityId([7; 16]), profile_id: 1, domain_id: 2,
        coordinate_convention_id: 3, coordinates: [0.0; 10], external_digest: None }
}
fn delta(vertex_index: u32) -> QuantizedMorphDelta {
    QuantizedMorphDelta { pos_delta: [32767, 0, 0], norm_delta: [0; 3], target_index: 0,
        tangent_oct_delta: [0; 2], vertex_index }
}
fn chunks() -> [Vec<u8>; 3] {
    let mut frames = vec![0; 256];
    let n = encode_mesh_frames(Some(&[[0.0, 0.0, 1.0]; 3]),
        Some(&[[1.0, 0.0, 0.0, 1.0]; 3]), &mut frames).unwrap();
    frames.truncate(n);
    let header = MorphSectionHeader { magic: MAGIC_MRP1, version: MORPH_SECTION_VERSION,
        target_count: 1, vertex_count: 3, flags: FLAG_MORPH_SPARSE,
        max_displacement: 1.0, ontogenetic_bind_mask: 0, reserved: [0; 8] };
    let mut morph = vec![0; 256];
    let n = encode_morph_section(&header, &[delta(0), delta(2)], &mut morph).unwrap();
    morph.truncate(n);
    let mut raster = vec![0; 256];
    let n = science::encode_field_section_with_rasters(&[science::ContinuousRasterField {
        name_hash: 41, unit_hash: 42, width: 2, height: 1, samples: &[2.5, 7.25],
    }], &mut raster).unwrap();
    raster.truncate(n);
    [frames, morph, raster]
}
fn records(chunks: &[Vec<u8>; 3], order: [usize; 3]) -> (Vec<u8>, Vec<TypedFieldRecord>) {
    let mut payload = Vec::new();
    let mut fields = vec![TypedFieldRecord::default(); 3];
    for index in order {
        fields[index] = TypedFieldRecord { field_id: (index as u64 + 1) * 10,
            kind: [FIELD_KIND_SURFACE_FRAMES, FIELD_KIND_MORPH_TARGETS, FIELD_KIND_SCALAR][index],
            flags: 0, unit_id: if index == 2 { 42 } else { 0 },
            payload_section_id: SectionType::FieldSidecar as u32,
            payload_offset: payload.len() as u64, payload_length: chunks[index].len() as u64 };
        payload.extend_from_slice(&chunks[index]);
    }
    (payload, fields)
}
fn envelope(payload: &[u8], fields: &[TypedFieldRecord], extensions: &[u16]) -> Vec<u8> {
    let section = SectionInput { section_type: SectionType::FieldSidecar,
        alignment_tier: AlignmentTier::Word, stride: 0, element_count: 0, payload };
    let mut manifest = [0; MANIFOLD_IDENTITY_V2_MAX_BYTES];
    let mut sections = [section; 2];
    let mut out = vec![0; 8192];
    let n = encode_container_with_manifold_identity_extensions(&Container10dHeader::proposed(),
        &[section], &identity(), fields, extensions, &mut manifest, &mut sections, &mut out).unwrap();
    out.truncate(n);
    seal_whole_file_crc32c(&mut out);
    out
}
fn fixture(order: [usize; 3]) -> Vec<u8> {
    let (payload, fields) = records(&chunks(), order);
    envelope(&payload, &fields, &[])
}

#[test]
fn frames_morph_and_scientific_raster_coexist_in_either_payload_order() {
    for order in [[0, 1, 2], [2, 1, 0]] {
        let data = fixture(order);
        let mut scratch = [TypedFieldRecord::default(); 3];
        let mut fields = [TypedFieldRecord::default(); 3];
        let reader = TypedPayloadResolver::new(&data, &[], &mut scratch, &mut fields).unwrap();
        assert!(reader.has_manifest());
        assert_eq!(reader.fields().len(), 3);
        let frame_payload = reader.surface_frames().unwrap().unwrap();
        assert_eq!(frame_payload.record.field_id, 10);
        assert_eq!(frame_payload.bytes.as_ptr(), reader.field(10).unwrap().unwrap().bytes.as_ptr());
        let frames = decode_mesh_frames(frame_payload.bytes).unwrap();
        assert_eq!(frames.normals.unwrap(), vec![[0.0, 0.0, 1.0]; 3]);
        assert_eq!(frames.tangents.unwrap(), vec![[1.0, 0.0, 0.0, 1.0]; 3]);
        let decoded = decode_10d_mesh_frames(&data, 3).unwrap().unwrap();
        assert_eq!(decoded.normals.unwrap(), vec![[0.0, 0.0, 1.0]; 3]);
        let mut deltas = [delta(99); 2];
        let (header, count) = read_container_morph_targets_into(&data, 3, &mut deltas).unwrap().unwrap();
        assert_eq!(header.vertex_count, 3);
        assert_eq!(count, 2);
        assert_eq!(deltas, [delta(0), delta(2)]);
        let science_payload = reader.field(30).unwrap().unwrap();
        let (_, descriptors) = science::decode_field_section(science_payload.bytes).unwrap();
        let (grid, samples) = science::decode_field_raster_samples(science_payload.bytes, &descriptors[0]).unwrap();
        assert_eq!((grid.width, grid.height), (2, 1));
        assert_eq!(samples, [2.5, 7.25]);
    }
}

#[test]
fn legacy_single_payload_survives_but_mid2_never_guesses_vector_normals() {
    for tangents in [false, true] {
        let mut frames = vec![0; 256];
        let n = encode_mesh_frames(Some(&[[0.0, 0.0, 1.0]; 3]),
            tangents.then_some(&[[1.0, 0.0, 0.0, 1.0]; 3][..]), &mut frames).unwrap();
        frames.truncate(n);
        let section = SectionInput { section_type: SectionType::FieldSidecar,
            alignment_tier: AlignmentTier::Word, stride: 0, element_count: 0, payload: &frames };
        let mut out = vec![0; 2048];
        let n = encode_container(&Container10dHeader::legacy_v1(), &[section], &mut out).unwrap();
        out.truncate(n);
        let before = out.clone();
        assert_eq!(read_container_mesh_frames(&out, 3).unwrap().unwrap().normals.unwrap(), vec![[0.0, 0.0, 1.0]; 3]);
        assert_eq!(out, before);
        let field = TypedFieldRecord { field_id: 10, kind: FIELD_KIND_VECTOR, flags: 0, unit_id: 0,
            payload_section_id: 8, payload_offset: 0, payload_length: frames.len() as u64 };
        assert_eq!(read_container_mesh_frames(&envelope(&frames, &[field], &[]), 3).unwrap(), None);
    }
}

#[test]
fn duplicate_frames_unknown_kind_and_bad_codec_fail_clearly() {
    let (mut payload, mut fields) = records(&chunks(), [0, 1, 2]);
    fields[1].kind = FIELD_KIND_SURFACE_FRAMES;
    assert!(matches!(read_container_mesh_frames(&envelope(&payload, &fields, &[]), 3),
        Err(TypedPayloadError::AmbiguousKind { kind: FIELD_KIND_SURFACE_FRAMES })));
    fields[1].kind = FIELD_KIND_MORPH_TARGETS;
    fields[2].kind = 0x7f01; // Negative unsupported extension fixture, never production registry.
    let data = envelope(&payload, &fields, &[0x7f01]);
    assert!(matches!(read_container_mesh_frames(&data, 3), Err(TypedPayloadError::Manifest(
        ManifoldContainerError::Manifest(ManifestV2Error::UnsupportedFieldKind(0x7f01))))));
    fields[2].kind = FIELD_KIND_SCALAR;
    payload[0] = b'!';
    assert!(matches!(read_container_mesh_frames(&envelope(&payload, &fields, &[]), 3),
        Err(TypedPayloadError::Frames(NormalSectionError::BadMagic))));
}

#[test]
fn counts_empty_declarations_wrong_binding_and_small_buffers_refuse_transactionally() {
    let data = fixture([0, 1, 2]);
    assert!(matches!(decode_10d_mesh_frames(&data, 2), Err(Compile10dError::NormalVertexCountMismatch { expected: 2, got: 3 })));
    let mut sentinel = [delta(77); 2];
    assert!(matches!(read_container_morph_targets_into(&data, 2, &mut sentinel), Err(TypedPayloadError::VertexCount { .. })));
    assert_eq!(sentinel, [delta(77); 2]);
    assert!(matches!(read_container_morph_targets_into(&data, 3, &mut sentinel[..1]), Err(TypedPayloadError::Morph(MorphSectionError::BufferTooSmall { .. }))));
    assert_eq!(sentinel, [delta(77); 2]);
    let mut scratch = [TypedFieldRecord::default(); 3];
    let mut out = [TypedFieldRecord { field_id: 99, ..TypedFieldRecord::default() }; 1];
    assert!(TypedPayloadResolver::new(&data, &[], &mut scratch, &mut out).is_err());
    assert_eq!(out[0].field_id, 99);
    let (payload, mut fields) = records(&chunks(), [0, 1, 2]);
    fields[0].unit_id = 12;
    assert!(matches!(read_container_mesh_frames(&envelope(&payload, &fields, &[]), 3), Err(TypedPayloadError::InvalidSemanticBinding { field_id: 10 })));
    fields[0].unit_id = 0;
    fields[0].payload_section_id = 0;
    fields[0].payload_length = 0;
    assert!(matches!(read_container_mesh_frames(&envelope(&payload, &fields, &[]), 3), Err(TypedPayloadError::InvalidSemanticBinding { field_id: 10 })));
}

#[test]
fn malformed_later_morph_delta_leaves_all_output_unchanged() {
    let mut encoded = chunks();
    // Second delta's vertex_index is the final u32; reseal by encoding envelope.
    let last = encoded[1].len() - 4;
    encoded[1][last..].copy_from_slice(&99u32.to_le_bytes());
    let (payload, fields) = records(&encoded, [2, 1, 0]);
    let data = envelope(&payload, &fields, &[]);
    let mut output = [delta(77); 2];
    assert!(matches!(read_container_morph_targets_into(&data, 3, &mut output),
        Err(TypedPayloadError::Morph(MorphSectionError::DeltaVertexOutOfRange { index: 1, .. }))));
    assert_eq!(output, [delta(77); 2]);
}

#[test]
fn invalid_ranges_preserve_manifest_output_and_writer_output() {
    let (payload, mut fields) = records(&chunks(), [0, 1, 2]);
    fields[0].payload_offset = payload.len() as u64;
    let section = SectionInput { section_type: SectionType::FieldSidecar,
        alignment_tier: AlignmentTier::Word, stride: 0, element_count: 0, payload: &payload };
    let mut manifest = [0; MANIFOLD_IDENTITY_V2_MAX_BYTES];
    let mut sections = [section; 2];
    let mut out = [0xa5; 8192];
    assert!(matches!(encode_container_with_manifold_identity(&Container10dHeader::proposed(),
        &[section], &identity(), &fields, &mut manifest, &mut sections, &mut out),
        Err(ManifoldContainerError::PayloadRangeOutOfBounds { .. })));
    assert!(out.iter().all(|b| *b == 0xa5));
    // Encode manifest alone, then generic envelope, to exercise malformed read.
    let n = encode_manifold_identity_v2(&identity(), &fields, &mut manifest).unwrap();
    let invalid = SectionInput { section_type: SectionType::ManifoldIdentityV2,
        alignment_tier: AlignmentTier::Word, stride: 0, element_count: 0, payload: &manifest[..n] };
    let mut data = vec![0; 8192];
    let n = encode_container(&Container10dHeader::proposed(), &[section, invalid], &mut data).unwrap();
    data.truncate(n);
    let mut scratch = [TypedFieldRecord::default(); 3];
    let marker = TypedFieldRecord { field_id: 99, ..TypedFieldRecord::default() };
    let mut output = [marker; 3];
    assert!(matches!(TypedPayloadResolver::new(&data, &[], &mut scratch, &mut output),
        Err(TypedPayloadError::Manifest(ManifoldContainerError::PayloadRangeOutOfBounds { .. }))));
    assert_eq!(output, [marker; 3]);
}
