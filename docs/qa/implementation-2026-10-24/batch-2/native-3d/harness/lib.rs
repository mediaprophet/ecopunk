//! Exact candidate modules linked against the cached actual canonical core.
//! This does not rebuild the whole core or qualify its current source linkage.
extern crate qualia_core_db;
pub use qualia_core_db::{q_hash, render, specialized_libs, NQuin};

pub mod poet_host {
    pub mod invoke {
        #[path = "C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/invoke/args.rs"]
        pub mod args;
        pub use qualia_core_db::poet_host::invoke::ids;
        #[cfg(not(feature = "baseline"))]
        #[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-24/batch-2/native-3d/candidate/studio_store.rs"]
        pub mod studio_store;
        #[cfg(not(feature = "baseline"))]
        #[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-24/batch-2/native-3d/candidate/studio_domains.rs"]
        pub mod studio_domains;
        #[cfg(feature = "baseline")]
        #[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-24/batch-2/native-3d/originals/studio_store.rs"]
        pub mod studio_store;
        #[cfg(feature = "baseline")]
        #[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-24/batch-2/native-3d/originals/studio_domains.rs"]
        pub mod studio_domains;
    }
}

#[cfg(test)]
mod regression {
    use super::poet_host::invoke::studio_store::*;
    #[test]
    fn undo_restores_source_and_advances_revision() {
        upsert_artifact(StudioArtifact::new("baseline-source-regression", "Geometry", "Original source"));
        let edited = mutate_artifact("baseline-source-regression", |art| art.title = "Edited source".into()).unwrap();
        let undone = undo_artifact("baseline-source-regression").unwrap();
        assert_eq!(undone.title, "Original source");
        assert!(undone.lamport_clock > edited.lamport_clock);
    }
}
