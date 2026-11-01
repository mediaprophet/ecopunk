//! Reopen the existing limited authored-review bundle with its Vibe source.
//!
//! This does not admit a Studio checkpoint, evaluate graph effects, grant
//! authority, or establish an accepted/complete species embodiment. Admitted
//! native resources are retained; unknown kinds/changed generators are refused.
use crate::species_authored_review::{self, AuthoredReviewAsset};
use crate::species_review_resources::{
    ReviewResourceLimits, ReviewResources, SpeciesReviewPackageSession,
};
use crate::species_review_source::{AuthoredSpeciesSource, SpeciesReviewSourceSession};
use qualia_core_db::bundle::{BundleReadLimits, BundleReader, BUNDLE_ENTRY_ALIGN, BUNDLE_HEADER_SIZE};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

const MAX_PACKAGE_BYTES: usize = 128 * 1024 * 1024;
const MAX_INDEX_BYTES: u64 = 256 * 1024;
const MAX_SESSION_BYTES: usize = 16 * 1024 * 1024;

/// A restored data-only source session and its reproducible current derivative.
/// The caller must apply its own project/role/source admission before adoption.
pub struct ReopenedReview {
    pub session: SpeciesReviewPackageSession,
    pub asset: AuthoredReviewAsset,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedSession {
    schema: String,
    vibe_language: String,
    revision: u64,
    source_sha256: String,
    submissions: Vec<String>,
}

fn admitted_bundle(bytes: &[u8], subject: u8) -> Result<BundleReader<'_>, String> {
    if subject > 1 || bytes.len() < BUNDLE_HEADER_SIZE || bytes.len() > MAX_PACKAGE_BYTES {
        return Err("Unsupported review subject or package size".into());
    }
    // Bound the canonical CBOR index before it can allocate from its input.
    let count = u32::from_le_bytes(bytes[8..12].try_into().map_err(|_| "Bundle count")?);
    let index_bytes =
        u64::from_le_bytes(bytes[20..28].try_into().map_err(|_| "Bundle index size")?);
    if !(4..=165).contains(&count)
        || index_bytes > MAX_INDEX_BYTES
        || bytes[40..64].iter().any(|&b| b != 0)
    {
        return Err("Review bundle index/header exceeds supported profile".into());
    }
    let reader = BundleReader::parse_with_limits(bytes, BundleReadLimits {
        file_bytes: MAX_PACKAGE_BYTES, index_bytes: MAX_INDEX_BYTES as usize,
        entries: 165, key_bytes: 4096, kind_bytes: 64,
        meta_bytes: MAX_INDEX_BYTES as usize, decoded_index_bytes: 2 * 1024 * 1024,
    }).map_err(|e| format!("Review bundle: {e:?}"))?;
    if reader.flags() != 0 || reader.entries().len() != count as usize {
        return Err("Review bundle flags or entry count mismatch".into());
    }
    let mut keys = BTreeSet::new();
    let mut ranges = Vec::new();
    for entry in reader.entries() {
        let expected_kind = match entry.key.as_str() {
            "authoring/source-session.json" | "authoring/evaluated-source.json" => "json",
            "review/manifest.json" => "manifest",
            "review/model.10d" => "10d",
            "authoring/anatomy.json" if subject == 1 => "json",
            key if key
                .strip_prefix("authoring/submissions/")
                .and_then(|k| k.strip_suffix(".vibe"))
                .is_some_and(|n| {
                    n.len() == 4
                        && n.bytes().all(|b| b.is_ascii_digit())
                        && n.parse::<usize>().is_ok_and(|i| (1..=128).contains(&i))
                }) =>
            {
                "vibe"
            }
            key if crate::species_review_resources::supported_key(key, &entry.kind) => {
                entry.kind.as_str()
            }
            _ => {
                return Err(
                    "Unknown review resource; explicit migration required before editing".into(),
                )
            }
        };
        if entry.kind != expected_kind
            || (entry.meta.is_some() && !entry.key.starts_with("resources/"))
            || !keys.insert(entry.key.as_str())
            || entry.offset % BUNDLE_ENTRY_ALIGN as u64 != 0
            || !reader.verify_entry(&entry.key)
        {
            return Err("Review resource kind, metadata, key, alignment or hash mismatch".into());
        }
        let end = entry
            .offset
            .checked_add(entry.length)
            .ok_or("Review resource range overflow")?;
        ranges.push((entry.offset, end));
    }
    ranges.sort_unstable();
    if ranges.windows(2).any(|r| r[0].1 > r[1].0) {
        return Err("Review resource payloads overlap".into());
    }
    Ok(reader)
}

/// Replay retained Vibe submissions under the same checked, data-only session
/// used by original authoring. Require the current generator to reproduce every
/// retained resource, including the .10d, before returning an editable session.
/// Reopen therefore refuses old generator/format revisions instead of implying
/// compatibility. Source replay and geometry construction are synchronous cold
/// work under existing session/derivative limits, not a streaming memory claim.
pub fn reopen(subject: u8, bytes: &[u8]) -> Result<ReopenedReview, String> {
    reopen_with_limits(subject, bytes, ReviewResourceLimits::default())
}
/// Caller-selected resource/cold-work caps cannot be enlarged by package data.
pub fn reopen_with_limits(
    subject: u8,
    bytes: &[u8],
    limits: ReviewResourceLimits,
) -> Result<ReopenedReview, String> {
    if bytes.len() > limits.package_bytes {
        return Err("Review package exceeds caller byte budget".into());
    }
    let reader = admitted_bundle(bytes, subject)?;
    let resources = ReviewResources::from_bundle(&reader, limits)?;
    let saved_bytes = reader
        .get("authoring/source-session.json")
        .ok_or("Review source session missing")?;
    if saved_bytes.len() > MAX_SESSION_BYTES {
        return Err("Review source session exceeds byte limit".into());
    }
    let saved: SavedSession =
        serde_json::from_slice(saved_bytes).map_err(|e| format!("Review session schema: {e}"))?;
    if saved.schema != "ecopunk.species-review-source-session/1"
        || saved.vibe_language != vibe::LANGUAGE_VERSION
        || saved.submissions.is_empty()
        || saved.submissions.len() > 128
        || saved.revision != saved.submissions.len() as u64
    {
        return Err("Review session schema, language or revision is unsupported".into());
    }
    let retained = saved.submissions.iter().try_fold(0usize, |total, s| {
        if s.len() > 1024 * 1024 {
            return Err("Review source submission exceeds limit");
        }
        total
            .checked_add(s.len())
            .filter(|&n| n <= 8 * 1024 * 1024)
            .ok_or("Review retained source exceeds limit")
    })?;
    if retained == 0 {
        return Err("Review source is empty".into());
    }
    let digest = format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&saved.submissions).map_err(|e| e.to_string())?)
    );
    if digest != saved.source_sha256 {
        return Err("Review source digest mismatch".into());
    }
    // Legacy review bundles have the JSON session only and can be inspected by
    // this profile, but are NOT adopted if current regenerated resources differ.
    // In particular native .vibe additions require explicit legacy migration.
    // New bundles keep every submission; partial/mismatched sets are refused.
    let vibe_entries = reader.entries().iter().filter(|e| e.kind == "vibe").count();
    if vibe_entries != 0 {
        if vibe_entries != saved.submissions.len()
            || saved.submissions.iter().enumerate().any(|(i, source)| {
                reader.get(&format!("authoring/submissions/{:04}.vibe", i + 1))
                    != Some(source.as_bytes())
            })
        {
            return Err("Vibe resource inventory disagrees with retained source session".into());
        }
    }
    let evaluated_bytes = reader
        .get("authoring/evaluated-source.json")
        .ok_or("Evaluated review source missing")?;
    if evaluated_bytes.len() > 16 * 1024 * 1024 {
        return Err("Evaluated review source exceeds limit".into());
    }
    let mut session = SpeciesReviewSourceSession::default();
    let mut last = None;
    for source in &saved.submissions {
        last = Some(session.submit(source, |authored| {
            Ok(AuthoredSpeciesSource {
                data: authored.data.clone(),
                sources: authored.sources.clone(),
                revision: authored.revision,
                source_sha256: authored.source_sha256.clone(),
            })
        })?);
    }
    let authored = last.ok_or("Review source replay produced no value")?;
    if authored.source_sha256 != saved.source_sha256 || authored.revision != saved.revision {
        return Err(format!("Review source replay digest/revision mismatch: replay revision {}, retained revision {}, digest_equal={}",authored.revision,saved.revision,authored.source_sha256==saved.source_sha256));
    }
    verify_evaluated_projection(&authored.data, evaluated_bytes)?;
    let asset = resources.retain_in(species_authored_review::build(subject, &authored)?, limits)?;
    let regenerated = admitted_bundle(&asset.bundle, subject)?;
    if reader.entries().len() != regenerated.entries().len()
        || reader.entries().iter().any(|entry| {
            regenerated
                .entry(&entry.key)
                .is_none_or(|other| other.kind != entry.kind || other.meta != entry.meta)
                || reader.get(&entry.key) != regenerated.get(&entry.key)
        })
    {
        return Err("Review resources cannot be reproduced by current generator; explicit migration required".into());
    }
    Ok(ReopenedReview {
        session: SpeciesReviewPackageSession::from_admitted(subject, session, resources, limits),
        asset,
    })
}

/// The evaluated JSON is the existing canonical compatibility projection, not
/// the source owner. Reuse its writer rather than parsing f64 values through a
/// potentially non-roundtripping JSON reader. This is exact byte equality, with
/// no tolerance, numeric coercion, alternative parser or discarded distinction.
fn verify_evaluated_projection(data: &serde_json::Value, saved: &[u8]) -> Result<(), String> {
    let regenerated = serde_json::to_vec(data)
        .map_err(|e| format!("Review evaluated projection serialization: {e}"))?;
    if regenerated != saved {
        let first = regenerated
            .iter()
            .zip(saved)
            .position(|(a, b)| a != b)
            .unwrap_or(regenerated.len().min(saved.len()));
        return Err(format!("Review canonical evaluated projection differs at byte {first}: replay bytes {}, retained bytes {}",regenerated.len(),saved.len()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::bundle::BundleWriter;

    #[test]
    fn exact_numeric_projection_uses_replayed_vibe_owner_and_refuses_changed_bytes() {
        let mut source = SpeciesReviewSourceSession::default();
        let data=source.submit("let mut species_asset = { radius:0.33399999999999996, offset:0.10300000000000001, negative_zero:-0.0 };",|a|Ok(a.data.clone())).unwrap();
        let retained = serde_json::to_vec(&data).unwrap();
        verify_evaluated_projection(&data, &retained).unwrap();
        // A valid JSON reformat is also refused by this exact generated profile;
        // even more importantly a changed number may never be rounded to match.
        let mut changed = retained.clone();
        changed.push(b' ');
        assert!(verify_evaluated_projection(&data, &changed).is_err());
        let changed = retained.windows(4).position(|w| w == b"0.33").unwrap();
        let mut edited = retained.clone();
        edited[changed + 3] = b'4';
        assert!(verify_evaluated_projection(&data, &edited).is_err());
        let mut replay = SpeciesReviewSourceSession::default();
        let repeated = replay
            .submit(source.sources()[0].as_str(), |a| Ok(a.data.clone()))
            .unwrap();
        assert_eq!(serde_json::to_vec(&repeated).unwrap(), retained);
    }

    fn fixture() -> AuthoredReviewAsset {
        let mut session = SpeciesReviewSourceSession::default();
        session
            .submit(
                include_str!(
                    "../../../tools/species-production/white-gum-review-r1/white-gum-r2.vibe"
                ),
                |source| species_authored_review::build(0, source),
            )
            .unwrap()
    }
    fn replace(bundle: &[u8], key: &str, replacement: &[u8]) -> Vec<u8> {
        let r = BundleReader::parse(bundle).unwrap();
        let mut writer = BundleWriter::new();
        for e in r.entries() {
            writer
                .add_file(
                    &e.key,
                    &e.kind,
                    if e.key == key {
                        replacement.to_vec()
                    } else {
                        r.get(&e.key).unwrap().to_vec()
                    },
                    None,
                )
                .unwrap();
        }
        writer.build().unwrap()
    }
    #[test]
    fn reopened_source_can_edit_export_and_reopen_at_next_revision() {
        let original = fixture();
        let reader = BundleReader::parse(&original.bundle).unwrap();
        assert_eq!(
            reader.get("authoring/submissions/0001.vibe").unwrap(),
            include_str!("../../../tools/species-production/white-gum-review-r1/white-gum-r2.vibe")
                .as_bytes()
        );
        let mut reopened = reopen(0, &original.bundle).unwrap();
        assert_eq!(reopened.asset.render.bytes, original.render.bytes);
        assert_eq!(reopened.session.revision(), 1);
        let edit =
            include_str!("../../../tools/species-production/white-gum-review-r1/white-gum-r2.vibe")
                .replace("let mut species_asset =", "species_asset =")
                .replace("ring_slices: 12", "ring_slices: 16");
        assert!(edit.contains("ring_slices: 16"));
        let next = reopened.session.submit(&edit, 1).unwrap();
        assert_ne!(next.render.bytes, original.render.bytes);
        assert_eq!(reopen(0, &next.bundle).unwrap().session.revision(), 2);
    }
    #[test]
    fn coherent_repack_with_changed_source_or_model_is_refused() {
        let original = fixture();
        assert!(reopen(
            0,
            &replace(&original.bundle, "authoring/evaluated-source.json", b"{}")
        )
        .is_err());
        assert!(reopen(
            0,
            &replace(&original.bundle, "review/model.10d", b"different model")
        )
        .is_err());
        assert!(reopen(
            0,
            &replace(&original.bundle, "authoring/submissions/0001.vibe", b"= 1")
        )
        .is_err());
        let mut saved: serde_json::Value = serde_json::from_slice(
            BundleReader::parse(&original.bundle)
                .unwrap()
                .get("authoring/source-session.json")
                .unwrap(),
        )
        .unwrap();
        saved["revision"] = 2.into();
        assert!(reopen(
            0,
            &replace(
                &original.bundle,
                "authoring/source-session.json",
                &serde_json::to_vec(&saved).unwrap()
            )
        )
        .is_err());
        saved["revision"] = 1.into();
        saved["source_sha256"] = "stale".into();
        assert!(reopen(
            0,
            &replace(
                &original.bundle,
                "authoring/source-session.json",
                &serde_json::to_vec(&saved).unwrap()
            )
        )
        .is_err());
        let mut corrupted = original.bundle.clone();
        corrupted[64] ^= 1;
        assert!(reopen(0, &corrupted).is_err());
        assert!(reopen(1, &original.bundle).is_err());
    }
    #[test]
    fn unknown_resources_are_not_silently_discarded() {
        let original = fixture();
        let reader = BundleReader::parse(&original.bundle).unwrap();
        let mut writer = BundleWriter::new();
        for e in reader.entries() {
            writer
                .add_file(&e.key, &e.kind, reader.get(&e.key).unwrap().to_vec(), None)
                .unwrap();
        }
        writer
            .add_file("private/source.q42", "q42", vec![1, 2, 3], None)
            .unwrap();
        assert!(reopen(0, &writer.build().unwrap()).is_err());
    }
    #[test]
    fn legacy_native_source_omission_requires_explicit_migration() {
        let original = fixture();
        let reader = BundleReader::parse(&original.bundle).unwrap();
        let mut writer = BundleWriter::new();
        for e in reader.entries().iter().filter(|e| e.kind != "vibe") {
            writer
                .add_file(&e.key, &e.kind, reader.get(&e.key).unwrap().to_vec(), None)
                .unwrap();
        }
        let legacy = writer.build().unwrap();
        let error = match reopen(0, &legacy) {
            Ok(_) => panic!("legacy omission silently migrated"),
            Err(error) => error,
        };
        assert!(error.contains("explicit migration"));
        assert!(BundleReader::parse(&legacy)
            .unwrap()
            .verify_entry("review/model.10d"));
    }
}
