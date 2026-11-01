//! Verify the changed faces in the actual canonical compiled full-tree mesh.
//! Existing untouched foliage is not reclassified as junction topology.
use super::{attributes, range};
use crate::scene_builder_10d::SubmeshBuilder;
use serde_json::Value;
pub(crate) struct ProjectionWitness {
    vertices: usize,
    triangles: usize,
    first_face: usize,
    faces: Vec<([u32; 3], [f64; 3])>,
    minimum: f64,
    changed_vertices: std::ops::Range<usize>,
    topology_ids: Vec<u32>,
}
fn check_aliases(positions: &[[f32; 3]], ids: &[u32]) -> Result<(), String> {
    let mut by_position = std::collections::BTreeMap::new();
    let mut by_id = std::collections::BTreeMap::new();
    if positions.len() != ids.len() {
        return Err("Tree topology alias count mismatch".into());
    }
    for (p, &id) in positions.iter().zip(ids) {
        if p.iter().any(|x| !x.is_finite()) {
            return Err("Nonfinite tree projection".into());
        }
        let bits = p.map(|x| if x == 0.0 { 0 } else { x.to_bits() });
        if by_position.insert(bits, id).is_some_and(|old| old != id)
            || by_id.insert(id, bits).is_some_and(|old| old != bits)
        {
            return Err("Tree float/native projection merges distinct topology vertices or separates an attribute seam".into());
        }
    }
    Ok(())
}
pub(crate) fn projection_witness(
    wood: &SubmeshBuilder,
    foliage: &SubmeshBuilder,
    metadata: &Value,
) -> Result<ProjectionWitness, String> {
    let detail = &metadata["chain_union"];
    let r = range(&detail["wood_indices"])?;
    let changed_vertices = range(&detail["wood_vertices"])?;
    let minimum = detail["min_area_sq_m4"]
        .as_f64()
        .ok_or("Missing tree union area threshold")?;
    if r.start % 3 != 0
        || r.end % 3 != 0
        || r.end > wood.indices.len()
        || wood.indices.len() % 3 != 0
        || foliage.indices.len() % 3 != 0
        || !minimum.is_finite()
        || minimum <= 0.0
        || changed_vertices.end > wood.positions.len()
        || changed_vertices.is_empty()
    {
        return Err("Invalid tree union compiled witness".into());
    }
    let mut faces = Vec::with_capacity(r.len() / 3);
    let topology_ids = detail["render_vertex_lineage"]
        .as_array()
        .ok_or("Missing changed vertex provenance")?
        .iter()
        .map(|v| {
            v["topology_vertex"]
                .as_u64()
                .and_then(|i| u32::try_from(i).ok())
                .ok_or_else(|| "Invalid topology vertex provenance".to_string())
        })
        .collect::<Result<Vec<_>, String>>()?;
    check_aliases(&wood.positions[changed_vertices.clone()], &topology_ids)?;
    for t in wood.indices[r.clone()].chunks_exact(3) {
        let t = [t[0], t[1], t[2]];
        if t.iter().any(|&i| !changed_vertices.contains(&(i as usize))) {
            return Err("Changed tree face escapes attributed vertices".into());
        }
        let p = t.map(|i| wood.positions[i as usize].map(f64::from));
        let f = attributes::face(p[0], p[1], p[2]);
        attributes::check(f, f, minimum)?;
        faces.push((t, f));
    }
    Ok(ProjectionWitness {
        vertices: wood
            .positions
            .len()
            .checked_add(foliage.positions.len())
            .ok_or("Tree vertex count overflow")?,
        triangles: wood
            .indices
            .len()
            .checked_add(foliage.indices.len())
            .ok_or("Tree index count overflow")?
            / 3,
        first_face: r.start / 3,
        faces,
        minimum,
        changed_vertices,
        topology_ids,
    })
}
pub(crate) fn qualify_compiled_projection(
    bytes: &[u8],
    w: &ProjectionWitness,
) -> Result<(), String> {
    use qualia_core_db::container_10d::{self as native, Container10dHeader, SectionType};
    let header =
        Container10dHeader::parse(bytes).map_err(|e| format!("Tree .10d header: {e:?}"))?;
    let sections = native::parse_section_table(bytes, &header)
        .map_err(|e| format!("Tree .10d table: {e:?}"))?;
    let mut matches = sections
        .iter()
        .filter(|s| s.typ() == Some(SectionType::QuantizedMesh));
    let s = matches.next().ok_or("Tree .10d mesh absent")?;
    if matches.next().is_some() {
        return Err("Ambiguous tree .10d mesh".into());
    }
    let start = usize::try_from(s.byte_offset).map_err(|_| "Tree .10d offset overflow")?;
    let len = usize::try_from(s.byte_length).map_err(|_| "Tree .10d length overflow")?;
    let data = bytes
        .get(start..start.checked_add(len).ok_or("Tree .10d range overflow")?)
        .ok_or("Tree .10d mesh outside file")?;
    let (h, size) =
        native::parse_mesh_header(data).map_err(|e| format!("Tree mesh header: {e:?}"))?;
    if size != data.len()
        || h.vertex_count as usize != w.vertices
        || h.triangle_count as usize != w.triangles
    {
        return Err("Tree compile changed mesh counts".into());
    }
    let decoded = native::decode_mesh_section(data)
        .map_err(|e| format!("Tree canonical mesh decode: {e:?}"))?;
    let end = w
        .first_face
        .checked_add(w.faces.len())
        .ok_or("Tree witness range overflow")?;
    let changed = decoded
        .triangles
        .get(w.first_face..end)
        .ok_or("Tree compiled changed faces absent")?;
    check_aliases(
        &decoded.positions[w.changed_vertices.clone()],
        &w.topology_ids,
    )?;
    for (actual, (expected, reference)) in changed.iter().zip(&w.faces) {
        if actual != expected {
            return Err("Tree compile changed face correspondence".into());
        }
        let p = actual.map(|i| decoded.positions[i as usize].map(f64::from));
        attributes::check(attributes::face(p[0],p[1],p[2]),*reference,w.minimum)
            .map_err(|_|"Native .10d quantization collapsed/reversed a changed junction face; previous source retained".to_string())?;
    }
    // Read real typed frames/UVs and qualify only changed attribute-split
    // vertices. Legacy inactive vertices have no tangent correspondence claim.
    let limits = native::MeshFrameDecodeLimits {
        max_vertices: w.vertices,
        max_payload_bytes: bytes.len(),
        max_decoded_bytes: w
            .vertices
            .checked_mul(28)
            .ok_or("Tree frame bytes overflow")?,
    };
    let layout = native::inspect_container_mesh_frames(bytes, w.vertices, limits)
        .map_err(|e| format!("Tree frame layout: {e:?}"))?
        .ok_or("Tree changed faces require canonical frames")?;
    if !layout.has_normals || !layout.has_tangents {
        return Err("Tree changed faces lack canonical normal/tangent streams".into());
    }
    let mut normals = vec![[0.; 3]; w.vertices];
    let mut tangents = vec![[0.; 4]; w.vertices];
    let mut uv = vec![[0.; 2]; w.vertices];
    native::read_container_mesh_frames_into(bytes, w.vertices, limits, &mut normals, &mut tangents)
        .map_err(|e| format!("Tree frame readback: {e:?}"))?;
    native::read_texture_coordinate_section_into(bytes, w.vertices, &mut uv)
        .map_err(|e| format!("Tree UV readback: {e:?}"))?;
    let mut selected = SubmeshBuilder::new();
    selected
        .positions
        .extend_from_slice(&decoded.positions[w.changed_vertices.clone()]);
    selected
        .normals
        .extend_from_slice(&normals[w.changed_vertices.clone()]);
    selected
        .uvs
        .extend_from_slice(&uv[w.changed_vertices.clone()]);
    for t in changed {
        for &i in t {
            selected.indices.push(i - w.changed_vertices.start as u32);
        }
    }
    let expected = attributes::strict_frames(&selected)?;
    // Explicit source-face-barycentric-v1 projection admission tolerance.
    // No implied scientific or visual acceptance: refuse a stored frame that
    // no longer corresponds to the actual quantized chart orientation.
    for (i, (stored, expected)) in tangents[w.changed_vertices.clone()]
        .iter()
        .zip(&expected)
        .enumerate()
    {
        let normal = selected.normals[i];
        let tl = (stored[..3].iter().map(|v| v * v).sum::<f32>()).sqrt();
        let nl = (normal.iter().map(|v| v * v).sum::<f32>()).sqrt();
        let alignment = (0..3).map(|k| stored[k] * expected[k] / tl).sum::<f32>();
        let perpendicular = (0..3)
            .map(|k| stored[k] * normal[k] / (tl * nl))
            .sum::<f32>()
            .abs();
        if !alignment.is_finite()
            || alignment < 0.999
            || !perpendicular.is_finite()
            || perpendicular > 0.01
            || stored[3] != expected[3]
        {
            return Err("Stored tree tangent deviates from canonical decoded UV frame (v1 requires direction dot>=0.999, normal absdot<=0.01 and exact handedness)".into());
        }
    }
    Ok(())
}
