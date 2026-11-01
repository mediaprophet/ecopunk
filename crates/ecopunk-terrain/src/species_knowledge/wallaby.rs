use super::{
    authored::{node, rdf, references, vector},
    graph::{escaped, term, Graph},
    *,
};
use crate::swamp_wallaby_anatomical_r3 as r3;
use std::collections::{BTreeMap, BTreeSet};

#[path = "wallaby_recipes.rs"]
mod wallaby_recipes;

pub(super) fn populate(
    g: &mut Graph,
    study: &str,
    model: &str,
    evaluated: &[u8],
    index: &PackageReviewIndex,
    cancel: &AtomicBool,
) -> Result<(), String> {
    let text = std::str::from_utf8(evaluated).map_err(|_| "Authored r3 source is not UTF-8")?;
    let source = r3::parse_source(text)?;
    let definition = node(g, study, "AuthoredModelDefinition", &source.identifier, "/")?;
    g.triple(study, &term("hasModelDefinition"), &definition)?;
    g.triple(model, &term("realizesDefinition"), &definition)?;
    g.triple(&definition, &term("declaredIdentifier"), &source.identifier)?;
    g.triple(
        &definition,
        &term("declaredTaxonLabel"),
        &format!(
            "urn:ecopunk:authored-taxon-label:{}",
            escaped(&source.taxon)
        ),
    )?;
    g.triple(
        &definition,
        &term("parameterStatus"),
        &term("AuthoredEstimatesNotMeasurements"),
    )?;
    if source.numeric_profile == Some(r3::NumericProfile::PortableSoftwareV1) {
        // This records the explicit source-selected arithmetic policy. The fact
        // is not a cross-target execution/equality receipt or scientific claim.
        let profile = term("PortableSoftwareV1");
        let method = term("LibmBoundedAnatomySinCosPowV1");
        g.triple(&definition, &term("declaresNumericExecutionProfile"), &profile)?;
        g.triple(&profile, &rdf("type"), &term("AuthoredNumericExecutionProfile"))?;
        g.triple(&profile, &term("usesNumericMethodPolicy"), &method)?;
        g.triple(
            &profile,
            &term("numericProfileStatus"),
            &term("SourceDeclaredMethodNotExecutionProof"),
        )?;
        g.triple(&method, &rdf("type"), &term("NumericMethodPolicy"))?;
        g.triple(
            &method,
            &term("implementedBySoftware"),
            "https://crates.io/crates/libm/0.2.16",
        )?;
        g.triple(
            "https://crates.io/crates/libm/0.2.16",
            &rdf("type"),
            "http://www.w3.org/ns/prov#Entity",
        )?;
        g.triple(&method, &term("numericMethodScope"), &term("BoundedAnatomySinCosPow"))?;
    }
    g.triple(
        &definition,
        &term("coordinateFrame"),
        &term("YUpZFrontGroundZeroModelMetres"),
    )?;
    g.triple(&definition, &term("unit"), "http://qudt.org/vocab/unit/M")?;
    references(g, study, &source.references)?;
    let mut joints: Vec<String> = Vec::new();
    let mut named: BTreeMap<&str, String> = BTreeMap::new();
    for (i, joint) in source.joints.iter().enumerate() {
        if cancel.load(Ordering::Acquire) {
            return Err("Authored joint graph cancelled".into());
        }
        let j = node(
            g,
            study,
            "AuthoredJoint",
            &joint.id,
            &format!("/joints/{i}"),
        )?;
        g.triple(&definition, &term("hasJoint"), &j)?;
        vector(g, &j, "position", joint.position_m)?;
        g.triple(&j, &term("unit"), "http://qudt.org/vocab/unit/M")?;
        if let Some(parent) = joint.parent {
            g.triple(
                &j,
                &term("parentJoint"),
                joints.get(parent).ok_or("Joint graph parent absent")?,
            )?;
        }
        named.insert(joint.id.as_str(), j.clone());
        joints.push(j);
    }
    let mut materials = Vec::new();
    let mut material_ids = BTreeSet::new();
    for (i, palette) in source.palettes.iter().enumerate() {
        if !material_ids.insert(&palette.id) {
            return Err(
                "Duplicate authored material identifier requires explicit disambiguation".into(),
            );
        }
        let material = node(
            g,
            study,
            "AuthoredMaterial",
            &palette.id,
            &format!("/palettes/{i}"),
        )?;
        g.triple(&definition, &term("hasMaterial"), &material)?;
        g.number_f32(&material, &term("roughness"), palette.roughness)?;
        materials.push(material);
    }
    for (i, loft) in source.lofts.iter().enumerate() {
        let p = part(
            g,
            study,
            model,
            &definition,
            &loft.id,
            "LoftPart",
            &format!("/lofts/{i}"),
            materials.get(loft.material),
        )?;
        loft_skin(g, study, &p, loft, i, &joints, cancel)?;
        g.boolean(&p, &term("declaresCoat"), loft.coat)?;
    }
    for (chain_index, chain) in source.loft_chains.iter().enumerate() {
        if cancel.load(Ordering::Acquire) {
            return Err("Authored loft-chain graph cancelled".into());
        }
        // This is the retained source recipe used by the generator. Its ordered
        // windows and sampling controls are not evidence of a biological torso
        // branch, observed attachment, or accepted anatomical continuity.
        let chain_node = node(
            g,
            study,
            "AuthoredSerialLoftChainRecipe",
            &chain.id,
            &format!("/loft_chains/{chain_index}"),
        )?;
        g.triple(&definition, &term("hasLoftChainRecipe"), &chain_node)?;
        g.triple(&chain_node, &term("representationResource"), model)?;
        g.triple(
            &chain_node,
            &term("recipeStatus"),
            &term("AuthoredSerialRecipeNotBiologicalProof"),
        )?;
        g.integer(&chain_node, &term("radialSegments"), chain.radial_segments)?;
        g.integer(&chain_node, &term("subdivisions"), chain.subdivisions)?;
        for (member_index, member) in chain.members.iter().enumerate() {
            if !source.lofts.iter().any(|loft| loft.id == member.loft_id) {
                return Err("Authored loft-chain member has no source loft".into());
            }
            let member_node = node(
                g,
                study,
                "AuthoredLoftChainMember",
                &format!("{}:member:{member_index}", chain.id),
                &format!("/loft_chains/{chain_index}/members/{member_index}"),
            )?;
            let loft_node = format!("{study}:LoftPart:{}", escaped(&member.loft_id));
            g.triple(&chain_node, &term("hasOrderedChainMember"), &member_node)?;
            g.triple(&member_node, &term("memberLoftPart"), &loft_node)?;
            g.integer(&member_node, &term("memberOrdinal"), member_index)?;
            // Source section windows use inclusive zero-based endpoints.
            g.integer(&member_node, &term("firstSourceSectionInclusive"), member.first_section)?;
            g.integer(&member_node, &term("lastSourceSectionInclusive"), member.last_section)?;
        }
    }
    wallaby_recipes::junctions(g, study, model, &definition, &source, &materials)?;
    for (i, e) in source.ellipsoids.iter().enumerate() {
        let p = part(
            g,
            study,
            model,
            &definition,
            &e.id,
            "EllipsoidPart",
            &format!("/ellipsoids/{i}"),
            materials.get(e.material),
        )?;
        g.triple(&p, &term("influencedByJoint"), &joints[e.joint])?;
        vector(g, &p, "position", e.center_m)?;
        vector(g, &p, "radius", e.radii_m)?;
        g.boolean(&p, &term("declaresMirror"), e.mirror)?;
    }
    for (i, e) in source.ears.iter().enumerate() {
        let p = part(
            g,
            study,
            model,
            &definition,
            &e.id,
            "EarPart",
            &format!("/ears/{i}"),
            materials.get(e.material),
        )?;
        g.triple(&p, &term("innerMaterial"), &materials[e.inner_material])?;
        g.triple(&p, &term("influencedByJoint"), &joints[e.joint])?;
        g.boolean(&p, &term("declaresMirror"), e.mirror)?;
    }
    for (collection, kind) in [
        ("bones", "AuthoredBoneSpan"),
        ("muscles", "AuthoredMuscleSpan"),
    ] {
        let mut span_ids = BTreeSet::new();
        let spans: Vec<_> = if collection == "bones" {
            source.bones.iter().map(|s| (&s.id, s.joints)).collect()
        } else {
            source.muscles.iter().map(|s| (&s.id, s.joints)).collect()
        };
        for (i, (id, ends)) in spans.into_iter().enumerate() {
            if !span_ids.insert(id) {
                return Err("Duplicate anatomy-span identifier requires disambiguation".into());
            }
            let span = node(g, study, kind, id, &format!("/{collection}/{i}"))?;
            g.triple(&definition, &term("hasAnatomySpan"), &span)?;
            g.triple(&span, &term("startsAtJoint"), &joints[ends[0]])?;
            g.triple(&span, &term("endsAtJoint"), &joints[ends[1]])?;
        }
    }
    let coat = node(g, study, "AuthoredCoat", "coat", "/coat")?;
    g.triple(&definition, &term("hasCoat"), &coat)?;
    g.boolean(&coat, &term("enabled"), source.coat.enabled)?;
    g.triple(&coat, &term("material"), &materials[source.coat.material])?;
    g.number_f32(&coat, &term("length"), source.coat.length_m)?;
    g.triple(&coat, &term("unit"), "http://qudt.org/vocab/unit/M")?;
    wallaby_recipes::groom(g, study, model, &definition, &source, &materials)?;
    if let Some(motion) = source.motion.as_ref() {
        if motion.schema != crate::animal_motion::SCHEMA
            || motion.frame != crate::animal_motion::FRAME
            || motion.units != "m,s,radian"
        {
            return Err("Unsupported authored motion graph schema/frame/units".into());
        }
        let canonical = index
            .models
            .iter()
            .find(|m| m.key == "review/model.10d")
            .ok_or("Canonical source model absent")?;
        for (i, clip) in motion.clips.iter().enumerate() {
            let native_id = crate::animal_motion::clip_id(&source.identifier, &clip.id);
            let admitted = canonical
                .clips
                .iter()
                .find(|c| c.id == native_id.to_string())
                .ok_or("Authored clip is absent in canonical SKL1")?;
            if admitted.source_name.as_deref() != Some(clip.id.as_str())
                || admitted.duration_s.to_bits() != clip.duration_s.to_bits()
                || admitted.looping != clip.looping
                || admitted.tracks as usize != clip.tracks.len()
            {
                return Err("Authored clip metadata disagrees with canonical SKL1".into());
            }
            let c = format!("{model}:clip:{native_id}");
            let clip_definition = node(
                g,
                study,
                "AuthoredClipDefinition",
                &clip.id,
                &format!("/motion/clips/{i}"),
            )?;
            g.triple(&c, &term("realizesDefinition"), &clip_definition)?;
            g.boolean(&c, &term("looping"), clip.looping)?;
            g.triple(
                &c,
                &term("timeEncoding"),
                &term("ExactSecondsInRetainedSource"),
            )?;
            for (t, track) in clip.tracks.iter().enumerate() {
                let tr = node(
                    g,
                    study,
                    "AuthoredMotionTrack",
                    &format!("{}:track:{t}", clip.id),
                    &format!("/motion/clips/{i}/tracks/{t}"),
                )?;
                g.triple(&c, &term("hasTrack"), &tr)?;
                g.triple(
                    &tr,
                    &term("targetsJoint"),
                    named
                        .get(track.joint_id.as_str())
                        .ok_or("Motion graph joint absent")?,
                )?;
                g.integer(&tr, &term("keyCount"), track.keys.len())?;
                g.triple(
                    &tr,
                    &term("rotationFrame"),
                    &term("RestLocalWxyzDeltaRadians"),
                )?;
            }
            for (a, contact) in clip.contacts.iter().enumerate() {
                let ct = node(
                    g,
                    study,
                    "AuthoredContactConstraint",
                    &format!("{}:contact:{a}", clip.id),
                    &format!("/motion/clips/{i}/contacts/{a}"),
                )?;
                g.triple(&c, &term("hasContactConstraint"), &ct)?;
                for (role, id) in ["hipJoint", "kneeJoint", "ankleJoint"]
                    .into_iter()
                    .zip(&contact.chain)
                {
                    g.triple(
                        &ct,
                        &term(role),
                        named.get(id.as_str()).ok_or("Contact graph joint absent")?,
                    )?;
                }
                g.triple(&ct, &term("attachmentFrame"), &term("AnkleLocalMetres"))?;
                g.triple(&ct, &term("targetFrame"), &term("AuthoredModelMetres"))?;
                g.triple(
                    &ct,
                    &term("constraintStatus"),
                    &term("AuthoredConstraintNotContactProof"),
                )?;
            }
        }
    }
    Ok(())
}

/// Project the controls actually used by the source geometry owner. Explicit
/// stations replace the legacy endpoint pair; zero-weight slots remain authored
/// controls but do not assert that their joint influences the generated loft.
fn loft_skin(
    g: &mut Graph, study: &str, part: &str, loft: &r3::Loft, index: usize,
    joints: &[String], cancel: &AtomicBool,
) -> Result<(), String> {
    if loft.skin_stations.is_none() {
        for &joint in &loft.joints {
            g.triple(part, &term("influencedByJoint"), &joints[joint])?;
        }
        if let Some(mirror) = loft.mirror_joints {
            for joint in mirror {
                g.triple(part, &term("mirrorInfluencedByJoint"), &joints[joint])?;
            }
        }
        return Ok(());
    }
    g.triple(part, &term("skinInterpolationPolicy"),
        &term("NormalizedLinearSectionFourSlotSkin"))?;
    for (profile, field, relationship, influence, side) in [
        (&loft.skin_stations, "skin_stations", "hasSkinStation", "influencedByJoint", "primary"),
        (&loft.mirror_skin_stations, "mirror_skin_stations", "hasMirrorSkinStation", "mirrorInfluencedByJoint", "mirror"),
    ] {
        let Some(profile) = profile else { continue; };
        // The admitted source has at most128 joints; no dynamic set is needed.
        let mut positive = [false; 128];
        for (section, station) in profile.iter().enumerate() {
            if cancel.load(Ordering::Acquire) {
                return Err("Authored skin graph cancelled".into());
            }
            let id = format!("loft:{index}:{side}:section:{section}");
            let path = format!("/lofts/{index}/{field}/{section}");
            let s = node(g, study, "AuthoredSkinStation", &id, &path)?;
            g.triple(part, &term(relationship), &s)?;
            g.integer(&s, &term("sectionIndex"), section)?;
            for slot in 0..2 {
                let joint = station.joints[slot];
                let target = joints.get(joint).ok_or("Skin graph joint absent")?;
                if station.weights[slot] > 0.0 {
                    *positive.get_mut(joint).ok_or("Skin graph joint exceeds profile")? = true;
                }
                let w = node(g, study, "AuthoredSkinInfluence",
                    &format!("{id}:slot:{slot}"), &format!("{path}/weights/{slot}"))?;
                g.triple(&s, &term("hasSkinInfluence"), &w)?;
                g.integer(&w, &term("influenceSlot"), slot)?;
                g.triple(&w, &term("targetsJoint"), target)?;
                g.number_f32(&w, &term("sourceSkinWeight"), station.weights[slot])?;
            }
        }
        for (joint, present) in positive.into_iter().enumerate() {
            if present { g.triple(part, &term(influence), &joints[joint])?; }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "wallaby_skin_tests.rs"]
mod skin_tests;
fn part(
    g: &mut Graph,
    study: &str,
    model: &str,
    definition: &str,
    id: &str,
    kind: &str,
    path: &str,
    material: Option<&String>,
) -> Result<String, String> {
    let p = node(g, study, kind, id, path)?;
    g.triple(definition, &term("hasPart"), &p)?;
    g.triple(&p, &term("representationResource"), model)?;
    g.triple(
        &p,
        &term("bindingStatus"),
        &term("SourceDeclaredPartBinding"),
    )?;
    g.triple(
        &p,
        &term("material"),
        material.ok_or("Authored part material absent")?,
    )?;
    g.triple(&p, &rdf("type"), &term("AuthoredPart"))?;
    g.triple(&p, &term("unit"), "http://qudt.org/vocab/unit/M")?;
    Ok(p)
}

#[cfg(test)]
#[path = "wallaby_chain_tests.rs"]
mod chain_tests;





