//! Bounded source evaluation for editable species review candidates.
//! Reuses Vibe's checked source session. Geometry validation/export happens
//! before promotion; this is not human acceptance or a complete HMC master.
use sha2::{Digest, Sha256};
use vibe::{Budget, DiagCode, Diagnostic, Host, Span, Value};
use vibe::session::{SessionLimits, SourceSession};

pub struct AuthoredSpeciesSource {
    pub data: serde_json::Value,
    pub sources: Vec<String>,
    pub revision: u64,
    pub source_sha256: String,
}

pub struct SpeciesReviewSourceSession {
    session: SourceSession,
}

impl Default for SpeciesReviewSourceSession {
    fn default() -> Self {
        let limits = SessionLimits {
            submission_bytes: 1024 * 1024,
            retained_source_bytes: 8 * 1024 * 1024,
            retained_value_bytes: 16 * 1024 * 1024,
            revisions: 128,
            ..SessionLimits::default()
        };
        Self { session: SourceSession::new(Vec::new(), limits) }
    }
}

impl SpeciesReviewSourceSession {
    pub fn revision(&self) -> u64 { self.session.revision() }
    pub fn sources(&self) -> &[String] { self.session.sources() }

    /// Evaluate a new definition/edit, validate and build the exact candidate,
    /// then retain the source only on success. The builder must be side-effect
    /// free: Vibe does not roll back external effects.
    pub fn submit<T>(
        &mut self,
        source: &str,
        build: impl FnOnce(&AuthoredSpeciesSource) -> Result<T, String>,
    ) -> Result<T, String> {
        if source.len() > 1024 * 1024 {
            return Err("Species source exceeds 1 MiB submission limit".into());
        }
        if self.session.sources().iter().map(String::len).sum::<usize>() + source.len() > 8 * 1024 * 1024 {
            return Err("Species session exceeds 8 MiB retained source limit".into());
        }
        let revision = self.session.revision() + 1;
        let mut sources = self.session.sources().to_vec();
        sources.push(source.to_owned());
        let serialized = serde_json::to_vec(&sources).map_err(|e| e.to_string())?;
        let source_sha256 = format!("{:x}", Sha256::digest(&serialized));
        let mut output = None;
        self.session.submit_validated_bindings(
            source, &mut DataOnlyHost, Budget::default(), |result, bindings, _| {
                let value = bindings.value("species_asset").unwrap_or(result);
                let mut remaining = 250_000usize;
                let data = value_to_json(value, 0, &mut remaining).map_err(diagnostic)?;
                let authored = AuthoredSpeciesSource { data, sources, revision, source_sha256 };
                output = Some(build(&authored).map_err(diagnostic)?);
                Ok(())
            },
        ).map_err(|e| format!("Source revision {} retained: {}", e.active_revision, e.diagnostic.to_json()))?;
        output.ok_or_else(|| "Source validator produced no candidate".into())
    }
}

fn diagnostic(message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(DiagCode::E701, Span::point(0), message.into())
}

fn value_to_json(value: &Value, depth: usize, remaining: &mut usize) -> Result<serde_json::Value, String> {
    if depth > 64 || *remaining == 0 { return Err("Species data exceeds host depth/value limits".into()); }
    *remaining -= 1;
    Ok(match value {
        Value::Null => serde_json::Value::Null,
        Value::Bool(v) => (*v).into(),
        Value::I64(v) => (*v).into(),
        Value::U64(v) => (*v).into(),
        Value::F64(v) => serde_json::Number::from_f64(*v).map(serde_json::Value::Number)
            .ok_or("Species data contains a non-finite number")?,
        Value::String(v) => v.clone().into(),
        Value::List(values) => serde_json::Value::Array(values.iter()
            .map(|v| value_to_json(v, depth + 1, remaining)).collect::<Result<_, _>>()?),
        Value::Record(values) => serde_json::Value::Object(values.iter()
            .map(|(key, v)| Ok((key.clone(), value_to_json(v, depth + 1, remaining)?)))
            .collect::<Result<_, String>>()?),
        _ => return Err("Species definition must contain plain authored data".into()),
    })
}

/// Species data expressions have no graph, publication or engine-effect grants.
/// Geometry is dispatched by the validated adapter after source evaluation.
struct DataOnlyHost;
impl Host for DataOnlyHost {
    fn graph_query(&mut self, _: &[Value], _: u64, _: Span) -> Result<Value, Diagnostic> { Err(diagnostic("Graph queries are not part of species data evaluation")) }
    fn graph_stage(&mut self, _: &Value, _: Span) -> Result<Value, Diagnostic> { Err(diagnostic("Graph writes are not part of species data evaluation")) }
    fn graph_commit(&mut self, _: Span) -> Result<Value, Diagnostic> { Err(diagnostic("Graph commits are not part of species data evaluation")) }
    fn aura_validate(&mut self, _: &Value, _: &Value, _: Span) -> Result<Value, Diagnostic> { Err(diagnostic("Graph validation is not part of species data evaluation")) }
    fn pulse_publish(&mut self, _: &str, _: &Value, _: Span) -> Result<Value, Diagnostic> { Err(diagnostic("Publication is not part of species data evaluation")) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_geometry_does_not_promote_source_or_revision() {
        let mut session = SpeciesReviewSourceSession::default();
        session.submit("let mut species_asset = { radius: 1.0 };", |s| Ok(s.data.clone())).unwrap();
        let rejected = session.submit("species_asset = { radius: -1.0 };", |_| Err::<(), _>("negative radius".into()));
        assert!(rejected.is_err());
        assert_eq!(session.revision(), 1);
        assert_eq!(session.sources().len(), 1);
        let accepted = session.submit("= species_asset", |s| Ok(s.data.clone())).unwrap();
        assert_eq!(accepted["radius"], 1.0);
    }
    #[test]
    fn edits_reuse_the_checked_session() {
        let mut session = SpeciesReviewSourceSession::default();
        session.submit("let mut species_asset = { radius: 1.0 };", |_| Ok(())).unwrap();
        let edited = session.submit("species_asset = { radius: 2.0 };", |s| Ok((s.data.clone(), s.revision, s.sources.len()))).unwrap();
        assert_eq!(edited.0["radius"], 2.0);
        assert_eq!((edited.1, edited.2), (2, 2));
    }
}
