//! Explicit serial loft reconstruction over the existing anatomy/CAD owners.
//! This joins consecutive limb envelopes, not branching torso attachments.
mod build;
mod qualification;
#[cfg(test)]
mod tests;
use super::*;
pub(super) use build::append;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LoftChainMember {
    pub loft_id: String,
    pub first_section: usize,
    pub last_section: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LoftChain {
    pub id: String,
    pub members: Vec<LoftChainMember>,
    pub radial_segments: usize,
    pub subdivisions: usize,
}

// These are cold logical reservations, not measured allocator/RSS or time.
// The parser admits <=16 chains; aggregate geometry remains under the parent's
// 250k pre-coat reservation. Topology's actual probe budget is shared across all
// chains/signs instead of resetting it for each output.
const MAX_COLD_BYTES: usize = 64 * 1024 * 1024;
const MAX_TOPOLOGY_WORK: usize = 100_000_000;

pub(super) fn contains(s:&Source,id:&str)->bool {
    s.loft_chains.iter().any(|c|c.members.iter().any(|m|m.loft_id==id))
}

fn member<'a>(s:&'a Source,m:&LoftChainMember)->Result<&'a Loft,String> {
    let mut found=s.lofts.iter().filter(|l|l.id==m.loft_id);
    let l=found.next().ok_or_else(||format!("Missing chain loft {}",m.loft_id))?;
    if found.next().is_some() {return Err(format!("Ambiguous chain loft {}",m.loft_id));}
    Ok(l)
}

/// Additional conservative vertex reservation; legacy member vertices are
/// intentionally still reserved by the parent although they are not emitted.
pub(super) fn validate(s:&Source)->Result<usize,String> {
    if s.loft_chains.len()>16 {return Err("At most 16 anatomical loft chains".into());}
    let mut used=std::collections::BTreeSet::new();
    let mut ids=std::collections::BTreeSet::new();
    let mut vertices=0usize;
    for c in &s.loft_chains {
        if c.id.is_empty() || c.id.len()>128 || !ids.insert(&c.id)
            || s.lofts.iter().any(|l|l.id==c.id)
            || !(2..=8).contains(&c.members.len())
            || !(8..=64).contains(&c.radial_segments)
            || !(1..=16).contains(&c.subdivisions) {
            return Err(format!("Invalid loft chain {}",c.id));
        }
        let first=member(s,&c.members[0])?;
        let mirrored=first.mirror_joints.is_some();
        let mut count=0usize;
        let mut previous:Option<[f32;3]>=None;
        for (mi,m) in c.members.iter().enumerate() {
            let l=member(s,m)?;
            if m.loft_id.len()>128 || !used.insert(&m.loft_id)
                || l.mirror_joints.is_some()!=mirrored
                || m.first_section>=m.last_section || m.last_section>=l.sections.len()
                || (mi==0 && m.first_section!=0)
                || (mi+1==c.members.len() && m.last_section+1!=l.sections.len()) {
                return Err(format!("Invalid/overlapping chain member {}",m.loft_id));
            }
            for section in &l.sections[m.first_section..=m.last_section] {
                if let Some(p)=previous {
                    let delta=sub(section.center_m,p);
                    if dot(delta,delta)<1e-10 {return Err("Coincident chain centres; no implicit weld".into());}
                }
                previous=Some(section.center_m);
            }
            count=count.checked_add(m.last_section-m.first_section+1).ok_or("Chain section overflow")?;
        }
        if count>64 {return Err("Chain exceeds 64 retained sections".into());}
        let rows=(count-1).checked_mul(c.subdivisions).and_then(|n|n.checked_add(1)).ok_or("Chain row overflow")?;
        let n=rows.checked_mul(c.radial_segments+1).and_then(|n|n.checked_add(2)).ok_or("Chain vertex overflow")?;
        qualification::reserve(rows,c.radial_segments)?;
        vertices=vertices.checked_add(n.checked_mul(if mirrored{2}else{1}).ok_or("Chain mirror overflow")?)
            .ok_or("Chain reservation overflow")?;
    }
    Ok(vertices)
}

#[derive(Clone, Copy)]
struct Origin { member:usize, section:usize }

fn station(l:&Loft,index:usize,mirrored:bool)->SkinStation {
    let authored=if mirrored {l.mirror_skin_stations.as_ref()}else{l.skin_stations.as_ref()};
    if let Some(profile)=authored {return profile[index].clone();}
    let t=index as f32/(l.sections.len()-1) as f32;
    SkinStation {joints:if mirrored{l.mirror_joints.unwrap()}else{l.joints},weights:[1.0-t,t]}
}

fn combined(s:&Source,c:&LoftChain)->Result<(Loft,Vec<Origin>),String> {
    let first=member(s,&c.members[0])?;
    let mut loft=first.clone();
    loft.id=c.id.clone();loft.sections.clear();
    loft.radial_segments=c.radial_segments;loft.subdivisions=c.subdivisions;
    let mut primary=Vec::new();let mut mirror=Vec::new();let mut origins=Vec::new();
    for (mi,m) in c.members.iter().enumerate() {
        let l=member(s,m)?;
        for index in m.first_section..=m.last_section {
            loft.sections.push(l.sections[index].clone());
            primary.push(station(l,index,false));
            if loft.mirror_joints.is_some(){mirror.push(station(l,index,true));}
            origins.push(Origin{member:mi,section:index});
        }
    }
    loft.skin_stations=Some(primary);
    loft.mirror_skin_stations=loft.mirror_joints.map(|_|mirror);
    Ok((loft,origins))
}
