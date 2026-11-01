//! Authored geometric recipes projected as source facts, not executed evidence.
use super::super::{
    authored::{node, vector},
    graph::{escaped, term, Graph},
};
use crate::swamp_wallaby_anatomical_r3 as r3;
use std::collections::BTreeSet;

pub(super) fn groom(
    g: &mut Graph,
    study: &str,
    model: &str,
    definition: &str,
    source: &r3::Source,
    materials: &[String],
) -> Result<(), String> {
    let Some(recipe) = source.groom.as_ref() else {
        return Ok(());
    };
    let recipe_node = node(g, study, "AuthoredGroomRecipe", "groom", "/groom")?;
    g.triple(definition, &term("hasGroomRecipe"), &recipe_node)?;
    g.triple(&recipe_node, &term("representationResource"), model)?;
    g.triple(&recipe_node, &term("groomRecipeStatus"), &term("SourceDeclaredGroomControlsNotExecutionProof"))?;
    g.triple(&recipe_node, &term("geometryMethod"), &term("AreaPointRegionRejectionNativeBezierFibresV1"))?;
    g.triple(&recipe_node, &term("groomSamplingRule"), &term("AreaWeightedPointRegionRejectionBarycentricRoots"))?;
    g.triple(&recipe_node, &term("groomCurveRule"), &term("CubicBezierFibreCentreline"))?;
    g.triple(&recipe_node, &term("geometryOwner"), "urn:ecopunk:source-owner:ecopunk-terrain:swamp-wallaby-r3:groom")?;
    g.integer(&recipe_node, &term("seed"), recipe.seed as usize)?;
    g.number_f64(&recipe_node, &term("densityPerSquareMetre"), recipe.density_per_m2)?;
    g.integer(&recipe_node, &term("maximumStrands"), recipe.max_strands)?;
    g.integer(&recipe_node, &term("segmentsPerStrand"), recipe.segments)?;
    for (axis, value) in ["Minimum", "Maximum"].into_iter().zip(recipe.length_m) {
        g.number_f32(&recipe_node, &term(&format!("strandLength{axis}")), value)?;
    }
    for (axis, value) in ["Minimum", "Maximum"].into_iter().zip(recipe.width_m) {
        g.number_f32(&recipe_node, &term(&format!("strandWidth{axis}")), value)?;
    }
    g.triple(&recipe_node, &term("lengthUnit"), "http://qudt.org/vocab/unit/M")?;
    g.triple(&recipe_node, &term("widthUnit"), "http://qudt.org/vocab/unit/M")?;
    g.number_f32(&recipe_node, &term("lift"), recipe.lift_m)?;
    g.triple(&recipe_node, &term("liftUnit"), "http://qudt.org/vocab/unit/M")?;
    vector(g, &recipe_node, "direction", recipe.direction)?;
    g.number_f32(&recipe_node, &term("guardFraction"), recipe.guard_fraction)?;
    g.number_f32(&recipe_node, &term("rootTone"), recipe.root_tone)?;
    g.number_f32(&recipe_node, &term("tipTone"), recipe.tip_tone)?;
    g.number_f64(&recipe_node, &term("maximumInfluenceLoss"), recipe.max_influence_loss)?;
    g.integer(&recipe_node, &term("coldReservationBytes"), recipe.cold_bytes)?;
    g.integer(&recipe_node, &term("workUnitBudget"), recipe.work_units)?;
    g.triple(&recipe_node, &term("groomMaterial"), materials.get(recipe.material).ok_or("Groom material absent from authored palette")?)?;

    let mut seen_materials = BTreeSet::new();
    for (ordinal, &index) in recipe.surface_materials.iter().enumerate() {
        let material = materials.get(index).ok_or("Groom surface material absent from authored palette")?;
        if !seen_materials.insert(index) {
            return Err("Duplicate groom surface material in authored knowledge projection".into());
        }
        let binding = node(g, study, "AuthoredGroomSurfaceMaterial", &format!("material:{ordinal}"), &format!("/groom/surface_materials/{ordinal}"))?;
        g.triple(&recipe_node, &term("selectsSurfaceMaterial"), &binding)?;
        g.integer(&binding, &term("materialOrdinal"), ordinal)?;
        g.triple(&binding, &term("authoredMaterial"), material)?;
    }
    for (ordinal, region) in recipe.regions.iter().enumerate() {
        let region_node = node(g, study, "AuthoredGroomRegion", &region.id, &format!("/groom/regions/{ordinal}"))?;
        g.triple(&recipe_node, &term("hasGroomRegion"), &region_node)?;
        g.integer(&region_node, &term("regionOrdinal"), ordinal)?;
        vector(g, &region_node, "boundsMinimum", region.bounds_min_m)?;
        vector(g, &region_node, "boundsMaximum", region.bounds_max_m)?;
        g.triple(&region_node, &term("boundsUnit"), "http://qudt.org/vocab/unit/M")?;
        vector(g, &region_node, "direction", region.direction)?;
        g.number_f32(&region_node, &term("densityScale"), region.density_scale)?;
        g.number_f32(&region_node, &term("lengthScale"), region.length_scale)?;
        g.number_f32(&region_node, &term("widthScale"), region.width_scale)?;
        for (material_ordinal, &index) in region.surface_materials.iter().enumerate() {
            let material = materials.get(index).ok_or("Groom region material absent from authored palette")?;
            let binding = node(g, study, "AuthoredGroomRegionSurfaceMaterial", &format!("{}:material:{material_ordinal}", region.id), &format!("/groom/regions/{ordinal}/surface_materials/{material_ordinal}"))?;
            g.triple(&region_node, &term("selectsRegionSurfaceMaterial"), &binding)?;
            g.integer(&binding, &term("materialOrdinal"), material_ordinal)?;
            g.triple(&binding, &term("authoredMaterial"), material)?;
        }
    }
    Ok(())
}

pub(super) fn junctions(
    g: &mut Graph,
    study: &str,
    model: &str,
    definition: &str,
    source: &r3::Source,
    materials: &[String],
) -> Result<(), String> {
    let Some(set) = source.surface_junctions.as_ref() else {
        return Ok(());
    };
    let set_node = node(g, study, "AuthoredSurfaceJunctionSet", "surface-junctions", "/surface_junctions")?;
    g.triple(definition, &term("hasSurfaceJunctionSet"), &set_node)?;
    g.triple(&set_node, &term("representationResource"), model)?;
    g.triple(&set_node, &term("junctionSetStatus"), &term("SourceDeclaredJunctionControlsNotExecutionProof"))?;
    g.triple(&set_node, &term("junctionMethod"), &term("ExplicitSampledSurfaceGraftV1"))?;
    g.triple(&set_node, &term("geometryOwner"), "urn:ecopunk:source-owner:ecopunk-terrain:swamp-wallaby-r3:junctions")?;
    g.integer(&set_node, &term("junctionColdReservationBytes"), set.cold_bytes)?;
    g.integer(&set_node, &term("junctionWorkUnitBudget"), set.work_units)?;
    for (ordinal, recipe) in set.recipes.iter().enumerate() {
        if !source.lofts.iter().any(|loft| loft.id == recipe.torso_loft)
            || !source.loft_chains.iter().any(|chain| chain.id == recipe.limb_chain)
        {
            return Err("Surface-junction recipe references an absent authored loft or chain".into());
        }
        let recipe_node = node(g, study, "AuthoredSurfaceJunctionRecipe", &recipe.id, &format!("/surface_junctions/recipes/{ordinal}"))?;
        g.triple(&set_node, &term("hasJunctionRecipe"), &recipe_node)?;
        g.integer(&recipe_node, &term("recipeOrdinal"), ordinal)?;
        g.triple(&recipe_node, &term("representationResource"), model)?;
        g.triple(&recipe_node, &term("junctionRecipeStatus"), &term("SourceDeclaredJunctionControlsNotExecutionProof"))?;
        g.triple(&recipe_node, &term("junctionContinuityStatus"), &term("DiscreteSampledGraftNotAnalyticContinuity"))?;
        g.triple(&recipe_node, &term("junctionMethod"), &term("ExplicitSampledSurfaceGraftV1"))?;
        g.triple(&recipe_node, &term("torsoLoftPart"), &format!("{study}:LoftPart:{}", escaped(&recipe.torso_loft)))?;
        g.triple(&recipe_node, &term("limbLoftChainRecipe"), &format!("{study}:AuthoredSerialLoftChainRecipe:{}", escaped(&recipe.limb_chain)))?;
        g.boolean(&recipe_node, &term("mirroredJunction"), recipe.mirrored)?;
        g.integer(&recipe_node, &term("bodyRowStartInclusive"), recipe.body_rows[0])?;
        g.integer(&recipe_node, &term("bodyRowEndInclusive"), recipe.body_rows[1])?;
        g.integer(&recipe_node, &term("bodyColumnStartCyclic"), recipe.body_column_start)?;
        g.integer(&recipe_node, &term("bodyColumnCount"), recipe.body_column_count)?;
        g.integer(&recipe_node, &term("limbCutRow"), recipe.limb_cut_row)?;
        g.integer(&recipe_node, &term("ringRotation"), recipe.rotation)?;
        g.integer(&recipe_node, &term("junctionSubdivisions"), recipe.subdivisions)?;
        g.number_f64(&recipe_node, &term("bodyTangentScale"), recipe.body_tangent_scale)?;
        g.number_f64(&recipe_node, &term("limbTangentScale"), recipe.limb_tangent_scale)?;
        g.number_f64(&recipe_node, &term("maxJunctionInfluenceLoss"), recipe.max_influence_loss)?;
        g.number_f64(&recipe_node, &term("minBoundaryNormalDot"), recipe.min_boundary_normal_dot)?;
        if let Some([x, y, z]) = recipe.interior_bulge_m {
            g.triple(&recipe_node, &term("interiorBulgeMethod"), &term("DegreeElevatedCubicModelVectorInteriorBulgeV1"))?;
            g.number_f64(&recipe_node, &term("interiorBulgeXMetres"), x)?;
            g.number_f64(&recipe_node, &term("interiorBulgeYMetres"), y)?;
            g.number_f64(&recipe_node, &term("interiorBulgeZMetres"), z)?;
            g.triple(&recipe_node, &term("interiorBulgeUnit"), &term("Metre"))?;
            g.triple(&recipe_node, &term("interiorBulgeFrame"), &term("SourceRestModelXYZ"))?;
        } else {
            g.triple(&recipe_node, &term("interiorBulgeMethod"), &term("CubicInteriorUnmodified"))?;
        }
        g.triple(&recipe_node, &term("junctionMaterial"), materials.get(recipe.material).ok_or("Junction material absent from authored palette")?)?;
    }
    Ok(())
}

