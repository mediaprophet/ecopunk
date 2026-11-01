use super::*;
use qualia_core_db::{q_hash, NQuin};
use sha2::Digest;
use std::collections::{BTreeSet, HashMap};
const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

const PROV: &str = "http://www.w3.org/ns/prov#";
const DC: &str = "http://purl.org/dc/terms/";
pub(super) struct Graph {
    pub lex: HashMap<u64, String>,
    pub facts: Vec<NQuin>,
    limits: KnowledgeLimits,
    lex_bytes: usize,
    pub restricted: bool,
    inline_objects: BTreeSet<u64>,
}
impl Graph {
    pub fn new(limits: KnowledgeLimits) -> Self {
        Self {
            lex: HashMap::new(),
            facts: Vec::new(),
            limits,
            lex_bytes: 0,
            restricted: false,
            inline_objects: BTreeSet::new(),
        }
    }
    pub(super) fn id(&mut self, iri: &str) -> Result<u64, String> {
        if iri.len() > 4096
            || !(iri.starts_with("urn:")
                || iri.starts_with("https://")
                || iri.starts_with("http://"))
            || iri
                .chars()
                .any(|c| c.is_control() || c.is_whitespace() || "<>\"{}|\\^`".contains(c))
            || iri == "urn:"
            || iri.ends_with("://")
        {
            return Err("Knowledge term is not an admitted IRI".into());
        }
        for (at, byte) in iri.bytes().enumerate() {
            if byte == b'%'
                && (at + 2 >= iri.len()
                    || !iri.as_bytes()[at + 1..at + 3]
                        .iter()
                        .all(u8::is_ascii_hexdigit))
            {
                return Err("Knowledge term contains malformed percent encoding".into());
            }
        }
        let id = q_hash(iri);
        if self.inline_objects.contains(&id) {
            return Err(
                "Knowledge IRI collides with an inline object; no equivalence inferred".into(),
            );
        }
        if id == 0 {
            return Err("Knowledge IRI has zero identifier".into());
        }
        if let Some(old) = self.lex.get(&id) {
            if old != iri {
                return Err("Knowledge identifier collision; no equivalence inferred".into());
            }
            return Ok(id);
        }
        self.lex_bytes = self
            .lex_bytes
            .checked_add(iri.len())
            .ok_or("Knowledge lexicon size overflow")?;
        if self.lex_bytes > self.limits.lexicon_bytes {
            return Err("Knowledge lexicon exceeds caller budget".into());
        }
        self.lex
            .try_reserve(1)
            .map_err(|_| "Knowledge lexicon allocation refused")?;
        self.lex.insert(id, iri.into());
        Ok(id)
    }
    pub(super) fn triple(&mut self, s: &str, p: &str, o: &str) -> Result<(), String> {
        let object = self.id(o)?;
        self.object(s, p, object)
    }
    fn object(&mut self, s: &str, p: &str, o: u64) -> Result<(), String> {
        if self.facts.len() >= self.limits.statements {
            return Err("Knowledge statement cap exceeded".into());
        }
        let (s, p) = (self.id(s)?, self.id(p)?);
        self.facts
            .try_reserve(1)
            .map_err(|_| "Knowledge statement allocation refused")?;
        let mut record = NQuin {
            subject: s,
            predicate: p,
            object: o,
            context: 0,
            metadata: 0,
            parity: NQuin::calculate_parity(s, p, o, 0, 0),
        };
        if self.restricted {
            record.set_sensitivity_byte(NQuin::SENSITIVITY_RESTRICTED);
        }
        record.parity = NQuin::calculate_parity(
            record.subject,
            record.predicate,
            record.object,
            record.context,
            record.metadata,
        );
        self.facts.push(record);
        Ok(())
    }
    pub(super) fn number_f32(&mut self, s: &str, p: &str, value: f32) -> Result<(), String> {
        if !value.is_finite() {
            return Err("Nonfinite authored numeric value".into());
        }
        self.inline(
            s,
            p,
            &value.to_string(),
            "http://www.w3.org/2001/XMLSchema#float",
        )
    }
    /// Native Q42LEX typed xsd:double; never route binary64 through f32 inline.
    pub(super) fn number_f64(&mut self, s: &str, p: &str, value: f64) -> Result<(), String> {
        use qualia_core_db::resolver::typed_double;
        let term = typed_double::from_f64(value)
            .ok_or("Nonfinite authored binary64 numeric value")?;
        let object = typed_double::token(&term);
        if object == 0 || self.inline_objects.contains(&object) {
            return Err("Typed double token collides with a reserved object".into());
        }
        if let Some(old) = self.lex.get(&object) {
            if old != &term {
                return Err("Typed double lexicon collision; no equivalence inferred".into());
            }
        } else {
            let next = self.lex_bytes.checked_add(term.len())
                .ok_or("Knowledge typed double lexicon size overflow")?;
            if next > self.limits.lexicon_bytes {
                return Err("Knowledge typed double exceeds lexicon budget".into());
            }
            self.lex.try_reserve(1)
                .map_err(|_| "Knowledge typed double allocation refused")?;
            self.lex.insert(object, term);
            self.lex_bytes = next;
        }
        self.object(s, p, object)
    }
    pub(super) fn integer(&mut self, s: &str, p: &str, value: usize) -> Result<(), String> {
        self.inline(
            s,
            p,
            &value.to_string(),
            "http://www.w3.org/2001/XMLSchema#integer",
        )
    }
    pub(super) fn boolean(&mut self, s: &str, p: &str, value: bool) -> Result<(), String> {
        self.inline(
            s,
            p,
            if value { "true" } else { "false" },
            "http://www.w3.org/2001/XMLSchema#boolean",
        )
    }
    fn inline(&mut self, s: &str, p: &str, lexical: &str, datatype: &str) -> Result<(), String> {
        let object = qualia_core_db::resolver::pack_typed_literal(lexical, datatype)
            .ok_or("Authored numeric value has no admitted canonical inline encoding")?;
        if self.lex.contains_key(&object) {
            return Err("Inline object collides with retained lexicon IRI".into());
        }
        self.inline_objects.insert(object);
        self.object(s, p, object)
    }
}
#[cfg(test)]
#[path = "graph_numeric_tests.rs"]
mod numeric_tests;
pub(super) fn term(local: &str) -> String {
    format!("{NAMESPACE}{local}")
}
fn hex_digest(v: &str) -> bool {
    v.len() == 64 && v.bytes().all(|b| b.is_ascii_hexdigit())
}
pub(super) fn escaped(value: &str) -> String {
    let mut s = String::new();
    for b in value.bytes() {
        if b.is_ascii_alphanumeric() || b"/._-".contains(&b) {
            s.push(b as char);
        } else {
            s.push_str(&format!("%{b:02X}"));
        }
    }
    s
}
pub(super) fn entry(study: &str, key: &str) -> String {
    format!("{study}:entry:{}", escaped(key))
}
pub(super) fn populate(
    g: &mut Graph,
    index: &PackageReviewIndex,
    sources: &[SourceNamespaceRef<'_>],
    cancel: &AtomicBool,
) -> Result<(), String> {
    if !hex_digest(&index.package_sha256)
        || !hex_digest(&index.source_sha256)
        || !hex_digest(&index.catalogue.sha256)
        || index.resources.len() > 165
        || index.models.len() > 32
        || index.coverage.len() > 256
        || sources.len() > 32
        || index
            .source_revision
            .parse::<u64>()
            .ok()
            .is_none_or(|v| v == 0 || v > 128)
    {
        return Err("Knowledge input is outside exact native review profile".into());
    }
    parse_ontology(g)?;
    let ontology = term("Ontology");
    g.triple(
        &ontology,
        &format!("{PROV}wasDerivedFrom"),
        &format!(
            "urn:sha256:{:x}",
            sha2::Sha256::digest(ONTOLOGY_TTL.as_bytes())
        ),
    )?;
    let study = format!(
        "urn:ecopunk:species-study-snapshot:{}",
        index.package_sha256
    );
    g.triple(&study, &format!("{RDF}type"), &term("SpeciesStudy"))?;
    g.triple(
        &study,
        &term("claimStatus"),
        &term("AuthoredStudyUnaccepted"),
    )?;
    g.triple(
        &study,
        &format!("{PROV}wasDerivedFrom"),
        &format!("urn:sha256:{}", index.package_sha256),
    )?;
    g.triple(
        &study,
        &format!("{DC}conformsTo"),
        &format!("urn:sha256:{}", index.catalogue.sha256),
    )?;
    g.triple(
        &study,
        &format!("{PROV}wasDerivedFrom"),
        &format!(
            "urn:ecopunk:vibe-history:{}:revision:{}",
            index.source_sha256, index.source_revision
        ),
    )?;
    let mut resources = BTreeSet::new();
    for r in &index.resources {
        if cancel.load(Ordering::Acquire) {
            return Err("Knowledge generation cancelled".into());
        }
        if r.key.len() > 1024
            || r.kind.is_empty()
            || r.kind.len() > 64
            || !hex_digest(&r.sha256)
            || !resources.insert(r.key.as_str())
        {
            return Err("Knowledge resource key/kind/digest refused".into());
        }
        let resource = entry(&study, &r.key);
        g.triple(&study, &term("hasResource"), &resource)?;
        g.triple(&resource, &format!("{RDF}type"), &term("AssetResource"))?;
        g.triple(
            &resource,
            &format!("{PROV}wasDerivedFrom"),
            &format!("urn:sha256:{}", r.sha256),
        )?;
        g.triple(
            &resource,
            &format!("{DC}format"),
            &format!("urn:ecopunk:media-kind:{}", escaped(&r.kind)),
        )?;
    }
    for model in &index.models {
        if !resources.contains(model.key.as_str()) || model.clips.len() > 32 {
            return Err("Knowledge model is not bound to retained resource".into());
        }
        let model_iri = entry(&study, &model.key);
        for c in &model.clips {
            let id =
                c.id.parse::<u64>()
                    .map_err(|_| "Knowledge clip identifier invalid")?;
            if id == 0 {
                return Err("Knowledge clip identifier zero".into());
            }
            let clip = format!("{model_iri}:clip:{id}");
            g.triple(&model_iri, &term("hasClip"), &clip)?;
            g.triple(&clip, &format!("{RDF}type"), &term("AuthoredClip"))?;
            g.triple(
                &clip,
                &term("claimStatus"),
                &term("AuthoredStudyUnaccepted"),
            )?;
            if let Some(name) = &c.source_name {
                if name.len() > 128 {
                    return Err("Knowledge clip source name exceeds limit".into());
                }
                g.triple(
                    &clip,
                    &format!("{DC}identifier"),
                    &format!("urn:ecopunk:authored-clip-name:{}", escaped(name)),
                )?;
            }
        }
    }
    for (i, c) in index.coverage.iter().enumerate() {
        let claim = format!("{study}:coverage:{i}");
        let status = match c.status {
            "declared_unaccepted" => "DeclaredUnaccepted",
            "missing_or_unassessed" => "MissingOrUnassessed",
            _ => return Err("Unknown native review coverage status".into()),
        };
        g.triple(&study, &term("hasCoverageClaim"), &claim)?;
        g.triple(&claim, &format!("{RDF}type"), &term("CoverageClaim"))?;
        g.triple(&claim, &term("coverageStatus"), &term(status))?;
        g.triple(
            &claim,
            &format!("{DC}subject"),
            &format!(
                "urn:ecopunk:review-dimension:{}:{}",
                escaped(c.dimension),
                escaped(c.value)
            ),
        )?;
        for id in &c.catalogue_ids {
            g.triple(
                &claim,
                &format!("{DC}conformsTo"),
                &format!(
                    "urn:ecopunk:model-deliverable:{}:{}",
                    escaped(&index.catalogue.version),
                    escaped(id)
                ),
            )?;
        }
    }
    // The graph defines its own terms and namespace; external source links remain
    // explicit REFERENCE-ONLY declarations, not invented imported RDF assertions.
    let own = term("NativeOntologyNamespace");
    g.triple(
        &own,
        &format!("{RDF}type"),
        &term("SourceNamespaceDeclaration"),
    )?;
    g.triple(&own, &term("sourceNamespace"), NAMESPACE)?;
    g.triple(&own, &format!("{DC}source"), &ontology)?;
    for (i, s) in sources.iter().enumerate() {
        let record = format!("{study}:source-reference:{i}");
        g.triple(&study, &format!("{DC}references"), &record)?;
        g.triple(
            &record,
            &format!("{RDF}type"),
            &term("SourceNamespaceDeclaration"),
        )?;
        g.triple(&record, &term("claimStatus"), &term("ReferenceOnly"))?;
        g.triple(&record, &format!("{DC}source"), s.source_iri)?;
        g.triple(&record, &term("sourceNamespace"), s.namespace_iri)?;
        g.triple(&record, &format!("{DC}format"), &term(s.format.term()))?;
    }
    Ok(())
}

fn parse_ontology(graph: &mut Graph) -> Result<(), String> {
    use rio_api::{
        model::{Subject, Term},
        parser::TriplesParser,
    };
    let mut parser =
        rio_turtle::TurtleParser::new(std::io::Cursor::new(ONTOLOGY_TTL.as_bytes()), None);
    let mut emit = |triple: rio_api::model::Triple<'_>| -> Result<(), std::io::Error> {
        let Subject::NamedNode(subject) = triple.subject else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Technical ontology requires named subjects",
            ));
        };
        let Term::NamedNode(object) = triple.object else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Technical ontology requires named objects",
            ));
        };
        graph
            .triple(subject.iri, triple.predicate.iri, object.iri)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    };
    parser
        .parse_all(&mut emit)
        .map_err(|e| format!("Existing RDF ontology parser: {e}"))
}
