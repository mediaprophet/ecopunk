use super::*;
use crate::{
    species_authored_review::AuthoredReviewAsset,
    species_review_resources::{ReviewResourceLimits, ReviewResources},
};
use qualia_core_db::{
    bundle::{
        variant_matrix::FAMILY_MANIFEST_LEGACY_KEY, BundleReader, HmcHeaderProbe,
        TwinFamilyManifest, FAMILY_MANIFEST_ENTRY_KEY, FAMILY_MANIFEST_KIND,
    },
    container_10d::{
        self as c, skeletal_container::read_skeletal_section, Container10dHeader, SectionType,
    },
};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    sync::atomic::{AtomicBool, Ordering},
};
/// Cold inspection only of a candidate already admitted by the existing native
/// source/package owner. This does not make legacy Ciborium BundleReader a
/// bounded decoder for arbitrary attacker-controlled uploaded CBOR.
pub fn inspect_admitted(
    subject: u8,
    asset: &AuthoredReviewAsset,
    limits: ReviewResourceLimits,
    cancel: &AtomicBool,
) -> Result<PackageReviewIndex, String> {
    if subject > 1 || cancel.load(Ordering::Acquire) {
        return Err("Unsupported species review or cancelled inspection".into());
    }
    if asset.render.metadata["taxon"].as_str()
        != Some(if subject == 0 {
            "Eucalyptus dalrympleana"
        } else {
            "Wallabia bicolor"
        })
    {
        return Err("Selected review profile disagrees with admitted species metadata".into());
    }
    let bytes = &asset.bundle;
    if bytes.len() > limits.package_bytes.min(128 * 1024 * 1024)
        || bytes
            .len()
            .checked_mul(4)
            .and_then(|n| n.checked_add(1024 * 1024))
            .is_none_or(|n| n > limits.cold_work_bytes)
    {
        return Err(format!("Package review exceeds caller file/cold envelope: {} file bytes (limit {}), inspection needs {:?} cold bytes (limit {})",
            bytes.len(), limits.package_bytes.min(128 * 1024 * 1024),
            bytes.len().checked_mul(4).and_then(|n|n.checked_add(1024*1024)), limits.cold_work_bytes));
    }
    let probe = HmcHeaderProbe::parse_header_range(bytes)
        .map_err(|e| format!("Package review header: {e:?}"))?;
    if probe.entry_count == 0
        || probe.entry_count > 165
        || probe.index_length > 256 * 1024
        || probe.flags != 0
        || probe.total_length != bytes.len() as u64
        || bytes[40..64].iter().any(|&b| b != 0)
    {
        return Err("Package review exceeds admitted header/index profile".into());
    }
    let reader =
        BundleReader::parse(bytes).map_err(|e| format!("Package canonical read: {e:?}"))?;
    if reader.entries().len() != probe.entry_count as usize {
        return Err("Package index count differs from header".into());
    }
    let mut keys = BTreeSet::new();
    let mut ranges = Vec::new();
    let mut resources = Vec::new();
    for e in reader.entries() {
        if cancel.load(Ordering::Acquire) {
            return Err("Package review cancelled".into());
        }
        if e.key.is_empty()
            || e.key.len() > 1024
            || e.key.contains('\0')
            || e.key.starts_with('/')
            || e.key.split('/').any(|p| p == ".." || p.is_empty())
            || e.kind.len() > 64
            || !keys.insert(e.key.as_str())
            || !reader.verify_entry(&e.key)
            || e.offset % 64 != 0
            || e.meta.as_ref().map_or(0, Vec::len) > limits.metadata_bytes
        {
            return Err("Package resource identity/hash/metadata/alignment refused".into());
        }
        let end = e
            .offset
            .checked_add(e.length)
            .ok_or("Package resource range overflow")?;
        if e.offset < 64 || end > probe.index_offset {
            return Err("Package resource overlaps header/index".into());
        }
        ranges.push((e.offset, end));
        resources.push(ResourceReview {
            key: e.key.clone(),
            kind: e.kind.clone(),
            bytes: usize::try_from(e.length).map_err(|_| "Package resource size")?,
            sha256: hex(&e.sha256),
            metadata_bytes: e.meta.as_ref().map_or(0, Vec::len),
            metadata_sha256: e.meta.as_ref().map(|b| hash(b)),
            interpretation:
                "Exact retained bytes; metadata/source/rights and graph assertions are unadopted",
        });
    }
    ranges.sort_unstable();
    if ranges.windows(2).any(|r| r[0].1 > r[1].0) {
        return Err("Package resource payloads overlap".into());
    }
    // Reuse the source-preserving native Q42/M64 resource admission owner. It
    // validates its declared structural profile, not Full Q42 semantic/index order.
    let _native = ReviewResources::from_bundle(&reader, limits)?;
    let model = reader
        .entry("review/model.10d")
        .ok_or("Review master absent")?;
    let manifest = reader
        .entry("review/manifest.json")
        .ok_or("Review manifest absent")?;
    if model.kind != "10d"
        || manifest.kind != "manifest"
        || reader.get(&model.key) != Some(asset.render.bytes.as_slice())
        || reader.get(&manifest.key)
            != Some(
                serde_json::to_vec(&asset.render.metadata)
                    .map_err(|e| e.to_string())?
                    .as_slice(),
            )
    {
        return Err("Package model/manifest do not bind the exact admitted candidate".into());
    }
    let mut models = Vec::new();
    for e in reader.entries().iter().filter(|e| e.kind == "10d") {
        if models.len() >= 32 || cancel.load(Ordering::Acquire) {
            return Err("Package model count exceeded or cancelled".into());
        }
        models.push(inspect_model(
            &e.key,
            reader.get(&e.key).ok_or("Model bytes absent")?,
            &asset.render.metadata,
        )?);
    }
    let family = family(&reader, &resources, &models)?;
    if family
        .as_ref()
        .is_some_and(|f| f.classification != if subject == 0 { "flora" } else { "fauna" })
    {
        return Err("Family classification conflicts with selected species review profile".into());
    }
    let coverage = coverage::build(subject, family.as_ref(), &resources);
    let catalogue_bytes = include_bytes!(
        "../../../../docs/planning/tech/assets/model-production-template/deliverables.json"
    );
    let catalogue: serde_json::Value =
        serde_json::from_slice(catalogue_bytes).map_err(|e| e.to_string())?;
    let catalogue = CatalogueReview {
        id: catalogue["catalogue_id"]
            .as_str()
            .ok_or("Catalogue identifier missing")?
            .into(),
        version: catalogue["version"]
            .as_str()
            .ok_or("Catalogue version missing")?
            .into(),
        sha256: hash(catalogue_bytes),
    };
    let revision = asset.render.metadata["source_revision"]
        .as_u64()
        .filter(|&r| r > 0 && r <= 128)
        .ok_or("Review source revision absent/outside profile")?;
    let source_sha256 = asset.render.metadata["source_sha256"]
        .as_str()
        .filter(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit()))
        .ok_or("Review source history digest absent")?
        .to_owned();
    if cancel.load(Ordering::Acquire) {
        return Err("Package review cancelled before publication".into());
    }
    Ok(PackageReviewIndex{schema:"ecopunk.species-package-review-projection/v1",package_sha256:hash(bytes),package_bytes:bytes.len(),subject,source_revision:revision.to_string(),source_sha256,catalogue,resources,models,family,coverage,scope:"Read-only projection of an admitted limited species HMC; exact resource integrity is not corpus adoption, species-form coverage, authority, live behavior or acceptance"})
}
fn inspect_model(
    key: &str,
    bytes: &[u8],
    metadata: &serde_json::Value,
) -> Result<ModelReview, String> {
    if bytes.len() > 42 * 1024 * 1024 {
        return Err("Package model exceeds canonical read cap".into());
    }
    let header =
        Container10dHeader::parse(bytes).map_err(|e| format!("Package model header: {e:?}"))?;
    if c::integrity::compute_whole_file_crc32c(bytes) != header.header_crc32c {
        return Err("Package model whole-file seal differs".into());
    }
    let sections = c::parse_section_table(bytes, &header)
        .map_err(|e| format!("Package model sections: {e:?}"))?;
    let meshes: Vec<_> = sections
        .iter()
        .filter(|d| d.typ() == Some(SectionType::QuantizedMesh))
        .collect();
    if meshes.len() != 1 {
        return Err("Package review requires one unambiguous mesh per .10d model".into());
    }
    let d = meshes[0];
    let payload = &bytes[d.byte_offset as usize..(d.byte_offset as usize + d.byte_length as usize)];
    let (mh, len) =
        c::parse_mesh_header(payload).map_err(|e| format!("Package mesh header: {e:?}"))?;
    if len != payload.len() || mh.vertex_count > 1_000_000 || mh.triangle_count > 2_000_000 {
        return Err("Package mesh exceeds reviewed counts/layout".into());
    }
    let decoded = c::decode_mesh_section(payload)
        .map_err(|e| format!("Package native mesh admission: {e:?}"))?;
    if decoded.positions.len() != mh.vertex_count as usize
        || decoded.triangles.len() != mh.triangle_count as usize
    {
        return Err("Package native mesh counts differ".into());
    }
    let mut clips = Vec::new();
    if sections
        .iter()
        .any(|d| d.typ() == Some(SectionType::SkeletalAnimation))
    {
        let rig = read_skeletal_section(bytes)
            .map_err(|e| format!("Package canonical SKL1 admission: {e:?}"))?;
        for i in 0..rig.counts().clips {
            let c = rig.clip(i).ok_or("Package clip absent")?;
            let mut keys = 0u32;
            for t in c.first_track..c.first_track + c.track_count {
                keys = keys
                    .checked_add(
                        rig.track(t as usize)
                            .ok_or("Package clip track absent")?
                            .key_count as u32,
                    )
                    .ok_or("Package clip key count overflow")?;
            }
            let name = if key == "review/model.10d" {
                metadata["detail"]["rig"]["authored_motion"]["clips"]
                    .as_array()
                    .and_then(|a| {
                        a.iter()
                            .find(|v| v["render_clip_id"].as_u64() == Some(c.id))
                    })
                    .and_then(|v| v["source_clip_id"].as_str())
                    .map(str::to_owned)
            } else {
                None
            };
            clips.push(ClipReview{id:c.id.to_string(),source_name:name,duration_s:c.duration,looping:c.looping,tracks:c.track_count,keys,scope:"Actual canonical SKL1 clip; data presence is not accepted behavior/contact or rendered animation"});
        }
    }
    Ok(ModelReview {
        key: key.into(),
        sha256: hash(bytes),
        vertices: mh.vertex_count,
        triangles: mh.triangle_count,
        section_types: sections.iter().map(|d| format!("{:?}", d.typ())).collect(),
        clips,
    })
}
fn family(
    reader: &BundleReader<'_>,
    resources: &[ResourceReview],
    models: &[ModelReview],
) -> Result<Option<FamilyReview>, String> {
    let canonical = reader.entry(FAMILY_MANIFEST_ENTRY_KEY);
    let legacy = reader.entry(FAMILY_MANIFEST_LEGACY_KEY);
    if canonical.is_some() && legacy.is_some() {
        return Err("Two family declarations require explicit reconciliation".into());
    }
    let Some(entry) = canonical.or(legacy) else {
        return Ok(None);
    };
    if entry.kind != FAMILY_MANIFEST_KIND || entry.length > 64 * 1024 {
        return Err("Family declaration exceeds admitted kind/byte profile".into());
    }
    let original =
        TwinFamilyManifest::decode_from_cbor(reader.get(&entry.key).ok_or("Family bytes absent")?)
            .map_err(|e| format!("Canonical family codec: {e:?}"))?;
    if original.variants.is_empty()
        || original.variants.len() > 32
        || original.family_id.trim().is_empty()
        || original.family_id.len() > 1024
        || original.common_name.len() > 1024
        || original.classification.len() > 64
    {
        return Err("Family declaration exceeds review limits".into());
    }
    let mut ids = BTreeSet::new();
    let mut variants = Vec::new();
    for v in original.variants {
        if v.variant_id.is_empty()
            || v.variant_id.len() > 256
            || !ids.insert(v.variant_id.clone())
            || !models.iter().any(|m| m.key == v.asset_key)
            || v.entity_ontology_key.as_ref().is_some_and(|k| {
                !resources
                    .iter()
                    .any(|r| r.key == *k && r.kind == "q42" && r.key.starts_with("resources/"))
            })
        {
            return Err("Family variant has duplicate/missing model or graph binding".into());
        }
        variants.push(VariantReview {
            variant_id: v.variant_id,
            phenotype: v.phenotype,
            stage: v.stage,
            lod: v.lod,
            state: v.state,
            asset_key: v.asset_key,
            entity_ontology_key: v.entity_ontology_key,
            procedural_seed: v.procedural_seed.map(|v| v.to_string()),
            status: "Source-declared variant only; biological and target acceptance unassessed",
        });
    }
    if !ids.contains(&original.default_variant) {
        return Err("Family declared default is not an explicit variant".into());
    }
    Ok(Some(FamilyReview {
        family_id: original.family_id,
        common_name: original.common_name,
        classification: original.classification,
        declared_default_variant: original.default_variant,
        variants,
        status:
            "Exact native family declaration; review selection never substitutes missing variants",
    }))
}
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
