//! Authored, area-distributed curved fibres on the final native skin surface.
//! Core CAD evaluates the curves; Core's four-weight ABI remains the skin owner.
use super::*;
use qualia_core_db::{render::skeletal::SkinWeight,
    specialized_libs::computational_geometry::parametric_cad::bezier_eval_into};

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Groom {
    pub seed: u32,
    pub density_per_m2: f64,
    pub max_strands: usize,
    pub segments: usize,
    pub length_m: [f32; 2],
    pub width_m: [f32; 2],
    pub lift_m: f32,
    pub direction: [f32; 3],
    pub guard_fraction: f32,
    pub root_tone: f32,
    pub tip_tone: f32,
    pub max_influence_loss: f64,
    pub material: usize,
    pub surface_materials: Vec<usize>,
    pub regions: Vec<GroomRegion>,
    pub cold_bytes: usize,
    pub work_units: usize,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GroomRegion {
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub surface_materials: Vec<usize>,
    pub bounds_min_m: [f32; 3],
    pub bounds_max_m: [f32; 3],
    pub direction: [f32; 3],
    pub density_scale: f32,
    pub length_scale: f32,
    pub width_scale: f32,
}

#[derive(Clone, Serialize)]
pub struct GroomReceipt {
    pub method: &'static str,
    pub strands: usize,
    pub vertices: usize,
    pub triangles: usize,
    pub coated_area_m2: f64,
    pub charged_work_units: usize,
    pub cold_reservation_bytes: usize,
    pub maximum_discarded_influence: f64,
    pub region_strands: Vec<(String, usize)>,
}

struct Work { remaining: usize, initial: usize }
impl Work {
    fn spend(&mut self, n: usize) -> Result<(), String> {
        self.remaining=self.remaining.checked_sub(n).ok_or("Groom operation budget exhausted")?;
        Ok(())
    }
}
fn checked_product(a: usize,b: usize)->Result<usize,String>{a.checked_mul(b).ok_or_else(||"Groom size overflow".into())}
fn vertices_per_strand(g:&Groom)->usize{g.segments*2+1}
fn triangles_per_strand(g:&Groom)->usize{(g.segments-1)*2+1}
fn output_reservation(g:&Groom)->Result<usize,String>{
    checked_product(checked_product(g.max_strands,vertices_per_strand(g))?,192)?
        .checked_add(checked_product(checked_product(checked_product(g.max_strands,triangles_per_strand(g))?,3)?,8)?)
        .and_then(|n|n.checked_add(1024*1024)).ok_or("Groom reservation overflow".into())
}
fn input_reservation(a:&AnatomyMeshAccumulator,parts:&Vec<Part>)->Result<usize,String>{
    let mut bytes=0usize;
    macro_rules! stream {($name:ident)=>{{
        // Empty vectors may still have retained capacity.
        let size=element_size(&a.$name);
        bytes=bytes.checked_add(checked_product(a.$name.capacity(),size)?).ok_or("Groom input reservation overflow")?;
    }}}
    stream!(positions);stream!(normals);stream!(colors);stream!(uvs);
    stream!(layer_tags);stream!(joint_indices);stream!(joint_weights);stream!(indices);
    bytes=bytes.checked_add(part_slot_reservation(parts.capacity())?).ok_or("Groom Part reservation overflow")?;
    checked_product(bytes,2)
}
// This reservation is serialized and charged as logical copy work. Part contains
// String/usize fields: its native64 layout is 64 bytes, but wasm32 is 32 bytes.
// Reserve the conservative native64 slot on both targets so identical source
// does not acquire target-dependent provenance. This covers the Part vector
// slots, not String allocations or measured RSS; the existing fixed cold
// envelope remains in force. Refuse a future larger layout before mutation.
const LOGICAL_PART_SLOT_BYTES:usize=64;
fn part_slot_reservation(capacity:usize)->Result<usize,String>{
    if std::mem::size_of::<Part>()>LOGICAL_PART_SLOT_BYTES {
        return Err("Groom Part layout exceeds portable reservation profile".into());
    }
    checked_product(capacity,LOGICAL_PART_SLOT_BYTES)
}
fn element_size<T>(_:&Vec<T>)->usize{std::mem::size_of::<T>()}

pub(super) fn validate(s:&Source)->Result<usize,String>{
    let Some(g)=s.groom.as_ref() else{return Ok(0)};
    if s.coat.enabled {return Err("Select authored groom or legacy coat, never both".into());}
    if !g.density_per_m2.is_finite() || !(1.0..=300_000.0).contains(&g.density_per_m2)
        || !(1..=32_000).contains(&g.max_strands) || !(2..=5).contains(&g.segments)
        || !positive(&g.length_m) || g.length_m[0]>g.length_m[1] || g.length_m[1]>0.08
        || !positive(&g.width_m) || g.width_m[0]>g.width_m[1] || g.width_m[1]>0.004
        || !g.lift_m.is_finite() || !(0.0001..=0.04).contains(&g.lift_m)
        || !coordinate(&g.direction) || dot(g.direction,g.direction)<1e-8
        || !g.guard_fraction.is_finite() || !(0.0..=1.0).contains(&g.guard_fraction)
        || [g.root_tone,g.tip_tone].iter().any(|v|!v.is_finite()||!(0.1..=2.0).contains(v))
        || !g.max_influence_loss.is_finite() || !(0.0..=0.05).contains(&g.max_influence_loss)
        || g.surface_materials.is_empty() || g.surface_materials.len()>32 || g.regions.len()>16
        || g.cold_bytes>128*1024*1024 || g.work_units==0 || g.work_units>100_000_000 {
        return Err("Groom source outside admitted geometric/resource profile".into());
    }
    palette(s,g.material)?;
    let mut seen=std::collections::BTreeSet::new();
    for &m in &g.surface_materials {palette(s,m)?;if !seen.insert(m){return Err("Duplicate groom surface material".into());}}
    let mut ids=std::collections::BTreeSet::new();
    for r in &g.regions {
        if r.id.is_empty()||r.id.len()>128||!ids.insert(&r.id)||!coordinate(&r.bounds_min_m)||!coordinate(&r.bounds_max_m)
            || (0..3).any(|i|r.bounds_min_m[i]>=r.bounds_max_m[i])
            || !coordinate(&r.direction)||dot(r.direction,r.direction)<1e-8
            || !r.density_scale.is_finite()||!(0.0..=4.0).contains(&r.density_scale)
            || r.surface_materials.len()>32 || r.surface_materials.iter().any(|m|!g.surface_materials.contains(m))
            || [r.length_scale,r.width_scale].iter().any(|x|!x.is_finite()||!(0.1..=4.0).contains(x)) {
            return Err("Invalid explicit groom region".into());
        }
    }
    let vertices=checked_product(g.max_strands,vertices_per_strand(g))?;
    // Includes doubled output capacities, skin arrays and a fixed cold allowance;
    // named logical reservation, not an allocator/RSS measurement.
    let reservation=output_reservation(g)?;
    if reservation>g.cold_bytes{return Err(format!("Groom needs {reservation} cold bytes; source admits {}",g.cold_bytes));}
    Ok(vertices)
}

fn hash(mut x:u32)->u32{x^=x>>16;x=x.wrapping_mul(0x7feb352d);x^=x>>15;x=x.wrapping_mul(0x846ca68b);x^(x>>16)}
fn random(seed:u32,lane:u32)->f32{((hash(seed^lane.wrapping_mul(0x9e3779b9))>>8)as f32)*(1.0/16_777_216.0)}
fn selected_region(g:&Groom,p:[f32;3],material:usize)->Option<usize>{g.regions.iter().position(|r|
    (r.surface_materials.is_empty()||r.surface_materials.contains(&material))
    &&(0..3).all(|i|p[i]>=r.bounds_min_m[i]&&p[i]<=r.bounds_max_m[i]))}
fn bary<T:Copy>(values:[T;3],f:[f32;3],get:impl Fn(T)->[f32;3])->[f32;3]{
    let a=get(values[0]);let b=get(values[1]);let c=get(values[2]);
    std::array::from_fn(|i|a[i]*f[0]+b[i]*f[1]+c[i]*f[2])
}
fn maximum_density_scale(g:&Groom)->f64 {
    g.regions.iter().map(|r|f64::from(r.density_scale)).fold(1.0,f64::max)
}
/// Same proposals and point-region decision in admission and construction.
/// The upper density is bounded by the authored region profile (at most four).
fn proposal(g:&Groom,ordinal:u32,strand:usize)->(u32,[f32;3]){
    let seed=hash(hash(g.seed^ordinal)^(strand as u32).wrapping_mul(0xa511e9b3));
    let(mut u,mut v)=(random(seed,1),random(seed,2));
    if f64::from(u)+f64::from(v)>1.0 {u=1.0-u;v=1.0-v;}
    (seed,[(1.0-f64::from(u)-f64::from(v)).max(0.0)as f32,u,v])
}
fn accepted_region(g:&Groom,p:[f32;3],material:usize,seed:u32,maximum:f64)->Option<Option<usize>>{
    let region=selected_region(g,p,material);
    let density=region.map(|i|f64::from(g.regions[i].density_scale)).unwrap_or(1.0);
    (f64::from(random(seed,9))*maximum<density).then_some(region)
}
fn triangle_plan(g:&Groom,a:&AnatomyMeshAccumulator,ids:[usize;3],ordinal:u32,maximum:f64)
    ->Result<(usize,f64),String>{
    let p=ids.map(|i|a.positions[i]);
    let cross=cross(sub(p[1],p[0]),sub(p[2],p[0]));
    let area=0.5*((cross[0]as f64).powi(2)+(cross[1]as f64).powi(2)+(cross[2]as f64).powi(2)).sqrt();
    if !area.is_finite(){return Err("Nonfinite groom surface area".into());}
    let exact=area*g.density_per_m2*maximum;
    if !exact.is_finite()||exact>g.max_strands as f64*4.0{return Err("Groom triangle demand exceeds bounded proposal budget".into());}
    let whole=exact.floor()as usize;
    Ok((whole+usize::from(f64::from(random(g.seed^ordinal,0))<exact-whole as f64),area))
}

pub(super) fn append(s:&Source,a:&mut AnatomyMeshAccumulator,parts:&mut Vec<Part>)
    ->Result<Option<GroomReceipt>,String>{
    let Some(g)=s.groom.as_ref() else{return Ok(None)};
    let input_vertices=a.positions.len();
    if [a.normals.len(),a.colors.len(),a.uvs.len(),a.layer_tags.len(),a.joint_indices.len(),a.joint_weights.len()]
        .iter().any(|&n|n!=input_vertices) || a.indices.len()%3!=0
        || parts.iter().any(|p|p.indices.start%3!=0||p.indices.end%3!=0) {
        return Err("Groom source streams/part triangles do not correspond".into());
    }
    let reserved=validate(s)?;
    let mut work=Work{remaining:g.work_units,initial:g.work_units};
    let input_bytes=input_reservation(a,parts)?;
    let cold_bytes=output_reservation(g)?.checked_add(input_bytes).ok_or("Groom aggregate reservation overflow")?;
    if cold_bytes>g.cold_bytes{return Err(format!("Groom old/new buffers need {cold_bytes} cold bytes; source admits {}",g.cold_bytes));}
    // Covers old buffers retained alongside each replacement allocation and
    // their possible copy work. Logical envelope, not measured allocator RSS.
    work.spend(input_bytes)?;
    let maximum=maximum_density_scale(g);let proposal_limit=checked_product(g.max_strands,4)?;
    let base_parts=parts.len();let mut count=0usize;let mut area=0.0;let mut ordinal=0u32;let mut proposed=0usize;
    for part in &parts[..base_parts] {
        if !g.surface_materials.contains(&part.material){continue;}
        let indices=a.indices.get(part.indices.clone()).ok_or("Groom source part range invalid")?;
        for tri in indices.chunks_exact(3){
            work.spend(64+g.regions.len()*12)?;
            let ids=[tri[0]as usize,tri[1]as usize,tri[2]as usize];
            if ids.iter().any(|i|!part.vertices.contains(i)||*i>=a.positions.len()){return Err("Groom source triangle escapes part".into());}
            let (n,ar)=triangle_plan(g,a,ids,ordinal,maximum)?;
            proposed=proposed.checked_add(n).ok_or("Groom proposal overflow")?;
            if proposed>proposal_limit{return Err("Groom exceeds four-times strand proposal envelope".into());}
            for strand in 0..n {
                work.spend(128+g.regions.len()*48)?;
                let(seed,f)=proposal(g,ordinal,strand);let p=bary(ids,f,|i|a.positions[i]);
                if accepted_region(g,p,part.material,seed,maximum).is_some(){count+=1;}
                if count>g.max_strands{return Err(format!("Groom density requests {count}+ strands; explicit limit {}",g.max_strands));}
            }
            ordinal=ordinal.checked_add(1).ok_or("Groom triangle identity overflow")?;area+=ar;
        }
    }
    if count==0{return Err("Authored groom selected no surface fibres".into());}
    let added=checked_product(count,vertices_per_strand(g))?;
    if added>reserved || a.positions.len().checked_add(added).is_none_or(|n|n>750_000){return Err("Groom exceeds admitted skin capacity".into());}
    a.positions.try_reserve_exact(added).map_err(|_|"Groom positions allocation refused")?;
    a.normals.try_reserve_exact(added).map_err(|_|"Groom normals allocation refused")?;
    a.colors.try_reserve_exact(added).map_err(|_|"Groom colours allocation refused")?;
    a.uvs.try_reserve_exact(added).map_err(|_|"Groom UV allocation refused")?;
    a.layer_tags.try_reserve_exact(added).map_err(|_|"Groom tags allocation refused")?;
    a.joint_indices.try_reserve_exact(added).map_err(|_|"Groom weights allocation refused")?;
    a.joint_weights.try_reserve_exact(added).map_err(|_|"Groom weights allocation refused")?;
    a.indices.try_reserve_exact(checked_product(checked_product(count,triangles_per_strand(g))?,3)?)
        .map_err(|_|"Groom indices allocation refused")?;
    let begin_v=a.positions.len();let begin_i=a.indices.len();ordinal=0;
    let mut max_loss=0.0f64;let mut region_counts=vec![0usize;g.regions.len()+1];
    for part_index in 0..base_parts {
        if !g.surface_materials.contains(&parts[part_index].material){continue;}
        let range=parts[part_index].indices.clone();
        for at in (range.start..range.end).step_by(3) {
            work.spend(64+g.regions.len()*12)?;
            let ids=[a.indices[at]as usize,a.indices[at+1]as usize,a.indices[at+2]as usize];
            let (n,_)=triangle_plan(g,a,ids,ordinal,maximum)?;
            for strand in 0..n {
                work.spend(128+g.regions.len()*48)?;
                let(seed,f)=proposal(g,ordinal,strand);
                let p=bary(ids,f,|i|a.positions[i]);let normal=norm(bary(ids,f,|i|a.normals[i]));
                let Some(region)=accepted_region(g,p,parts[part_index].material,seed,maximum)else{continue};
                work.spend(512+g.segments*128)?;
                region_counts[region.map(|i|i+1).unwrap_or(0)]+=1;
                let r=region.map(|i|&g.regions[i]);
                let guide=r.map(|r|r.direction).unwrap_or(g.direction);
                let projected=sub(guide,mul(normal,dot(guide,normal)));
                let flow=if dot(projected,projected)>1e-10 {norm(projected)}else{
                    norm(cross(normal,if normal[1].abs()<0.8{[0.0,1.0,0.0]}else{[1.0,0.0,0.0]}))};
                let side=norm(cross(flow,normal));
                let flow=norm(add(flow,mul(side,(random(seed,3)-0.5)*0.45)));
                let guard=random(seed,4)<g.guard_fraction;
                let length=(g.length_m[0]+(g.length_m[1]-g.length_m[0])*random(seed,5))
                    *r.map(|r|r.length_scale).unwrap_or(1.0)*if guard{1.3}else{0.8};
                let width=(g.width_m[0]+(g.width_m[1]-g.width_m[0])*random(seed,6))
                    *r.map(|r|r.width_scale).unwrap_or(1.0);
                let root=add(p,mul(normal,0.00012));
                let lift=g.lift_m*(0.6+random(seed,7)*0.8)*r.map(|r|r.length_scale).unwrap_or(1.0);
                let cp=[root,add(add(root,mul(flow,length*0.22)),mul(normal,lift*0.7)),
                    add(add(root,mul(flow,length*0.68)),mul(normal,lift)),
                    add(add(root,mul(flow,length)),mul(normal,lift*0.45))].map(point);
                let source_weights=ids.map(|i|SkinWeight{joints:a.joint_indices[i],weights:a.joint_weights[i]});
                let factors=[f64::from(f[0]),f64::from(f[1]),f64::from(f[2])];
                let sum=factors.iter().sum::<f64>();
                let(weight,loss)=skin_profile::blend_weights(&source_weights,&factors.map(|x|x/sum),g.max_influence_loss)?;
                max_loss=max_loss.max(loss);
                let base_color=std::array::from_fn::<_,4,_>(|c|a.colors[ids[0]][c]*f[0]+a.colors[ids[1]][c]*f[1]+a.colors[ids[2]][c]*f[2]);
                let start=a.positions.len()as u32;let mut scratch=[Point3::default();4];
                for row in 0..=g.segments {
                    let t=row as f64/g.segments as f64;
                    let centre=bezier_eval_into(&cp,t,&mut scratch).map_err(|e|format!("Core groom curve refused: {e:?}"))?;
                    let centre=[centre.x as f32,centre.y as f32,centre.z as f32];
                    let tone=g.root_tone+(g.tip_tone-g.root_tone)*(t*t)as f32;
                    let variation=if guard{1.0+0.35*random(seed,8)}else{0.60+0.45*random(seed,8)};
                    let color=std::array::from_fn(|i|if i==3{1.0}else{(base_color[i]*tone*variation).clamp(0.0,1.0)});
                    let half=width*0.5*(1.0-t as f32);
                    let offsets=if row==g.segments{1}else{2};
                    for column in 0..offsets {
                        let pos=if offsets==1{centre}else{add(centre,mul(side,if column==0{-half}else{half}))};
                        push_vertex(a,pos,normal,color,[column as f32,t as f32],weight.joints[0]as usize);
                        let last=a.positions.len()-1;a.joint_indices[last]=weight.joints;a.joint_weights[last]=weight.weights;
                    }
                }
                for row in 0..g.segments-1 {let k=start+(row*2)as u32;a.indices.extend_from_slice(&[k,k+1,k+2,k+1,k+3,k+2]);}
                let k=start+((g.segments-1)*2)as u32;a.indices.extend_from_slice(&[k,k+1,k+2]);
            }
            ordinal=ordinal.checked_add(1).ok_or("Groom identity overflow")?;
        }
    }
    if a.positions.len()-begin_v!=added{return Err("Groom output differs from admitted strand count".into());}
    record(parts,a,format!("area-curved-groom:{}",g.seed),g.material,begin_v,begin_i);
    let region_strands=std::iter::once(("default".to_owned(),region_counts[0]))
        .chain(g.regions.iter().enumerate().map(|(i,r)|(r.id.clone(),region_counts[i+1]))).collect();
    Ok(Some(GroomReceipt{method:"area-point-region-rejection-native-bezier-fibres-v1",strands:count,
        vertices:added,triangles:(a.indices.len()-begin_i)/3,coated_area_m2:area,
        charged_work_units:work.initial-work.remaining,
        cold_reservation_bytes:cold_bytes,
        maximum_discarded_influence:max_loss,region_strands}))
}

#[cfg(test)]
#[path="groom_tests.rs"]
mod tests;
