//! Internal integrity inspection of an owned snapshot, including restricted
//! records. No facts are exported and this grants no query clearance/adoption.
use super::{
    graph::{entry, term},
    *,
};
use qualia_core_db::{
    q42_lex::Q42LexMmap,
    q42_volume::{
        decode_superblock_quins, OpfsSliceRangeSource, Q42RangeVolume,
        MAX_COMPRESSED_SUPERBLOCK_SIZE, SUPERBLOCK_SIZE,
    },
    q_hash, NQuin,
};
use sha2::{Digest, Sha256};
const PROV: &str = "http://www.w3.org/ns/prov#wasDerivedFrom";
const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const STUDY_PREFIX: &str = "urn:ecopunk:species-study-snapshot:";
const RESOURCE_PREFIX: &str = "resources/knowledge/authored-model-r";

/// Append-only snapshot key. Existing resource bytes/keys/metadata must remain
/// intact; matching a binding is not permission to discard additional claims.
pub fn authored_resource_key(revision: u64, source_sha256: &str) -> Result<String, String> {
    if !(1..=128).contains(&revision) || !canonical_sha(source_sha256) {
        return Err("Authored knowledge key needs canonical source SHA/revision".into());
    }
    Ok(format!("{RESOURCE_PREFIX}{revision}-{source_sha256}.q42"))
}
pub(super) fn supported_key(key: &str) -> bool {
    if key == RESOURCE_KEY {
        return true;
    }
    let Some((revision, digest)) = key
        .strip_prefix(RESOURCE_PREFIX)
        .and_then(|s| s.strip_suffix(".q42"))
        .and_then(|s| s.split_once('-'))
    else {
        return false;
    };
    let Ok(revision) = revision.parse::<u64>() else {
        return false;
    };
    authored_resource_key(revision, digest).is_ok_and(|expected| expected == key)
}
fn canonical_sha(text: &str) -> bool {
    text.len() == 64
        && text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

pub struct AuthoredKnowledgeBinding {
    pub base_package_sha256: String,
    pub graph_sha256: [u8; 32],
    pub model_sha256: [u8; 32],
    pub source_revision: u64,
    pub restricted: bool,
}

/// Verify exact source/model/revision bindings when reopening a production
/// session. Caller first admits the actual HMC/source/model through existing
/// owners. The base-HMC digest is retained separately: it MUST NOT be compared
/// to the final enclosing HMC SHA, which also contains this graph.
///
/// This is a bounded sequential canonical scan, not a Full verification result,
/// authorization to reveal restricted facts, or proof that every other graph
/// assertion is true/adopted. Unknown foreign Q42 partitions are not inspected.
pub fn validate_authored_binding(
    resource: &ResourceData,
    expected_source_sha256: &str,
    expected_revision: u64,
    model_bytes: &[u8],
    limits: KnowledgeLimits,
    cancel: &AtomicBool,
) -> Result<AuthoredKnowledgeBinding, String> {
    if !(resource.key == RESOURCE_KEY
        || resource.key == authored_resource_key(expected_revision, expected_source_sha256)?)
        || resource.kind != "q42"
        || !hex(expected_source_sha256)
        || !(1..=128).contains(&expected_revision)
        || model_bytes.len() > 42 * 1024 * 1024
        || limits.statements == 0
        || limits.statements > 16_384
        || limits.lexicon_bytes > 1024 * 1024
        || limits.output_bytes > 16 * 1024 * 1024
        || resource.bytes.len() > limits.output_bytes
        || reservation(limits)? > limits.cold_bytes
        || cancel.load(Ordering::Acquire)
    {
        return Err("Authored graph binding input/budget/cancellation refused".into());
    }
    ReviewResources::admit(
        vec![resource.clone()],
        ReviewResourceLimits {
            q42_quins: limits.statements,
            q42_blocks: limits.statements.div_ceil(512),
            resource_bytes: limits.output_bytes,
            total_bytes: limits.output_bytes,
            cold_work_bytes: limits.cold_bytes,
            ..Default::default()
        },
    )?;
    let volume = Q42RangeVolume::open(OpfsSliceRangeSource::new(resource.bytes.as_slice()))
        .map_err(|e| format!("Authored graph native range reader: {e}"))?;
    let h = volume.header();
    let start = usize::try_from(h.lex_offset).map_err(|_| "Graph lexicon offset")?;
    let end = start
        .checked_add(usize::try_from(h.lex_length).map_err(|_| "Graph lexicon length")?)
        .ok_or("Graph lexicon span")?;
    if h.lex_length > limits.lexicon_bytes as u64 * 8 + 1024 * 1024 {
        return Err("Graph lexical section exceeds binding profile".into());
    }
    let lex = Q42LexMmap::from_bytes(
        resource
            .bytes
            .get(start..end)
            .ok_or("Graph lexicon outside resource")?,
    )
    .map_err(|e| format!("Authored graph native lexicon: {e:?}"))?;
    let mut compressed = vec![0; MAX_COMPRESSED_SUPERBLOCK_SIZE];
    let mut decoded = vec![0; SUPERBLOCK_SIZE];
    let mut records = Vec::new();
    let mut context = None;
    for block in 0..volume.block_count() as usize {
        if cancel.load(Ordering::Acquire) {
            return Err("Authored graph binding cancelled".into());
        }
        volume
            .read_superblock_into(block, &mut compressed, &mut decoded)
            .map_err(|e| format!("Authored graph block: {e}"))?;
        let next = decode_superblock_quins(&decoded)
            .map_err(|e| format!("Authored graph records: {e}"))?;
        if records
            .len()
            .checked_add(next.len())
            .is_none_or(|n| n > limits.statements)
        {
            return Err("Authored graph record bound exceeded".into());
        }
        for q in &next {
            if !q.verify_ecc_parity()
                || q.metadata != 0
                || !(q.context == 0
                    || q.context == (u64::from(NQuin::SENSITIVITY_RESTRICTED) << 56))
            {
                return Err("Authored graph record is outside owned classification/profile".into());
            }
            if context.is_some_and(|c| c != q.context) {
                return Err("Mixed source classification needs explicit graph migration".into());
            }
            context = Some(q.context);
        }
        records
            .try_reserve(next.len())
            .map_err(|_| "Graph binding allocation refused")?;
        records.extend(next);
    }
    let studies: Vec<_> = records
        .iter()
        .filter(|q| q.predicate == q_hash(RDF_TYPE) && q.object == q_hash(&term("SpeciesStudy")))
        .collect();
    if studies.len() != 1 {
        return Err("Owned graph needs one unambiguous source-study snapshot".into());
    }
    if lexical(&lex, studies[0].predicate)? != RDF_TYPE
        || lexical(&lex, studies[0].object)? != term("SpeciesStudy")
    {
        return Err("Owned graph study type is not exact lexical binding".into());
    }
    let study = lexical(&lex, studies[0].subject)?;
    let base = study
        .strip_prefix(STUDY_PREFIX)
        .filter(|s| hex(s))
        .ok_or("Owned graph base snapshot identifier invalid")?;
    let source =
        format!("urn:ecopunk:vibe-history:{expected_source_sha256}:revision:{expected_revision}");
    exact(
        &records,
        &lex,
        &study,
        PROV,
        &[&format!("urn:sha256:{base}"), &source],
    )?;
    exact(
        &records,
        &lex,
        &study,
        &term("snapshotKind"),
        &[&term("OwnedGraphExcludedBaseHmcSnapshot")],
    )?;
    exact(
        &records,
        &lex,
        &study,
        &term("knowledgeScope"),
        &[&term("AuthoredModelSemantics")],
    )?;
    let model = entry(&study, "review/model.10d");
    let model_sha: [u8; 32] = Sha256::digest(model_bytes).into();
    exact(
        &records,
        &lex,
        &model,
        PROV,
        &[&format!("urn:sha256:{:x}", Sha256::digest(model_bytes))],
    )?;
    exact(
        &records,
        &lex,
        &model,
        RDF_TYPE,
        &[&term("AssetResource"), &term("EditableModelRepresentation")],
    )?;
    exact(
        &records,
        &lex,
        &model,
        &term("representationStatus"),
        &[&term("AuthoredStudyUnaccepted")],
    )?;
    exact(
        &records,
        &lex,
        ONTOLOGY_IRI,
        PROV,
        &[&format!(
            "urn:sha256:{:x}",
            Sha256::digest(ONTOLOGY_TTL.as_bytes())
        )],
    )?;
    if cancel.load(Ordering::Acquire) {
        return Err("Authored graph binding cancelled before receipt".into());
    }
    Ok(AuthoredKnowledgeBinding {
        base_package_sha256: base.into(),
        graph_sha256: Sha256::digest(&resource.bytes).into(),
        model_sha256: model_sha,
        source_revision: expected_revision,
        restricted: context == Some(u64::from(NQuin::SENSITIVITY_RESTRICTED) << 56),
    })
}
fn hex(text: &str) -> bool {
    text.len() == 64 && text.bytes().all(|b| b.is_ascii_hexdigit())
}
fn lexical(lex: &Q42LexMmap<'_>, raw: u64) -> Result<String, String> {
    let (namespace, local) = lex
        .lookup_parts(raw)
        .ok_or("Owned graph binding term missing from native lexicon")?;
    if namespace
        .len()
        .checked_add(local.len())
        .is_none_or(|n| n > 4096)
    {
        return Err("Owned graph binding term exceeds bound".into());
    }
    Ok(format!("{namespace}{local}"))
}
fn exact(
    records: &[NQuin],
    lex: &Q42LexMmap<'_>,
    subject: &str,
    predicate: &str,
    objects: &[&str],
) -> Result<(), String> {
    let mut seen = vec![false; objects.len()];
    let mut count = 0;
    for q in records
        .iter()
        .filter(|q| q.subject == q_hash(subject) && q.predicate == q_hash(predicate))
    {
        if lexical(lex, q.subject)? != subject || lexical(lex, q.predicate)? != predicate {
            return Err("Owned graph term hash is not exact lexical binding".into());
        }
        let object = lexical(lex, q.object)?;
        let i = objects
            .iter()
            .position(|expected| *expected == object)
            .ok_or("Owned graph contains conflicting model/source binding")?;
        if seen[i] {
            return Err("Owned graph contains duplicated binding assertions".into());
        }
        seen[i] = true;
        count += 1;
    }
    if count != objects.len() || seen.contains(&false) {
        return Err("Owned graph expected model/source binding is absent".into());
    }
    Ok(())
}
