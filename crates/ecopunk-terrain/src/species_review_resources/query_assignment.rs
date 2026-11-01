//! Manual source assignment tied to a recomputed public cohort proposal.
//! The caller supplies the actual edited Vibe source and an explicit semantic
//! mapping declaration. Metres alone never proves radius or biological fitness.
use super::{
    query_estimate::{CohortLimits, CohortRequest, EstimateError},
    SpeciesReviewPackageSession,
};
use crate::species_authored_review::AuthoredReviewAsset;
use serde_json::Value;
#[derive(Debug)]
pub enum AssignmentError {
    Source(String),
    Estimate(EstimateError),
    TargetContract,
}
fn radius(data: &Value, organ: &str) -> Result<f64, String> {
    if !matches!(
        data.get("source_version").and_then(Value::as_str),
        Some(
            "white-gum-authored-review-r2"
                | "white-gum-connected-study-v1"
                | "white-gum-chain-union-v1"
        )
    ) || data.pointer("/botanical/schema").and_then(Value::as_str) != Some("poet.botanical/v2")
    {
        return Err(
            "Manual radius assignment requires the supported white-gum botanical v2 contract"
                .into(),
        );
    }
    let organs = data
        .pointer("/botanical/organs")
        .and_then(Value::as_array)
        .ok_or("Missing authored organ mapping")?;
    let index = organs
        .iter()
        .position(|o| o.get("key").and_then(Value::as_str) == Some(organ))
        .ok_or("Selected authored organ is absent")?;
    data.get("radii_m")
        .and_then(Value::as_array)
        .and_then(|r| r.get(index))
        .and_then(Value::as_f64)
        .ok_or_else(|| "Selected metre-radius source value is absent".into())
}
impl SpeciesReviewPackageSession {
    /// This does not synthesize an edit or claim semantic/source acceptance.
    /// It verifies an explicit manual radius assignment and retains its caller
    /// declaration/proposal digest in ordinary Vibe source history. The existing
    /// full source/model validator still decides whether the edit can promote.
    pub fn submit_manual_radius_assignment(
        &mut self,
        source: &str,
        expected_revision: u64,
        request: CohortRequest,
        organ_key: &str,
        mapping_declaration: &str,
    ) -> Result<AuthoredReviewAsset, AssignmentError> {
        if self.revision() != expected_revision {
            return Err(AssignmentError::Source("Source assignment is stale".into()));
        }
        if source.len() > 1024 * 1024 {
            return Err(AssignmentError::Source(
                "Source exceeds submission budget".into(),
            ));
        }
        if request.unit_iri != "http://qudt.org/vocab/unit/M"
            || organ_key.is_empty()
            || organ_key.len() > 96
            || !organ_key
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-_".contains(&b))
            || mapping_declaration.trim().is_empty()
            || mapping_declaration.len() > 2048
            || mapping_declaration
                .chars()
                .any(|c| c.is_control() || matches!(c, '\u{2028}' | '\u{2029}'))
        {
            return Err(AssignmentError::TargetContract);
        }
        let proposal = self
            .resources()
            .propose_estimate(request, CohortLimits::default())
            .map_err(AssignmentError::Estimate)?;
        let value = proposal.value();
        let digest = proposal
            .cohort_sha256()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        let request = proposal.request();
        let resource_sha = request
            .resource_sha256
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        let metadata_sha = self
            .resources()
            .entries()
            .iter()
            .find(|r| r.key == request.resource_key)
            .and_then(|r| r.summary().metadata_digest)
            .map(|d| d.iter().map(|b| format!("{b:02x}")).collect::<String>())
            .unwrap_or_else(|| "none".into());
        let note=format!("// Unadopted public Q42 estimate proposal {digest}\n// Resource {}; SHA256 {resource_sha}; opaque metadata SHA256 {metadata_sha}\n// Method {}; declared unit {}; source metre-radius organ {}\n// Value predicate {:016x}; unit predicate {:016x}; subject filter {:?}; context filter {:?}\n// Caller semantic mapping (not authority or scientific acceptance): {}\n",
            request.resource_key,proposal.method().identifier(),proposal.unit_iri(),organ_key,request.value_predicate,request.unit_predicate,request.subject,request.context,mapping_declaration);
        if note
            .len()
            .checked_add(source.len())
            .is_none_or(|n| n > 1024 * 1024)
        {
            return Err(AssignmentError::Source(
                "Annotated source exceeds submission budget".into(),
            ));
        }
        let source = format!("{note}{source}");
        let organ = organ_key.to_owned();
        self.submit_checked(&source, expected_revision, move |authored| {
            let assigned = radius(&authored.data, &organ)?;
            if assigned != value {
                return Err(
                    "Manual source radius does not equal the exact recomputed estimate proposal"
                        .into(),
                );
            }
            Ok(())
        })
        .map_err(AssignmentError::Source)
    }
}
