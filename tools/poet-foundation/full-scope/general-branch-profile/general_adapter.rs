//! Staged diagnostic adapter for an explicitly adopted general botanical tree.
//! Builds existing QualiaDB tapered segments with source-aligned endpoint radii.
//! Global field combination may merge unrelated edges: this is NOT an admission
//! certificate or permission to publish a regenerated asset.
use crate::graphical::authoring::botanical::{BotanicalLimits, BotanicalRecipe};
use qualia_core_db::specialized_libs::computational_geometry as cg;

#[derive(Clone, Copy, Debug)]
pub struct SourceEdge {
    /// Local source organ indices; these are not extractor sample-edge IDs.
    pub parent: usize,
    pub child: usize,
    pub field: cg::tapered_branch_field::TaperedFieldSegment,
}

impl Default for SourceEdge {
    fn default() -> Self {
        Self { parent: 0, child: 0, field: cg::tapered_branch_field::TaperedFieldSegment {
            a: cg::Point3::default(), b: cg::Point3::default(), radius_a: 0.0, radius_b: 0.0,
        }}
    }
}
/// Writes source-order parent indices to scratch; all input identities are
/// checked by the existing recipe owner first. Output edges remain untouched
/// on validation failure. `max_pair_checks` bounds the quadratic adjacency and
/// same-direction screen, including worst-case parent lookup.
/// This screen does not prove unrelated edges separated or prevent re-overlap.
pub fn adopted_general_edges_into(
    graph: &BotanicalRecipe, radii_m: &[[f64; 2]], limits: BotanicalLimits,
    min_axis_m: f64, max_same_direction_cosine: f64, max_pair_checks: usize,
    parent_scratch: &mut [usize], out: &mut [SourceEdge],
) -> Result<usize, String> {
    graph.validate(limits)?;
    let n = graph.organs.len();
    let edges = n.checked_sub(1).ok_or("Empty graph")?;
    let worst = n.checked_mul(n).and_then(|n| n.checked_mul(3)).ok_or("Adjacency work overflow")?;
    if worst > max_pair_checks { return Err("Adjacency work exceeds host limit".into()); }
    if parent_scratch.len() < n || out.len() < edges || radii_m.len() != edges {
        return Err("General graph buffer length mismatch".into());
    }
    if !min_axis_m.is_finite() || min_axis_m <= 0.0
        || !max_same_direction_cosine.is_finite() || !(0.0..1.0).contains(&max_same_direction_cosine)
        || radii_m.iter().flatten().any(|r| !r.is_finite() || *r <= 0.0 || *r > 1e6) {
        return Err("General graph parameters outside bounds".into());
    }
    parent_scratch[0] = usize::MAX;
    for i in 1..n {
        let key = graph.organs[i].parent.as_deref().ok_or("Missing parent")?;
        let parent = graph.organs[..i].iter().position(|o| o.key == key).ok_or("Parent is not adopted")?;
        let a = graph.organs[parent].position_m; let b = graph.organs[i].position_m;
        let delta = [b[0]-a[0], b[1]-a[1], b[2]-a[2]];
        let length = delta[0].hypot(delta[1]).hypot(delta[2]);
        if !length.is_finite() || length < min_axis_m || length < f64::MIN_POSITIVE {
            return Err("Degenerate branch axis".into());
        }
        parent_scratch[i] = parent;
    }
    for child_a in 1..n { for child_b in (child_a+1)..n {
        let pa = parent_scratch[child_a]; let pb = parent_scratch[child_b];
        let shared = if pa == pb { Some(pa) } else if pa == child_b { Some(pa) }
            else if pb == child_a { Some(pb) } else { None };
        if let Some(center) = shared {
            let other_a = if child_a == center { pa } else { child_a };
            let other_b = if child_b == center { pb } else { child_b };
            let base = graph.organs[center].position_m;
            let outward = |i: usize| {
                let p = graph.organs[i].position_m;
                let d = [p[0]-base[0],p[1]-base[1],p[2]-base[2]];
                let len = d[0].hypot(d[1]).hypot(d[2]);
                [d[0]/len,d[1]/len,d[2]/len]
            };
            let a=outward(other_a); let b=outward(other_b);
            let cosine = a[0]*b[0] + a[1]*b[1] + a[2]*b[2];
            if !cosine.is_finite() || cosine >= max_same_direction_cosine {
                return Err("Adjacent branches have ambiguous same-direction axes".into());
            }
        }
    }}
    for child in 1..n {
        let parent = parent_scratch[child];
        let a=graph.organs[parent].position_m; let b=graph.organs[child].position_m;
        out[child-1] = SourceEdge {parent, child, field: cg::tapered_branch_field::TaperedFieldSegment {
            a:cg::Point3::new(a[0],a[1],a[2]), b:cg::Point3::new(b[0],b[1],b[2]),
            radius_a:radii_m[child-1][0], radius_b:radii_m[child-1][1],
        }};
    }
    Ok(edges)
}
