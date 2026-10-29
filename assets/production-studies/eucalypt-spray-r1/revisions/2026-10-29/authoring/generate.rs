//! Bounded content-production helper using existing compiled QualiaDB kernels.
//! Source edits are evaluated from spray.vibe without rebuilding this helper.
//! It is not a replacement for Poet's workspace or a new data/state engine.
use std::{collections::HashMap, f64::consts::PI, fs, path::Path};
use qualia_core_db::{q_hash, NQuin};
use qualia_core_db::specialized_libs::computational_geometry as cg;
use qualia_core_db::render::{assets::Mesh, compile_10d as compile, gpu::PortalGpu,
    telemetry::SystemTelemetry, surface_authoring::encode_surface_png_into};
use qualia_core_db::container_10d::{MaterialRecord, SubmeshRange};
use qualia_core_db::bundle::{BundleWriter, BundleReader};
use vibe::{Value, Budget, LocalHost, session::{SourceSession, SessionLimits}};

type V = [f64;3];
fn add(a:V,b:V)->V {std::array::from_fn(|i|a[i]+b[i])}
fn sub(a:V,b:V)->V {std::array::from_fn(|i|a[i]-b[i])}
fn mul(a:V,s:f64)->V {a.map(|x|x*s)}
fn cross(a:V,b:V)->V {[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]}
fn dot(a:V,b:V)->f64 {a.iter().zip(b).map(|(x,y)|x*y).sum()}
fn unit(a:V)->V {let n=dot(a,a).sqrt(); assert!(n>1e-12);mul(a,1.0/n)}
fn p(a:V)->cg::Point3 {cg::Point3::new(a[0],a[1],a[2])}
fn f(v:&Value,k:&str)->f64 {let Value::Record(r)=v else{panic!("record required")}; match r.get(k).expect(k) {Value::F64(x)=>*x,Value::I64(x)=>*x as f64,_=>panic!("number {k}")}}
fn list<'a>(v:&'a Value,k:&str)-> &'a [Value] {let Value::Record(r)=v else{panic!("record")};let Value::List(x)=&r[k] else{panic!("list")};x}
fn noise(x:f64)->f64 {(x*1.91).sin()*0.50+(x*7.73+0.3).sin()*0.28+(x*19.3).sin()*0.12}

struct Asset {
    pts:Vec<cg::Point3>, tris:Vec<[u32;3]>, colors:Vec<[f32;4]>, uv:Vec<[f32;2]>,
    ranges:Vec<SubmeshRange>, names:Vec<String>, materials:Vec<MaterialRecord>,
}
impl Asset {
 fn new()->Self {
  let materials=(1..=5).map(|id| {let mut m=MaterialRecord::legacy_default();m.id=id;m.roughness=match id{1=>0.82,2=>0.49,3=>0.58,4=>0.68,_=>0.78};m.specular=0.32;m.double_sided=false;m}).collect();
  Self{pts:vec![],tris:vec![],colors:vec![],uv:vec![],ranges:vec![],names:vec![],materials}
 }
 fn append(&mut self,name:&str,mat:u64,pts:&[cg::Point3],tris:&[[u32;3]],colors:&[[f32;4]],uv:&[[f32;2]]) {
  assert_eq!(pts.len(),colors.len());assert_eq!(pts.len(),uv.len());
  assert!(self.pts.len()+pts.len()<180_000);assert!(self.tris.len()+tris.len()<360_000);
  let offset=self.pts.len() as u32;let first=self.tris.len() as u32*3;
  self.pts.extend_from_slice(pts);self.colors.extend_from_slice(colors);self.uv.extend_from_slice(uv);
  self.tris.extend(tris.iter().map(|t|t.map(|i|i+offset)));
  self.ranges.push(SubmeshRange{first_index:first,index_count:tris.len() as u32*3,material_id:mat,semantic_id:q_hash(&format!("urn:ecopunk:study:eucalypt-spray-r1:{name}"))});self.names.push(name.into());
 }
 fn tube(&mut self,name:&str,axis:&[V],radii:&[f64],slices:usize,mat:u64,color:[f64;3],bark:bool) {
  let section=(0..slices).map(|i|{let a=2.*PI*i as f64/slices as f64;let r=if bark{1.+0.055*(a*7.).sin()+0.025*(a*13.).sin()}else{1.};cg::Point2::new(r*a.cos(),r*a.sin())}).collect::<Vec<_>>();
  let axis=axis.iter().copied().map(p).collect::<Vec<_>>();
  let stations=radii.iter().map(|r|cg::ExtrusionStation{scale_x:*r,scale_y:*r,twist_rad:0.0}).collect::<Vec<_>>();
  let z=p([0.;3]);let mut frames=vec![cg::AffineFrame3{origin:z,e0:z,e1:z,e2:z};axis.len()];
  let mut vertices=vec![z;axis.len()*slices+2];let mut triangles=vec![[0;3];axis.len()*slices*2+2*slices];
  let(nv,nt)=cg::extrude_profile_along_path(&axis,&section,Some(&stations),true,true,&mut frames,&mut vertices,&mut triangles).expect("Core sweep");
  let mut colors=vec![];let mut uv=vec![];
  for i in 0..nv {let angle=(i%slices)as f64/slices as f64;let t=(i/slices)as f64/(axis.len()-1)as f64;
   let variation=if bark{0.11*noise(angle*48.+t*2.)+0.04*noise(t*130.)}else{0.025*noise(i as f64)};
   colors.push([(color[0]*(1.+variation))as f32,(color[1]*(1.+variation))as f32,(color[2]*(1.+variation))as f32,1.]);uv.push([angle as f32,t.min(1.)as f32]);}
  self.append(name,mat,&vertices[..nv],&triangles[..nt],&colors,&uv);
 }
}

#[derive(Clone,Copy)]
struct Leaf {base:V,along:V,across:V,normal:V,length:f64,width:f64,thickness:f64,age:f64}
impl Leaf {
 fn at(self,t:f64,s:f64,back:bool)->V {
  let half=0.5*self.width*(PI*t).sin().max(0.002).powf(0.83)*(1.-0.18*t);
  let lateral=s*half+0.16*self.length*(PI*t).sin()*t;
  let z=0.0045*(PI*t).sin()*s*s+0.009*t*t*t-0.003*(PI*t).sin()
      +0.00022*(1.-s.abs())*(PI*t).sin()+if back{-self.thickness/2.}else{self.thickness/2.};
  add(self.base,add(mul(self.along,t*self.length),add(mul(self.across,lateral),mul(self.normal,z))))
 }
 fn color(self,t:f64,s:f64,back:bool)->[f32;4] {
  let mature=[0.105,0.235,0.155];let young=[0.40,0.215,0.115];
  let a=(self.age*1.8).min(1.);let light=1.+0.065*noise(t*76.+s*24.)+0.05*s.abs();
  let c=std::array::from_fn::<_,3,_>(|i|(mature[i]*a+young[i]*(1.-a))*light);
  if back {[(c[0]*0.85+0.10)as f32,(c[1]*0.90+0.10)as f32,(c[2]*0.85+0.105)as f32,1.]}
  else {[c[0]as f32,c[1]as f32,c[2]as f32,1.]}
 }
}
fn make_leaf(asset:&mut Asset,leaf:Leaf,id:usize,rows:usize,cols:usize,vein_pairs:usize) {
 let mut pts=vec![];let mut uv=vec![];let mut colors=vec![];let mut tris=vec![];
 let layer=(rows+1)*(cols+1);
 for back in [false,true] {for r in 0..=rows{let t=0.0008+0.9984*r as f64/rows as f64;
  for c in 0..=cols{let s=2.*c as f64/cols as f64-1.;pts.push(p(leaf.at(t,s,back)));colors.push(leaf.color(t,s,back));uv.push([c as f32/cols as f32,r as f32/rows as f32]);}}}
 for r in 0..rows {for c in 0..cols {let a=(r*(cols+1)+c)as u32;let b=a+1;let d=a+(cols+1)as u32;let e=d+1;
  tris.extend([[a,b,d],[b,e,d]]);let o=layer as u32;tris.extend([[a+o,d+o,b+o],[b+o,d+o,e+o]]);}}
 // Match top/bottom winding to the authored +normal direction.
 let a=pts[tris[0][0]as usize];let b=pts[tris[0][1]as usize];let c=pts[tris[0][2]as usize];
 let cr=cross(sub([b.x,b.y,b.z],[a.x,a.y,a.z]),sub([c.x,c.y,c.z],[a.x,a.y,a.z]));
 if dot(cr,leaf.normal)<0. {for t in &mut tris{t.swap(1,2);}}
 let mut boundary=vec![];
 for c in 0..=cols{boundary.push(c as u32);}for r in 1..=rows{boundary.push((r*(cols+1)+cols)as u32);}
 for c in (0..cols).rev(){boundary.push((rows*(cols+1)+c)as u32);}for r in (1..rows).rev(){boundary.push((r*(cols+1))as u32);}
 for i in 0..boundary.len(){let a=boundary[i];let b=boundary[(i+1)%boundary.len()];let o=layer as u32;tris.extend([[a,a+o,b],[b,a+o,b+o]]);}
 asset.append(&format!("leaf-{id:02}-lamina"),2,&pts,&tris,&colors,&uv);
 let axis=(0..=32).map(|i|add(leaf.at(0.01+0.97*i as f64/32.,0.,false),mul(leaf.normal,0.00018))).collect::<Vec<_>>();
 let radii=(0..=32).map(|i|0.00048*(1.-0.82*i as f64/32.)).collect::<Vec<_>>();
 asset.tube(&format!("leaf-{id:02}-midrib"),&axis,&radii,8,3,[0.24,0.36,0.20],false);
 for v in 0..vein_pairs {let t0=0.10+v as f64*(0.74/vein_pairs as f64);for side in [-1.,1.]{
  let axis=(0..=6).map(|i|{let a=i as f64/6.;add(leaf.at(t0+0.115*a,side*0.91*a,false),mul(leaf.normal,0.00013))}).collect::<Vec<_>>();
  let radii=(0..=6).map(|i|0.00019*(1.-0.7*i as f64/6.)).collect::<Vec<_>>();
  asset.tube(&format!("leaf-{id:02}-vein-{v}-{}",if side<0.{"L"}else{"R"}),&axis,&radii,6,3,[0.20,0.30,0.165],false);
 }}
}

fn main()->Result<(),Box<dyn std::error::Error>> {
 let dir=std::env::args().nth(1).expect("output/source directory");let dir=Path::new(&dir);
 let source=fs::read_to_string(dir.join("authoring/spray.vibe"))?;
 let mut session=SourceSession::new(vec![],SessionLimits::default());let mut host=LocalHost::default();
 let params=session.submit(&source,&mut host,Budget::default()).map_err(|e|format!("Vibe source refused: {e:?}"))?.value;
 let rows=f(&params,"leaf_rows")as usize;let cols=f(&params,"leaf_columns")as usize;let pairs=f(&params,"vein_pairs")as usize;
 assert!((12..=96).contains(&rows)&&(6..=24).contains(&cols)&&pairs<=16);
 let length=f(&params,"stem_length_m");assert!((0.1..=2.).contains(&length));
 let curve=|t:f64|[0.034*(PI*t).sin()+0.012*(t*2.*PI).sin(),length*(t-0.5),0.018*(2.*PI*t).sin()];
 let mut asset=Asset::new();
 let axis=(0..=72).map(|i|curve(i as f64/72.)).collect::<Vec<_>>();
 let radii=(0..=72).map(|i|{let t=i as f64/72.;f(&params,"stem_base_radius_m")*(1.-t)+f(&params,"stem_tip_radius_m")*t}).collect::<Vec<_>>();
 asset.tube("primary-twig",&axis,&radii,32,1,[0.34,0.24,0.14],true);
 let leaves=list(&params,"leaves");assert!(leaves.len()<=24);
 for(id,entry)in leaves.iter().enumerate(){let at=f(entry,"at");let side=f(entry,"side");let roll=f(entry,"roll");let tilt=f(entry,"tilt");
  assert!((0.02..0.98).contains(&at)&&side.abs()==1.);
  let attach=curve(at);let along=unit([side*0.89,0.48,tilt]);let base=add(attach,mul(along,0.022));
  let across0=unit(cross([0.,0.,1.],along));let norm0=unit(cross(along,across0));
  let across=add(mul(across0,roll.cos()),mul(norm0,roll.sin()));let normal=unit(cross(along,across));
  let petiole=(0..=8).map(|i|{let a=i as f64/8.;add(attach,add(mul(sub(base,attach),a),[0.,0.,-0.004*(PI*a).sin()]))}).collect::<Vec<_>>();
  let pr=(0..=8).map(|i|0.0010*(1.-0.56*i as f64/8.)).collect::<Vec<_>>();
  asset.tube(&format!("leaf-{id:02}-petiole"),&petiole,&pr,12,1,[0.30,0.25,0.14],false);
  let leaf=Leaf{base,along,across,normal,length:f(entry,"length_m"),width:f(entry,"width_m"),thickness:f(&params,"leaf_thickness_m"),age:f(entry,"age")};
  assert!(leaf.length>0.04&&leaf.length<0.3&&leaf.width>0.008&&leaf.width<0.06);
  make_leaf(&mut asset,leaf,id,rows,cols,pairs);
 }
 let mut n=vec![p([0.;3]);asset.pts.len()];cg::ddg_operators::vertex_normals(&asset.pts,&asset.tris,&mut n).map_err(|e|format!("Core normals: {e:?}"))?;
 let normals=n.iter().map(|p|[p.x as f32,p.y as f32,p.z as f32]).collect::<Vec<_>>();
 assert!(normals.iter().all(|n|n.iter().all(|x|x.is_finite())&&n.iter().map(|x|x*x).sum::<f32>()>0.5));
 let positions=asset.pts.iter().map(|p|[p.x as f32,p.y as f32,p.z as f32]).collect::<Vec<_>>();
 let min=std::array::from_fn(|i|positions.iter().map(|p|p[i]).fold(f32::INFINITY,f32::min));
 let max=std::array::from_fn(|i|positions.iter().map(|p|p[i]).fold(f32::NEG_INFINITY,f32::max));
 let mesh=Mesh{positions,triangles:asset.tris.clone(),min,max};let indices=mesh.triangles.iter().flatten().copied().collect::<Vec<_>>();
 let mut tangents=vec![[0.;4];mesh.positions.len()];let mut scratch=vec![[0.;6];mesh.positions.len()];
 qualia_core_db::render::mesh_tangent_frames::tangents_from_uvs_into(&mesh.positions,&asset.uv,&normals,&indices,&mut tangents,&mut scratch).map_err(|e|format!("Core tangent generation: {e:?}"))?;
 let bytes=compile::compile_mesh_to_10d_with_extras_and_frames(&mesh,&[],None,compile::Compile10dExtras::default(),Some(&asset.colors),Some(&normals),Some(&tangents))?;
 let bytes=qualia_core_db::container_10d::attach_texture_coordinate_section(&bytes,&asset.uv).map_err(|e|format!("UV: {e:?}"))?;
 let bytes=qualia_core_db::container_10d::attach_material_section(&bytes,&asset.materials,&asset.ranges,indices.len()as u32).map_err(|e|format!("materials: {e:?}"))?;
 fs::write(dir.join("eucalypt-spray.10d"),&bytes)?;
 let decoded=compile::decode_10d_mesh(&bytes)?;assert_eq!(decoded.positions.len(),mesh.positions.len());assert_eq!(decoded.triangles,mesh.triangles);
 assert!(compile::decode_10d_mesh_frames(&bytes,mesh.positions.len())?.is_some());
 // Portable editable geometry derivative; native .10d/HMC remains primary.
 let mut obj=String::from("# Original illustrative geometry. Generated through QualiaDB native kernels.\nmtllib eucalypt-spray.mtl\n");
 for p in &mesh.positions{obj.push_str(&format!("v {} {} {}\n",p[0],p[1],p[2]));}
 for uv in &asset.uv{obj.push_str(&format!("vt {} {}\n",uv[0],uv[1]));}
 for n in &normals{obj.push_str(&format!("vn {} {} {}\n",n[0],n[1],n[2]));}
 for(r,name)in asset.ranges.iter().zip(&asset.names){obj.push_str(&format!("g {name}\nusemtl material-{}\n",r.material_id));for tri in indices[r.first_index as usize..(r.first_index+r.index_count)as usize].chunks_exact(3){obj.push_str(&format!("f {0}/{0}/{0} {1}/{1}/{1} {2}/{2}/{2}\n",tri[0]+1,tri[1]+1,tri[2]+1));}}
 fs::write(dir.join("eucalypt-spray.obj"),obj)?;
 let mut mtl=String::new();for(i,c)in [[0.34,0.24,0.14],[0.105,0.235,0.155],[0.20,0.30,0.165],[0.4,0.3,0.2],[0.3,0.3,0.3]].iter().enumerate(){mtl.push_str(&format!("newmtl material-{}\nKd {} {} {}\nKs 0.15 0.15 0.15\nNs 48\n\n",i+1,c[0],c[1],c[2]));}fs::write(dir.join("eucalypt-spray.mtl"),mtl)?;
 let id="urn:ecopunk:study:eucalypt-spray-r1";let mut lex=HashMap::new();let mut quins=vec![];
 let mut assertion=|s:&str,p:&str,o:&str|{for term in [s,p,o]{lex.insert(q_hash(term),term.to_owned());}let mut q=NQuin{subject:q_hash(s),predicate:q_hash(p),object:q_hash(o),context:0,metadata:0,parity:0};q.recalculate_parity();quins.push(q);};
 assertion(id,"http://www.w3.org/1999/02/22-rdf-syntax-ns#type","urn:ecopunk:AuthoredIllustrativeBranchStudy");
 assertion(id,"http://www.w3.org/ns/prov#wasGeneratedBy","urn:ecopunk:activity:eucalypt-spray-r1-native-generation");
 assertion(id,"http://www.w3.org/ns/prov#wasDerivedFrom","urn:ecopunk:source:eucalypt-spray-r1-vibe");
 for name in &asset.names{let part=format!("{id}:{name}");assertion(id,"urn:ecopunk:hasAuthoredComponent",&part);}
 quins.sort_by_key(|q|q.object);
 let mut builder=qualia_core_db::q42_volume::UnifiedVolumeBuilder::with_lex_map(&lex).map_err(|e|format!("Q42 lex: {e:?}"))?;
 builder.push_block(0,&quins).map_err(|e|format!("Q42 block: {e:?}"))?;let q42=builder.finish_to_bytes();fs::write(dir.join("knowledge.q42"),&q42)?;
 let mut ttl=format!("@prefix prov: <http://www.w3.org/ns/prov#> .\n<{id}> a <urn:ecopunk:AuthoredIllustrativeBranchStudy>; prov:wasDerivedFrom <urn:ecopunk:source:eucalypt-spray-r1-vibe> .\n");for name in &asset.names{ttl.push_str(&format!("<{id}> <urn:ecopunk:hasAuthoredComponent> <{id}:{name}> .\n"));}fs::write(dir.join("knowledge.ttl"),ttl)?;
 println!("asset vertices={} triangles={} components={} knowledge_assertions={}",mesh.positions.len(),mesh.triangles.len(),asset.names.len(),quins.len());
 println!("chosen_design_bounds_m min={min:?} max={max:?}");
 let side=1024;let mut gpu=PortalGpu::new_offscreen(side,side,1)?;
 gpu.set_ambient_enabled(false);gpu.set_tensor_projection_enabled(false);gpu.set_clear_color(0.035,0.048,0.055,1.);
 gpu.set_lighting(-0.35,0.65,0.85,1.8,0.24);gpu.set_shadows_enabled(true);gpu.set_screen_space_ao_enabled(true);
 gpu.upload_mesh_colored_with_frames_and_materials_and_uv0(&mesh.positions,&asset.colors,Some(&normals),Some(&tangents),Some(&asset.uv),&indices,&asset.materials,&asset.ranges)?;
 for(name,yaw,pitch,distance,target)in [("hero.png",0.12,0.10,1.25,[0.02,0.015,0.]),("detail.png",-0.18,0.08,0.59,[0.04,-0.14,0.])]{
  gpu.set_camera_target(yaw,pitch,distance,target[0],target[1],target[2]);gpu.render(0.0,&SystemTelemetry::default())?;
  let mut rgba=vec![0;gpu.required_rgba8_bytes()];gpu.read_rgba8_into(&mut rgba)?;assert!(rgba.chunks_exact(4).any(|p|p[0]>80||p[1]>100));
  let mut png=vec![0;rgba.len()+side as usize*4+8192];let n=encode_surface_png_into(side,&rgba,&mut png).map_err(|e|format!("PNG: {e:?}"))?;fs::write(dir.join(name),&png[..n])?;println!("rendered {name}: {side}x{side}");
 }
 let mut bundle=BundleWriter::new();
 for(name,kind)in [("eucalypt-spray.10d","10d"),("knowledge.q42","q42"),("knowledge.ttl","ttl"),("authoring/spray.vibe","vibe"),("authoring/generate.rs","source"),("authoring/build-helper.ps1","source"),("README.md","document"),("dossier/README.md","document"),("dossier/claims.json","claim-projection"),("hero.png","png"),("detail.png","png")]{bundle.add_file(name,kind,fs::read(dir.join(name))?,None).map_err(|e|format!("Bundle entry: {e:?}"))?;}
 let hmc=bundle.build().map_err(|e|format!("HMC build: {e:?}"))?;let read=BundleReader::parse(&hmc).map_err(|e|format!("HMC parse: {e:?}"))?;
 for name in ["eucalypt-spray.10d","knowledge.q42","authoring/spray.vibe","hero.png"]{if !read.verify_entry(name){return Err(format!("HMC integrity verification failed: {name}").into());}}
 fs::write(dir.join("eucalypt-spray.hmc"),hmc)?;println!("Native mesh/frame roundtrip and HMC integrity verification completed.");
 Ok(())
}
