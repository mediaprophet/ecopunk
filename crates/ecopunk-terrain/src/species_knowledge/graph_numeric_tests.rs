//! Prepared source tests; run only after the selected coding batch is complete.
use super::*;
use qualia_core_db::{q42_lex::Q42LexMmap, resolver::typed_double};

#[test]
fn adjacent_binary64_terms_survive_native_q42lex() {
    let mut graph=Graph::new(KnowledgeLimits::default());
    let values=[1.0,f64::from_bits(1.0f64.to_bits()+1),-0.0,f64::from_bits(1),f64::MAX];
    for value in values {graph.number_f64("urn:study","urn:value",value).unwrap();}
    let bytes=qualia_core_db::q42_lex::serialize_string_lexicon(&graph.lex).unwrap();
    let lex=Q42LexMmap::from_bytes(&bytes).unwrap();
    for (fact,value) in graph.facts.iter().zip(values) {
        let mut term=String::new();assert!(lex.resolve_term_into(fact.object,&mut term));
        assert_eq!(typed_double::decode(&term).unwrap().1.to_bits(),value.to_bits());
        assert!(qualia_core_db::resolver::classify_inline_literal(fact.object).is_none());
    }
    assert_ne!(graph.facts[0].object,graph.facts[1].object);
}

#[test]
fn binary64_rejects_nonfinite_collision_and_lexicon_budget() {
    let mut graph=Graph::new(KnowledgeLimits::default());
    assert!(graph.number_f64("urn:s","urn:p",f64::NAN).is_err());
    assert!(graph.facts.is_empty());assert!(graph.lex.is_empty());
    let token=typed_double::token(&typed_double::from_f64(1.0).unwrap());
    graph.lex.insert(token,"urn:unrelated".into());
    assert!(graph.number_f64("urn:s","urn:p",1.0).is_err());
    assert!(graph.facts.is_empty());assert_eq!(graph.lex[&token],"urn:unrelated");
    let mut limits=KnowledgeLimits::default();limits.lexicon_bytes=0;
    let mut graph=Graph::new(limits);
    assert!(graph.number_f64("urn:s","urn:p",1.0).is_err());
    assert!(graph.lex.is_empty());assert!(graph.facts.is_empty());
}

#[test]
fn finite_binary64_fact_uses_native_volume_and_public_lexical_inspection() {
    use crate::species_review_resources::query::{FactCursor,FactPattern,QueryLimits};
    let limits=KnowledgeLimits::default();let mut graph=Graph::new(limits);
    let value=f64::from_bits(1.0f64.to_bits()+1);
    graph.number_f64("urn:s","urn:p",value).unwrap();
    let resource=super::super::finish(graph,limits,&AtomicBool::new(false)).unwrap();
    let digest=sha2::Sha256::digest(&resource.bytes).into();
    let resources=ReviewResources::admit(vec![resource],ReviewResourceLimits::default()).unwrap();
    let page=resources.inspect_page(RESOURCE_KEY,digest,FactPattern::default(),FactCursor::default(),QueryLimits::default()).unwrap();
    assert_eq!(page.rows.len(),1);assert!(page.rows[0].literal.is_none());
    let term=page.rows[0].object.lexical.as_deref().unwrap();
    assert_eq!(typed_double::decode(term).unwrap().1.to_bits(),value.to_bits());
    assert_eq!(term,typed_double::from_f64(value).unwrap());
}

#[test]
fn public_core_ingest_dictionary_and_query_metadata_keep_binary64() {
    use qualia_core_db::{query::{ingest_formats::parse_bytes_to_raw,ingest_job::IngestRdfFormat},sparql_ast::{LiteralTable,TextResolver,literal_term_hash}};
    for value in [1.0,f64::from_bits(1.0f64.to_bits()+1),-0.0,f64::from_bits(1),f64::MAX] {
        let term=typed_double::from_f64(value).unwrap();
        let input=format!("<urn:s> <urn:p> {term} .\n");
        let raw=parse_bytes_to_raw(IngestRdfFormat::NTriples,input.as_bytes()).unwrap();
        assert_eq!(raw.len(),1);assert!(raw[0].packed_object.is_none());assert_eq!(raw[0].object,term);
        let hash=typed_double::token(&term);let lexical=typed_double::decode(&term).unwrap().0;
        assert_eq!(hash,literal_term_hash(lexical,None,Some(typed_double::DATATYPE)));
        let mut terms=std::collections::HashMap::new();terms.insert(hash,term);
        let bytes=qualia_core_db::q42_lex::serialize_string_lexicon(&terms).unwrap();
        let lex=Q42LexMmap::from_bytes(&bytes).unwrap();
        let lookup=|key|lex.lookup_hash(key).map(str::to_owned);let literals=LiteralTable::new();
        let resolver=TextResolver::with_lexicon(&literals,&lookup);
        assert_eq!(resolver.datatype_of(hash).as_deref(),Some(typed_double::DATATYPE));
        assert_eq!(resolver.lang_of(hash).as_deref(),Some(""));
        assert_eq!(resolver.resolve_text(hash).unwrap().parse::<f64>().unwrap().to_bits(),value.to_bits());
    }
    let json=br#"{"urn:s":{"urn:p":[{"type":"literal","value":"1.0000000000000002","datatype":"http://www.w3.org/2001/XMLSchema#double"}]}}"#;
    let raw=parse_bytes_to_raw(IngestRdfFormat::RdfJson,json).unwrap();
    assert_eq!(typed_double::decode(&raw[0].object).unwrap().1.to_bits(),1.0000000000000002f64.to_bits());
    for lexical in ["INF","NaN","illtyped"] {
        let input=format!("<urn:s> <urn:p> \"{lexical}\"^^<{}> .",typed_double::DATATYPE);
        let raw=parse_bytes_to_raw(IngestRdfFormat::NTriples,input.as_bytes()).unwrap();
        assert!(raw[0].packed_object.is_none());assert!(typed_double::decode(&raw[0].object).is_none());
        let hash=typed_double::token(&raw[0].object);let lookup=|key|(key==hash).then(||raw[0].object.clone());
        let literals=LiteralTable::new();let resolver=TextResolver::with_lexicon(&literals,&lookup);
        assert_eq!(resolver.datatype_of(hash).as_deref(),Some(typed_double::DATATYPE));
        assert_eq!(resolver.lang_of(hash).as_deref(),Some(""));
    }
}

#[test]
fn public_cbor_token_only_path_refuses_binary64_without_a_lexical_owner() {
    use ciborium::value::Value as C;
    struct Sink;
    impl qualia_core_db::sparql_library::quin_sink::QuinSink for Sink {
        fn push(&mut self,_:qualia_core_db::NQuin)->std::io::Result<()> {Ok(())}
    }
    let pair=|key:&str,value:C|(C::Text(key.into()),value);
    let literal=C::Map(vec![pair("@value",C::Text("1.0000000000000002".into())),pair("@type",C::Text(typed_double::DATATYPE.into()))]);
    let object=C::Map(vec![pair("@id",C::Text("urn:s".into())),pair("urn:p",literal)]);
    let mut bytes=Vec::new();ciborium::ser::into_writer(&C::Array(vec![object]),&mut bytes).unwrap();
    let error=qualia_core_db::sparql_library::parsers::cbor_parser::parse_cbor_ld_star_sink(&bytes,0,&mut Sink).unwrap_err();
    assert!(error.to_string().contains("typed double requires a retained lexical owner"));
}
