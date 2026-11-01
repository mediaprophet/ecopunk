//! Reuse the checked Vibe source session while preserving admitted resources.
use super::{ResourceData, ReviewResourceLimits, ReviewResources};
use crate::{
    species_authored_review::{self, AuthoredReviewAsset},
    species_review_source::SpeciesReviewSourceSession,
};

pub struct SpeciesReviewPackageSession {
    subject: u8,
    source: SpeciesReviewSourceSession,
    resources: ReviewResources,
    limits: ReviewResourceLimits,
}
impl SpeciesReviewPackageSession {
    // Sibling corpus projection uses the admitted profile without exposing
    // mutable session fields or bypassing the transactional source owner.
    pub(super) fn corpus_profile(&self) -> (u8, ReviewResourceLimits) {
        (self.subject, self.limits)
    }
    pub fn new(subject: u8, limits: ReviewResourceLimits) -> Result<Self, String> {
        if subject > 1 {
            return Err("Unsupported authored species".into());
        }
        Ok(Self {
            subject,
            source: Default::default(),
            resources: Default::default(),
            limits,
        })
    }
    pub(crate) fn from_admitted(
        subject: u8,
        source: SpeciesReviewSourceSession,
        resources: ReviewResources,
        limits: ReviewResourceLimits,
    ) -> Self {
        Self {
            subject,
            source,
            resources,
            limits,
        }
    }
    pub fn revision(&self) -> u64 {
        self.source.revision()
    }
    pub fn sources(&self) -> &[String] {
        self.source.sources()
    }
    pub fn resources(&self) -> &ReviewResources {
        &self.resources
    }
    /// Exact source revision guard. Ordinary edits retain every admitted native
    /// resource and its opaque metadata. Detached validation precedes promotion.
    pub fn submit(
        &mut self,
        source: &str,
        expected_revision: u64,
    ) -> Result<AuthoredReviewAsset, String> {
        self.submit_checked(source, expected_revision, |_| Ok(()))
    }
    pub(crate) fn submit_checked(
        &mut self,
        source: &str,
        expected_revision: u64,
        validate: impl FnOnce(
            &crate::species_review_source::AuthoredSpeciesSource,
        ) -> Result<(), String>,
    ) -> Result<AuthoredReviewAsset, String> {
        if self.revision() != expected_revision {
            return Err("Source edit is stale; current revision/resources retained".into());
        }
        let (subject, limits, resources) = (self.subject, self.limits, &self.resources);
        self.source.submit(source, |authored| {
            validate(authored)?;
            resources.retain_in(species_authored_review::build(subject, authored)?, limits)
        })
    }
    /// Prepare a dependent runtime from the exact resource-preserving candidate
    /// before the source revision is promoted. Failure preserves both owners.
    pub(crate) fn submit_prepared<T>(
        &mut self,
        source: &str,
        expected_revision: u64,
        build: impl FnOnce(
            u8,
            &crate::species_review_source::AuthoredSpeciesSource,
        ) -> Result<AuthoredReviewAsset, String>,
        prepare: impl FnOnce(
            &crate::species_review_source::AuthoredSpeciesSource,
            &AuthoredReviewAsset,
        ) -> Result<T, String>,
    ) -> Result<(AuthoredReviewAsset, T), String> {
        if self.revision() != expected_revision {
            return Err("Source edit is stale; current revision/resources retained".into());
        }
        let (subject, limits, resources) = (self.subject, self.limits, &self.resources);
        self.source.submit(source, |authored| {
            let asset = resources.retain_in(build(subject, authored)?, limits)?;
            let prepared = prepare(authored, &asset)?;
            Ok((asset, prepared))
        })
    }
    /// Replace the complete detached resource inventory atomically with an
    /// actual authored source submission. Resource-only edits may submit the
    /// existing '= species_asset' expression and visibly advance revision.
    /// Failed resource/source/model/package validation preserves both owners.
    pub fn submit_with_resources(
        &mut self,
        source: &str,
        expected_revision: u64,
        resources: Vec<ResourceData>,
    ) -> Result<AuthoredReviewAsset, String> {
        self.submit_with_resources_prepared(source, expected_revision, resources,
            species_authored_review::build, |_, _| Ok(())).map(|(asset, ())| asset)
    }
    /// Replace resources and prepare a dependent runtime in the same detached
    /// transaction. The resource inventory is promoted only after preparation;
    /// failed decoding, runtime admission or cancellation leaves both owners.
    pub(crate) fn submit_with_resources_prepared<T>(
        &mut self,
        source: &str,
        expected_revision: u64,
        resources: Vec<ResourceData>,
        build: impl FnOnce(u8, &crate::species_review_source::AuthoredSpeciesSource)
            -> Result<AuthoredReviewAsset, String>,
        prepare: impl FnOnce(&crate::species_review_source::AuthoredSpeciesSource,
            &AuthoredReviewAsset) -> Result<T, String>,
    ) -> Result<(AuthoredReviewAsset, T), String> {
        if self.revision() != expected_revision {
            return Err("Source/resource edit is stale; current revision retained".into());
        }
        let resources = ReviewResources::admit(resources, self.limits)?;
        let (subject, limits) = (self.subject, self.limits);
        let result = self.source.submit(source, |authored| {
            let asset = resources.retain_in(build(subject, authored)?, limits)?;
            let prepared = prepare(authored, &asset)?;
            Ok((asset, prepared))
        })?;
        self.resources = resources;
        Ok(result)
    }
    /// Derive native resources from the exact detached candidate, without a
    /// second source submission that would invalidate their revision binding.
    pub(crate) fn submit_with_generated_resources_prepared<T>(
        &mut self,
        source: &str,
        expected_revision: u64,
        build: impl FnOnce(u8, &crate::species_review_source::AuthoredSpeciesSource)
            -> Result<AuthoredReviewAsset, String>,
        generate: impl FnOnce(&AuthoredReviewAsset) -> Result<Vec<ResourceData>, String>,
        prepare: impl FnOnce(&crate::species_review_source::AuthoredSpeciesSource,
            &AuthoredReviewAsset) -> Result<T, String>,
    ) -> Result<(AuthoredReviewAsset, T), String> {
        self.submit_with_authored_resources_prepared(source, expected_revision, build,
            |_, asset| generate(asset), prepare)
    }
    /// The exact checked source and its candidate are available together for
    /// native graph generation. No caller-reparsed or detached source is used.
    pub(crate) fn submit_with_authored_resources_prepared<T>(
        &mut self,
        source: &str,
        expected_revision: u64,
        build: impl FnOnce(u8, &crate::species_review_source::AuthoredSpeciesSource)
            -> Result<AuthoredReviewAsset, String>,
        generate: impl FnOnce(&crate::species_review_source::AuthoredSpeciesSource,
            &AuthoredReviewAsset) -> Result<Vec<ResourceData>, String>,
        prepare: impl FnOnce(&crate::species_review_source::AuthoredSpeciesSource,
            &AuthoredReviewAsset) -> Result<T, String>,
    ) -> Result<(AuthoredReviewAsset, T), String> {
        if self.revision() != expected_revision {
            return Err("Source/resource edit is stale; current revision retained".into());
        }
        let (subject, limits) = (self.subject, self.limits);
        let (asset, prepared, resources) = self.source.submit(source, |authored| {
            let base = build(subject, authored)?;
            let resources = ReviewResources::admit(generate(authored, &base)?, limits)?;
            let asset = resources.retain_in(base, limits)?;
            let prepared = prepare(authored, &asset)?;
            Ok((asset, prepared, resources))
        })?;
        self.resources = resources;
        Ok((asset, prepared))
    }
}
