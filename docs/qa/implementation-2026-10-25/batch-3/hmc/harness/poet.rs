//! Exact staged native service + store owners, actual cached codec/render types.
extern crate qualia_core_db as canonical_core;
extern crate self as qualia_core_db;
pub use canonical_core::{q_hash, render, bundle, specialized_libs, q42_volume, NQuin, QUINS_PER_BLOCK};
pub mod poet_host {
    pub mod invoke {
        pub use canonical_core::poet_host::invoke::ids;
        #[path = "C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/args.rs"]
        pub mod args;
        #[path = "C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/studio_domains.rs"]
        pub mod studio_domains;
        #[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3/hmc/candidate/studio_store.rs"]
        pub mod studio_store;
    }
}
#[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3/hmc/harness/service-only.rs"]
pub mod checkpoints;

