//! Synthetic public QualiaDB caller check; no native UI, asset or GPU claim.
use qualia_core_db::specialized_libs::computational_geometry as cg;

#[test]
fn poet_calls_qualified_caller_buffered_hard_cap_sweep() {
    use cg::hard_cap_sweep::{hard_cap_sweep_counts, hard_cap_sweep_normals_into};
    let axis = [cg::Point3::new(0.0,0.0,0.0),cg::Point3::new(0.0,0.0,1.0),cg::Point3::new(0.2,0.0,2.0)];
    let section = (0..8).map(|i| { let a=std::f64::consts::TAU*i as f64/8.0; cg::Point2::new(a.cos(),a.sin()) }).collect::<Vec<_>>();
    let counts=hard_cap_sweep_counts(axis.len(),section.len()).unwrap();
    let zero=cg::Point3::new(0.0,0.0,0.0);
    let mut frames=vec![cg::AffineFrame3 { origin:zero,e0:zero,e1:zero,e2:zero };counts.frames];
    let mut positions=vec![zero;counts.vertices]; let mut indices=vec![[0u32;3];counts.triangles];
    let mut normals=vec![zero;counts.vertices]; let mut source_vertices=vec![0u32;counts.vertices];
    assert_eq!(hard_cap_sweep_normals_into(&axis,&section,None,&mut frames,&mut positions,&mut indices,&mut normals,&mut source_vertices).unwrap(),counts);
    assert_eq!(counts.vertices,42); assert_eq!(counts.triangles,48);
    for i in 0..8 {
        let bottom=26+i; let top=34+i;
        assert_eq!(source_vertices[bottom],i as u32);
        assert_eq!(source_vertices[top],(16+i) as u32);
        assert_eq!(positions[bottom],positions[i]); assert_eq!(positions[top],positions[16+i]);
        let dot=normals[bottom].x*normals[i].x+normals[bottom].y*normals[i].y+normals[bottom].z*normals[i].z;
        assert!(dot.abs()<0.5,"cap must remain visibly distinct from adjacent wall normal");
    }
}
