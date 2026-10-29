//! Editable wallaby-directed content study using native QualiaDB kernels.
//! Numerical design choices are hypotheses, not measured anatomy. No rig/coat groom claim.
use std::{collections::HashMap, fs, path::Path};
use qualia_core_db::{q_hash, NQuin};
use qualia_core_db::specialized_libs::computational_geometry as cg;
use cg::sdf_ops::{sdf_sphere, sdf_capsule, sdf_smooth_union_stable};
use qualia_core_db::render::{assets::Mesh, compile_10d as compile, gpu::PortalGpu,
    telemetry::SystemTelemetry, surface_authoring::encode_surface_png_into};
use qualia_core_db::container_10d::{MaterialRecord, SubmeshRange};
use qualia_core_db::bundle::{BundleReader, BundleWriter};
use vibe::{Value, Budget, LocalHost, session::{SourceSession, SessionLimits}};
type V = [f64;3];
const ID: &str = "urn:ecopunk:study:swamp-wallaby-review-r1";
fn p(v:V)->cg::Point3 { cg::Point3::new(v[0],v[1],v[2]) }
fn f(v:&Value,k:&str)->f64 { let Value::Record(r)=v else {panic!("record required")}; let x=match &r[k] {Value::F64(x)=>*x,Value::I64(x)=>*x as f64,_=>panic!("number required: {k}")}; assert!(x.is_finite()); x }
fn s<'a>(v:&'a Value,k:&str)-> &'a str {let Value::Record(r)=v else{panic!("record")};let Value::String(x)=&r[k] else{panic!("string {k}")}; x }
fn list<'a>(v:&'a Value,k:&str)-> &'a [Value] {let Value::Record(r)=v else{panic!("record")};let Value::List(x)=&r[k] else{panic!("list")};x}
fn xyz(v:&Value)->V {[f(v,"x"),f(v,"y"),f(v,"z")]}
struct Shape {name:String,c:V,r:V,tilt:f64}
impl Shape {
 fn field(&self,at:V)->f64 {
  let d=std::array::from_fn::<_,3,_>(|i|at[i]-self.c[i]);let (si,co)=self.tilt.sin_cos();
  // Authored affine placement of Core sphere field. Conservative scaled distance,
  // not a claim of exact ellipsoid distance, mechanics or biological inference.
  let local=[d[0]/self.r[0],(d[1]*co+d[2]*si)/self.r[1],(-d[1]*si+d[2]*co)/self.r[2]];
  sdf_sphere(p(local),p([0.;3]),1.0)*self.r.iter().copied().fold(f64::INFINITY,f64::min)
 }
}
struct Tail {c:V,r:f64}
fn field(at:V,shapes:&[Shape],tail:&[Tail],blend:f64)->f64 {
 let mut d=1e6;for shape in shapes {d=sdf_smooth_union_stable(d,shape.field(at),blend)}
 for pair in tail.windows(2) { for j in 0..6 {let t=j as f64/6.;let u=(j+1)as f64/6.;let a=std::array::from_fn(|i|pair[0].c[i]*(1.-t)+pair[1].c[i]*t);let b=std::array::from_fn(|i|pair[0].c[i]*(1.-u)+pair[1].c[i]*u);let r=pair[0].r*(1.-t)+pair[1].r*t; d=sdf_smooth_union_stable(d,sdf_capsule(p(at),p(a),p(b),r),blend*0.5);}}
 d
}
fn shade(at:V,n:V,params:&Value)->[f32;4] {
 let dark=[f(params,"dorsal_r"),f(params,"dorsal_g"),f(params,"dorsal_b")];
 let warm=[f(params,"ventral_r"),f(params,"ventral_g"),f(params,"ventral_b")];
 let cheek=[f(params,"cheek_r"),f(params,"cheek_g"),f(params,"cheek_b")];
 // Descriptive material masks; exact colors and boundaries require reviewed photos.
 let belly=((n[2]-0.05)*1.6).clamp(0.,1.)*((at[1]-0.22)*5.).clamp(0.,1.)*((0.87-at[1])*6.).clamp(0.,1.);
 let stripe=(-((at[1]-(0.932+0.18*(at[2]-0.19)))/0.012).powi(2)).exp()
  *((at[2]-0.14)*25.).clamp(0.,1.)*((0.29-at[2])*35.).clamp(0.,1.)*((at[0].abs()-0.025)*40.).clamp(0.,1.);
 let ear=((at[1]-1.015)*14.).clamp(0.,1.)*(n[2]*1.8).clamp(0.,1.);
 let grain=1.+0.055*(at[0]*361.+at[1]*421.+at[2]*191.).sin()*(at[1]*167.-at[2]*239.).sin();
 let mut c=std::array::from_fn::<_,3,_>(|i|dark[i]*(1.-belly)+warm[i]*belly);
 for i in 0..3 {c[i]=c[i]*(1.-stripe*0.8)+cheek[i]*stripe*0.8;c[i]=c[i]*(1.-ear*0.38)+warm[i]*ear*0.38;c[i]*=grain;}
 [c[0]as f32,c[1]as f32,c[2]as f32,1.]
}
fn main()->Result<(),Box<dyn std::error::Error>> {
 let arg=std::env::args().nth(1).expect("asset directory");let dir=Path::new(&arg);
 let source=fs::read_to_string(dir.join("authoring/wallaby.vibe"))?;
 let mut session=SourceSession::new(vec![],SessionLimits::default());let mut host=LocalHost::default();
 let params=session.submit(&source,&mut host,Budget::default()).map_err(|e|format!("Vibe: {e:?}"))?.value;
 let shapes=list(&params,"shapes").iter().map(|v|Shape{name:s(v,"name").into(),c:xyz(v),r:[f(v,"rx"),f(v,"ry"),f(v,"rz")],tilt:f(v,"tilt")}).collect::<Vec<_>>();
 let tail=list(&params,"tail").iter().map(|v|Tail{c:xyz(v),r:f(v,"r")}).collect::<Vec<_>>();
 assert!((1..=48).contains(&shapes.len())&&(2..=16).contains(&tail.len()));
 for sh in &shapes {assert!(sh.r.iter().all(|r|*r>=0.01&&*r<=0.4));assert!(sh.c.iter().all(|x|x.abs()<2.));}
 for t in &tail {assert!(t.r>=0.002&&t.r<0.2&&t.c.iter().all(|x|x.abs()<2.));}
 let blend=f(&params,"blend_m");assert!((0.0..=0.05).contains(&blend));
 let nx=f(&params,"grid_x")as usize;let ny=f(&params,"grid_y")as usize;let nz=f(&params,"grid_z")as usize;
 assert!((32..=96).contains(&nx)&&(48..=160).contains(&ny)&&(64..=256).contains(&nz));
 let lo=[-0.32,-0.04,-1.22];let hi=[0.32,1.30,0.43];
 let delta=[(hi[0]-lo[0])/(nx-1)as f64,(hi[1]-lo[1])/(ny-1)as f64,(hi[2]-lo[2])/(nz-1)as f64];
 let mut points=Vec::<cg::Point3>::new();let mut triangles=Vec::<[u32;3]>::new();let mut welded=HashMap::<[i64;3],u32>::new();
 // Eight-cell slabs keep Core marching-cubes scratch bounded. Shared seam vertices
 // use 0.1-micrometre quantization; qualification must still inspect topology.
 for z0 in (0..nz-1).step_by(8) {
  let depth=(nz-1-z0).min(8)+1;let mut grid=Vec::with_capacity(nx*ny*depth);
  for z in 0..depth {for y in 0..ny {for x in 0..nx {grid.push(field([lo[0]+x as f64*delta[0],lo[1]+y as f64*delta[1],lo[2]+(z0+z)as f64*delta[2]],&shapes,&tail,blend));}}}
  let cells=(nx-1)*(ny-1)*(depth-1);let mut verts=vec![p([0.;3]);cells*30];let mut tris=vec![[0;3];cells*10];
  let(nv,nt)=cg::marching_cubes(&grid,nx,ny,depth,delta[0],delta[1],delta[2],lo[0],lo[1],lo[2]+z0 as f64*delta[2],0.,&mut verts,&mut tris).map_err(|e|format!("Core marching cubes: {e:?}"))?;
  let mut map=Vec::with_capacity(nv);for q in &verts[..nv] {let key=[(q.x*1e7).round()as i64,(q.y*1e7).round()as i64,(q.z*1e7).round()as i64];let ix=*welded.entry(key).or_insert_with(||{let i=points.len()as u32;points.push(*q);i});map.push(ix);}
  for t in &tris[..nt] {let t=t.map(|i|map[i as usize]);if t[0]!=t[1]&&t[1]!=t[2]&&t[0]!=t[2]{triangles.push(t)}}
  if points.len()>400_000||triangles.len()>800_000 {return Err("authored mesh bound exceeded".into());}
 }
 assert!(!points.is_empty()&&!triangles.is_empty());
 let mut positions=points.iter().map(|p|[p.x as f32,p.y as f32,p.z as f32]).collect::<Vec<_>>();
 let mut normals=Vec::with_capacity(points.len());let mut colors=Vec::with_capacity(points.len());let mut uv=Vec::with_capacity(points.len());
 for q in &points {let at=[q.x,q.y,q.z];let mut n=[0.;3];for i in 0..3{let mut a=at;let mut b=at;a[i]+=0.001;b[i]-=0.001;n[i]=field(a,&shapes,&tail,blend)-field(b,&shapes,&tail,blend);}
  let len=n.iter().map(|v|v*v).sum::<f64>().sqrt();assert!(len>1e-10);let n=n.map(|v|v/len);normals.push(n.map(|v|v as f32));colors.push(shade(at,n,&params));uv.push([((q.z-lo[2])/(hi[2]-lo[2]))as f32,((q.y-lo[1])/(hi[1]-lo[1]))as f32]);}
 // Orient each triangle against its field gradient rather than trusting a particular
 // historical extraction winding. This is geometric validation, not biological proof.
 for t in &mut triangles {let a=positions[t[0]as usize];let b=positions[t[1]as usize];let c=positions[t[2]as usize];let u=std::array::from_fn::<_,3,_>(|i|b[i]-a[i]);let v=std::array::from_fn::<_,3,_>(|i|c[i]-a[i]);let cross=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]];if cross.iter().zip(normals[t[0]as usize]).map(|(a,b)|a*b).sum::<f32>()<0.{t.swap(1,2);}}
 let mut ranges=vec![SubmeshRange{first_index:0,index_count:triangles.len()as u32*3,material_id:1,semantic_id:q_hash(&format!("{ID}:continuous-skin"))}];
 // Eyes and nose are distinct editable parts. Core supplies their sphere topology.
 for(name,center,scale,color,mat)in [("left-eye",[-0.066,0.978,0.226],[0.012,0.014,0.013],[0.018,0.014,0.010,1.],2),("right-eye",[0.066,0.978,0.226],[0.012,0.014,0.013],[0.018,0.014,0.010,1.],2),("nose",[0.,0.934,0.353],[0.029,0.022,0.020],[0.025,0.022,0.020,1.],3)] {
  let sphere=cg::uv_sphere(1.,16,24)?;let base=positions.len()as u32;let first=triangles.len()as u32*3;
  for v in &sphere.positions{positions.push(std::array::from_fn(|i|center[i]+v[i]*scale[i]));let n=std::array::from_fn::<_,3,_>(|i|v[i]/scale[i]);let len=n.iter().map(|x|x*x).sum::<f32>().sqrt();normals.push(n.map(|x|x/len));colors.push(color);uv.push([0.5+v[0]*0.5,0.5+v[1]*0.5]);}
  triangles.extend(sphere.triangles.iter().map(|t|t.map(|i|i+base)));ranges.push(SubmeshRange{first_index:first,index_count:sphere.triangles.len()as u32*3,material_id:mat,semantic_id:q_hash(&format!("{ID}:{name}"))});
 }
 let min=std::array::from_fn(|i|positions.iter().map(|p|p[i]).fold(f32::INFINITY,f32::min));let max=std::array::from_fn(|i|positions.iter().map(|p|p[i]).fold(f32::NEG_INFINITY,f32::max));
 let mesh=Mesh{positions,triangles,min,max};let indices=mesh.triangles.iter().flatten().copied().collect::<Vec<_>>();
 let mut tangents=vec![[0.;4];mesh.positions.len()];let mut scratch=vec![[0.;6];mesh.positions.len()];qualia_core_db::render::mesh_tangent_frames::tangents_from_uvs_into(&mesh.positions,&uv,&normals,&indices,&mut tangents,&mut scratch).map_err(|e|format!("tangents: {e:?}"))?;
 let materials=(1..=3).map(|id|{let mut m=MaterialRecord::legacy_default();m.id=id;m.roughness=if id==2{0.24}else{0.82};m.specular=if id==2{0.6}else{0.25};m}).collect::<Vec<_>>();
 let bytes=compile::compile_mesh_to_10d_with_extras_and_frames(&mesh,&[],None,compile::Compile10dExtras::default(),Some(&colors),Some(&normals),Some(&tangents))?;
 let bytes=qualia_core_db::container_10d::attach_texture_coordinate_section(&bytes,&uv).map_err(|e|format!("UV: {e:?}"))?;
 let bytes=qualia_core_db::container_10d::attach_material_section(&bytes,&materials,&ranges,indices.len()as u32).map_err(|e|format!("materials: {e:?}"))?;
 fs::write(dir.join("swamp-wallaby.10d"),&bytes)?;let decoded=compile::decode_10d_mesh(&bytes)?;assert_eq!(decoded.triangles,mesh.triangles);assert_eq!(decoded.positions.len(),mesh.positions.len());assert!(compile::decode_10d_mesh_frames(&bytes,mesh.positions.len())?.is_some());
 println!("candidate vertices={} triangles={} bounds_m={min:?}..{max:?}",mesh.positions.len(),mesh.triangles.len());
 let mut lex=HashMap::new();let mut quins=vec![];let mut assertion=|s:&str,p:&str,o:&str|{for t in [s,p,o]{lex.insert(q_hash(t),t.to_owned());}let mut q=NQuin{subject:q_hash(s),predicate:q_hash(p),object:q_hash(o),context:0,metadata:0,parity:0};q.recalculate_parity();quins.push(q);};
 assertion(ID,"http://www.w3.org/1999/02/22-rdf-syntax-ns#type","urn:ecopunk:SyntheticUnacceptedSpeciesStudy");assertion(ID,"urn:ecopunk:workingTaxonLabel","Wallabia bicolor");assertion(ID,"urn:ecopunk:reviewStatus","pending-coordinator-and-Timothy");assertion(ID,"http://www.w3.org/ns/prov#wasDerivedFrom","urn:ecopunk:source:swamp-wallaby-review-r1:wallaby.vibe");
 for sh in &shapes{assertion(ID,"urn:ecopunk:hasAuthoredShape",&format!("{ID}:{}",sh.name));}for name in ["tail","left-eye","right-eye","nose"]{assertion(ID,"urn:ecopunk:hasAuthoredComponent",&format!("{ID}:{name}"));}
 quins.sort_by_key(|q|q.object);let mut builder=qualia_core_db::q42_volume::UnifiedVolumeBuilder::with_lex_map(&lex).map_err(|e|format!("Q42: {e:?}"))?;builder.push_block(0,&quins).map_err(|e|format!("Q42: {e:?}"))?;fs::write(dir.join("knowledge.q42"),builder.finish_to_bytes())?;
 let side=1024;let mut gpu=PortalGpu::new_offscreen(side,side,1)?;gpu.set_ambient_enabled(false);gpu.set_tensor_projection_enabled(false);gpu.set_clear_color(0.16,0.18,0.20,1.);gpu.set_lighting(-0.5,0.8,0.7,2.0,0.35);gpu.set_shadows_enabled(true);gpu.set_screen_space_ao_enabled(true);gpu.upload_mesh_colored_with_frames_and_materials_and_uv0(&mesh.positions,&colors,Some(&normals),Some(&tangents),Some(&uv),&indices,&materials,&ranges)?;
 let mut names=Vec::new();for camera in list(&params,"cameras") {let name=s(camera,"name");assert!(name.chars().all(|c|c.is_ascii_alphanumeric()||c=='-'));gpu.set_camera_target(f(camera,"yaw")as f32,f(camera,"pitch")as f32,f(camera,"distance")as f32,f(camera,"tx")as f32,f(camera,"ty")as f32,f(camera,"tz")as f32);gpu.render(0.,&SystemTelemetry::default())?;let mut rgba=vec![0;gpu.required_rgba8_bytes()];gpu.read_rgba8_into(&mut rgba)?;let mut png=vec![0;rgba.len()+side as usize*4+8192];let n=encode_surface_png_into(side,&rgba,&mut png).map_err(|e|format!("PNG: {e:?}"))?;let filename=format!("{name}.png");fs::write(dir.join(&filename),&png[..n])?;println!("rendered {filename}");names.push(filename);}
 let mut bundle=BundleWriter::new();let mut entries=vec![("swamp-wallaby.10d".to_string(),"10d"),("knowledge.q42".into(),"q42"),("authoring/wallaby.vibe".into(),"vibe"),("authoring/generate.rs".into(),"source"),("README.md".into(),"document"),("dossier/claims.json".into(),"claim-projection"),("dossier/references.md".into(),"document"),("dossier/inventory.json".into(),"document")];for name in names{entries.push((name,"png"));}for(name,kind)in &entries{bundle.add_file(name,kind,fs::read(dir.join(name))?,None).map_err(|e|format!("HMC entry: {e:?}"))?;}let hmc=bundle.build().map_err(|e|format!("HMC: {e:?}"))?;let reader=BundleReader::parse(&hmc).map_err(|e|format!("HMC parse: {e:?}"))?;for(name,_)in &entries{assert!(reader.verify_entry(name),"HMC entry failed: {name}");}fs::write(dir.join("swamp-wallaby-study.hmc"),hmc)?;println!("Native roundtrip and ALL {} HMC entry checks completed; human acceptance pending.",entries.len());
 Ok(())
}
