extern crate qualia_core_db;
mod topology {
    pub use qualia_core_db::specialized_libs::computational_geometry::{build_triangle_half_edges, required_edge_slots, EdgeSlot, HalfEdge, TopologyError, INVALID_INDEX};
}
mod connectivity {
    pub use qualia_core_db::specialized_libs::computational_geometry::{label_components, ConnectivityError};
}
#[path = "surface_topology_qualification.rs"]
mod surface_topology_qualification;

#[path = "micro_public_tests.rs"]
mod micro_public_tests;
