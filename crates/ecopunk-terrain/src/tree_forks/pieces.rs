//! Translation of retained sweep triangles into the existing collar engine seam.
use super::cuts::{self, Sample};
use crate::scene_builder_10d::SubmeshBuilder;
use crate::smooth_collars::{CollarPiece, CollarPort, VertexSource};
use std::collections::BTreeMap;
use std::ops::Range;

pub(in crate::white_gum_authored_review) struct ChainSpan {
    pub vertices: Range<usize>,
    pub indices: Range<usize>,
    pub effective_radii_m: Vec<f64>,
}

struct PieceBuilder<'a> {
    source: &'a SubmeshBuilder,
    span: &'a ChainSpan,
    stations: usize,
    sides: usize,
    mesh: SubmeshBuilder,
    sources: Vec<VertexSource>,
    aliases: Vec<u32>,
    render: BTreeMap<([u32; 3], [u64; 3]), u32>,
    topology: BTreeMap<([u32; 3], [u64; 3]), u32>,
    topology_count: u32,
    fade_coordinates: Vec<f64>,
    fade: Option<(usize, bool, f64, f64)>,
    exact_guard: std::collections::BTreeSet<([u32; 3], [u64; 3])>,
}
impl<'a> PieceBuilder<'a> {
    fn new(source: &'a SubmeshBuilder, span: &'a ChainSpan, stations: usize, sides: usize) -> Self {
        Self {
            source,
            span,
            stations,
            sides,
            mesh: SubmeshBuilder::new(),
            sources: Vec::new(),
            aliases: Vec::new(),
            render: BTreeMap::new(),
            topology: BTreeMap::new(),
            topology_count: 0,
            fade_coordinates: Vec::new(),
            fade: None,
            exact_guard: std::collections::BTreeSet::new(),
        }
    }
    fn canonical(&self, vertex: u32) -> Result<u32, String> {
        let v = vertex as usize;
        if !self.span.vertices.contains(&v) {
            return Err("Stub source escapes chain vertices".into());
        }
        let local = v - self.span.vertices.start;
        let rings = self
            .stations
            .checked_mul(self.sides)
            .ok_or("Stub ring size overflow")?;
        let mapped = if (rings + 2..rings + 2 + self.sides).contains(&local) {
            local - rings - 2
        } else if (rings + 2 + self.sides..rings + 2 + 2 * self.sides).contains(&local) {
            (self.stations - 1) * self.sides + local - rings - 2 - self.sides
        } else {
            local
        };
        u32::try_from(self.span.vertices.start + mapped)
            .map_err(|_| "Stub topology exceeds u32".into())
    }
    fn add(&mut self, sample: &Sample) -> Result<u32, String> {
        let key = sample.identity();
        if let Some(&v) = self.render.get(&key) {
            return Ok(v);
        }
        let mut topology_key = key;
        for v in &mut topology_key.0 {
            *v = self.canonical(*v)?;
        }
        let alias = if let Some(&v) = self.topology.get(&topology_key) {
            v
        } else {
            let v = self.topology_count;
            self.topology_count = self
                .topology_count
                .checked_add(1)
                .ok_or("Stub topology count overflow")?;
            self.topology.insert(topology_key, v);
            v
        };
        let v =
            u32::try_from(self.mesh.positions.len()).map_err(|_| "Stub render count overflow")?;
        self.mesh.positions.push(sample.position);
        self.mesh.normals.push(sample.normal);
        self.mesh.colors.push(sample.color);
        self.mesh.uvs.push(sample.uv);
        self.sources.push(VertexSource {
            original_wood_vertices: Some(sample.source_vertices),
            barycentric: sample.barycentric,
        });
        self.aliases.push(alias);
        let fade = if self.exact_guard.contains(&key) {
            0.0
        } else if let Some((edge, at_start, guard, transition)) = self.fade {
            let mut station = 0.0;
            for (&id, &weight) in sample.source_vertices.iter().zip(&sample.barycentric) {
                let canonical = self.canonical(id)? as usize - self.span.vertices.start;
                let rings = self.stations * self.sides;
                let index = if canonical == rings {
                    0
                } else if canonical == rings + 1 {
                    self.stations - 1
                } else {
                    canonical / self.sides
                };
                station += index as f64 * weight;
            }
            let q = station - edge as f64;
            let s = if at_start { q } else { 1.0 - q };
            (guard - s) / transition
        } else {
            1.0
        };
        if !fade.is_finite() {
            return Err("Nonfinite source strip fade coordinate".into());
        }
        self.fade_coordinates.push(fade);
        self.render.insert(key, v);
        Ok(v)
    }
    fn face(&mut self, samples: [&Sample; 3]) -> Result<(), String> {
        let a = self.add(samples[0])?;
        let b = self.add(samples[1])?;
        let c = self.add(samples[2])?;
        self.mesh.indices.extend_from_slice(&[a, b, c]);
        Ok(())
    }
    fn original_cap(&mut self, at_start: bool) -> Result<(), String> {
        let rings = self.stations * self.sides;
        let center = self.span.vertices.start + rings + usize::from(!at_start);
        let rim = self.span.vertices.start + rings + 2 + if at_start { 0 } else { self.sides };
        let c = Sample::original(
            self.source,
            u32::try_from(center).map_err(|_| "Cap center overflow")?,
        )?;
        for side in 0..self.sides {
            let a = Sample::original(self.source, (rim + side) as u32)?;
            let b = Sample::original(self.source, (rim + (side + 1) % self.sides) as u32)?;
            self.face(if at_start { [&c, &b, &a] } else { [&c, &a, &b] })?;
        }
        Ok(())
    }
    fn cut_cap(
        &mut self,
        ring: &[Sample],
        at_start: bool,
        outward: [f64; 3],
    ) -> Result<Vec<u32>, String> {
        let mut center = [0.0f64; 3];
        for sample in ring {
            for axis in 0..3 {
                center[axis] += f64::from(sample.position[axis]) / ring.len() as f64;
            }
        }
        let centroid =
            u32::try_from(self.mesh.positions.len()).map_err(|_| "Cut cap count overflow")?;
        self.mesh.positions.push(center.map(|v| v as f32));
        self.mesh.normals.push(outward.map(|v| v as f32));
        self.mesh.colors.push([0.0, 0.0, 0.0, 1.0]);
        self.mesh.uvs.push([0.0, 0.0]);
        // Construction-only point. These faces must all survive unchanged and
        // be removed by the engine before any delivered collar is admitted.
        self.sources.push(VertexSource {
            original_wood_vertices: None,
            barycentric: [0.0; 3],
        });
        self.fade_coordinates.push(-1.0);
        self.aliases.push(self.topology_count);
        self.topology_count = self
            .topology_count
            .checked_add(1)
            .ok_or("Cut cap topology overflow")?;
        let mut faces = Vec::with_capacity(ring.len());
        for side in 0..ring.len() {
            let a = self.add(&ring[side])?;
            let b = self.add(&ring[(side + 1) % ring.len()])?;
            faces.push(
                u32::try_from(self.mesh.indices.len() / 3).map_err(|_| "Cut cap face overflow")?,
            );
            let triangle = if at_start {
                [centroid, b, a]
            } else {
                [centroid, a, b]
            };
            self.mesh.indices.extend_from_slice(&triangle);
        }
        Ok(faces)
    }
}

/// Closed source stub. Both cut and guard rings refer to the exact source
/// triangles, with normals/UV chart seams carried by explicit topology aliases.
pub(super) fn stub(
    source: &SubmeshBuilder,
    span: &ChainSpan,
    organs: &[usize],
    sides: usize,
    at_start: bool,
    cut_fraction: f64,
    guard_fraction: f64,
    transition_fraction: f64,
    edge_key: String,
    joint_key: String,
    endpoint_keys: [String; 2],
    piece_index: usize,
    outward_axis: [f64; 3],
    source_edge_length: f64,
) -> Result<(CollarPiece, CollarPort), String> {
    if organs.len() < 2
        || span.effective_radii_m.len() != organs.len()
        || !cut_fraction.is_finite()
        || !(0.0..0.5).contains(&cut_fraction)
        || !guard_fraction.is_finite()
        || guard_fraction <= 0.0
        || guard_fraction >= 1.0
        || !transition_fraction.is_finite()
        || transition_fraction <= 0.0
        || transition_fraction > 1.0
    {
        return Err("Invalid exact source stub controls".into());
    }
    let station = if at_start { 0 } else { organs.len() - 2 };
    let cut_q = if at_start {
        cut_fraction
    } else {
        1.0 - cut_fraction
    };
    let guard_q = if at_start {
        cut_fraction * guard_fraction
    } else {
        1.0 - cut_fraction * guard_fraction
    };
    let cut = cuts::ring(source, span.vertices.start, station, sides, cut_q)?;
    let guard = cuts::ring(source, span.vertices.start, station, sides, guard_q)?;
    let endpoint = cuts::ring(
        source,
        span.vertices.start,
        station,
        sides,
        if at_start { 0.0 } else { 1.0 },
    )?;
    let mut builder = PieceBuilder::new(source, span, organs.len(), sides);
    let guard_s = cut_fraction * guard_fraction;
    builder.fade = Some((station, at_start, guard_s, guard_s * transition_fraction));
    builder
        .exact_guard
        .extend(guard.iter().map(Sample::identity));
    if at_start {
        cuts::strip(&endpoint, &guard, |t| builder.face(t))?;
        cuts::strip(&guard, &cut, |t| builder.face(t))?;
    } else {
        cuts::strip(&cut, &guard, |t| builder.face(t))?;
        cuts::strip(&guard, &endpoint, |t| builder.face(t))?;
    }
    builder.original_cap(at_start)?;
    // Stub outer cap is the source-order upper cap for a start stub and lower
    // cap for an end stub; its outward direction is always away from the joint.
    let cut_cap_faces = builder.cut_cap(&cut, !at_start, outward_axis)?;
    let cut_vertices = cut
        .iter()
        .map(|s| builder.add(s))
        .collect::<Result<Vec<_>, _>>()?;
    let guard_vertices = guard
        .iter()
        .map(|s| builder.add(s))
        .collect::<Result<Vec<_>, _>>()?;
    let mut guard_center = [0.0; 3];
    for sample in &guard {
        for axis in 0..3 {
            guard_center[axis] += f64::from(sample.position[axis]) / guard.len() as f64;
        }
    }
    let transition_length =
        source_edge_length * cut_fraction * guard_fraction * transition_fraction;
    if !transition_length.is_finite() || transition_length <= 0.0 {
        return Err("Invalid source stub transition length".into());
    }
    Ok((
        CollarPiece {
            mesh: builder.mesh,
            topology_aliases: builder.aliases,
            vertex_sources: builder.sources,
            fade_coordinates: builder.fade_coordinates,
            edge_child_key: edge_key.clone(),
            endpoint_keys,
        },
        CollarPort {
            piece: piece_index,
            edge_child_key: edge_key,
            joint_key,
            cut_fraction,
            cut_vertices,
            guard_vertices,
            cut_cap_faces,
            outward_axis,
            guard_center,
            transition_length,
        },
    ))
}

/// An explicitly coupled cluster owns this complete interior maximal chain.
/// Exact old render streams/indices are copied; geometric aliases come only
/// from the source hard-cap derivative layout.
pub(super) fn interior(
    source: &SubmeshBuilder,
    span: &ChainSpan,
    stations: usize,
    sides: usize,
    edge_key: String,
    endpoint_keys: [String; 2],
) -> Result<CollarPiece, String> {
    let mut builder = PieceBuilder::new(source, span, stations, sides);
    for &vertex in source
        .indices
        .get(span.indices.clone())
        .ok_or("Interior chain index range invalid")?
    {
        let sample = Sample::original(source, vertex)?;
        let index = builder.add(&sample)?;
        builder.mesh.indices.push(index);
    }
    Ok(CollarPiece {
        mesh: builder.mesh,
        topology_aliases: builder.aliases,
        fade_coordinates: builder.fade_coordinates,
        vertex_sources: builder.sources,
        edge_child_key: edge_key,
        endpoint_keys,
    })
}
