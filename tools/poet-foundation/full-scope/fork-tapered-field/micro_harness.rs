extern crate qualia_core_db;
mod primitives { pub use qualia_core_db::specialized_libs::computational_geometry::Point3; }
mod parametric_cad { pub use qualia_core_db::specialized_libs::computational_geometry::{sdf_eval,SdfOp,SdfPrimitive}; }
#[path = "sdf_ops.rs"] mod sdf_ops;
#[path = "tapered_branch_field.rs"] mod tapered_branch_field;
#[path = "micro_public_tests.rs"] mod micro_public_tests;
