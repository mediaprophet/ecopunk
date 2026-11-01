use super::*;

#[derive(Clone,Copy)]
pub(super) struct Layout {pub base:usize,pub rows:usize,pub radial:usize,pub face:usize}
impl Layout {
    pub fn vertex(self,row:usize,col:usize)->u32{(self.base+row*(self.radial+1)+col%self.radial)as u32}
    pub fn end(self)->usize{self.base+self.rows*(self.radial+1)+2}
}
pub(super) struct Plan {
    pub recipe:usize,pub body:Layout,pub limb:Layout,
    pub body_loop:Vec<u32>,pub body_inside:Vec<u32>,
    pub limb_loop:Vec<u32>,pub limb_next:Vec<u32>,
    pub removed:Vec<u32>,
}
pub(super) fn chain_rows(s:&Source,c:&LoftChain)->Result<usize,String>{
    if !(2..=8).contains(&c.members.len()){return Err("Junction chain member count".into())}
    let mut n=0;
    for m in &c.members {
        let l=s.lofts.iter().find(|l|l.id==m.loft_id).ok_or("Junction chain member missing")?;
        if m.first_section>=m.last_section || m.last_section>=l.sections.len(){return Err("Junction chain member range".into())}
        n=sum(n,m.last_section-m.first_section+1)?;
    }
    if !(2..=64).contains(&n){return Err("Junction chain section count".into())}
    sum(product(n-1,c.subdivisions)?,1)
}
pub(super) fn source_layout<'a>(s:&'a Source,r:&JunctionRecipe)->Result<(&'a Loft,&'a LoftChain,usize),String>{
    if s.lofts.iter().filter(|l|l.id==r.torso_loft).count()!=1 || s.loft_chains.iter().filter(|c|c.id==r.limb_chain).count()!=1 {
        return Err("Junction source identifiers missing or ambiguous".into());
    }
    let body=s.lofts.iter().find(|l|l.id==r.torso_loft).ok_or("Junction torso loft missing")?;
    let chain=s.loft_chains.iter().find(|c|c.id==r.limb_chain).ok_or("Junction limb chain missing")?;
    if body.mirror_joints.is_some() || s.loft_chains.iter().any(|c|c.members.iter().any(|m|m.loft_id==body.id))
        || !(2..=64).contains(&body.sections.len()) || !(8..=64).contains(&body.radial_segments) || !(1..=16).contains(&body.subdivisions)
        || !(8..=64).contains(&chain.radial_segments) || !(1..=16).contains(&chain.subdivisions) {
        return Err("Junction requires one unchained axial torso and bounded limb chain".into());
    }
    let first=chain.members.first().and_then(|m|s.lofts.iter().find(|l|l.id==m.loft_id)).ok_or("Junction chain empty")?;
    if r.mirrored && first.mirror_joints.is_none(){return Err("Junction mirror is not authored".into())}
    Ok((body,chain,sum(product(body.sections.len()-1,body.subdivisions)?,1)?))
}
fn layout(parts:&[Part],prefix:&str,exact:bool,rows:usize,radial:usize)->Result<Layout,String>{
    let found:Vec<_>=parts.iter().filter(|p|if exact{p.source_id==prefix}else{p.source_id.starts_with(prefix)}).collect();
    let first=found.first().ok_or_else(||format!("Junction source surface missing {prefix}"))?;
    let base=first.vertices.start;
    let end=sum(sum(base,product(rows,radial+1)?)?,2)?;
    let start=found.iter().map(|p|p.indices.start).min().unwrap();
    let stop=found.iter().map(|p|p.indices.end).max().unwrap();
    if found.iter().any(|p|p.vertices.start!=base||p.vertices.end!=end||p.indices.start%3!=0||p.indices.end%3!=0)
        || start%3!=0 || stop-start!=product(product(rows,radial)?,6)? {
        return Err(format!("Junction source layout mismatch {prefix}"));
    }
    Ok(Layout{base,rows,radial,face:start/3})
}
pub(super) fn aliases(plans:&[Plan],n:usize)->Vec<u32>{
    let mut out:Vec<u32>=(0..n as u32).collect();
    for p in plans {for l in [p.body,p.limb] {for row in 0..l.rows {
        out[l.base+row*(l.radial+1)+l.radial]=l.vertex(row,0);
    }}}
    out
}
pub(super) fn plans(s:&Source,mesh:&AnatomyMeshAccumulator,parts:&[Part],budget:&mut Budget)->Result<Vec<Plan>,String>{
    let set=s.surface_junctions.as_ref().unwrap();
    budget.charge(product(parts.len(),product(set.recipes.len(),32)?)?)?;
    let mut plans=Vec::with_capacity(set.recipes.len());
    let mut guard_cells=BTreeSet::new();
    for (recipe,r) in set.recipes.iter().enumerate(){
        let (source_body,chain,rows)=source_layout(s,r)?;
        let body=layout(parts,&format!("{}:1",r.torso_loft),true,rows,source_body.radial_segments)?;
        let limb=layout(parts,&format!("chain:{}:{}:",r.limb_chain,if r.mirrored{-1}else{1}),false,chain_rows(s,chain)?,chain.radial_segments)?;
        if body.end()>mesh.positions.len()||limb.end()>mesh.positions.len(){return Err("Junction layout escapes source streams".into())}
        // Guard-cell overlap would make prescribed source supports ambiguous.
        for row in r.body_rows[0]-1..=r.body_rows[1] {for c in 0..r.body_column_count+2 {
            budget.charge(32)?;
            let col=(r.body_column_start+body.radial+c-1)%body.radial;
            if !guard_cells.insert((row,col)){return Err("Junction torso ports/guard bands overlap".into())}
        }}
        let mut coordinates=Vec::with_capacity(limb.radial);
        let [lo,hi]=r.body_rows;let w=r.body_column_count;
        for c in 0..=w {coordinates.push((lo,c));}
        for row in lo+1..=hi {coordinates.push((row,w));}
        for c in (0..w).rev(){coordinates.push((hi,c));}
        for row in (lo+1..hi).rev(){coordinates.push((row,0));}
        if coordinates.len()!=limb.radial{return Err("Junction perimeter does not match limb ring".into())}
        let mut body_loop=Vec::with_capacity(limb.radial);let mut inside=Vec::with_capacity(limb.radial);
        for (row,c) in coordinates {
            let inner_row=if row==lo{row+1}else if row==hi{row-1}else{row};
            let inner_c=if c==0{1}else if c==w{w-1}else{c};
            body_loop.push(body.vertex(row,r.body_column_start+c));
            inside.push(body.vertex(inner_row,r.body_column_start+inner_c));
        }
        let mut removed=Vec::new();
        for row in lo..hi {for c in 0..w {
            let face=body.face+(row*body.radial+(r.body_column_start+c)%body.radial)*2;
            removed.extend([face as u32,(face+1)as u32]);
        }}
        for face in limb.face..limb.face+r.limb_cut_row*limb.radial*2 {removed.push(face as u32)}
        let cap=limb.face+(limb.rows-1)*limb.radial*2;
        for face in cap..cap+limb.radial {removed.push(face as u32)}
        plans.push(Plan{recipe,body,limb,body_loop,body_inside:inside,
            limb_loop:(0..limb.radial).map(|c|limb.vertex(r.limb_cut_row,c)).collect(),
            limb_next:(0..limb.radial).map(|c|limb.vertex(r.limb_cut_row+1,c)).collect(),removed});
    }
    let alias=aliases(&plans,mesh.positions.len());
    let removed:BTreeSet<u32>=plans.iter().flat_map(|p|p.removed.iter().copied()).collect();
    let mut edges=BTreeMap::new();
    for p in &plans {for ring in [&p.body_loop,&p.limb_loop] {for i in 0..ring.len(){
        let (a,b)=(ring[i],ring[(i+1)%ring.len()]);edges.insert((a.min(b),a.max(b)),(0usize,0i32));
    }}}
    budget.charge(product(mesh.indices.len(),32)?)?;
    for (face,t) in mesh.indices.chunks_exact(3).enumerate(){if !removed.contains(&(face as u32)){
        for e in 0..3 {let a=*alias.get(t[e]as usize).ok_or("Junction source index out of range")?;
            let b=*alias.get(t[(e+1)%3]as usize).ok_or("Junction source index out of range")?;
            if let Some(v)=edges.get_mut(&(a.min(b),a.max(b))){v.0+=1;v.1+=if a<b{1}else{-1};}
        }
    }}
    for p in &mut plans {
        orient(&mut p.body_loop,&mut p.body_inside,&edges,-1)?;
        orient(&mut p.limb_loop,&mut p.limb_next,&edges,1)?;
        p.limb_loop.rotate_left(set.recipes[p.recipe].rotation);
        p.limb_next.rotate_left(set.recipes[p.recipe].rotation);
    }
    Ok(plans)
}
fn orient(ring:&mut Vec<u32>,support:&mut Vec<u32>,edges:&BTreeMap<(u32,u32),(usize,i32)>,wanted:i32)->Result<(),String>{
    let mut direction=None;
    for i in 0..ring.len(){let(a,b)=(ring[i],ring[(i+1)%ring.len()]);
        let &(count,sign)=edges.get(&(a.min(b),a.max(b))).ok_or("Junction boundary edge missing")?;
        if count!=1{return Err("Junction cut is not a single retained boundary".into())}
        let d=sign*if a<b{1}else{-1};
        if direction.is_some_and(|v|v!=d){return Err("Junction boundary winding is inconsistent".into())}
        direction=Some(d);
    }
    if direction!=Some(wanted){ring.reverse();support.reverse();}
    Ok(())
}
