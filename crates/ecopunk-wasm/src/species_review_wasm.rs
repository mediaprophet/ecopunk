//! Thin WASM projection of existing source, resource and .10d owners.
use ecopunk_terrain::species_review_resources::{
    ResourceData, ReviewResourceLimits, SpeciesReviewPackageSession,
};
use wasm_bindgen::prelude::*;

/// Retains admitted native resources through ordinary source edits and reopen.
#[wasm_bindgen]
pub struct SpeciesReviewSession {
    source: SpeciesReviewPackageSession,
}
#[wasm_bindgen]
impl SpeciesReviewSession {
    #[wasm_bindgen(constructor)]
    pub fn new(subject: u8) -> Result<SpeciesReviewSession, JsValue> {
        Ok(Self {
            source: SpeciesReviewPackageSession::new(subject, ReviewResourceLimits::default())
                .map_err(js_error)?,
        })
    }
    pub fn revision(&self) -> u32 {
        self.source.revision() as u32
    }
    /// Inspect exact retained records and replacement guards after reopen.
    /// Metadata remains opaque; these are not adopted semantic facts.
    pub fn native_resource_inventory(&self) -> Result<JsValue, JsValue> {
        let array = js_sys::Array::new();
        for resource in self.source.resources().entries() {
            let summary = resource.summary();
            let row = js_sys::Object::new();
            js_sys::Reflect::set(&row, &"key".into(), &summary.key.into())?;
            js_sys::Reflect::set(&row, &"kind".into(), &summary.kind.into())?;
            js_sys::Reflect::set(
                &row,
                &"bytes".into(),
                &JsValue::from_f64(summary.bytes as f64),
            )?;
            js_sys::Reflect::set(
                &row,
                &"digest".into(),
                &js_sys::Uint8Array::from(summary.digest.as_slice()).into(),
            )?;
            js_sys::Reflect::set(
                &row,
                &"metadata_bytes".into(),
                &JsValue::from_f64(summary.metadata_bytes as f64),
            )?;
            let digest = summary.metadata_digest.map_or(JsValue::NULL, |d| {
                js_sys::Uint8Array::from(d.as_slice()).into()
            });
            js_sys::Reflect::set(&row, &"metadata_digest".into(), &digest)?;
            let metadata = resource
                .meta
                .as_deref()
                .map_or(JsValue::NULL, |m| js_sys::Uint8Array::from(m).into());
            js_sys::Reflect::set(&row, &"metadata".into(), &metadata)?;
            array.push(&row);
        }
        Ok(array.into())
    }
    pub fn submit(&mut self, source: &str, expected_revision: u32) -> Result<JsValue, JsValue> {
        asset_result(
            self.source
                .submit(source, u64::from(expected_revision))
                .map_err(js_error)?,
        )
    }
    /// Explicit add/replace plus source edit. Metadata is opaque retained CBOR;
    /// Q42 namespace/rights assertions remain unverified caller claims. Existing
    /// replacement requires its exact payload SHA as well as source revision.
    pub fn submit_native_resource(
        &mut self,
        source: &str,
        expected_revision: u32,
        key: &str,
        kind: &str,
        bytes: &[u8],
        metadata: Option<Vec<u8>>,
        expected_digest: Option<Vec<u8>>,
    ) -> Result<JsValue, JsValue> {
        if self.source.revision() != u64::from(expected_revision) {
            return Err(JsValue::from_str("Source/resource edit is stale"));
        }
        let current = self.source.resources().entries();
        if let Some(old) = current.iter().find(|r| r.key == key) {
            let expected = expected_digest.as_deref().ok_or_else(|| {
                JsValue::from_str("Replacement requires the exact prior resource SHA-256")
            })?;
            if expected != old.summary().digest.as_slice() {
                return Err(JsValue::from_str("Resource replacement digest is stale"));
            }
        } else if expected_digest.is_some() {
            return Err(JsValue::from_str("Expected replacement resource is absent"));
        }
        // Bound bridge copying before constructing the complete candidate set.
        let limits = ReviewResourceLimits::default();
        let total = current
            .iter()
            .filter(|r| r.key != key)
            .try_fold(bytes.len(), |n, r| {
                n.checked_add(r.bytes.len())
                    .and_then(|n| n.checked_add(r.meta.as_ref().map_or(0, Vec::len)))
            });
        if bytes.len() > limits.resource_bytes
            || metadata.as_ref().map_or(0, Vec::len) > limits.metadata_bytes
            || key.len() > 250
            || kind.len() > 8
            || total
                .and_then(|n| n.checked_add(metadata.as_ref().map_or(0, Vec::len)))
                .is_none_or(|n| n > limits.total_bytes)
        {
            return Err(JsValue::from_str(
                "Resource bridge copy exceeds admitted limits",
            ));
        }
        let mut resources: Vec<ResourceData> =
            current.iter().filter(|r| r.key != key).cloned().collect();
        resources.push(ResourceData {
            key: key.into(),
            kind: kind.into(),
            bytes: bytes.to_vec(),
            meta: metadata,
        });
        asset_result(
            self.source
                .submit_with_resources(source, u64::from(expected_revision), resources)
                .map_err(js_error)?,
        )
    }
}
fn js_error(error: String) -> JsValue {
    JsValue::from_str(&error)
}
fn asset_result(
    asset: ecopunk_terrain::species_authored_review::AuthoredReviewAsset,
) -> Result<JsValue, JsValue> {
    let result = js_sys::Object::new();
    js_sys::Reflect::set(
        &result,
        &"bytes".into(),
        &js_sys::Uint8Array::from(asset.render.bytes.as_slice()).into(),
    )?;
    js_sys::Reflect::set(
        &result,
        &"bundle".into(),
        &js_sys::Uint8Array::from(asset.bundle.as_slice()).into(),
    )?;
    js_sys::Reflect::set(
        &result,
        &"metadata".into(),
        &js_sys::JSON::parse(&asset.render.metadata.to_string())?,
    )?;
    Ok(result.into())
}
#[wasm_bindgen]
pub fn build_species_review_asset(subject: u8) -> Result<JsValue, JsValue> {
    let asset =
        ecopunk_terrain::species_review::build_species_review_10d(subject).map_err(js_error)?;
    let result = js_sys::Object::new();
    js_sys::Reflect::set(
        &result,
        &"bytes".into(),
        &js_sys::Uint8Array::from(asset.bytes.as_slice()).into(),
    )?;
    js_sys::Reflect::set(
        &result,
        &"metadata".into(),
        &js_sys::JSON::parse(&asset.metadata.to_string())?,
    )?;
    Ok(result.into())
}
/// Returns {session,bytes,bundle,metadata}; ordinary submit retains all admitted
/// resources. This does not adopt a Studio checkpoint or grant authority.
#[wasm_bindgen]
pub fn reopen_species_review_session(subject: u8, bundle: &[u8]) -> Result<JsValue, JsValue> {
    let restored =
        ecopunk_terrain::species_review_reopen::reopen(subject, bundle).map_err(js_error)?;
    let result = asset_result(restored.asset)?;
    let session: JsValue = SpeciesReviewSession {
        source: restored.session,
    }
    .into();
    js_sys::Reflect::set(&result, &"session".into(), &session)?;
    Ok(result)
}

fn parse_hash(text: &str) -> Result<u64, JsValue> {
    if text.len() != 16 {
        return Err(JsValue::from_str(
            "Quin identifiers require exactly16 hexadecimal digits",
        ));
    }
    u64::from_str_radix(text, 16).map_err(|_| JsValue::from_str("Invalid Quin identifier"))
}
fn digest32(bytes: &[u8]) -> Result<[u8; 32], JsValue> {
    bytes
        .try_into()
        .map_err(|_| JsValue::from_str("Expected exact SHA256 bytes"))
}
fn term_view(
    term: &ecopunk_terrain::species_review_resources::query::TermView,
) -> serde_json::Value {
    serde_json::json!({"raw_hex":format!("{:016x}",term.raw),"namespace":term.namespace,"lexical":term.lexical})
}
fn fact_view(
    fact: &ecopunk_terrain::species_review_resources::query::FactView,
) -> serde_json::Value {
    serde_json::json!({"words_hex":fact.words.map(|w|format!("{w:016x}")),"subject":term_view(&fact.subject),
        "predicate":term_view(&fact.predicate),"object":term_view(&fact.object),"context":term_view(&fact.context),
        "literal":fact.literal.map(|value|serde_json::json!({"datatype":value.datatype_iri(),"lexical":value.to_string()})),
        "embedded_object":fact.embedded_object})
}
fn cohort_request(
    key: &str,
    digest: &[u8],
    value_predicate: &str,
    unit_predicate: &str,
    unit_iri: &str,
    method: &str,
    subject: Option<String>,
    context: Option<String>,
) -> Result<ecopunk_terrain::species_review_resources::query_estimate::CohortRequest, JsValue> {
    if key.len() > 250 || unit_iri.len() > 2048 {
        return Err(JsValue::from_str("Corpus selector exceeds host bounds"));
    }
    use ecopunk_terrain::species_review_resources::query_estimate::{
        CohortRequest, EstimateMethod,
    };
    Ok(CohortRequest {
        resource_key: key.into(),
        resource_sha256: digest32(digest)?,
        subject: subject.as_deref().map(parse_hash).transpose()?,
        context: context.as_deref().map(parse_hash).transpose()?,
        value_predicate: parse_hash(value_predicate)?,
        unit_predicate: parse_hash(unit_predicate)?,
        unit_iri: unit_iri.into(),
        method: match method {
            "mean" => EstimateMethod::Mean,
            "median" => EstimateMethod::Median,
            _ => {
                return Err(JsValue::from_str(
                    "Select declared mean or median; no fallback method",
                ))
            }
        },
    })
}
#[wasm_bindgen]
impl SpeciesReviewSession {
    /// Public inspection only: no client-supplied private/classified clearance.
    /// Strings/JSON here are bounded UI projections, not a graph serializer.
    pub fn native_q42_page(
        &self,
        expected_revision: u32,
        key: &str,
        digest: &[u8],
        subject: Option<String>,
        predicate: Option<String>,
        object: Option<String>,
        context: Option<String>,
        block: u32,
        quin: u32,
    ) -> Result<String, JsValue> {
        use ecopunk_terrain::species_review_resources::query::{
            FactCursor, FactPattern, QueryLimits,
        };
        if self.source.revision() != u64::from(expected_revision) {
            return Err(JsValue::from_str("Corpus source revision is stale"));
        }
        let pattern = FactPattern {
            subject: subject.as_deref().map(parse_hash).transpose()?,
            predicate: predicate.as_deref().map(parse_hash).transpose()?,
            object: object.as_deref().map(parse_hash).transpose()?,
            context: context.as_deref().map(parse_hash).transpose()?,
        };
        let page = self
            .source
            .resources()
            .inspect_page(
                key,
                digest32(digest)?,
                pattern,
                FactCursor {
                    block_index: block as usize,
                    quin_offset: quin as usize,
                },
                QueryLimits::default(),
            )
            .map_err(|e| JsValue::from_str(&format!("Public corpus inspection refused: {e:?}")))?;
        Ok(serde_json::json!({"status":"unadopted_public_review","source_revision":expected_revision,"resource_key":page.resource_key,
            "resource_sha256":page.resource_sha256.map(|b|format!("{b:02x}")).concat(),"rows":page.rows.iter().map(fact_view).collect::<Vec<_>>(),
            "next":page.next.map(|c|serde_json::json!({"block":c.block_index,"quin":c.quin_offset})),
            "policy":"public passthrough only; sequential scan; no source-rights or assertion adoption"}).to_string())
    }
    pub fn native_q42_estimate(
        &self,
        expected_revision: u32,
        key: &str,
        digest: &[u8],
        value_predicate: &str,
        unit_predicate: &str,
        unit_iri: &str,
        method: &str,
        subject: Option<String>,
        context: Option<String>,
    ) -> Result<String, JsValue> {
        use ecopunk_terrain::species_review_resources::query_estimate::CohortLimits;
        if self.source.revision() != u64::from(expected_revision) {
            return Err(JsValue::from_str("Corpus source revision is stale"));
        }
        let request = cohort_request(
            key,
            digest,
            value_predicate,
            unit_predicate,
            unit_iri,
            method,
            subject,
            context,
        )?;
        let proposal = self
            .source
            .resources()
            .propose_estimate(request, CohortLimits::default())
            .map_err(|e| JsValue::from_str(&format!("Corpus proposal refused: {e:?}")))?;
        Ok(serde_json::json!({"status":"unadopted_arithmetic_proposal","source_revision":expected_revision,"resource_key":key,
            "resource_sha256":proposal.request().resource_sha256.map(|b|format!("{b:02x}")).concat(),
            "cohort_sha256":proposal.cohort_sha256().map(|b|format!("{b:02x}")).concat(),"method":proposal.method().identifier(),
            "cohort_policy":"distinct measurement subject/context; duplicate or conflicting assertions refused; no exclusions",
            "unit_iri":proposal.unit_iri(),"value":proposal.value(),"value_premises":proposal.value_premises().iter().map(fact_view).collect::<Vec<_>>(),
            "unit_premises":proposal.unit_premises().iter().map(fact_view).collect::<Vec<_>>(),
            "scope":"Arithmetic over supplied assertions; comparability, scientific truth, rights and source adoption unaccepted"}).to_string())
    }
    /// Caller supplies the edited source plus explicit semantic mapping. Only
    /// active white-gum organ radii in metres are supported; no value is inserted.
    pub fn submit_manual_radius_assignment(
        &mut self,
        source: &str,
        expected_revision: u32,
        key: &str,
        digest: &[u8],
        value_predicate: &str,
        unit_predicate: &str,
        unit_iri: &str,
        method: &str,
        subject: Option<String>,
        context: Option<String>,
        organ_key: &str,
        mapping_declaration: &str,
    ) -> Result<JsValue, JsValue> {
        let request = cohort_request(
            key,
            digest,
            value_predicate,
            unit_predicate,
            unit_iri,
            method,
            subject,
            context,
        )?;
        let asset = self
            .source
            .submit_manual_radius_assignment(
                source,
                u64::from(expected_revision),
                request,
                organ_key,
                mapping_declaration,
            )
            .map_err(|e| JsValue::from_str(&format!("Manual source assignment refused: {e:?}")))?;
        asset_result(asset)
    }
}
