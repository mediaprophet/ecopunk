//! Read-only replay adapter for an already authorised native source snapshot.
use super::*;
use crate::{species_review_source::{AuthoredSpeciesSource, SpeciesReviewSourceSession},
    species_review_resources::CorpusOptions};

impl AnimalMotionReviewSession {
    /// This data-only replay grants no host effects and writes no native source.
    /// The caller retains and checks its separate native document/authority stamp.
    pub(crate) fn from_authoritative_source_cells(
        cells: &[String], resources: ReviewResources, limits: ReviewResourceLimits,
        rig: RigLimits, motion: MotionLimits, options: &CorpusOptions<'_>,
        cancel: &AtomicBool,
    ) -> Result<Self, String> {
        if cells.is_empty() || cells.len() > 128 {
            return Err("Native species projection requires 1 through 128 data-only source cells".into());
        }
        let bytes = cells.iter().try_fold(0usize, |n, s| n.checked_add(s.len()))
            .ok_or("Native source byte count overflow")?;
        if bytes > 8 * 1024 * 1024 || cells.iter().any(|s| s.len() > 1024 * 1024) {
            return Err("Native species source exceeds the admitted replay profile".into());
        }
        // Revalidate the native resource projection under these caller limits.
        let resources = ReviewResources::admit(resources.into_entries(), limits)?;
        let mut source = SpeciesReviewSourceSession::default();
        let mut final_authored = None;
        for (index, cell) in cells.iter().enumerate() {
            if cancel.load(Ordering::Acquire) { return Err("Native source projection cancelled".into()); }
            // Intermediate cells update only the existing data-only evaluator;
            // expensive model/graph/runtime construction happens once at the end.
            final_authored = source.submit(cell, |a| Ok(if index + 1 == cells.len() {
                Some(AuthoredSpeciesSource { data: a.data.clone(), sources: a.sources.clone(),
                    revision: a.revision, source_sha256: a.source_sha256.clone() })
            } else { None }))?;
        }
        let authored = final_authored.ok_or("Native source replay has no final species value")?;
        // This factory is the existing r3 motion profile. Refuse another species
        // before invoking any geometry constructor for that other profile.
        let data = serde_json::to_string(&authored.data).map_err(|e| e.to_string())?;
        let typed = crate::swamp_wallaby_anatomical_r3::parse_source(&data)?;
        let cancelled = || cancel.load(Ordering::Acquire);
        let asset = crate::species_authored_review::build_with_production_limits(1, &authored,
            qualia_core_db::specialized_libs::computational_geometry::branch_graph_surface::BranchGraphLimits::default(),
            rig, crate::tree_junction::TreeJunctionLimits::default(),
            crate::tree_forks::AllForkLimits::default(), &cancelled)?;
        let key = crate::species_knowledge::authored_resource_key(
            authored.revision, &authored.source_sha256)?;
        let entries = if let Some(graph) = resources.entries().iter().find(|r| r.key == key) {
            // Reopening a native snapshot preserves the existing graph, including
            // custom assertions. A matching key alone is never sufficient.
            crate::species_knowledge::validate_authored_binding(graph,
                &authored.source_sha256, authored.revision, &asset.render.bytes,
                options.knowledge, cancel)?;
            resources.into_entries()
        } else {
            crate::species_review_resources::generate_corpus_resources(
                1, &authored, &asset, resources.into_entries(), limits, options, cancel)?
        };
        let resources = ReviewResources::admit(entries, limits)?;
        let asset = resources.retain_in(asset, limits)?;
        let player = if typed.motion.is_some() {
            Some(MotionPlayer::admit(&asset.render.bytes, &typed, rig, motion, cancel)?)
        } else { None };
        if cancelled() { return Err("Native source projection cancelled".into()); }
        Ok(Self {
            source: SpeciesReviewPackageSession::from_admitted(1, source, resources, limits),
            current: Some(Candidate { asset, player, vibe_source_sha256: authored.source_sha256 }),
            rig_limits: rig, motion_limits: motion,
        })
    }
}
