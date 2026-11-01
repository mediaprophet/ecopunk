use super::super::ValidatedSource;
use super::{cuts, pieces, plan, topology, AllForkLimits, AllForkSpec, ChainSpan};
use crate::scene_builder_10d::SubmeshBuilder;
use crate::smooth_collars::{self, CollarInput};
use qualia_core_db::specialized_libs::computational_geometry::{
    tapered_branch_field::TaperedFieldSegment, Point3,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

type SampleKey = ([u32; 3], [u64; 3]);
struct Draft {
    mesh: SubmeshBuilder,
    changed: Vec<u32>,
    aliases: Vec<u32>,
    samples: BTreeMap<SampleKey, u32>,
    topology_samples: BTreeMap<SampleKey, u32>,
    next_alias: u32,
    original_vertices: usize,
    limits: AllForkLimits,
    lineage: Vec<Value>,
}
impl Draft {
    fn bounded(&self, vertices: usize, indices: usize) -> Result<(), String> {
        if vertices > self.limits.output_vertices
            || indices > self.limits.output_indices
            || vertices > u32::MAX as usize
        {
            return Err("Complete all-fork derivative exceeds aggregate caller mesh limits".into());
        }
        Ok(())
    }
    fn sample(&mut self, s: &cuts::Sample) -> Result<u32, String> {
        if let Some(&v) = self.samples.get(&s.identity()) {
            return Ok(v);
        }
        self.bounded(
            self.mesh
                .positions
                .len()
                .checked_add(1)
                .ok_or("Cut vertex overflow")?,
            self.mesh
                .indices
                .len()
                .checked_add(self.changed.len())
                .ok_or("Cut index overflow")?,
        )?;
        let mut key = s.identity();
        for v in &mut key.0 {
            *v = *self
                .aliases
                .get(*v as usize)
                .ok_or("Cut source alias outside retained mesh")?;
        }
        // Geometric construction identity is separate from its attribute chart.
        let alias = if s.barycentric == [1.0, 0.0, 0.0] {
            key.0[0]
        } else {
            let marker = key;
            // All exact cuts are generated through the same source-order
            // endpoint representation; no nearest or position-based joining.
            if let Some(&v) = self.topology_samples.get(&marker) {
                v
            } else {
                let id = self.next_alias;
                self.next_alias = self
                    .next_alias
                    .checked_add(1)
                    .ok_or("Cut topology overflow")?;
                id
            }
        };
        let index = self.mesh.positions.len() as u32;
        self.mesh.positions.push(s.position);
        self.mesh.normals.push(s.normal);
        self.mesh.colors.push(s.color);
        self.mesh.uvs.push(s.uv);
        self.aliases.push(alias);
        self.lineage.push(
            json!({"topology_vertex":alias,"kind":"exact_source_triangle_clip",
            "original_wood_vertices":s.source_vertices,"barycentric":s.barycentric}),
        );
        self.samples.insert(s.identity(), index);
        self.topology_samples.insert(key, alias);
        Ok(index)
    }
    fn face(&mut self, face: [&cuts::Sample; 3]) -> Result<(), String> {
        self.bounded(
            self.mesh.positions.len(),
            self.mesh
                .indices
                .len()
                .checked_add(self.changed.len())
                .and_then(|v| v.checked_add(3))
                .ok_or("Changed face overflow")?,
        )?;
        for sample in face {
            let v = self.sample(sample)?;
            self.changed.push(v);
        }
        Ok(())
    }
}
fn canonical_aliases(
    wood: &SubmeshBuilder,
    spans: &[ChainSpan],
    plan: &plan::ForkPlan,
    sides: usize,
) -> Result<Vec<u32>, String> {
    let mut aliases = (0..wood.positions.len())
        .map(|v| u32::try_from(v).map_err(|_| "Retained topology exceeds u32".to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    for (span, chain) in spans.iter().zip(&plan.chains) {
        let rings = chain
            .organs
            .len()
            .checked_mul(sides)
            .ok_or("Source rings overflow")?;
        if span.vertices.len() != rings + 2 + 2 * sides {
            return Err("All-fork sweep lineage layout mismatch".into());
        }
        for side in 0..sides {
            aliases[span.vertices.start + rings + 2 + side] = (span.vertices.start + side) as u32;
            aliases[span.vertices.start + rings + 2 + sides + side] =
                (span.vertices.start + (chain.organs.len() - 1) * sides + side) as u32;
        }
    }
    Ok(aliases)
}
fn segment(
    source: &ValidatedSource,
    chain: &plan::ChainPlan,
    span: &ChainSpan,
    edge: usize,
    lo: f64,
    hi: f64,
) -> Result<TaperedFieldSegment, String> {
    let a = source.data.botanical.organs[chain.organs[edge]]
        .position_m
        .map(f64::from);
    let b = source.data.botanical.organs[chain.organs[edge + 1]]
        .position_m
        .map(f64::from);
    let point = |t: f64| {
        Point3::new(
            a[0] + (b[0] - a[0]) * t,
            a[1] + (b[1] - a[1]) * t,
            a[2] + (b[2] - a[2]) * t,
        )
    };
    let r0 = *span
        .effective_radii_m
        .get(edge)
        .ok_or("Missing source station radius")?;
    let r1 = *span
        .effective_radii_m
        .get(edge + 1)
        .ok_or("Missing source station radius")?;
    Ok(TaperedFieldSegment {
        a: point(lo),
        b: point(hi),
        radius_a: r0 + (r1 - r0) * lo,
        radius_b: r0 + (r1 - r0) * hi,
    })
}
fn bytes_for(v: usize, i: usize) -> Result<usize, String> {
    v.checked_mul(512)
        .and_then(|n| i.checked_mul(32).and_then(|i| n.checked_add(i)))
        .ok_or("All-fork retained/projection storage overflow".into())
}

pub(in crate::white_gum_authored_review) fn apply(
    wood: &mut SubmeshBuilder,
    foliage: &SubmeshBuilder,
    metadata: &mut Value,
    source: &ValidatedSource,
    spans: &[ChainSpan],
    spec: &AllForkSpec,
    limits: AllForkLimits,
    cancelled: &dyn Fn() -> bool,
) -> Result<(), String> {
    super::validate_spec(spec)?;
    if cancelled() {
        return Err("All-fork adoption cancelled".into());
    }
    let plan = plan::build(source, spec.cut_fraction, limits.work, cancelled)?;
    if plan.chains.len() != spans.len() || plan.junctions.is_empty() {
        return Err("All-fork source/chain inventory mismatch or no required junction".into());
    }
    let (clusters, cluster_work) = plan::clusters(
        source,
        &plan,
        &spec.coupled_junctions,
        limits
            .work
            .checked_sub(plan.structural_work)
            .ok_or("All-fork planning work exhausted")?,
        cancelled,
    )?;
    let mut work = plan
        .structural_work
        .checked_add(cluster_work)
        .ok_or("All-fork work overflow")?;
    // Before cloning: caller-capped complete mesh, canonical readback/witness,
    // old streams, source/stub/maps and final metadata envelope. This is logical
    // storage, not allocator RSS. Per-collar retained lineage is charged below.
    let mut retained = bytes_for(limits.output_vertices, limits.output_indices)?
        .checked_add(bytes_for(
            wood.positions.len() + foliage.positions.len(),
            wood.indices.len() + foliage.indices.len(),
        )?)
        .and_then(|n| n.checked_add(64 * 1024 * 1024))
        .ok_or("All-fork storage overflow")?;
    if retained >= limits.bytes {
        return Err("All-fork aggregate retained/projection envelope exceeds caller bytes".into());
    }
    let sides = source.data.botanical.ring_slices;
    let aliases = canonical_aliases(wood, spans, &plan, sides)?;
    let original_vertices = wood.positions.len();
    let mut draft = Draft {
        mesh: wood.clone(),
        changed: Vec::new(),
        aliases,
        samples: BTreeMap::new(),
        topology_samples: BTreeMap::new(),
        next_alias: u32::try_from(original_vertices)
            .map_err(|_| "All-fork topology exceeds u32")?,
        original_vertices,
        limits,
        lineage: Vec::new(),
    };
    draft.mesh.indices.clear();
    let mut internal = vec![false; plan.chains.len()];
    for cluster in &clusters {
        for &chain in &cluster.interior_chains {
            internal[chain] = true;
        }
    }
    let mut chain_ranges = Vec::with_capacity(plan.chains.len());
    for (index, (chain, span)) in plan.chains.iter().zip(spans).enumerate() {
        if cancelled() {
            return Err("All-fork trim cancelled; original retained".into());
        }
        let start = draft.mesh.indices.len();
        let changed_start = draft.changed.len();
        if !internal[index] {
            for edge in 0..chain.organs.len() - 1 {
                work = work
                    .checked_add(sides * 4096)
                    .ok_or("All-fork trim work overflow")?;
                if work > limits.work {
                    return Err("All-fork aggregate trim work exhausted".into());
                }
                let lo = if edge == 0 && chain.start_joint.is_some() {
                    spec.cut_fraction
                } else {
                    0.0
                };
                let hi = if edge + 2 == chain.organs.len() && chain.end_joint.is_some() {
                    1.0 - spec.cut_fraction
                } else {
                    1.0
                };
                if lo == 0.0 && hi == 1.0 {
                    let a = span.indices.start + edge * sides * 6;
                    draft.mesh.indices.extend_from_slice(
                        wood.indices
                            .get(a..a + sides * 6)
                            .ok_or("Source wall face range invalid")?,
                    );
                } else {
                    let lower = cuts::ring(wood, span.vertices.start, edge, sides, lo)?;
                    let upper = cuts::ring(wood, span.vertices.start, edge, sides, hi)?;
                    cuts::strip(&lower, &upper, |t| draft.face(t))?;
                }
            }
            let caps = span.indices.start + (chain.organs.len() - 1) * sides * 6;
            if chain.start_joint.is_none() {
                draft
                    .mesh
                    .indices
                    .extend_from_slice(&wood.indices[caps..caps + sides * 3]);
            }
            if chain.end_joint.is_none() {
                draft
                    .mesh
                    .indices
                    .extend_from_slice(&wood.indices[caps + sides * 3..caps + sides * 6]);
            }
        }
        chain_ranges.push(json!({"chain":index,"retained_indices":[start,draft.mesh.indices.len()],
            "changed_relative_indices":[changed_start,draft.changed.len()],"interior_to_coupled_region":internal[index]}));
    }
    let retained_structural_end = draft.mesh.indices.len();
    // Sprays are after all chain ranges in the existing generator. Retain every
    // original shoot/foliage stream and face exactly; no implicit graft claim.
    let chain_end = spans.last().ok_or("No source chains")?.indices.end;
    draft.mesh.indices.extend_from_slice(
        wood.indices
            .get(chain_end..)
            .ok_or("Invalid retained spray index range")?,
    );
    let mut collar_records = Vec::with_capacity(clusters.len());
    for cluster in &clusters {
        if cancelled() {
            return Err("All-fork collar adoption cancelled; original retained".into());
        }
        // Charge caller-side source sampling/maps before constructing closed
        // stubs. Each stub has <=6*sides+2 attributed vertices and30*sides
        // indices; the factors cover bounded key probes and interpolation.
        let mut constructor_work = cluster
            .ports
            .len()
            .checked_mul((6 * sides + 2) * 256 + 30 * sides * 32)
            .ok_or("All-fork stub work overflow")?;
        for &index in &cluster.interior_chains {
            constructor_work = constructor_work
                .checked_add(
                    spans[index]
                        .vertices
                        .len()
                        .checked_mul(256)
                        .and_then(|n| {
                            spans[index]
                                .indices
                                .len()
                                .checked_mul(32)
                                .and_then(|i| n.checked_add(i))
                        })
                        .ok_or("All-fork interior work overflow")?,
                )
                .ok_or("All-fork constructor work overflow")?;
        }
        work = work
            .checked_add(constructor_work)
            .ok_or("All-fork constructor work overflow")?;
        if work > limits.work {
            return Err("All-fork aggregate constructor work exhausted".into());
        }
        let mut pieces = Vec::new();
        let mut ports = Vec::new();
        let mut segments = Vec::new();
        for &index in &cluster.interior_chains {
            if cancelled() {
                return Err("All-fork interior source construction cancelled".into());
            }
            let chain = &plan.chains[index];
            let span = &spans[index];
            let key = |n: usize| source.data.botanical.organs[n].key.clone();
            pieces.push(pieces::interior(
                wood,
                span,
                chain.organs.len(),
                sides,
                key(chain.organs[1]),
                [
                    key(chain.organs[0]),
                    key(*chain.organs.last().ok_or("Empty interior chain")?),
                ],
            )?);
            for edge in 0..chain.organs.len() - 1 {
                segments.push(segment(source, chain, span, edge, 0.0, 1.0)?);
            }
        }
        for &[joint_index, incident_index] in &cluster.ports {
            if cancelled() {
                return Err("All-fork boundary source construction cancelled".into());
            }
            let joint = &plan.junctions[joint_index];
            let incident = &joint.incidents[incident_index];
            let chain = &plan.chains[incident.chain];
            let span = &spans[incident.chain];
            let edge = if incident.at_start {
                0
            } else {
                chain.organs.len() - 2
            };
            let edge_child = chain.organs[edge + 1];
            let a = source.data.botanical.organs[chain.organs[edge]]
                .position_m
                .map(f64::from);
            let b = source.data.botanical.organs[edge_child]
                .position_m
                .map(f64::from);
            let delta = std::array::from_fn::<_, 3, _>(|k| b[k] - a[k]);
            let length = delta.iter().map(|v| v * v).sum::<f64>().sqrt();
            if !length.is_finite() || length <= 0.0 {
                return Err("Invalid source incident axis".into());
            }
            let outward = delta.map(|v| v / length * if incident.at_start { 1.0 } else { -1.0 });
            let key = |n: usize| source.data.botanical.organs[n].key.clone();
            let (piece, port) = pieces::stub(
                wood,
                span,
                &chain.organs,
                sides,
                incident.at_start,
                spec.cut_fraction,
                spec.guard_fraction,
                spec.transition_fraction,
                key(edge_child),
                key(joint.organ),
                [key(chain.organs[edge]), key(edge_child)],
                pieces.len(),
                outward,
                length,
            )?;
            pieces.push(piece);
            ports.push(port);
            segments.push(segment(
                source,
                chain,
                span,
                edge,
                if incident.at_start {
                    0.0
                } else {
                    1.0 - spec.cut_fraction
                },
                if incident.at_start {
                    spec.cut_fraction
                } else {
                    1.0
                },
            )?);
        }
        let ids = cluster
            .junctions
            .iter()
            .map(|&j| {
                source.data.botanical.organs[plan.junctions[j].organ]
                    .key
                    .clone()
            })
            .collect::<Vec<_>>();
        let mut collar_limits = limits.collar;
        collar_limits.bytes = collar_limits.bytes.min(
            limits
                .bytes
                .checked_sub(retained)
                .ok_or("All-fork retained bytes exhausted")?,
        );
        collar_limits.work = collar_limits.work.min(
            limits
                .work
                .checked_sub(work)
                .ok_or("All-fork aggregate work exhausted")?,
        );
        let collar = smooth_collars::build(
            CollarInput {
                pieces: &pieces,
                ports: &ports,
                field_segments: &segments,
                joint_ids: &ids,
            },
            spec.collar,
            collar_limits,
            cancelled,
        )
        .map_err(|e| {
            format!(
                "Required all-fork region {:?} refused, no fallback: {e}",
                ids
            )
        })?;
        work = work
            .checked_add(collar.work_used)
            .ok_or("All-fork collar work overflow")?;
        work = work
            .checked_add(
                collar
                    .mesh
                    .positions
                    .len()
                    .checked_mul(256)
                    .and_then(|n| {
                        collar
                            .mesh
                            .indices
                            .len()
                            .checked_mul(32)
                            .and_then(|i| n.checked_add(i))
                    })
                    .ok_or("All-fork collar adoption work overflow")?,
            )
            .ok_or("All-fork aggregate work overflow")?;
        retained = retained
            .checked_add(collar.retained_bytes)
            .ok_or("All-fork collar storage overflow")?;
        if work > limits.work || retained > limits.bytes {
            return Err("Complete all-fork aggregate budget exhausted".into());
        }
        if collar.ports.len() != ports.len()
            || collar.render_topology_vertices.len() != collar.mesh.positions.len()
            || collar.vertex_lineage.len() != collar.mesh.positions.len()
        {
            return Err("Collar source/topology inventory mismatch".into());
        }
        let offset = u32::try_from(draft.mesh.positions.len())
            .map_err(|_| "Collar vertex offset overflow")?;
        draft.bounded(
            draft
                .mesh
                .positions
                .len()
                .checked_add(collar.mesh.positions.len())
                .ok_or("Collar vertex overflow")?,
            draft
                .mesh
                .indices
                .len()
                .checked_add(draft.changed.len())
                .and_then(|i| i.checked_add(collar.mesh.indices.len()))
                .ok_or("Collar index overflow")?,
        )?;
        let mut topology_map = vec![None; collar.topology_positions.len()];
        for (port, mapping) in ports.iter().zip(&collar.ports) {
            let input = &pieces[port.piece];
            if mapping.cut_topology.len() != port.cut_vertices.len()
                || mapping.guard_topology.len() != port.guard_vertices.len()
            {
                return Err("Collar boundary correspondence mismatch".into());
            }
            for (&local, &topology) in port.cut_vertices.iter().zip(&mapping.cut_topology) {
                let vertex = input
                    .vertex_sources
                    .get(local as usize)
                    .ok_or("Cut source missing")?;
                let sample = cuts::Sample {
                    position: input.mesh.positions[local as usize],
                    normal: input.mesh.normals[local as usize],
                    color: input.mesh.colors[local as usize],
                    uv: input.mesh.uvs[local as usize],
                    source_vertices: vertex
                        .original_wood_vertices
                        .ok_or("Construction point escaped outer cut cap")?,
                    barycentric: vertex.barycentric,
                };
                let retained_vertex = draft.sample(&sample)?;
                let slot = topology_map
                    .get_mut(topology as usize)
                    .ok_or("Collar cut topology out of range")?;
                let alias = draft.aliases[retained_vertex as usize];
                if slot.is_some_and(|old| old != alias) {
                    return Err("Collar aliases distinct source cut vertices".into());
                }
                let expected = sample.position.map(f64::from);
                if collar.topology_positions[topology as usize] != expected {
                    return Err("Collar moved an exact source cut ring".into());
                }
                *slot = Some(alias);
            }
        }
        // No sample call may occur after this offset: all cut samples must
        // already have been created by the retained exact source strips.
        if draft.mesh.positions.len() != offset as usize {
            return Err("A collar port has no matching retained source cut".into());
        }
        for slot in &mut topology_map {
            if slot.is_none() {
                *slot = Some(draft.next_alias);
                draft.next_alias = draft
                    .next_alias
                    .checked_add(1)
                    .ok_or("Collar topology overflow")?;
            }
        }
        let changed_start = draft.changed.len();
        for &index in &collar.mesh.indices {
            if index as usize >= collar.mesh.positions.len() {
                return Err("Collar index out of range".into());
            }
            draft.changed.push(
                offset
                    .checked_add(index)
                    .ok_or("Collar index offset overflow")?,
            );
        }
        for (local, &topology) in collar.render_topology_vertices.iter().enumerate() {
            if local % 1024 == 0 && cancelled() {
                return Err("All-fork output adoption cancelled".into());
            }
            let alias = topology_map
                .get(topology as usize)
                .and_then(|v| *v)
                .ok_or("Missing collar geometric alias")?;
            draft.aliases.push(alias);
            draft
                .lineage
                .push(json!({"topology_vertex":alias,"kind":"core_smooth_collar",
                "collar":collar_records.len(),"local_vertex":local}));
        }
        draft.mesh.positions.extend(collar.mesh.positions);
        draft.mesh.normals.extend(collar.mesh.normals);
        draft.mesh.colors.extend(collar.mesh.colors);
        draft.mesh.uvs.extend(collar.mesh.uvs);
        collar_records.push(
            json!({"joint_keys":ids,"changed_relative_indices":[changed_start,draft.changed.len()],
            "wood_vertices":[offset,draft.mesh.positions.len()],"receipt":collar.receipt,
            "vertex_lineage":collar.vertex_lineage,"face_bindings":collar.face_bindings,
            "ports":collar.ports,"work":collar.work_used}),
        );
    }
    let changed_start = draft.mesh.indices.len();
    draft.mesh.indices.extend_from_slice(&draft.changed);
    let structural = draft.mesh.indices[..retained_structural_end]
        .iter()
        .copied()
        .chain(draft.changed.iter().copied())
        .collect::<Vec<_>>();
    let qualification = topology::qualify(
        &draft.mesh,
        &draft.aliases,
        &structural,
        limits
            .bytes
            .checked_sub(retained)
            .ok_or("All-fork topology bytes exhausted")?,
        limits
            .work
            .checked_sub(work)
            .ok_or("All-fork topology work exhausted")?,
        cancelled,
    )?;
    work = work
        .checked_add(qualification.1)
        .ok_or("All-fork qualification work overflow")?;
    if cancelled() {
        return Err("All-fork final publication cancelled; original retained".into());
    }
    let readback = json!({"wood_indices":[changed_start,draft.mesh.indices.len()],
        "wood_vertices":[original_vertices,draft.mesh.positions.len()],
        "min_area_sq_m4":spec.collar.minimum_triangle_area_sq_m4,
        "render_vertex_lineage":draft.lineage});
    let mut new_metadata = metadata.clone();
    let bindings = new_metadata["bindings"]
        .as_array_mut()
        .ok_or("Missing authored source bindings")?;
    for (index, binding) in bindings.iter_mut().take(spans.len()).enumerate() {
        binding["pre_all_fork_wood_indices"] = binding["wood_indices"].take();
        binding["pre_all_fork_wood_vertices"] = binding["wood_vertices"].take();
        binding["all_fork_index_binding"] = chain_ranges[index].clone();
        binding["fork_surface"] = json!("complete source-enumerated collar adoption");
    }
    // Spray face offsets moved after retired chain faces; source vertex and
    // foliage ranges are unchanged and exact original identities are retained.
    let spray_offset = retained_structural_end as i64 - chain_end as i64;
    for binding in bindings.iter_mut().skip(spans.len()) {
        let old = binding["wood_indices"]
            .as_array()
            .ok_or("Missing retained spray indices")?;
        let a = old[0].as_u64().ok_or("Invalid retained spray index")? as i64 + spray_offset;
        let b = old[1].as_u64().ok_or("Invalid retained spray index")? as i64 + spray_offset;
        if a < 0 || b < a {
            return Err("Retained spray index mapping overflow".into());
        }
        binding["wood_indices"] = json!([a, b]);
    }
    new_metadata["candidate"] = json!("white-gum-all-forks-v1");
    new_metadata["junctions"] =
        json!("every required authored source junction adopted; no skipped fork or hard fallback");
    new_metadata["all_forks"] = json!({"schema":spec.schema,"plan":plan,"clusters":clusters,
        "collars":collar_records,"readback":readback,"structural_topology":qualification.0,
        "structural_work":work,"logical_reserved_bytes":retained,"measured_peak_memory":null,
        "normals_uv_scope":"Fixed source cut/guard charts; Core-smoothed interior with explicit attribute lineage; actual canonical .10d readback required before promotion",
        "retained_source_vertices":draft.original_vertices,"retained_sprays":source.data.sprays.len(),
        "retained_leaves":source.total_leaves,"source_dispositions":"indices refer to retained authored_source.botanical.organs; every source organ and edge enumerated",
        "limitations":["Topology does not establish absence of geometric self-intersection","Root anatomy is authored hypothetical","Spray junctions remain separately retained source geometry","Human botanical and visual acceptance pending"]});
    // These are the only authoritative writes in this operation. A failed
    // required region, projection, budget or cancellation leaves both intact.
    *wood = draft.mesh;
    *metadata = new_metadata;
    Ok(())
}
