//! Authored anatomical candidate through existing Ecopunk anatomy/Core CAD owners.
//! r3 source is editable JSON; no engine rebuild is needed when passing edited JSON.
//! Not a measured individual, complete anatomical package, accepted asset or qualified rig.
use crate::scene_builder_10d::SubmeshBuilder;
use crate::zoological_anatomy::{AnatomicalLoftRing, AnatomyMeshAccumulator, ZoologicalAnatomicalLayer as Layer};
use qualia_core_db::specialized_libs::computational_geometry::{parametric_cad::bezier_eval, Point3};
use qualia_core_db::specialized_libs::computational_geometry::{math_geometry::AffineFrame3, sweep_extrusion::compute_bishop_frames};
use serde::{Deserialize, Serialize};
use std::ops::Range;
pub use crate::zoological_anatomy::NumericProfile;
mod skin_profile;
pub use skin_profile::SkinStation;
mod loft_chain;
pub use loft_chain::{LoftChain, LoftChainMember};
mod groom;
pub use groom::{Groom, GroomRegion, GroomReceipt};
mod junctions;
pub use junctions::{JunctionSet, JunctionRecipe, JunctionReceipt};
pub use junctions::{prepare_junction_projection, qualify_junction_projection, JunctionProjection, JunctionProjectionReport};

pub const DEFAULT_SOURCE: &str = include_str!("swamp_wallaby_anatomical_r3.json");
pub const SOURCE_SCHEMA: &str = "ecopunk.anatomical-review-source/1";

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub schema: String,
    pub identifier: String,
    pub taxon: String,
    pub units: String,
    pub coordinate_frame: String,
    pub parameter_status: String,
    /// Omission keeps legacy platform math; portability is explicit source lineage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_profile: Option<NumericProfile>,
    pub references: Vec<String>,
    pub joints: Vec<JointLandmark>,
    pub palettes: Vec<Palette>,
    pub lofts: Vec<Loft>,
    pub ellipsoids: Vec<Ellipsoid>,
    pub ears: Vec<Ear>,
    pub bones: Vec<Bone>,
    pub muscles: Vec<Muscle>,
    pub coat: Coat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groom: Option<Groom>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface_junctions: Option<JunctionSet>,
    /// Explicit serial surface reconstruction; absent sources keep legacy geometry.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loft_chains: Vec<LoftChain>,
    /// Optional authored study motion; omitted for legacy static sources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motion: Option<crate::animal_motion::MotionSource>,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JointLandmark { pub id: String, pub parent: Option<usize>, pub position_m: [f32; 3] }
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Palette { pub id: String, pub dorsal: [f32; 4], pub flank: [f32; 4], pub ventral: [f32; 4], pub roughness: f32 }
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Section { pub center_m: [f32; 3], pub radii_m: [f32; 2] }
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Loft {
    pub id: String, pub material: usize, pub sections: Vec<Section>,
    pub joints: [usize; 2], pub mirror_joints: Option<[usize; 2]>,
    pub radial_segments: usize, pub subdivisions: usize, pub coat: bool,
    /// Optional explicit section influences. Omission retains legacy endpoint blending.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skin_stations: Option<Vec<SkinStation>>,
    /// Explicit counterpart for mirrored geometry; joint identifiers are not inferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_skin_stations: Option<Vec<SkinStation>>,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ellipsoid { pub id: String, pub material: usize, pub center_m: [f32; 3], pub radii_m: [f32; 3], pub joint: usize, pub mirror: bool }
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ear { pub id: String, pub root_m: [f32; 3], pub tip_m: [f32; 3], pub width_m: f32, pub cup_depth_m: f32, pub thickness_m: f32, pub material: usize, pub inner_material: usize, pub joint: usize, pub mirror: bool }
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bone { pub id: String, pub joints: [usize; 2], pub shaft_radius_m: f32, pub condyle_radius_m: f32 }
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Muscle { pub id: String, pub joints: [usize; 2], pub radii_m: [f32; 2], pub bow_m: [f32; 3] }
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Coat { pub enabled: bool, pub vertex_stride: usize, pub length_m: f32, pub width_m: f32, pub lift_m: f32, pub direction: [f32; 3], pub material: usize }

/// Exact spans preserve material identity and source-to-vertex lineage for admission.
pub struct Part { pub source_id: String, pub material: usize, pub vertices: Range<usize>, pub indices: Range<usize> }
pub struct Candidate {
    pub source: Source,
    pub skin: AnatomyMeshAccumulator,
    pub osteology: AnatomyMeshAccumulator,
    pub myology: AnatomyMeshAccumulator,
    pub parts: Vec<Part>,
    pub groom: Option<GroomReceipt>,
    pub surface_junctions: Option<JunctionReceipt>,
    pub bounds_min_m: [f32; 3],
    pub bounds_max_m: [f32; 3],
}

fn norm(a: [f32; 3]) -> [f32; 3] { let d = dot(a,a).sqrt().max(1e-10); mul(a,1.0/d) }
fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[0]+b[0],a[1]+b[1],a[2]+b[2]] }
fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[0]-b[0],a[1]-b[1],a[2]-b[2]] }
fn mul(a: [f32; 3], k: f32) -> [f32; 3] { [a[0]*k,a[1]*k,a[2]*k] }
fn dot(a: [f32; 3],b: [f32; 3])->f32 { a[0]*b[0]+a[1]*b[1]+a[2]*b[2] }
fn cross(a:[f32;3],b:[f32;3])->[f32;3] { [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]] }
fn point(v:[f32;3])->Point3 { Point3::new(v[0] as f64,v[1] as f64,v[2] as f64) }
fn bezier(c:[[f32;3];4],t:f32)->Result<[f32;3],String> {
    let p = bezier_eval(&c.map(point),t as f64).map_err(|e|format!("CAD curve rejected: {e:?}"))?;
    Ok([p.x as f32,p.y as f32,p.z as f32])
}
fn palette(source:&Source,i:usize)->Result<&Palette,String> { source.palettes.get(i).ok_or_else(||format!("Unknown palette {i}")) }
fn finite(v:&[f32])->bool { v.iter().all(|x|x.is_finite()) }
fn coordinate(v:&[f32])->bool { finite(v)&&v.iter().all(|x|x.abs()<=3.0) }
fn positive(v:&[f32])->bool { finite(v)&&v.iter().all(|x|*x>0.0&&*x<=3.0) }

/// Bounded source admission. Values are authored metres, never inferred measurements.
pub fn parse_source(json:&str)->Result<Source,String> {
    if json.len()>2_000_000 { return Err("Source exceeds 2 MB review budget".into()); }
    let s:Source=serde_json::from_str(json).map_err(|e|format!("Anatomy source: {e}"))?;
    if s.schema!=SOURCE_SCHEMA||s.units!="m"||s.coordinate_frame!="+Y up; +Z front; ground y=0" { return Err("Unsupported anatomy schema/units/frame".into()); }
    if s.joints.is_empty()||s.joints.len()>128||s.palettes.is_empty()||s.palettes.len()>32||s.lofts.len()>256||s.ellipsoids.len()>128||s.ears.len()>16||s.bones.len()>256||s.muscles.len()>256 { return Err("Anatomy source count outside review budget".into()); }
    if s.identifier.is_empty()||s.taxon!="Wallabia bicolor"||s.joints[0].parent.is_some() {return Err("Invalid subject/root identity".into());}
    let mut ids=std::collections::BTreeSet::new();
    for (i,j) in s.joints.iter().enumerate() {
        if j.id.is_empty()||!ids.insert(&j.id)||!coordinate(&j.position_m)||j.parent.map(|p|p>=i).unwrap_or(i!=0) { return Err(format!("Invalid joint {}",j.id)); }
    }
    for p in &s.palettes { if [&p.dorsal,&p.flank,&p.ventral].iter().any(|c|c.iter().any(|x|!x.is_finite()||!(0.0..=1.0).contains(x)))||!p.roughness.is_finite()||!(0.0..=1.0).contains(&p.roughness) { return Err(format!("Invalid palette {}",p.id)); } }
    let valid_joints=|j:[usize;2]|j.iter().all(|i|*i<s.joints.len());
    let mut estimate=0usize;
    for l in &s.lofts {
        palette(&s,l.material)?;
        if l.sections.len()<2||l.sections.len()>64||!(8..=64).contains(&l.radial_segments)||!(1..=16).contains(&l.subdivisions)||!valid_joints(l.joints)||l.mirror_joints.map(|j|!valid_joints(j)).unwrap_or(false) { return Err(format!("Invalid loft {}",l.id)); }
        skin_profile::validate(l,s.joints.len())?;
        for st in &l.sections { if !coordinate(&st.center_m)||!positive(&st.radii_m) {return Err(format!("Invalid section {}",l.id));} }
        for pair in l.sections.windows(2) { if dot(sub(pair[1].center_m,pair[0].center_m),sub(pair[1].center_m,pair[0].center_m))<1e-10 {return Err(format!("Coincident loft centres {}",l.id));} }
        estimate+=(l.sections.len()*l.subdivisions+1)*(l.radial_segments+1)*(if l.mirror_joints.is_some(){2}else{1});
    }
    for e in &s.ellipsoids {palette(&s,e.material)?;if !coordinate(&e.center_m)||!positive(&e.radii_m)||e.joint>=s.joints.len(){return Err(format!("Invalid ellipsoid {}",e.id));}}
    for e in &s.ears {palette(&s,e.material)?;palette(&s,e.inner_material)?;if !coordinate(&e.root_m)||!coordinate(&e.tip_m)||!positive(&[e.width_m,e.cup_depth_m,e.thickness_m])||e.joint>=s.joints.len()||dot(sub(e.tip_m,e.root_m),sub(e.tip_m,e.root_m))<1e-8{return Err(format!("Invalid ear {}",e.id));}}
    let nonzero_span=|j:[usize;2]|{if !valid_joints(j){return false;}let d=sub(s.joints[j[0]].position_m,s.joints[j[1]].position_m);dot(d,d)>1e-10};
    for b in &s.bones {if !nonzero_span(b.joints)||!positive(&[b.shaft_radius_m,b.condyle_radius_m]){return Err(format!("Invalid bone {}",b.id));}}
    for m in &s.muscles {if !nonzero_span(m.joints)||!positive(&m.radii_m)||!coordinate(&m.bow_m){return Err(format!("Invalid muscle {}",m.id));}}
    palette(&s,s.coat.material)?;
    if s.coat.vertex_stride<2||!positive(&[s.coat.length_m,s.coat.width_m,s.coat.lift_m])||!coordinate(&s.coat.direction)||dot(s.coat.direction,s.coat.direction)<1e-10||estimate>250_000 {return Err("Coat/mesh outside review budget".into());}
    estimate=estimate.checked_add(loft_chain::validate(&s)?).ok_or("Loft chain estimate overflow")?;
    estimate=estimate.checked_add(junctions::validate(&s)?).ok_or("Junction estimate overflow")?;
    if estimate>250_000 {return Err("Combined loft/chain vertex reservation exceeded".into());}
    let detail=s.ellipsoids.len()*2*25*17+s.ears.len()*2*2*25*21;
    let coat_estimate=if s.coat.enabled{(estimate/s.coat.vertex_stride+s.lofts.len()*2)*5}else{0};
    let groom_estimate=groom::validate(&s)?;
    if estimate+detail+coat_estimate+groom_estimate>750_000{return Err("Preflight skin/coat/detail/groom vertex budget exceeded".into());}
    let mut part_ids=std::collections::BTreeSet::new();
    for id in s.lofts.iter().map(|x|&x.id).chain(s.ellipsoids.iter().map(|x|&x.id)).chain(s.ears.iter().map(|x|&x.id)){if id.is_empty()||!part_ids.insert(id){return Err("Duplicate/empty geometry source identifier".into());}}
    Ok(s)
}

fn sampled_sections(l:&Loft,p:&Palette,sign:f32,joints:[usize;2])->Result<Vec<AnatomicalLoftRing>,String> {
    let n=l.sections.len(); let mut result=Vec::new();
    for i in 0..n-1 {
        let a=l.sections[i].center_m; let b=l.sections[i+1].center_m;
        let prev=l.sections[i.saturating_sub(1)].center_m; let next=l.sections[(i+2).min(n-1)].center_m;
        // Cubic interpolating profile expressed as explicit Bézier control points.
        // Shared tangent at each authored landmark keeps centreline C1 continuous.
        let c1=add(a,mul(sub(b,prev),if i==0 {1.0/3.0}else{1.0/6.0}));
        let c2=sub(b,mul(sub(next,a),if i+2>=n {1.0/3.0}else{1.0/6.0}));
        for k in 0..l.subdivisions {
            let t=k as f32/l.subdivisions as f32;
            let mut c=bezier([a,c1,c2,b],t)?;c[0]*=sign;
            // Smooth positive radii interpolation avoids overshoot below zero.
            let u=t*t*(3.0-2.0*t);let ra=l.sections[i].radii_m;let rb=l.sections[i+1].radii_m;
            let f=(i as f32+t)/(n-1) as f32;
            result.push(AnatomicalLoftRing{center:c,radii:[ra[0]+(rb[0]-ra[0])*u,ra[1]+(rb[1]-ra[1])*u],dorsal_color:p.dorsal,flank_color:p.flank,ventral_color:p.ventral,primary_joint:joints[0],secondary_joint:joints[1],primary_weight:1.0-f});
        }
    }
    let mut end=*result.last().ok_or("Empty sampled profile")?;end.center=l.sections[n-1].center_m;end.center[0]*=sign;end.radii=l.sections[n-1].radii_m;end.primary_weight=0.0;result.push(end);Ok(result)
}

/// Preserve the anatomy owner's topology/UV/weights/countershade, but author
/// the section axes through Core's actual rotation-minimizing frame owner.
/// The existing loft's per-ring ref_up threshold would twist near-vertical limbs.
fn apply_bishop_sections(a:&mut AnatomyMeshAccumulator,rings:&[AnatomicalLoftRing],radial:usize,base:usize)->Result<(),String>{
    let path:Vec<Point3>=rings.iter().map(|r|point(r.center)).collect();
    let mut frames=vec![AffineFrame3{origin:Point3::default(),e0:Point3::default(),e1:Point3::default(),e2:Point3::default()};rings.len()];
    let tangent=norm(sub(rings[1].center,rings[0].center));
    // Match the first section's intended local lateral axis, then transport it.
    let reference=if tangent[1].abs()<0.92{[0.0,1.0,0.0]}else{[0.0,0.0,1.0]};
    let initial=point(norm(cross(reference,tangent)));
    compute_bishop_frames(&path,Some(initial),&mut frames).map_err(|e|format!("Anatomical Bishop frames rejected: {e:?}"))?;
    for (r,(station,frame)) in rings.iter().zip(frames.iter()).enumerate(){
        let u=[frame.e0.x as f32,frame.e0.y as f32,frame.e0.z as f32];
        let v=[frame.e1.x as f32,frame.e1.y as f32,frame.e1.z as f32];
        for col in 0..=radial{let theta=col as f32/radial as f32*core::f32::consts::TAU;let(ct,st)=(NumericProfile::cos(a.numeric_profile(),theta),NumericProfile::sin(a.numeric_profile(),theta));let index=base+r*(radial+1)+col;
            a.positions[index]=add(station.center,add(mul(u,station.radii[0]*ct),mul(v,station.radii[1]*st)));
            a.normals[index]=norm(add(mul(u,ct/station.radii[0]),mul(v,st/station.radii[1])));
        }
    }
    Ok(())
}

/// Correct local triangle orientation against the owner's outward radial normals,
/// then recompute area-weighted surface normals, including taper/cap geometry.
fn finish_surface(a:&mut AnatomyMeshAccumulator,start_v:usize,start_i:usize) {
    for tri in a.indices[start_i..].chunks_exact_mut(3) {
        let [i,j,k]=[tri[0]as usize,tri[1]as usize,tri[2]as usize];
        let n=cross(sub(a.positions[j],a.positions[i]),sub(a.positions[k],a.positions[i]));
        if dot(n,add(add(a.normals[i],a.normals[j]),a.normals[k]))<0.0 {tri.swap(1,2);}
    }
    recompute_surface_normals(a,start_v,start_i);
}

/// Area-weighted normal accumulation without changing established topology.
/// Legacy callers retain their orientation policy in finish_surface above.
fn recompute_surface_normals(a:&mut AnatomyMeshAccumulator,start_v:usize,start_i:usize) {
    let fallback=a.normals[start_v..].to_vec();
    for n in &mut a.normals[start_v..] {*n=[0.0;3];}
    for tri in a.indices[start_i..].chunks_exact(3) {
        let [i,j,k]=[tri[0]as usize,tri[1]as usize,tri[2]as usize];
        let n=cross(sub(a.positions[j],a.positions[i]),sub(a.positions[k],a.positions[i]));
        for index in [i,j,k] {a.normals[index]=add(a.normals[index],n);}
    }
    for (offset,n) in a.normals[start_v..].iter_mut().enumerate() {*n=if dot(*n,*n)<1e-16 {fallback[offset]}else{norm(*n)};}
}
fn record(parts:&mut Vec<Part>,a:&AnatomyMeshAccumulator,id:String,material:usize,v:usize,i:usize) {parts.push(Part{source_id:id,material,vertices:v..a.positions.len(),indices:i..a.indices.len()});}

fn push_vertex(a:&mut AnatomyMeshAccumulator,p:[f32;3],n:[f32;3],color:[f32;4],uv:[f32;2],j:usize)->u32 {
    let i=a.positions.len()as u32;a.positions.push(p);a.normals.push(n);a.colors.push(color);a.uvs.push(uv);a.layer_tags.push(Layer::ExternalIntegument);a.joint_indices.push([j as u16,0,0,0]);a.joint_weights.push([1.0,0.0,0.0,0.0]);i
}

fn ear_shell(a:&mut AnatomyMeshAccumulator,e:&Ear,source:&Source,sign:f32,parts:&mut Vec<Part>) ->Result<(),String>{
    let root=[e.root_m[0]*sign,e.root_m[1],e.root_m[2]];let tip=[e.tip_m[0]*sign,e.tip_m[1],e.tip_m[2]];
    let outer=palette(source,e.material)?;let inner=palette(source,e.inner_material)?;
    const ROWS:usize=24;const COLS:usize=20;
    let begin_v=a.positions.len();let begin_i=a.indices.len();
    // Genuine concave inner and convex outer patches with a closed edge strip.
    // All dimensions remain authored source values; no stock ear morph applied.
    for side in 0..2 {
        let v0=a.positions.len();let i0=a.indices.len();
        for row in 0..=ROWS {let t=row as f32/ROWS as f32;let width=e.width_m*NumericProfile::pow(source.numeric_profile,NumericProfile::sin(source.numeric_profile,core::f32::consts::PI*t).max(0.015),0.65);
            for col in 0..=COLS {let u=col as f32/COLS as f32*2.0-1.0;
                let cup=e.cup_depth_m*(1.0-u*u)*NumericProfile::sin(source.numeric_profile,core::f32::consts::PI*t);
                let z=root[2]+(tip[2]-root[2])*t-cup-if side==1 {e.thickness_m}else{0.0};
                let p=[root[0]+(tip[0]-root[0])*t+u*width,root[1]+(tip[1]-root[1])*t,z];
                let material=if side==0 {inner}else{outer};
                let mut color=material.flank;let rim=NumericProfile::pow(source.numeric_profile,u.abs(),8.0);for c in 0..3 {color[c]=color[c]*(1.0-rim)+outer.dorsal[c]*rim;}
                push_vertex(a,p,[0.0,0.0,if side==0 {1.0}else{-1.0}],color,[col as f32/COLS as f32,t],e.joint);
            }
        }
        for row in 0..ROWS {for col in 0..COLS {let p=(v0+row*(COLS+1)+col)as u32;let q=p+(COLS+1)as u32;a.indices.extend_from_slice(&[p,p+1,q,q,p+1,q+1]);}}
        finish_surface(a,v0,i0);record(parts,a,format!("{}:{}:{}",e.id,sign,side),if side==0 {e.inner_material}else{e.material},v0,i0);
    }
    let patch=(ROWS+1)*(COLS+1);let i0=a.indices.len();
    let mut boundary=Vec::new();for c in 0..=COLS{boundary.push(c);}for r in 1..=ROWS{boundary.push(r*(COLS+1)+COLS);}for c in (0..COLS).rev(){boundary.push(ROWS*(COLS+1)+c);}for r in (1..ROWS).rev(){boundary.push(r*(COLS+1));}
    for k in 0..boundary.len(){let p=(begin_v+boundary[k])as u32;let q=(begin_v+boundary[(k+1)%boundary.len()])as u32;let pp=p+patch as u32;let qq=q+patch as u32;a.indices.extend_from_slice(&[p,qq,q,p,pp,qq]);}
    // Rim references the two patch vertex ranges; retain explicit span lineage.
    parts.push(Part{source_id:format!("{}:{}:rim",e.id,sign),material:e.material,vertices:begin_v..a.positions.len(),indices:i0..a.indices.len()});
    let _=begin_i;Ok(())
}

pub fn build_from_json(json:&str)->Result<Candidate,String> {
    let source=parse_source(json)?;let mut skin=AnatomyMeshAccumulator::with_numeric_profile(source.numeric_profile);let mut parts=Vec::new();let mut coat_ranges=Vec::new();
    for l in &source.lofts {let p=palette(&source,l.material)?;
        if loft_chain::contains(&source,&l.id) {continue;}
        for (sign,joints) in [(1.0,Some(l.joints)),(-1.0,l.mirror_joints)] {let Some(joints)=joints else{continue};
            let rings=sampled_sections(l,p,sign,joints)?;let v=skin.positions.len();let i=skin.indices.len();
            skin.push_countershaded_loft(&rings,l.radial_segments,Layer::ExternalIntegument,true);
            apply_bishop_sections(&mut skin,&rings,l.radial_segments,v)?;
            skin_profile::apply(&mut skin,l,sign<0.0,v)?;
            let cap_start=v+rings.len()*(l.radial_segments+1);
            skin.normals[cap_start]=norm(sub(rings[0].center,rings[1].center));
            skin.normals[cap_start+1]=norm(sub(rings[rings.len()-1].center,rings[rings.len()-2].center));
            finish_surface(&mut skin,v,i);
            // Share seam normals while preserving duplicated UV seam vertices.
            for r in 0..rings.len(){let first=v+r*(l.radial_segments+1);let last=first+l.radial_segments;let n=norm(add(skin.normals[first],skin.normals[last]));skin.normals[first]=n;skin.normals[last]=n;}
            if l.coat {coat_ranges.push(v..v+rings.len()*(l.radial_segments+1));}
            record(&mut parts,&skin,format!("{}:{}",l.id,sign),l.material,v,i);
        }
    }
    loft_chain::append(&source,&mut skin,&mut parts,&mut coat_ranges)?;
    let surface_junctions=junctions::apply(&source,&mut skin,&mut parts)?;
    for e in &source.ellipsoids {let p=palette(&source,e.material)?;
        for sign in [1.0,-1.0] {if sign<0.0&&!e.mirror{continue;}let v=skin.positions.len();let i=skin.indices.len();
            // Existing anatomy owner's ellipsoid, used only for the inset eyeball/naris.
            skin.push_ellipsoid([e.center_m[0]*sign,e.center_m[1],e.center_m[2]],e.radii_m,24,16,p.flank,Layer::ExternalIntegument,e.joint,e.joint,1.0);
            finish_surface(&mut skin,v,i);record(&mut parts,&skin,format!("{}:{}",e.id,sign),e.material,v,i);
        }
    }
    for e in &source.ears {ear_shell(&mut skin,e,&source,1.0,&mut parts)?;if e.mirror{ear_shell(&mut skin,e,&source,-1.0,&mut parts)?;}}
    // Explicit short directional coat ribbons: an editable geometry study,
    // not a claim of a finished groom, hair shader, simulation or film fur.
    if source.coat.enabled {let c=&source.coat;let v=skin.positions.len();let i=skin.indices.len();
        for range in coat_ranges {for seed in range.step_by(c.vertex_stride) {
            let p=skin.positions[seed];let n=skin.normals[seed];if p[1]<0.07{continue;}
            let down=norm(c.direction);let tangent=norm(sub(down,mul(n,dot(down,n))));if dot(tangent,tangent)<0.5{continue;}
            let width=mul(norm(cross(tangent,n)),c.width_m*0.5);let root=add(p,mul(n,0.00025));let middle=add(add(root,mul(tangent,c.length_m*0.55)),mul(n,c.lift_m));let tip=add(add(root,mul(tangent,c.length_m)),mul(n,c.lift_m*0.45));
            let mut color=skin.colors[seed];let breakup=0.84+0.28*((seed*73%101)as f32/100.0);for channel in &mut color[..3]{*channel=(*channel*breakup).min(1.0);}
            let j=skin.joint_indices[seed][0]as usize;let base=skin.positions.len()as u32;
            for (pos,uv) in [(sub(root,width),[0.0,0.0]),(add(root,width),[1.0,0.0]),(sub(middle,mul(width,0.65)),[0.0,0.55]),(add(middle,mul(width,0.65)),[1.0,0.55]),(tip,[0.5,1.0])] {push_vertex(&mut skin,pos,n,color,uv,j);let last=skin.positions.len()-1;skin.joint_indices[last]=skin.joint_indices[seed];skin.joint_weights[last]=skin.joint_weights[seed];}
            // Two-sided geometric strips keep the current opaque material route usable.
            for tri in [[0,1,2],[1,3,2],[2,3,4]]{skin.indices.extend_from_slice(&[base+tri[0],base+tri[1],base+tri[2],base+tri[2],base+tri[1],base+tri[0]]);}
        }}record(&mut parts,&skin,"directional-short-coat-study".into(),c.material,v,i);
    }
    let groom=groom::append(&source,&mut skin,&mut parts)?;
    let mut osteology=AnatomyMeshAccumulator::with_numeric_profile(source.numeric_profile);let mut myology=AnatomyMeshAccumulator::with_numeric_profile(source.numeric_profile);
    for b in &source.bones {osteology.push_long_bone_with_epiphyses(source.joints[b.joints[0]].position_m,source.joints[b.joints[1]].position_m,b.shaft_radius_m,b.condyle_radius_m,[0.0;3],[0.76,0.72,0.60,1.0],Layer::Osteology,b.joints[0],b.joints[1]);}
    for m in &source.muscles {myology.push_fusiform_muscle(source.joints[m.joints[0]].position_m,source.joints[m.joints[1]].position_m,m.radii_m[0],m.radii_m[1],m.bow_m,[0.38,0.07,0.055,1.0],[0.76,0.72,0.61,1.0],Layer::MyologyAndTendons,m.joints[0],m.joints[1]);}
    let mut min=[f32::INFINITY;3];let mut max=[f32::NEG_INFINITY;3];
    for p in &skin.positions {for axis in 0..3{min[axis]=min[axis].min(p[axis]);max[axis]=max[axis].max(p[axis]);}}
    if skin.positions.len()>750_000||skin.positions.iter().flatten().any(|v|!v.is_finite()){return Err("Generated anatomy outside finite/budget contract".into());}
    Ok(Candidate{source,skin,osteology,myology,parts,groom,surface_junctions,bounds_min_m:min,bounds_max_m:max})
}

/// Static review projection only. Caller retains Candidate for editable/rig data.
pub fn append_static_projection(candidate:&Candidate,body:&mut SubmeshBuilder) {
    let base=body.positions.len()as u32;body.positions.extend_from_slice(&candidate.skin.positions);body.normals.extend_from_slice(&candidate.skin.normals);body.colors.extend_from_slice(&candidate.skin.colors);body.uvs.extend_from_slice(&candidate.skin.uvs);body.indices.extend(candidate.skin.indices.iter().map(|i|base+*i));
}
pub fn build_swamp_wallaby_authored_review(source:&str)->Result<Candidate,String> {build_from_json(source)}
/// Compatibility wrapper. Runtime WASM edits should call build_from_json instead.
pub fn build_swamp_wallaby_review_specimen(body:&mut SubmeshBuilder) {
    let candidate=build_from_json(DEFAULT_SOURCE).expect("bundled anatomical review source must be valid");append_static_projection(&candidate,body);
}

#[cfg(test)]
#[path = "swamp_wallaby_anatomical_r3/numeric_profile_tests.rs"]
mod numeric_profile_tests;
