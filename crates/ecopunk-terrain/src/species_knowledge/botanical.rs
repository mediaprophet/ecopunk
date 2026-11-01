//! Narrow field projection of the existing admitted botanical/v2 source. This
//! does not introduce another authored source schema or run a growth simulator.
use super::{
    authored::{node, vector},
    graph::{term, Graph},
    *,
};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Deserialize)]
struct Projection {
    source_version: String,
    botanical: Botanical,
    radii_m: Vec<f32>,
    roles: Vec<String>,
    sprays: Vec<Spray>,
}
#[derive(Deserialize)]
struct Botanical {
    schema: String,
    version: u32,
    part_id: String,
    organs: Vec<Organ>,
}
#[derive(Deserialize)]
struct Organ {
    key: String,
    parent: Option<String>,
    position_m: [f32; 3],
}
#[derive(Deserialize)]
struct Spray {
    key: String,
    edge_end: String,
    edge_fraction: f32,
    leaf_count: usize,
    leaf_length_m: f32,
    leaf_width_m: f32,
}
pub(super) fn populate(
    g: &mut Graph,
    study: &str,
    model: &str,
    data: &[u8],
    cancel: &AtomicBool,
) -> Result<(), String> {
    let source: Projection = serde_json::from_slice(data)
        .map_err(|e| format!("Existing botanical source projection: {e}"))?;
    if !matches!(
        source.source_version.as_str(),
        "white-gum-authored-review-r2"
            | "white-gum-connected-study-v1"
            | "white-gum-chain-union-v1"
            | "white-gum-all-forks-v1"
    ) || source.botanical.schema != "poet.botanical/v2"
        || source.botanical.version != 2
        || source.botanical.organs.len() > 256
        || source.botanical.organs.is_empty()
        || source.roles.len() != source.botanical.organs.len()
        || source.radii_m.len() != source.roles.len()
        || source.sprays.len() > 256
    {
        return Err("Unsupported admitted botanical knowledge projection".into());
    }
    let definition = node(
        g,
        study,
        "AuthoredModelDefinition",
        &source.botanical.part_id,
        "/botanical",
    )?;
    g.triple(study, &term("hasModelDefinition"), &definition)?;
    g.triple(model, &term("realizesDefinition"), &definition)?;
    g.triple(
        &definition,
        &term("declaredTaxonLabel"),
        "urn:ecopunk:authored-taxon-label:Eucalyptus%20dalrympleana",
    )?;
    g.triple(
        &definition,
        &term("parameterStatus"),
        &term("AuthoredEstimatesNotMeasurements"),
    )?;
    g.triple(&definition, &term("unit"), "http://qudt.org/vocab/unit/M")?;
    g.triple(
        &definition,
        &term("coordinateFrame"),
        &term("AuthoredBotanicalModelMetres"),
    )?;
    let mut organs = BTreeMap::new();
    for (i, organ) in source.botanical.organs.iter().enumerate() {
        if cancel.load(Ordering::Acquire) {
            return Err("Botanical knowledge cancelled".into());
        }
        if organs.contains_key(&organ.key)
            || !source.radii_m[i].is_finite()
            || source.radii_m[i] <= 0.0
        {
            return Err("Duplicate botanical organ or invalid source radius".into());
        }
        let n = node(
            g,
            study,
            "AuthoredBotanicalOrgan",
            &organ.key,
            &format!("/botanical/organs/{i}"),
        )?;
        g.triple(&definition, &term("hasPart"), &n)?;
        vector(g, &n, "position", organ.position_m)?;
        g.number_f32(&n, &term("sourceRadius"), source.radii_m[i])?;
        g.triple(&n, &term("unit"), "http://qudt.org/vocab/unit/M")?;
        let role = match source.roles[i].as_str() {
            "root_collar" => "RootCollarRole",
            "trunk_metamer" => "TrunkMetamerRole",
            "scaffold_branch" => "ScaffoldBranchRole",
            "secondary_branchlet" => "SecondaryBranchletRole",
            "lateral_root" => "LateralRootRole",
            "secondary_root" => "SecondaryRootRole",
            "primary_root_hypothesis" => "PrimaryRootHypothesisRole",
            _ => return Err("Unknown botanical role cannot become ontology fact".into()),
        };
        g.triple(&n, &term("declaredRole"), &term(role))?;
        g.triple(&n, &term("representationResource"), model)?;
        g.triple(
            &n,
            &term("bindingStatus"),
            &term("SourceDeclaredPartBinding"),
        )?;
        organs.insert(organ.key.clone(), n);
    }
    for organ in &source.botanical.organs {
        if let Some(parent) = organ.parent.as_ref() {
            g.triple(
                &organs[&organ.key],
                &term("parentOrgan"),
                organs.get(parent).ok_or("Botanical parent organ absent")?,
            )?;
        }
    }
    let mut spray_ids = BTreeSet::new();
    for (i, spray) in source.sprays.iter().enumerate() {
        if !spray_ids.insert(&spray.key)
            || !spray.edge_fraction.is_finite()
            || !(0.0..=1.0).contains(&spray.edge_fraction)
        {
            return Err("Duplicate/invalid botanical spray".into());
        }
        let n = node(
            g,
            study,
            "AuthoredFoliageSpray",
            &spray.key,
            &format!("/sprays/{i}"),
        )?;
        g.triple(&definition, &term("hasPart"), &n)?;
        g.triple(
            &n,
            &term("attachesToEdgeEnd"),
            organs
                .get(&spray.edge_end)
                .ok_or("Foliage attachment organ absent")?,
        )?;
        g.number_f32(&n, &term("edgeFraction"), spray.edge_fraction)?;
        g.integer(&n, &term("leafCount"), spray.leaf_count)?;
        g.number_f32(&n, &term("leafLength"), spray.leaf_length_m)?;
        g.number_f32(&n, &term("leafWidth"), spray.leaf_width_m)?;
        g.triple(&n, &term("unit"), "http://qudt.org/vocab/unit/M")?;
        g.triple(&n, &term("representationResource"), model)?;
    }
    // These are source radii, not measured tissues or the altered departure
    // radii used by selected mesh projections. Existing generator metadata and
    // attribute lineage remain authoritative for that projection distinction.
    g.triple(
        &definition,
        &term("radiusScope"),
        &term("AuthoredSourceRadiiNotEffectiveMeshMeasurements"),
    )?;
    Ok(())
}
