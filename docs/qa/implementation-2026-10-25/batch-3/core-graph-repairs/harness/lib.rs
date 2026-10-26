//! Exact candidate adapter/parser/accessor sources against real cached native
//! Q42 codecs, AST/planner/executor, StudioArtifact and Rio ingest. No mock core.
extern crate qualia_core_db as canonical_core;
pub use canonical_core::*;
pub mod q42 {
    pub use canonical_core::q42::*;
    pub use crate::candidate_lexicon as q42_lexicon;
}

#[path = "C:/github/QualiaDB/crates/qualia-core-db/src/sparql_library/sparql_parser.rs"]
pub mod sparql_parser;
pub mod sparql_library {
    pub use canonical_core::sparql_library::*;
    pub use crate::sparql_parser;
    pub mod sparql_grammar {
        pub use canonical_core::sparql_library::sparql_grammar::{expr, tokenizer, update};
        pub use expr::parse_expression;
        pub use tokenizer::{tokenize, Token};
        pub use update::{is_update, parse_update};
        #[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3/core-graph-repairs/candidate/src/sparql_library/sparql_grammar/pattern.rs"]
        pub mod pattern;
        pub use pattern::parse_where_group;
    }
    #[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3/core-graph-repairs/candidate/src/sparql_library/sparql_mm.rs"]
    pub mod sparql_mm;
}
#[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3/core-graph-repairs/candidate/src/q42/q42_lexicon.rs"]
pub mod candidate_lexicon;
#[path = "C:/github/QualiaDB/crates/qualia-core-db/src/services/daemon_query.rs"]
pub mod daemon_query;
pub mod studio_store {
    pub use canonical_core::poet_host::invoke::studio_store::StudioArtifact;
    #[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3/core-graph-repairs/candidate/src/poet_host/invoke/studio_cml/mod.rs"]
    pub mod cml;
}
pub mod solid_ldp {
    mod bundle { pub use canonical_core::services::solid_ldp::{build_migration_bundle, SolidMigrationOptions}; }
    mod consent { pub use canonical_core::services::solid_ldp::{plan_sanctuary_migration_notice, SanctuaryExportChoice, SanctuaryMigrationNotice}; }
    #[path = "C:/github/QualiaDB/crates/qualia-core-db/src/services/solid_ldp/export_q42.rs"]
    pub mod export_q42;
}

#[cfg(test)]
mod graph_repairs {
    #[test]
    fn native_rio_default_ingest_keeps_lexical_source_without_licence_inference() {
        let directory = tempfile::tempdir().unwrap();
        let input = directory.path().join("source.ttl");
        let output = directory.path().join("source.q42");
        std::fs::write(&input, "@prefix dc: <http://purl.org/dc/terms/> .\n@prefix ns1: <http://purl.org/ontology/mo/> .\n@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n<http://purl.org/ontology/mo/Track> rdfs:isDefinedBy ns1: .\n<http://purl.org/ontology/mo/> dc:title \"The Music Ontology\" .\n").unwrap();
        canonical_core::query::ingest::streaming_import_rdf(input.to_str().unwrap(), output.to_str().unwrap()).unwrap();
        let report = canonical_core::q42_volume::Q42InspectReport::from_path(&output).unwrap();
        assert!(!report.lexicon_has_no_terms);
        assert_eq!(report.flags & canonical_core::q42_volume::FLAG_PERMISSIVE_COMMONS, 0);
        let volume = canonical_core::q42_volume::Q42Volume::open(&output).unwrap();
        let terms = crate::candidate_lexicon::Q42Lexicon::from_volume(&volume).unwrap();
        assert!(terms.reverse.values().any(|term| term == "http://purl.org/ontology/mo/"));
        assert!(terms.reverse.values().any(|term| term.contains("The Music Ontology")));
    }
}
