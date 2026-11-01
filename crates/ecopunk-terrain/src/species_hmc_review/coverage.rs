//! Requirement coverage is a review projection, never inferred model content.
use super::*;
pub(super) fn build(
    subject: u8,
    family: Option<&FamilyReview>,
    resources: &[ResourceReview],
) -> Vec<CoverageReview> {
    let variants = family.map(|f| f.variants.as_slice()).unwrap_or(&[]);
    let mut out = Vec::new();
    let mut row = |dimension, value, ids: Vec<&'static str>, declared, applicability, note| {
        out.push(CoverageReview {
            dimension,
            value,
            catalogue_ids: ids,
            status: if declared {
                "declared_unaccepted"
            } else {
                "missing_or_unassessed"
            },
            applicability,
            note,
        })
    };
    row("package_resource","q42",vec!["KN-03","KN-04","KN-12"],resources.iter().any(|r|r.kind=="q42"),"required_corpus_coverage_review","Present means retained resource only; no adopted graph, query-index completeness, source-rights or biological assertion acceptance.");
    row("package_resource","m64",vec!["PKG-01","SRC-01"],resources.iter().any(|r|r.kind=="m64"),"resource_applicability_review","No M64 is fabricated to fill missing tensor/media data. Actual required resource modalities must be declared for the species case.");
    row("family_manifest","explicit_variants",vec!["ID-05"],family.is_some(),"required_variant_inventory","An unclassified current master is not automatically an adult, male, female or monomorphic organism.");
    if subject == 1 {
        for (value, p) in [
            ("male", BiologicalPhenotype::Male),
            ("female", BiologicalPhenotype::Female),
        ] {
            row("phenotype",value,vec!["ID-05","FAUNA-06"],variants.iter().any(|v|v.phenotype==p),"requested_wallaby_variant","Declared phenotype is retained source classification, not measured anatomy, universal sex equivalence or accepted geometry.");
        }
        row("reproductive_condition","pregnancy",vec!["FAUNA-06","BHV-07"],false,"requested_wallaby_case","No pregnancy representation or supported reproductive condition is inferred from a female label, body scale or generic abdomen control.");
        row("parent_offspring_relation","joey",vec!["FAUNA-06","BHV-07"],false,"requested_wallaby_case","Joey organism/stage and parent-pouch attachment/relationship require distinct authored and sourced bindings; none inferred from juvenile or female labels.");
    } else {
        row("reproduction","phenology_and_propagation",vec!["ID-05","FLORA-03","BHV-07"],false,"species_applicability_review","Botanical reproductive biology, seasonal/phenological forms and propagation require sourced applicable states; animal pregnancy labels are not applied to flora.");
        row("phenotype","species_sex_expression",vec!["ID-05","FLORA-03"],false,"species_applicability_review","Do not import mammalian male/female form assumptions into botanical reproductive biology.");
    }
    for (value, s) in [
        ("seed_or_spore", LifecycleStage::SeedOrSpore),
        ("juvenile", LifecycleStage::Juvenile),
        ("adult", LifecycleStage::Adult),
        ("senescent", LifecycleStage::Senescent),
        ("deceased", LifecycleStage::Deceased),
    ] {
        if subject == 1 && s == LifecycleStage::SeedOrSpore {
            continue;
        }
        row("lifecycle_stage",value,if subject==0{vec!["ID-05","FLORA-03","BHV-07"]}else{vec!["ID-05","FAUNA-06","BHV-07"]},variants.iter().any(|v|v.stage==s),"species_stage_applicability_review","Discrete stage declarations do not establish chronological age, transition dynamics or all compatible phenotype/state combinations.");
    }
    row("age","chronological_age",if subject==0{vec!["BHV-07","FLORA-03"]}else{vec!["BHV-07","FAUNA-06"]},false,"species_applicability_review","No numeric age, maturity schedule or age-to-size mapping is invented. Lifecycle stage and chronological age are distinct.");
    row("growth","developmental_transition",vec!["BHV-07","MOV-09"],false,"required_growth_review","Variant inventory alone is not coherent developmental anatomy/rig/material/behaviour; uniform scaling alone is not growth.");
    out
}
