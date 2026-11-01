//! Explicit sampled torso ports joined to existing limb-chain rings.
//! Core owns curve evaluation, strip topology and topology qualification.
//! This is a discrete authored graft, not a Boolean or analytic C1 assertion.
use super::*;
use std::collections::{BTreeMap, BTreeSet};
mod ports;
mod build;
mod attributes;
mod qualification;
mod projection;
pub use projection::{JunctionProjection,JunctionProjectionReport,prepare_junction_projection,qualify_junction_projection};
#[cfg(test)] mod tests;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JunctionSet {
    pub cold_bytes: usize,
    pub work_units: usize,
    pub recipes: Vec<JunctionRecipe>,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JunctionRecipe {
    pub id: String,
    pub torso_loft: String,
    pub limb_chain: String,
    pub mirrored: bool,
    /// Inclusive sampled row endpoints, excluding terminal torso rows.
    pub body_rows: [usize; 2],
    /// Cyclic sampled columns; the opening may cross the authored U seam.
    pub body_column_start: usize,
    pub body_column_count: usize,
    pub limb_cut_row: usize,
    pub rotation: usize,
    pub subdivisions: usize,
    pub body_tangent_scale: f64,
    pub limb_tangent_scale: f64,
    /// Optional degree-five interior displacement control in source-rest model
    /// XYZ metres before scene placement; mirrors are authored explicitly. Actual displacement is
    /// 10*t²*(1-t)² times this vector (maximum factor 0.625), not its full length.
    /// None retains the original four-control cubic evaluation path.
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub interior_bulge_m: Option<[f64;3]>,
    pub material: usize,
    pub max_influence_loss: f64,
    pub min_boundary_normal_dot: f64,
}
#[derive(Serialize)]
pub struct CollarVertexLineage {
    pub output_vertex: u32,
    pub body_vertex: u32,
    pub body_interior_vertex: u32,
    pub limb_vertex: u32,
    pub limb_next_vertex: u32,
    pub parameter: f32,
}
#[derive(Serialize)]
pub struct JunctionRecord {
    pub recipe: JunctionRecipe,
    pub removed_source_faces: Vec<u32>,
    pub vertices: Vec<CollarVertexLineage>,
    pub topology_aliases: Vec<[u32; 2]>,
    pub boundary_min_normal_dot: f64,
    pub maximum_discarded_weight: f64,
}
#[derive(Serialize)]
pub struct JunctionReceipt {
    pub schema: &'static str,
    pub scope: &'static str,
    pub uv_policy: &'static str,
    pub normal_policy: &'static str,
    pub source_aliases: Vec<[u32; 2]>,
    pub records: Vec<JunctionRecord>,
    pub work_used: usize,
    pub reserved_bytes: usize,
    pub component_faces: usize,
}

pub(super) struct Budget { remaining: usize, initial: usize }
impl Budget {
    fn charge(&mut self, n: usize) -> Result<(), String> {
        self.remaining=self.remaining.checked_sub(n).ok_or("Junction structural work exhausted")?;
        Ok(())
    }
}
fn sum(a:usize,b:usize)->Result<usize,String>{a.checked_add(b).ok_or("Junction count overflow".into())}
fn product(a:usize,b:usize)->Result<usize,String>{a.checked_mul(b).ok_or("Junction count overflow".into())}

/// Source-only preflight before parent geometry allocation. Returns additional
/// render vertices; the parent retains its existing aggregate anatomy cap.
pub(super) fn validate(s:&Source)->Result<usize,String> {
    let Some(set)=s.surface_junctions.as_ref() else{return Ok(0)};
    if set.recipes.is_empty() || set.recipes.len()>8 || set.cold_bytes>1024*1024*1024
        || set.cold_bytes<8*1024*1024 || set.work_units==0 || set.work_units>2_000_000_000 {
        return Err("Junction set exceeds explicit memory/work/recipe profile".into());
    }
    let mut ids=BTreeSet::new();let mut limbs=BTreeSet::new();let mut added=0;
    for r in &set.recipes {
        let (body,chain,rows)=ports::source_layout(s,r)?;
        let span=r.body_rows[1].checked_sub(r.body_rows[0]).ok_or("Reversed torso port rows")?;
        if r.id.is_empty() || r.id.len()>128 || r.torso_loft.len()>128 || r.limb_chain.len()>128 || !ids.insert(&r.id)
            || !limbs.insert((&r.limb_chain,r.mirrored)) || r.torso_loft!=set.recipes[0].torso_loft
            || r.body_rows[0]==0 || sum(r.body_rows[1],1)? >= rows || span<2
            || r.body_column_start>=body.radial_segments || r.body_column_count<2
            || sum(r.body_column_count,2)? >= body.radial_segments
            || product(2,sum(span,r.body_column_count)?)?!=chain.radial_segments
            || r.rotation>=chain.radial_segments || !(2..=32).contains(&r.subdivisions)
            || r.limb_cut_row==0 || sum(r.limb_cut_row,1)? >= ports::chain_rows(s,chain)?
            || r.material>=s.palettes.len()
            || !r.body_tangent_scale.is_finite() || !(0.05..=4.0).contains(&r.body_tangent_scale)
            || !r.limb_tangent_scale.is_finite() || !(0.05..=4.0).contains(&r.limb_tangent_scale)
            || r.interior_bulge_m.is_some_and(|v|v.iter().any(|x|!x.is_finite()||x.abs()>0.2)
                || attributes::dot64(v,v)>0.2f64*0.2 || attributes::dot64(v,v)<1e-6f64*1e-6)
            || !r.max_influence_loss.is_finite() || !(0.0..=0.1).contains(&r.max_influence_loss)
            || !r.min_boundary_normal_dot.is_finite() || !(0.0..=1.0).contains(&r.min_boundary_normal_dot) {
            return Err(format!("Invalid explicit junction recipe {}",r.id));
        }
        added=sum(added,product(r.subdivisions+1,chain.radial_segments+1)?)?;
    }
    Ok(added)
}

/// All mutations occur on a detached candidate. Any refusal leaves the input
/// streams and Part ownership unchanged. Existing bounded generation surrounds
/// this cold pass; no independent timeline, source or geometry store is created.
pub(super) fn apply(s:&Source,mesh:&mut AnatomyMeshAccumulator,parts:&mut Vec<Part>)
    ->Result<Option<JunctionReceipt>,String> {
    let added=validate(s)?;
    let Some(set)=s.surface_junctions.as_ref() else{return Ok(None)};
    if mesh.positions.len()>750_000 || mesh.indices.len()>9_000_000 || mesh.indices.len()%3!=0 || parts.len()>4096 {
        return Err("Junction input exceeds bounded anatomy counts".into());
    }
    let vertices=sum(mesh.positions.len(),added)?;
    let extra_indices=product(added,6)?;
    let indices=sum(mesh.indices.len(),extra_indices)?;
    // Includes source+detached streams, bounded Vec growth, index/face ownership,
    // topology/hash scratch, pair boxes, typed lineage and later JSON projection.
    // Allocator metadata/RSS are not asserted by this logical reservation.
    let mut bytes=sum(sum(product(vertices,512)?,product(indices,96)?)?,
        sum(product(added,2048)?,8*1024*1024)?)?;
    let topology=qualia_core_db::specialized_libs::computational_geometry::surface_topology_qualification::surface_qualification_storage_bounds(
        u32::try_from(vertices).map_err(|_|"Junction topology vertex overflow")?,indices/3)
        .map_err(|e|format!("Junction topology reservation: {e:?}"))?;
    use qualia_core_db::specialized_libs::computational_geometry::{HalfEdge,EdgeSlot};
    bytes=sum(bytes,product(topology.half_edges,std::mem::size_of::<HalfEdge>())?)?;
    bytes=sum(bytes,product(topology.edge_slots,std::mem::size_of::<EdgeSlot>())?)?;
    bytes=sum(bytes,sum(product(vertices,4)?,sum(topology.visited_half_edges,product(indices/3,8)?)?)?)?;
    for part in parts.iter() {bytes=sum(bytes,product(part.source_id.len(),8)?)?;}
    if bytes>set.cold_bytes {return Err(format!("Junction cold reservation {bytes} exceeds {}",set.cold_bytes))}
    let mut budget=Budget{remaining:set.work_units,initial:set.work_units};
    budget.charge(sum(product(vertices,32)?,product(indices,8)?)?)?;
    budget.charge(product(product(parts.len(),set.recipes.len())?,2048)?)?;
    for part in parts.iter() {budget.charge(product(part.source_id.len(),4)?)?;}
    if mesh.indices.iter().any(|&v|v as usize>=mesh.positions.len()) {return Err("Junction source index escapes streams".into())}
    let plans=ports::plans(s,mesh,parts,&mut budget)?;
    let mut candidate=attributes::copy(mesh,added,extra_indices)?;
    let (mut next_parts, mut receipt)=build::apply(s,&mut candidate,parts,&plans,bytes,&mut budget)?;
    qualification::check(&mut candidate,&next_parts,&plans,&mut receipt,&mut budget)?;
    receipt.work_used=budget.initial-budget.remaining;
    // No fallible operation follows promotion.
    std::mem::swap(mesh,&mut candidate);
    std::mem::swap(parts,&mut next_parts);
    Ok(Some(receipt))
}
