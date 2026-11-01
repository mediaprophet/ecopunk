//! Cold owner reservations, checked before native snapshot replay begins.
use super::*;

#[derive(Clone, Copy, Debug)]
pub struct NativeProjectionBudget { pub cold_bytes: usize }
impl Default for NativeProjectionBudget {
    fn default() -> Self { Self { cold_bytes: 2 * 1024 * 1024 * 1024 } }
}
impl NativeProjectionBudget {
    /// Conservative simultaneous owner reservations, not a process-RSS measure.
    /// The evaluator, model, resource and player owners retain their own limits.
    pub(crate) fn admit(self, cells: &[String], resources: &ReviewResources,
        limits: ReviewResourceLimits, rig: RigLimits, motion: MotionLimits,
        options: &CorpusOptions<'_>) -> Result<(), String> {
        self.admit_with_extra(cells,resources,0,limits,rig,motion,options)
    }
    /// Reserve the complete prospective edit before a host clones source/resources.
    pub fn admit_source_edit(self,cells:&[String],resources:&ReviewResources,extra_bytes:usize,
        limits:ReviewResourceLimits,rig:RigLimits,motion:MotionLimits,options:&CorpusOptions<'_>)->Result<(),String> {
        if cells.len()>=128 || extra_bytes>1024*1024 {return Err("Prospective source edit exceeds existing replay profile".into());}
        self.admit_with_extra(cells,resources,extra_bytes,limits,rig,motion,options)
    }
    fn admit_with_extra(self,cells:&[String],resources:&ReviewResources,extra_bytes:usize,
        limits:ReviewResourceLimits,rig:RigLimits,motion:MotionLimits,options:&CorpusOptions<'_>)->Result<(),String> {
        let source = cells.iter().try_fold(0usize, |n, s| n.checked_add(s.len()))
            .and_then(|n|n.checked_add(extra_bytes))
            .ok_or("Projection source reservation overflow")?;
        if source>8*1024*1024 {return Err("Retained source exceeds existing replay profile".into());}
        let resource = resources.entries().iter().try_fold(0usize, |n, r|
            n.checked_add(r.bytes.len()).and_then(|v| v.checked_add(r.meta.as_ref().map_or(0, Vec::len))))
            .ok_or("Projection resource reservation overflow")?;
        let scaled = |n: usize, k: usize| n.checked_mul(k).ok_or("Projection reservation overflow");
        // Definitions survive across cells: reserve against total retained
        // source, not only the largest current parser input.
        let reservations = [scaled(source, 8)?, scaled(source, 64)?, scaled(resource, 2)?,
            // Existing data-only profile: 250,000 values, 16 MiB retained values.
            250_000 * 256, 4 * 16 * 1024 * 1024,
            scaled(limits.package_bytes, 4)?, rig.cold_bytes, rig.player_bytes,
            motion.cold_bytes, options.knowledge.cold_bytes];
        let required = reservations.into_iter().try_fold(0usize, |n, r| n.checked_add(r))
            .ok_or("Combined native projection reservation overflow")?;
        if required > self.cold_bytes {
            return Err("Native projection exceeds caller combined cold reservation".into());
        }
        Ok(())
    }
}
