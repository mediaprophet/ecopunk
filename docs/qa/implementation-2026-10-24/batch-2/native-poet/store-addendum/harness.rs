//! Exact staged store source with actual cached core/Vibe types and kernels.
extern crate qualia_core_db;
pub use qualia_core_db::{q_hash, render, specialized_libs, NQuin};
pub mod poet_host {
    pub mod invoke {
        #[path = "C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/args.rs"]
        pub mod args;
        pub use qualia_core_db::poet_host::invoke::ids;
        #[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-24/batch-2/native-poet/store-addendum/candidate/studio_store.rs"]
        pub mod studio_store;
        #[path = "C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/studio_domains.rs"]
        pub mod studio_domains;
    }
}
