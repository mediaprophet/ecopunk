//! Explicit two-chain hard union in the retained full-tree review projection.
//! Other chains and sprays are retained. This is not a smooth botanical collar
//! or a whole-tree welded master. Only a successful detached build is published.
mod attributes;
mod projection;
#[cfg(test)]
mod tests;
mod union;
use crate::scene_builder_10d::SubmeshBuilder;
pub(crate) use projection::{projection_witness, qualify_compiled_projection, ProjectionWitness};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChainUnionSpec {
    pub schema: String,
    /// First-child keys of two maximal chains, not proximity selectors.
    pub chains: [String; 2],
    pub attribute_policy: String,
    pub min_area_sq_m4: f64,
}
#[derive(Clone, Copy, Debug)]
pub struct TreeJunctionLimits {
    pub input_faces: usize,
    pub fragments_per_face: usize,
    pub vertices: usize,
    pub faces: usize,
    pub bytes: usize,
    pub work: usize,
}
impl Default for TreeJunctionLimits {
    fn default() -> Self {
        Self {
            input_faces: 2048,
            fragments_per_face: 256,
            vertices: 8192,
            faces: 8192,
            bytes: 256 * 1024 * 1024,
            work: 64_000_000,
        }
    }
}
pub(super) struct Chain {
    pub mesh: SubmeshBuilder,
    pub topology_vertices: Vec<u32>,
    pub original_vertices: Vec<u32>,
}
fn range(v: &Value) -> Result<std::ops::Range<usize>, String> {
    let a = v
        .get(0)
        .and_then(Value::as_u64)
        .and_then(|x| usize::try_from(x).ok())
        .ok_or("Missing chain range")?;
    let b = v
        .get(1)
        .and_then(Value::as_u64)
        .and_then(|x| usize::try_from(x).ok())
        .ok_or("Missing chain range")?;
    if a > b {
        return Err("Reversed chain range".into());
    }
    Ok(a..b)
}
fn extract(mesh: &SubmeshBuilder, binding: &Value) -> Result<Chain, String> {
    let vr = range(&binding["wood_vertices"])?;
    let ir = range(&binding["wood_indices"])?;
    let source = binding["derivative_to_base_sweep_vertices"]
        .as_array()
        .ok_or("Missing exact sweep lineage")?;
    if vr.end > mesh.positions.len()
        || ir.end > mesh.indices.len()
        || source.len() != vr.len()
        || ir.len() % 3 != 0
    {
        return Err("Invalid chain binding".into());
    }
    let mut out = SubmeshBuilder::new();
    out.positions.extend_from_slice(&mesh.positions[vr.clone()]);
    out.normals.extend_from_slice(&mesh.normals[vr.clone()]);
    out.colors.extend_from_slice(&mesh.colors[vr.clone()]);
    out.uvs.extend_from_slice(&mesh.uvs[vr.clone()]);
    for &v in &mesh.indices[ir] {
        if !vr.contains(&(v as usize)) {
            return Err("Chain index escapes binding".into());
        }
        out.indices.push(v - vr.start as u32);
    }
    let topology_vertices = source
        .iter()
        .map(|v| {
            v.as_u64()
                .and_then(|x| u32::try_from(x).ok())
                .ok_or_else(|| "Invalid sweep topology lineage".into())
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Chain {
        mesh: out,
        topology_vertices,
        original_vertices: (vr.start as u32..vr.end as u32).collect(),
    })
}

pub(crate) fn apply(
    wood: &mut SubmeshBuilder,
    foliage: &SubmeshBuilder,
    metadata: &mut Value,
    spec: &ChainUnionSpec,
    limits: TreeJunctionLimits,
) -> Result<(), String> {
    if spec.schema != "ecopunk.incident-chain-union/v1"
        || spec.attribute_policy != "source-face-barycentric-v1"
        || spec.chains[0] == spec.chains[1]
        || !spec.min_area_sq_m4.is_finite()
        || spec.min_area_sq_m4 < 1e-28
        || spec.min_area_sq_m4 > 1e-6
    {
        return Err("Invalid explicit chain-union profile or attribute policy".into());
    }
    // Reserve full retained geometry, duplicate source copies, lineage JSON and
    // canonical decode/witness envelope before cloning selected streams. This is
    // logical storage, not allocator metadata/RSS. Host limits cannot be source-raised.
    // Per changed face: 1536 bytes typed streams/lineage (actual Vec capacities
    // checked), 3072 Value/map/key/array payload at final serialization, 1536
    // alias-map nodes and canonical readback/frame scratch, 2048 growth/copy
    // allowance. Whole retained streams/native decode use the separate 256/V
    // and 32/index terms; the fixed 8 MiB covers bounded source/binding records.
    let wrapper = wood
        .positions
        .len()
        .checked_add(foliage.positions.len())
        .ok_or("Tree retained vertex count overflow")?
        .checked_mul(256)
        .and_then(|n| {
            wood.indices
                .len()
                .checked_add(foliage.indices.len())?
                .checked_mul(32)
                .and_then(|i| n.checked_add(i))
        })
        .and_then(|n| {
            limits
                .faces
                .checked_mul(8192)
                .and_then(|i| n.checked_add(i))
        })
        .and_then(|n| n.checked_add(8 * 1024 * 1024))
        .ok_or("Tree union storage overflow")?;
    if wrapper > limits.bytes {
        return Err("Tree union retained/projection envelope exceeds caller bytes".into());
    }
    let bindings = metadata["bindings"]
        .as_array()
        .ok_or("Missing full-tree bindings")?;
    let mut chosen = [usize::MAX; 2];
    for (i, b) in bindings.iter().enumerate() {
        if b["kind"] != "chain" {
            continue;
        }
        for (j, key) in spec.chains.iter().enumerate() {
            if b["organ_keys"].get(1).and_then(Value::as_str) == Some(key) {
                if chosen[j] != usize::MAX {
                    return Err("Ambiguous chain key".into());
                }
                chosen[j] = i;
            }
        }
    }
    if chosen.contains(&usize::MAX) {
        return Err("Selected chain does not start at an existing root/fork".into());
    }
    let ka = bindings[chosen[0]]["organ_keys"]
        .as_array()
        .ok_or("Missing chain organs")?;
    let kb = bindings[chosen[1]]["organ_keys"]
        .as_array()
        .ok_or("Missing chain organs")?;
    let common = ka.iter().filter(|a| kb.contains(a)).collect::<Vec<_>>();
    if common.len() != 1
        || ![ka.first(), ka.last()].contains(&Some(common[0]))
        || ![kb.first(), kb.last()].contains(&Some(common[0]))
    {
        return Err(
            "Union requires exactly one shared authored endpoint; no proximity welding".into(),
        );
    }
    let joint = common[0].clone();
    let ranges = [
        range(&bindings[chosen[0]]["wood_indices"])?,
        range(&bindings[chosen[1]]["wood_indices"])?,
    ];
    let a = extract(wood, &bindings[chosen[0]])?;
    let b = extract(wood, &bindings[chosen[1]])?;
    let mut core_limits = limits;
    core_limits.bytes -= wrapper;
    let wrapper_work = wood
        .positions
        .len()
        .checked_add(foliage.positions.len())
        .ok_or("Tree work vertex count overflow")?
        .checked_mul(16)
        .and_then(|n| {
            wood.indices
                .len()
                .checked_add(foliage.indices.len())?
                .checked_mul(8)
                .and_then(|i| n.checked_add(i))
        })
        .ok_or("Tree union wrapper work overflow")?;
    core_limits.work = limits
        .work
        .checked_sub(wrapper_work)
        .ok_or("Tree retained-stream projection exceeds caller work")?;
    let (joined, lineage, receipt) = union::build(&a, &b, spec.min_area_sq_m4, core_limits)?;
    let new_vertices = wood
        .positions
        .len()
        .checked_add(joined.positions.len())
        .ok_or("Tree union vertex overflow")?;
    let new_indices = wood
        .indices
        .len()
        .checked_sub(ranges[0].len() + ranges[1].len())
        .and_then(|n| n.checked_add(joined.indices.len()))
        .ok_or("Tree union index overflow")?;
    if new_vertices > 400_000 || new_indices > 1_800_000 {
        return Err("Tree union exceeds full-tree review projection ceiling".into());
    }
    // Preserve old vertex IDs/streams exactly. Removed source chain vertices
    // remain explicitly inactive storage; only their faces are replaced.
    let mut indices = Vec::with_capacity(new_indices);
    let mut updated = bindings.clone();
    for (i, binding) in updated.iter_mut().enumerate() {
        let old = range(&binding["wood_indices"])?;
        let start = indices.len();
        if !chosen.contains(&i) {
            indices.extend_from_slice(&wood.indices[old.clone()]);
        }
        binding["pre_union_wood_indices"] = json!([old.start, old.end]);
        binding["wood_indices"] = json!([start, indices.len()]);
        if chosen.contains(&i) {
            binding["fork_surface"] =
                json!("input faces replaced by explicit union; retained vertices inactive");
        }
    }
    let vertex_start = wood.positions.len();
    let index_start = indices.len();
    indices.extend(joined.indices.iter().map(|i| *i + vertex_start as u32));
    wood.positions.extend(joined.positions);
    wood.normals.extend(joined.normals);
    wood.colors.extend(joined.colors);
    wood.uvs.extend(joined.uvs);
    wood.indices = indices;
    metadata["bindings"] = Value::Array(updated);
    metadata["candidate"] = json!("white-gum-chain-union-v1");
    metadata["junctions"]=json!("Only explicitly selected incident chain pair hard-unioned; other fork/root/shoot shells remain separate; no smooth collar or whole-master weld");
    metadata["chain_union"] = json!({"chains":spec.chains,"shared_authored_organ":joint,"input_bindings":chosen,
        "wood_vertices":[vertex_start,wood.positions.len()],"wood_indices":[index_start,wood.indices.len()],
        "min_area_sq_m4":spec.min_area_sq_m4,"render_vertex_lineage":lineage,"receipt":receipt,
        "attribute_policy":"source-face barycentric colors/UV and normalized source normals; explicit face attribute seams mapped to qualified topology vertices",
        "tangents":"changed faces pass strict Core UV/frame admission before and after canonical compile; compiler generates frames from retained UV/normals; stored frames are not an authored preservation claim",
        "geometry":"Existing Core exact-construction union with approximate splitting/classification and 1e-9 assembly tolerance; hard C0 seam, not smoothing",
        "topology":"selected pair only: one closed oriented manifold vertex-fan component; no general self-intersection or botanical quality certificate",
        "untouched":"All other chains, shoots and foliage keep exact vertex streams and face order; old selected vertices retained inactive",
        "acceptance":"source-only candidate; no executed or visual acceptance"});
    Ok(())
}
