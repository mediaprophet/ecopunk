//! Staged production store/journal/checkpoint source linked to actual cached
//! Qualia bundle and render types. No mock HMC or CompiledAsset implementation.
extern crate qualia_core_db;
pub use qualia_core_db::{q_hash, render, bundle, specialized_libs, q42_volume, NQuin, QUINS_PER_BLOCK};
pub mod poet_host {
    pub mod invoke {
        pub use qualia_core_db::poet_host::invoke::ids;
        #[path = "C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/args.rs"]
        pub mod args;
        #[path = "C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/studio_domains.rs"]
        pub mod studio_domains;
        #[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3/hmc/candidate/studio_store.rs"]
        pub mod studio_store;
    }
}
