//! Turtle Ontology Ingestion & .q42 Volume Compilation
//!
//! Parses Turtle (.ttl) files to 48-byte `NQuin` records and compiles
//! them into deterministic `.q42` Semantic Volumes.

use std::fs;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

use qualia_core_db::q42::q42_volume::write_sorted_quins_volume;
use qualia_core_db::{q_hash, NQuin};
use rio_api::parser::TriplesParser;
use rio_turtle::TurtleParser;

/// Parses Turtle RDF bytes into deterministic NQuins with valid parity checksums.
pub fn parse_turtle_to_nquins(ttl_bytes: &[u8], default_graph_iri: &str) -> io::Result<Vec<NQuin>> {
    let cursor = Cursor::new(ttl_bytes);
    let mut parser = TurtleParser::new(cursor, None);
    let mut quins = Vec::new();
    let graph_context = q_hash(default_graph_iri);

    let mut on_triple = |t: rio_api::model::Triple| -> Result<(), io::Error> {
        let s = q_hash(&t.subject.to_string());
        let p = q_hash(&t.predicate.to_string());
        let o = q_hash(&t.object.to_string());

        let metadata = 0x2000_0000_0000_0000u64;
        let parity = NQuin::calculate_parity(s, p, o, graph_context, metadata);

        quins.push(NQuin {
            subject: s,
            predicate: p,
            object: o,
            context: graph_context,
            metadata,
            parity,
        });

        Ok(())
    };

    parser
        .parse_all(&mut on_triple)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Turtle error: {e}")))?;

    Ok(quins)
}

/// Discovers, parses, and compiles all .ttl files in `ontologies_dir` into `east_coast_strata.q42`.
pub fn compile_all_ontologies(
    ontologies_dir: &Path,
    assets_dir: &Path,
) -> io::Result<(Vec<NQuin>, usize)> {
    let mut ttl_files: Vec<PathBuf> = Vec::new();
    if ontologies_dir.exists() {
        for entry in fs::read_dir(ontologies_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("ttl") {
                ttl_files.push(path);
            }
        }
    }
    ttl_files.sort();

    let mut total_quins = Vec::new();

    for ttl_path in &ttl_files {
        let stem = ttl_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        let file_name = ttl_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown.ttl");
        let graph_iri = format!("urn:qualia:ontology:{stem}");
        let bytes = fs::read(ttl_path)?;
        let mut file_quins = parse_turtle_to_nquins(&bytes, &graph_iri)?;
        println!(
            "  * Parsed {:>4} NQuins from {} (graph: {})",
            file_quins.len(),
            file_name,
            graph_iri
        );
        total_quins.append(&mut file_quins);
    }

    let q42_out_path = assets_dir.join("east_coast_strata.q42");
    if !total_quins.is_empty() {
        let written = write_sorted_quins_volume(&q42_out_path, &total_quins)?;
        println!(
            "[OK] Compiled .q42 Semantic Volume: {} ({} NQuins written across {} ontologies)",
            q42_out_path.display(),
            written,
            ttl_files.len()
        );
    }

    let file_count = ttl_files.len();
    Ok((total_quins, file_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_turtle_to_nquins_with_parity() {
        let turtle_sample = r#"
            @prefix ex: <http://example.org/> .
            ex:Talbingo ex:elevation "385" .
            ex:Talbingo ex:state "NSW" .
        "#;
        let quins = parse_turtle_to_nquins(turtle_sample.as_bytes(), "urn:qualia:test").unwrap();
        assert_eq!(quins.len(), 2);
        for q in &quins {
            assert_eq!(
                q.parity,
                NQuin::calculate_parity(q.subject, q.predicate, q.object, q.context, q.metadata)
            );
        }
    }
}
