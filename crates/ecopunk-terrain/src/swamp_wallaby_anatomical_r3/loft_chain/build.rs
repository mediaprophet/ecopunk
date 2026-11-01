use super::*;

fn color(a:[f32;4],b:[f32;4],t:f32)->[f32;4] {
    std::array::from_fn(|i|a[i]+(b[i]-a[i])*t)
}

/// Existing loft wall order [row,col; row+1,col; row+1,col+1]
/// faces inward in its right-handed Bishop (u,v,tangent) frame. Reverse ALL
/// wall faces once. Its start/end cap fans already face -/+tangent. Inferring
/// each triangle's winding from radial shading normals breaks shared edges
/// at tight bends/tapers; those vectors are not a topological orientation.
/// Consistent winding does not prove the swept surface is locally unfolded
/// or free of self-intersections; retain the subsequent geometry/topology gate.
pub(super) fn finish_chain_surface(a:&mut AnatomyMeshAccumulator,base:usize,start:usize,rows:usize,radial:usize)->Result<(),String> {
    let wall=(rows-1).checked_mul(radial).and_then(|n|n.checked_mul(6)).ok_or("Chain wall layout overflow")?;
    let total=wall.checked_add(radial.checked_mul(6).ok_or("Chain cap layout overflow")?).ok_or("Chain layout overflow")?;
    if a.indices.len().checked_sub(start)!=Some(total)
        || a.positions.len().checked_sub(base)!=rows.checked_mul(radial+1).and_then(|n|n.checked_add(2)) {
        return Err("Chain does not match the existing loft owner layout".into());
    }
    for tri in a.indices[start..start+wall].chunks_exact_mut(3){tri.swap(1,2);}
    recompute_surface_normals(a,base,start);
    Ok(())
}

pub(in super::super) fn append(s:&Source,skin:&mut AnatomyMeshAccumulator,parts:&mut Vec<Part>,coat:&mut Vec<Range<usize>>)->Result<(),String> {
    let mut work=MAX_TOPOLOGY_WORK;
    for c in &s.loft_chains {
        let (loft,origins)=combined(s,c)?;
        for (sign,joints) in [(1.0,Some(loft.joints)),(-1.0,loft.mirror_joints)] {
            let Some(joints)=joints else{continue};
            let mut rings=sampled_sections(&loft,palette(s,loft.material)?,sign,joints)?;
            let stride=c.radial_segments+1;
            for (row,ring) in rings.iter_mut().enumerate() {
                let interval=(row/c.subdivisions).min(origins.len()-2);
                let t=if row==(origins.len()-1)*c.subdivisions{1.0}else{(row%c.subdivisions)as f32/c.subdivisions as f32};
                let p=palette(s,member(s,&c.members[origins[interval].member])?.material)?;
                let q=palette(s,member(s,&c.members[origins[interval+1].member])?.material)?;
                let u=t*t*(3.0-2.0*t);
                ring.dorsal_color=color(p.dorsal,q.dorsal,u);
                ring.flank_color=color(p.flank,q.flank,u);
                ring.ventral_color=color(p.ventral,q.ventral,u);
            }
            let v=skin.positions.len();let i=skin.indices.len();
            skin.push_countershaded_loft(&rings,c.radial_segments,Layer::ExternalIntegument,true);
            apply_bishop_sections(skin,&rings,c.radial_segments,v)?;
            skin_profile::apply(skin,&loft,sign<0.0,v)?;
            // Exact aliases of one geometric seam. Preserve the two U chart values.
            for row in 0..rings.len(){let a=v+row*stride;skin.positions[a+c.radial_segments]=skin.positions[a];}
            let caps=v+rings.len()*stride;
            skin.normals[caps]=norm(sub(rings[0].center,rings[1].center));
            skin.normals[caps+1]=norm(sub(rings[rings.len()-1].center,rings[rings.len()-2].center));
            finish_chain_surface(skin,v,i,rings.len(),c.radial_segments)?;
            for row in 0..rings.len(){
                let a=v+row*stride;let b=a+c.radial_segments;
                let n=norm(add(skin.normals[a],skin.normals[b]));skin.normals[a]=n;skin.normals[b]=n;
                let interval=(row/c.subdivisions).min(origins.len()-2);
                let t=if row+1==rings.len(){1.0}else{(row%c.subdivisions)as f32/c.subdivisions as f32};
                let origin=origins[interval+usize::from(t>=0.5)];
                if member(s,&c.members[origin.member])?.coat {
                    if let Some(last)=coat.last_mut().filter(|last|last.start>=v && last.end==a) {last.end=a+stride;}
                    else {coat.push(a..a+stride);}
                }
            }
            qualification::check(skin,v,i,rings.len(),c.radial_segments,&mut work)?;
            // Each strip has one explicit source interval. Split only at a
            // palette boundary midpoint; normals/weights/positions/UV stay shared.
            for interval in 0..origins.len()-1 {
                let a=origins[interval];let b=origins[interval+1];
                let l=member(s,&c.members[a.member])?;let r=member(s,&c.members[b.member])?;
                let middle=(c.subdivisions+1)/2;
                let ranges=if l.material==r.material {vec![(0,c.subdivisions,l.material)]}
                    else {vec![(0,middle,l.material),(middle,c.subdivisions,r.material)]};
                for (start,end,material) in ranges {
                    if start==end{continue;}
                    parts.push(Part{source_id:format!("chain:{}:{}:{}[{}]->{}[{}]:rows{}-{}",c.id,sign,l.id,a.section,r.id,b.section,start,end),
                        material,vertices:v..skin.positions.len(),
                        indices:i+(interval*c.subdivisions+start)*c.radial_segments*6..i+(interval*c.subdivisions+end)*c.radial_segments*6});
                }
            }
            let wall=(rings.len()-1)*c.radial_segments*6;
            for (end,origin) in [(0,origins[0]),(1,origins[origins.len()-1])] {
                let l=member(s,&c.members[origin.member])?;
                parts.push(Part{source_id:format!("chain:{}:{}:{}[{}]:external-cap",c.id,sign,l.id,origin.section),material:l.material,
                    vertices:v..skin.positions.len(),indices:i+wall+end*c.radial_segments*3..i+wall+(end+1)*c.radial_segments*3});
            }
        }
    }
    Ok(())
}
