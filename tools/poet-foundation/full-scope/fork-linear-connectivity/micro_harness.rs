extern crate qualia_core_db;
mod topology { pub use qualia_core_db::specialized_libs::computational_geometry::{build_triangle_half_edges,required_edge_slots,EdgeSlot,HalfEdge,TopologyError,INVALID_INDEX}; }
pub mod specialized_libs { pub mod computational_geometry { pub mod topology { pub use qualia_core_db::specialized_libs::computational_geometry::{build_triangle_half_edges,required_edge_slots,EdgeSlot}; } } }
#[path = "connectivity.rs"] mod connectivity;
#[path = "surface_topology_qualification.rs"] mod surface_topology_qualification;
#[path = "micro_public_tests.rs"] mod micro_public_tests;
