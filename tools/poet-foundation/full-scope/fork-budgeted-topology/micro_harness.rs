extern crate qualia_core_db; extern crate bytemuck; extern crate serde;
#[path = "topology.rs"] pub mod topology;
#[path = "C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/connectivity.rs"] pub mod connectivity;
pub mod specialized_libs { pub mod computational_geometry { pub mod topology { pub use crate::topology::*; } } }
#[path = "surface_topology_qualification.rs"] mod surface_topology_qualification;
#[path = "micro_public_tests.rs"] mod micro_public_tests;
