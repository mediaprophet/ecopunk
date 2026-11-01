//! Declared arithmetic review proposals over complete public Q42 cohorts.
//! No premise becomes adopted truth; uncertainty/conflicts are not zero-filled.
use super::{
    query::{FactCursor, FactPattern, FactView, QueryError, QueryLimits},
    ReviewResources,
};
use qualia_core_db::{resolver::InlineLiteral, solvers::statistics::descriptive};
use sha2::{Digest, Sha256};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EstimateMethod {
    Mean,
    Median,
}
impl EstimateMethod {
    pub fn identifier(self) -> &'static str {
        match self {
            Self::Mean => "Statistics.arithmetic_mean/v1",
            Self::Median => "Statistics.median/v1",
        }
    }
}
#[derive(Clone, Debug)]
pub struct CohortRequest {
    pub resource_key: String,
    pub resource_sha256: [u8; 32],
    pub subject: Option<u64>,
    pub context: Option<u64>,
    pub value_predicate: u64,
    pub unit_predicate: u64,
    pub unit_iri: String,
    pub method: EstimateMethod,
}
#[derive(Clone, Copy, Debug)]
pub struct CohortLimits {
    pub max_values: usize,
    pub max_unit_facts: usize,
    pub max_pages: usize,
    pub retained_bytes: usize,
    pub query: QueryLimits,
}
impl Default for CohortLimits {
    fn default() -> Self {
        Self {
            max_values: 256,
            max_unit_facts: 512,
            max_pages: 32,
            retained_bytes: 2 * 1024 * 1024,
            query: QueryLimits::default(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EstimateError {
    Query(QueryError),
    Incomplete,
    Budget,
    Empty,
    UnknownPremise,
    NonNumeric,
    Unit,
    ConflictingPremises,
}
/// Immutable computed proposal. Public callers can inspect, never deserialize
/// or edit this into an authorization/accepted-measurement record.
pub struct EstimateProposal {
    pub(super) request: CohortRequest,
    pub(super) value: f64,
    pub(super) digest: [u8; 32],
    values: Vec<FactView>,
    units: Vec<FactView>,
}
impl EstimateProposal {
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn method(&self) -> EstimateMethod {
        self.request.method
    }
    pub fn unit_iri(&self) -> &str {
        &self.request.unit_iri
    }
    pub fn cohort_sha256(&self) -> [u8; 32] {
        self.digest
    }
    pub fn request(&self) -> &CohortRequest {
        &self.request
    }
    pub fn value_premises(&self) -> &[FactView] {
        &self.values
    }
    pub fn unit_premises(&self) -> &[FactView] {
        &self.units
    }
}
fn charge(rows: &[FactView]) -> Option<usize> {
    rows.iter().try_fold(0usize, |n, r| {
        let mut bytes = std::mem::size_of::<FactView>();
        for term in [&r.subject, &r.predicate, &r.object, &r.context] {
            bytes = bytes
                .checked_add(term.lexical.as_ref().map_or(0, String::len))?
                .checked_add(term.namespace.as_ref().map_or(0, String::len))?;
        }
        n.checked_add(bytes)
    })
}
fn collect(
    resources: &ReviewResources,
    r: &CohortRequest,
    predicate: u64,
    cap: usize,
    limits: CohortLimits,
) -> Result<Vec<FactView>, EstimateError> {
    let mut cursor = FactCursor::default();
    let mut rows = Vec::new();
    let mut used = 0usize;
    for _ in 0..limits.max_pages {
        let page = resources
            .inspect_page(
                &r.resource_key,
                r.resource_sha256,
                FactPattern {
                    subject: r.subject,
                    predicate: Some(predicate),
                    object: None,
                    context: r.context,
                },
                cursor,
                limits.query,
            )
            .map_err(EstimateError::Query)?;
        if rows
            .len()
            .checked_add(page.rows.len())
            .is_none_or(|n| n > cap)
        {
            return Err(EstimateError::Budget);
        }
        used = used
            .checked_add(charge(&page.rows).ok_or(EstimateError::Budget)?)
            .ok_or(EstimateError::Budget)?;
        if used > limits.retained_bytes {
            return Err(EstimateError::Budget);
        }
        rows.extend(page.rows);
        match page.next {
            None => return Ok(rows),
            Some(next) => cursor = next,
        }
    }
    Err(EstimateError::Incomplete)
}
impl ReviewResources {
    /// Unit relation is explicit and exact: each measurement subject/context
    /// must have one unit assertion under the selected predicate, resolving to
    /// the declared IRI. No label/hash/scale equivalence or units is invented.
    pub fn propose_estimate(
        &self,
        request: CohortRequest,
        limits: CohortLimits,
    ) -> Result<EstimateProposal, EstimateError> {
        if request.value_predicate == request.unit_predicate
            || request.unit_iri.is_empty()
            || request.unit_iri.len() > 2048
            || !request.unit_iri.contains(':')
            || request
                .unit_iri
                .chars()
                .any(|c| c.is_control() || c.is_whitespace())
        {
            return Err(EstimateError::Unit);
        }
        let values = collect(
            self,
            &request,
            request.value_predicate,
            limits.max_values,
            limits,
        )?;
        let units = collect(
            self,
            &request,
            request.unit_predicate,
            limits.max_unit_facts,
            limits,
        )?;
        let bytes = charge(&values)
            .and_then(|n| n.checked_add(charge(&units)?))
            .and_then(|n| n.checked_add(values.len().checked_mul(8)?))
            .ok_or(EstimateError::Budget)?;
        if bytes > limits.retained_bytes {
            return Err(EstimateError::Budget);
        }
        if values.is_empty() {
            return Err(EstimateError::Empty);
        }
        let mut numbers = Vec::with_capacity(values.len());
        for (i, fact) in values.iter().enumerate() {
            let q = fact.raw_quin();
            if q.get_quin_type() != 0
                || fact.predicate.lexical.is_none()
                || fact.subject.lexical.is_none()
                || (fact.words[3] != 0 && fact.context.lexical.is_none())
            {
                return Err(EstimateError::UnknownPremise);
            }
            // Distinct measurement subjects/contexts are the declared cohort.
            // Duplicate/conflicting assertions are not silently averaged away.
            if values[..i]
                .iter()
                .any(|old| old.words[0] == fact.words[0] && old.words[3] == fact.words[3])
            {
                return Err(EstimateError::ConflictingPremises);
            }
            let mut matches = units
                .iter()
                .filter(|u| u.words[0] == fact.words[0] && u.words[3] == fact.words[3]);
            let unit = matches.next().ok_or(EstimateError::Unit)?;
            if matches.next().is_some() {
                return Err(EstimateError::ConflictingPremises);
            }
            if unit.raw_quin().get_quin_type() != 0
                || unit.predicate.lexical.is_none()
                || unit.object.lexical.as_deref() != Some(request.unit_iri.as_str())
            {
                return Err(EstimateError::Unit);
            }
            let number = match fact.literal {
                Some(InlineLiteral::Integer(n)) if n.unsigned_abs() <= 9_007_199_254_740_992 => {
                    n as f64
                }
                Some(InlineLiteral::Decimal(n)) if n.unsigned_abs() <= 9_007_199_254_740_992 => {
                    n as f64 / 1_000_000.0
                }
                Some(InlineLiteral::Float(n)) => f64::from(n),
                _ => return Err(EstimateError::NonNumeric),
            };
            if !number.is_finite() {
                return Err(EstimateError::NonNumeric);
            }
            numbers.push(number);
        }
        let value = match request.method {
            EstimateMethod::Mean => descriptive::mean(&numbers),
            EstimateMethod::Median => descriptive::median_in_place(&mut numbers),
        }
        .ok_or(EstimateError::Empty)?;
        if !value.is_finite() {
            return Err(EstimateError::NonNumeric);
        }
        let mut hash = Sha256::new();
        hash.update(b"ecopunk.public-cohort-proposal/1");
        hash.update(request.resource_sha256);
        for option in [request.subject, request.context] {
            match option {
                Some(value) => {
                    hash.update([1]);
                    hash.update(value.to_le_bytes());
                }
                None => hash.update([0]),
            }
        }
        hash.update(request.value_predicate.to_le_bytes());
        hash.update(request.unit_predicate.to_le_bytes());
        for text in [
            request.resource_key.as_str(),
            request.method.identifier(),
            request.unit_iri.as_str(),
            "distinct-measurement-subject-context/refuse-conflicts/v1",
        ] {
            hash.update((text.len() as u64).to_le_bytes());
            hash.update(text.as_bytes());
        }
        for group in [&values, &units] {
            hash.update((group.len() as u64).to_le_bytes());
            for fact in group {
                for word in fact.words {
                    hash.update(word.to_le_bytes());
                }
            }
        }
        Ok(EstimateProposal {
            request,
            value,
            digest: hash.finalize().into(),
            values,
            units,
        })
    }
}
