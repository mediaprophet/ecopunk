//! Optional authored section influences, projected into the existing Core skin ABI.
//! No joint inference, replacement deformation engine or geometry changes.
use super::{AnatomyMeshAccumulator, Loft};
use qualia_core_db::render::skeletal::SkinWeight;
use serde::{Deserialize, Serialize};

/// Two authored influences per section. Adjacent sections need at most four
/// canonical slots. Values are controls, not measured anatomical attachments.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkinStation {
    pub joints: [usize; 2],
    pub weights: [f32; 2],
}

pub(super) fn validate(loft: &Loft, joints: usize) -> Result<(), String> {
    if loft.skin_stations.is_none() {
        return if loft.mirror_skin_stations.is_none() { Ok(()) }
            else { Err(format!("Mirror skin stations require primary stations: {}",loft.id)) };
    }
    if loft.mirror_joints.is_some() != loft.mirror_skin_stations.is_some() {
        return Err(format!("Explicit skin stations need matching mirror ownership: {}",loft.id));
    }
    for profile in [&loft.skin_stations,&loft.mirror_skin_stations].into_iter().flatten() {
        if profile.len()!=loft.sections.len() || profile.len()>64 {
            return Err(format!("Skin station count must match authored sections: {}",loft.id));
        }
        for station in profile {
            // Same finite, nonnegative, unit-sum contract as Core's canonical
            // skin owner. Final SKL1 encode/readback and player admission remain
            // the authoritative validation of the material-remapped output.
            if station.joints.iter().any(|&j| j>=joints || j>u16::MAX as usize)
                || station.weights.iter().any(|&w| !w.is_finite() || !(0.0..=1.0).contains(&w))
                || (station.weights.iter().map(|&w|f64::from(w)).sum::<f64>()-1.0).abs()>1e-5 {
                return Err(format!("Invalid authored skin influence: {}",loft.id));
            }
        }
    }
    Ok(())
}

/// Stable joint ordering and duplicate coalescing in fixed scratch. Interpolation
/// follows the same authored station parameter as the actual sampled centreline,
/// not uniform whole-loft distance or inferred closest anatomical segment.
fn blend(a:&SkinStation,b:&SkinStation,t:f64)->SkinWeight {
    let mut ids=[0u16;4];
    let mut values=[0f64;4];
    let mut count=0usize;
    for (station,factor) in [(a,1.0-t),(b,t)] {
        for slot in 0..2 {
            let value=f64::from(station.weights[slot])*factor;
            if value==0.0 {continue;}
            let id=station.joints[slot] as u16;
            if let Some(at)=ids[..count].iter().position(|&j|j==id) {
                values[at]+=value;
            } else {
                let mut at=count;
                while at>0 && ids[at-1]>id {
                    ids[at]=ids[at-1]; values[at]=values[at-1]; at-=1;
                }
                ids[at]=id; values[at]=value; count+=1;
            }
        }
    }
    let sum=values.iter().sum::<f64>();
    SkinWeight {joints:ids,weights:values.map(|v|(v/sum) as f32)}
}

/// Barycentric/edge interpolation in fixed scratch using the existing Core
/// four-slot ABI. Joint identifiers are supplied by source vertices, never
/// inferred from distance. Any discarded influence is measured and bounded.
pub(super) fn blend_weights<const N: usize>(input: &[SkinWeight; N], factors: &[f64; N],
    max_loss: f64) -> Result<(SkinWeight, f64), String> {
    if N == 0 || N > 3 || !max_loss.is_finite() || !(0.0..=0.1).contains(&max_loss)
        || factors.iter().any(|x| !x.is_finite() || *x < 0.0)
        || (factors.iter().sum::<f64>()-1.0).abs()>1e-8 {
        return Err("Invalid bounded skin interpolation".into());
    }
    let mut slots=[(0u16,0f64);12]; let mut count=0;
    for (weight,factor) in input.iter().zip(factors) {
        if weight.weights.iter().any(|w| !w.is_finite() || *w<0.0)
            || (weight.weights.iter().map(|&w|f64::from(w)).sum::<f64>()-1.0).abs()>1e-5 {
            return Err("Invalid source skin weight".into());
        }
        for (&joint,&w) in weight.joints.iter().zip(&weight.weights) {
            let value=f64::from(w)*factor;
            if value==0.0 {continue;}
            if let Some(at)=slots[..count].iter().position(|s|s.0==joint) {slots[at].1+=value;}
            else {slots[count]=(joint,value);count+=1;}
        }
    }
    slots[..count].sort_unstable_by(|a,b|b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    let total=slots[..count].iter().map(|x|x.1).sum::<f64>();
    let retained=count.min(4);
    let kept=slots[..retained].iter().map(|x|x.1).sum::<f64>();
    if total<=0.0 || kept<=0.0 {return Err("Empty skin interpolation".into());}
    // Sum the discarded tail directly: total-kept can cancel a real tiny
    // influence. A zero-loss source policy admits no positive discarded slot.
    let dropped=slots[retained..count].iter().map(|x|x.1).sum::<f64>();
    let lost=dropped/total;
    if (max_loss==0.0 && dropped>0.0) || lost>max_loss {
        return Err(format!("Skin interpolation loses {lost} influence; limit {max_loss}"));
    }
    slots[..retained].sort_unstable_by_key(|s|s.0);
    let mut output=SkinWeight {joints:[0;4],weights:[0.0;4]};
    for (i,&(joint,value)) in slots[..retained].iter().enumerate() {
        output.joints[i]=joint; output.weights[i]=(value/kept) as f32;
    }
    Ok((output,lost))
}

pub(super) fn apply(mesh:&mut AnatomyMeshAccumulator,loft:&Loft,mirrored:bool,base:usize)
    ->Result<(),String> {
    let profile=if mirrored {loft.mirror_skin_stations.as_ref()} else {loft.skin_stations.as_ref()};
    let Some(profile)=profile else{return Ok(());};
    let rows=loft.sections.len().checked_sub(1).and_then(|n|n.checked_mul(loft.subdivisions))
        .and_then(|n|n.checked_add(1)).ok_or("Skin station layout overflow")?;
    let stride=loft.radial_segments.checked_add(1).ok_or("Skin station stride overflow")?;
    let caps=rows.checked_mul(stride).and_then(|n|base.checked_add(n))
        .ok_or("Skin station range overflow")?;
    let end=caps.checked_add(2).ok_or("Skin station cap overflow")?;
    if profile.len()!=loft.sections.len() || profile.len()<2 || loft.subdivisions==0
        || end>mesh.joint_indices.len() || end>mesh.joint_weights.len() {
        return Err("Skin station output does not match existing loft owner layout".into());
    }
    // The source parser bounds each loft to64 sections,16 subdivisions,64 radial
    // segments. This pass writes only existing streams, with fixed four-slot
    // scratch; no additional vertex/index buffer or per-row allocation.
    for row in 0..rows {
        let station=(row/loft.subdivisions).min(profile.len()-2);
        let t=if row+1==rows {1.0} else {(row%loft.subdivisions) as f64/loft.subdivisions as f64};
        let weight=blend(&profile[station],&profile[station+1],t);
        let first=base+row*stride;
        mesh.joint_indices[first..first+stride].fill(weight.joints);
        mesh.joint_weights[first..first+stride].fill(weight.weights);
    }
    for (cap,station) in [(caps,&profile[0]),(caps+1,&profile[profile.len()-1])] {
        let weight=blend(station,station,0.0);
        mesh.joint_indices[cap]=weight.joints;
        mesh.joint_weights[cap]=weight.weights;
    }
    Ok(())
}

#[cfg(test)]
#[path="skin_profile_tests.rs"]
mod tests;
