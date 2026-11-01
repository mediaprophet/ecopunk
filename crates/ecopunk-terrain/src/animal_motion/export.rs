//! Cold inspection-pose derivative. The editable SKL1 master is retained inside
//! provenance; its rig is never attached to already-deformed surface vertices.
use super::*;
use crate::species_review::SpeciesReviewAsset;
use qualia_core_db::{
    container_10d::{
        self as c, Container10dHeader, SectionDescriptor, SectionInput, SectionTableError,
        SectionType,
    },
    render::compile_10d,
};
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicBool, Ordering};
/// Logical incremental workspace/output caps, not measured RSS or peak stack.
#[derive(Clone, Copy)]
pub struct PoseExportLimits {
    pub max_container_bytes: usize,
    pub cold_bytes: usize,
}
impl Default for PoseExportLimits {
    fn default() -> Self {
        Self {
            max_container_bytes: 42 * 1024 * 1024,
            cold_bytes: 256 * 1024 * 1024,
        }
    }
}
/// Caller supplies the exact admitted master and a successful frame from that
/// player. Output remains a technical inspection derivative, not a new rig or
/// accepted biological motion. All source colour/UV/material/range bytes remain
/// exact; normals/tangents and positions are the Core-deformed frame.
pub fn export_frame_10d(
    master: &[u8],
    frame: &MotionFrame<'_>,
    limits: PoseExportLimits,
    cancel: &AtomicBool,
) -> Result<SpeciesReviewAsset, String> {
    if cancel.load(Ordering::Acquire) {
        return Err("Pose export cancelled".into());
    }
    let n = frame.positions.len();
    if master.len()
        > limits
            .max_container_bytes
            .min(c::provenance_section::MAX_PROVENANCE_FIELD)
        || n == 0
        || n > 262144
        || frame.normals.len() != n
        || frame.tangents.len() != n
        || frame.model_sha256 != <[u8; 32]>::from(Sha256::digest(master))
        || frame
            .positions
            .iter()
            .flatten()
            .chain(frame.normals.iter().flatten())
            .chain(frame.tangents.iter().flatten())
            .any(|v| !v.is_finite())
    {
        return Err("Pose master/frame binding or bounds refused".into());
    }
    let header = Container10dHeader::parse(master).map_err(|e| format!("Pose header: {e:?}"))?;
    if c::integrity::compute_whole_file_crc32c(master) != header.header_crc32c {
        return Err("Pose master whole-file integrity differs".into());
    }
    let sections =
        c::parse_section_table(master, &header).map_err(|e| format!("Pose table: {e:?}"))?;
    if sections.iter().any(|d| {
        !matches!(
            d.typ(),
            Some(
                SectionType::QuantizedMesh
                    | SectionType::FieldSidecar
                    | SectionType::SurfaceReading
                    | SectionType::ProvenanceSidecar
                    | SectionType::Materials
                    | SectionType::TextureCoordinates
                    | SectionType::SkeletalAnimation
            )
        )
    }) {
        return Err(
            "Pose derivative refuses unsupported master sections rather than dropping them".into(),
        );
    }
    let mesh_section = sections
        .iter()
        .find(|d| d.typ() == Some(SectionType::QuantizedMesh))
        .ok_or("Pose lacks master mesh")?;
    let payload = bytes(master, mesh_section)?;
    let (mh, _) = c::parse_mesh_header(payload).map_err(|e| format!("Pose mesh: {e:?}"))?;
    let index_count = (mh.triangle_count as usize)
        .checked_mul(3)
        .ok_or("Pose index overflow")?;
    let reserved = n
        .checked_mul(256)
        .and_then(|v| index_count.checked_mul(24).and_then(|i| v.checked_add(i)))
        .and_then(|v| master.len().checked_mul(5).and_then(|b| v.checked_add(b)))
        .ok_or("Pose workspace overflow")?;
    if mh.vertex_count as usize != n || index_count > 1572864 || reserved > limits.cold_bytes {
        return Err("Pose derivative exceeds caller workspace budget".into());
    }
    let mut mesh = c::decode_mesh_section(payload).map_err(|e| format!("Pose decode: {e:?}"))?;
    mesh.positions.copy_from_slice(frame.positions);
    mesh.min = [f32::INFINITY; 3];
    mesh.max = [f32::NEG_INFINITY; 3];
    for p in frame.positions {
        for a in 0..3 {
            mesh.min[a] = mesh.min[a].min(p[a]);
            mesh.max[a] = mesh.max[a].max(p[a]);
        }
    }
    let old_provenance = sections
        .iter()
        .find(|d| d.typ() == Some(SectionType::ProvenanceSidecar))
        .ok_or("Pose refuses provenance-free master")?;
    let old = c::provenance_section::decode_provenance_section(bytes(master, old_provenance)?)
        .map_err(|e| format!("Pose source context: {e:?}"))?;
    let metadata = phase_metadata(master, frame)?;
    let mut cborld = vec![0xd9, 0xd9, 0xf7];
    ciborium::into_writer(&metadata, &mut cborld)
        .map_err(|e| format!("Pose canonical CBOR metadata: {e}"))?;
    let prov = c::provenance_section::ProvenanceSidecar::new(
        master.to_vec(),
        "application/vnd.qualia.10d",
        old.licence(),
    )
    .with_metadata(cborld, 0, frame.model_sha256);
    let fresh = compile_10d::compile_mesh_to_10d_with_extras_and_frames(
        &mesh,
        &[],
        Some(&prov),
        Default::default(),
        None,
        Some(frame.normals),
        Some(frame.tangents),
    )
    .map_err(|e| format!("Pose canonical compile: {e:?}"))?;
    let h =
        Container10dHeader::parse(&fresh).map_err(|e| format!("Pose generated header: {e:?}"))?;
    let fresh_sections =
        c::parse_section_table(&fresh, &h).map_err(|e| format!("Pose generated table: {e:?}"))?;
    let mut inputs = Vec::new();
    inputs
        .try_reserve_exact(6)
        .map_err(|_| "Pose section allocation refused")?;
    for d in fresh_sections {
        inputs.push(input(&fresh, d)?);
    }
    for kind in [
        SectionType::SurfaceReading,
        SectionType::Materials,
        SectionType::TextureCoordinates,
    ] {
        let d = sections
            .iter()
            .find(|d| d.typ() == Some(kind))
            .ok_or("Pose requires retained colour/material/UV streams")?;
        inputs.push(input(master, d)?);
    }
    let needed = match c::encode_container(&h, &inputs, &mut []) {
        Err(SectionTableError::OutputBufferTooSmall { needed, .. }) => needed,
        Ok(n) => n,
        Err(e) => return Err(format!("Pose output sizing: {e:?}")),
    };
    if needed > limits.max_container_bytes {
        return Err("Pose output container cap exceeded".into());
    }
    let mut out = Vec::new();
    out.try_reserve_exact(needed)
        .map_err(|_| "Pose output allocation refused")?;
    out.resize(needed, 0);
    let written = c::encode_container(&h, &inputs, &mut out)
        .map_err(|e| format!("Pose output encode: {e:?}"))?;
    out.truncate(written);
    c::integrity::seal_whole_file_crc32c(&mut out);
    if cancel.load(Ordering::Acquire) {
        return Err("Pose export cancelled".into());
    }
    Ok(SpeciesReviewAsset {
        bytes: out,
        metadata,
    })
}
fn phase_metadata(master: &[u8], frame: &MotionFrame<'_>) -> Result<serde_json::Value, String> {
    if frame.contact_count > MAX_CONTACTS
        || !frame.receipt.time.is_finite()
        || frame.receipt.time < 0.
        || frame.receipt.clip_id == 0
    {
        return Err("Pose receipt refused".into());
    }
    let contacts:Vec<_>=frame.contacts[..frame.contact_count].iter().map(|r|serde_json::json!({"foot_joint":r.foot_joint,"target_model_m":r.target_model_m,"actual_model_m":r.actual_model_m,"error_m":r.error_m,"knee_angle_rad":r.knee_angle_rad})).collect();
    Ok(
        serde_json::json!({"@context":{"@vocab":"urn:ecopunk:authored-motion-inspection-pose/v1#"},"schema":"ecopunk.authored-motion-inspection-pose/v1","status":"unaccepted-authored-study","master_sha256":format!("{:x}",Sha256::digest(master)),"source_projection_sha256":hex(&frame.source_sha256),"clip_id":frame.receipt.clip_id,"time_s":frame.receipt.time,"model_frame":FRAME,"top_level_skl1":"omitted: already deformed pose; editable master retained exactly in provenance","contacts":contacts,"root_translation":"already embedded in output positions; do not apply delta again","rights":"inherits existing master license declaration; no new permission or biological acceptance"}),
    )
}
fn hex(v: &[u8]) -> String {
    v.iter().map(|b| format!("{b:02x}")).collect()
}
fn bytes<'a>(data: &'a [u8], d: &SectionDescriptor) -> Result<&'a [u8], String> {
    let end = (d.byte_offset as usize)
        .checked_add(d.byte_length as usize)
        .ok_or("Pose section overflow")?;
    data.get(d.byte_offset as usize..end)
        .ok_or_else(|| "Pose section outside resource".into())
}
fn input<'a>(data: &'a [u8], d: &SectionDescriptor) -> Result<SectionInput<'a>, String> {
    Ok(SectionInput {
        section_type: d.typ().ok_or("Pose section type")?,
        alignment_tier: d.tier().ok_or("Pose alignment")?,
        stride: d.stride,
        element_count: d.element_count,
        payload: bytes(data, d)?,
    })
}
