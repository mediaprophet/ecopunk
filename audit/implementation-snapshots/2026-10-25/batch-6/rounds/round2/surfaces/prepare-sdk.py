"""Read Core sources; write candidates and receipts ONLY beside this script in Ecopunk."""
from pathlib import Path
import hashlib, json, difflib
HERE = Path(__file__).resolve().parent
CORE = Path('C:/github/qualiaDB/crates/qualia-core-db')
rows = []

def candidate(rel, replacements):
    before = (CORE / rel).read_bytes()  # Read only: never write to CORE.
    text = before.decode('utf-8')
    newline = '\r\n' if b'\r\n' in before else '\n'
    for old, new in replacements:
        old, new = old.replace('\n', newline), new.replace('\n', newline)
        if text.count(old) != 1:
            raise RuntimeError('Unexpected source anchor: ' + rel + ': ' + old[:75])
        text = text.replace(old, new)
    save(rel, before, text.encode('utf-8'))

def save(rel, before, after):
    for dirname, value in [('before', before), ('after', after)]:
        target = HERE / dirname / rel
        if value is not None:
            target.parent.mkdir(parents=True, exist_ok=True)
            if target.exists():
                if target.read_bytes() != value: raise RuntimeError('Snapshot differs: ' + str(target))
            else:
                target.write_bytes(value)
    rows.append({'path': str(CORE / rel), 'relative': rel,
                 'before_sha256': None if before is None else hashlib.sha256(before).hexdigest(),
                 'after_sha256': hashlib.sha256(after).hexdigest()})

candidate('src/container_10d/mod.rs', [
    ('pub mod texture_coordinate_section;', 'pub mod texture_coordinate_section;\n/// Borrowed typed-field selection and existing-codec container readers.\npub mod typed_payloads;'),
    ('    FIELD_KIND_TENSOR, FIELD_KIND_VECTOR, MANIFOLD_IDENTITY_V2_AXIS_ORDER,',
     '    FIELD_KIND_TENSOR, FIELD_KIND_VECTOR, FIELD_KIND_SURFACE_FRAMES, FIELD_KIND_MORPH_TARGETS,\n    MANIFOLD_IDENTITY_V2_AXIS_ORDER,'),
    ('pub use section::{', 'pub use typed_payloads::{\n    read_container_mesh_frames, read_container_morph_targets_into, TypedPayload,\n    TypedPayloadError, TypedPayloadResolver,\n};\npub use section::{'),
])
candidate('src/container_10d/manifold_identity_v2.rs', [
    ('//! - `StableEntityId` is an opaque continuity handle supplied by a trusted,\n//!   global identity authority. The authority MUST guarantee global uniqueness\n//!   and collision-check issuance; this codec never derives it from content.',
     '//! - `StableEntityId` is an opaque stored continuity handle. This codec does\n//!   not verify its issuer or global uniqueness and never derives it from content.\n//!   Qualified referent resolution belongs to native graph/authority mechanisms.'),
    ('pub const BUILTIN_FIELD_KINDS: [u16; 5] = [',
     '/// Existing NRM1/FRM1 per-vertex surface frames, unitless FieldSidecar range.\npub const FIELD_KIND_SURFACE_FRAMES: u16 = 0x0200;\n/// Existing MRP1 morph-target codec, unitless FieldSidecar range.\npub const FIELD_KIND_MORPH_TARGETS: u16 = 0x0201;\n// Registry additions preserve MID2 layout. Older readers reject unsupported kinds\n// unless their explicit supported-extension allow-list admits these semantics.\npub const BUILTIN_FIELD_KINDS: [u16; 7] = ['),
    ('    FIELD_KIND_ELECTROMAGNETIC,\n];', '    FIELD_KIND_ELECTROMAGNETIC,\n    FIELD_KIND_SURFACE_FRAMES,\n    FIELD_KIND_MORPH_TARGETS,\n];'),
    ('/// Opaque stable continuity identity, globally unique under the trusted\n/// issuing authority\'s contract. Its bytes are never interpreted as a digest.',
     '/// Opaque stored continuity identifier. Issuer and uniqueness are not verified\n/// by this codec. Its bytes are never interpreted as a digest.'),
    ('    /// Canonical entity resolution uses the stable entity handle only. Edits\n    /// and replay frames for the same entity retain this handle even when the\n    /// manifold address/state changes. Splits or merges create new handles and\n    /// record lineage in an appropriate provenance section.',
     '    /// Compare stored handles only; equality does not establish universal\n    /// referent equivalence or issuer authority. Qualified resolution and\n    /// split/merge lineage remain graph/authority responsibilities.'),
])
candidate('src/container_10d/morph_section.rs', [
    ('    for (index, slot) in out_deltas[..delta_count].iter_mut().enumerate() {',
     '    // Validate the entire payload before publishing any caller output.\n    for index in 0..delta_count {'),
    ('        *slot = d;\n    }\n\n    Ok((header, delta_count))',
     '    }\n    for (index, slot) in out_deltas[..delta_count].iter_mut().enumerate() {\n        let start = MORPH_SECTION_HEADER_SIZE + index * QUANTIZED_MORPH_DELTA_SIZE;\n        *slot = bytemuck::pod_read_unaligned(&payload[start..start + QUANTIZED_MORPH_DELTA_SIZE]);\n    }\n\n    Ok((header, delta_count))'),
])

compile_text = (CORE/'src/render/compile_10d.rs').read_text(encoding='utf-8')
start = compile_text.index('    let header =\n', compile_text.index('pub fn decode_10d_mesh_frames('))
end = compile_text.index('\n}\n', start)
candidate('src/render/compile_10d.rs', [
    ('    /// The container section table encode/decode failed.',
     '    /// Typed MID2 payload selection or codec admission failed.\n    TypedPayload(crate::container_10d::TypedPayloadError),\n    /// The container section table encode/decode failed.'),
    ('            Self::Section(e) => write!(f, ".10d compile: section table: {e:?}"),',
     '            Self::TypedPayload(e) => write!(f, ".10d compile: {e}"),\n            Self::Section(e) => write!(f, ".10d compile: section table: {e:?}"),'),
    (compile_text[start:end], '''    // Selection/codec admission lives in the focused shared SDK; this oversized
    // legacy compiler retains only compatibility error projection (see SDK note).
    crate::container_10d::read_container_mesh_frames(container_10d, expected_vertex_count)
        .map_err(|error| match error {
            crate::container_10d::TypedPayloadError::Header(_) => Compile10dError::BadHeader,
            crate::container_10d::TypedPayloadError::Sections(e) => Compile10dError::Section(e),
            crate::container_10d::TypedPayloadError::Frames(e) => Compile10dError::Normals(e),
            crate::container_10d::TypedPayloadError::VertexCount { expected, got } =>
                Compile10dError::NormalVertexCountMismatch { expected, got },
            other => Compile10dError::TypedPayload(other),
        })'''),
])
candidate('src/render/portal/mod.rs', [
    ('        let mut authored_normals = None;\n        let mut authored_tangents = None;', ''),
    ('''                container_10d::SectionType::FieldSidecar => {
                    if container_10d::is_normal_section(payload) {
                        let frames = container_10d::decode_mesh_frames(payload)
                            .map_err(|e| JsValue::from_str(&format!("10d frames decode: {e}")))?;
                        authored_normals = frames.normals;
                        authored_tangents = frames.tangents;
                    }
                }
''', ''),
    ('        let mesh = mesh.ok_or_else(|| JsValue::from_str("10d: no mesh section"))?;\n        if authored_normals',
     '''        let mesh = mesh.ok_or_else(|| JsValue::from_str("10d: no mesh section"))?;
        // Shared SDK owns legacy/MID2 selection; keep this oversized portal as glue.
        let frames = container_10d::read_container_mesh_frames(&bytes_mut, mesh.vertex_count())
            .map_err(|e| JsValue::from_str(&format!("10d frames decode: {e}")))?;
        let (authored_normals, authored_tangents) = frames
            .map(|frames| (frames.normals, frames.tangents)).unwrap_or((None, None));
        if authored_normals'''),
])
for source, destination in [
    ('typed_payloads/mod.rs', 'src/container_10d/typed_payloads/mod.rs'),
    ('typed_payloads/readers.rs', 'src/container_10d/typed_payloads/readers.rs'),
    ('typed_field_payloads.rs', 'tests/typed_field_payloads.rs'),
]:
    target = CORE / destination
    if target.exists(): raise RuntimeError('New file exists: ' + str(target))
    save(destination, None, (HERE/'staged'/source).read_bytes())
(HERE/'sdk-save-manifest.json').write_text(json.dumps(rows, indent=2)+'\n', encoding='utf-8')
diff = []
for row in rows:
    rel=row['relative']
    old=(HERE/'before'/rel).read_text(encoding='utf-8').splitlines(True) if row['before_sha256'] else []
    new=(HERE/'after'/rel).read_text(encoding='utf-8').splitlines(True)
    diff.extend(difflib.unified_diff(old,new,fromfile='a/'+rel,tofile='b/'+rel))
(HERE/'foundation-sdk.patch').write_text(''.join(diff), encoding='utf-8')
print('Prepared',len(rows),'guarded source candidates; no Core writes')
