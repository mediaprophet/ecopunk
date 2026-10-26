//! Cold bounded union of inert source-entry metadata and compiler resources.
//! The accepted current record is never replaced by a selected package record.
use super::{codec, resources, snapshot, Checkpoint, CheckpointLimits};
use std::io::{self, Write};

struct CountSource {
    bytes: usize,
    cap: usize,
}
impl Write for CountSource {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.bytes = self
            .bytes
            .checked_add(bytes.len())
            .filter(|n| *n <= self.cap)
            .ok_or_else(|| io::Error::other("carrier source exceeds caller budget"))?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn source_size(checkpoint: &Checkpoint, limits: CheckpointLimits) -> Result<usize, String> {
    let mut count = CountSource {
        bytes: 0,
        cap: limits.source_bytes.min(limits.file_bytes),
    };
    serde_json::to_writer(&mut count, &checkpoint.record).map_err(|e| e.to_string())?;
    Ok(count.bytes)
}

impl Checkpoint {
    /// Retain exact inert inputs from a verified same-subject/source-domain HMC.
    /// Use `decode`/`load` to validate package bytes before calling this method.
    ///
    /// Absence of source metadata is no declaration and cannot erase existing
    /// metadata. Two present declarations must match byte for byte. Every
    /// shared resource key must agree in payload, kind and metadata, including
    /// presence/absence. No metadata is interpreted as authority or execution.
    ///
    /// Keeps this CURRENT record and journal, including its existing qualified
    /// parent. It never copies the input's source body, parent, or file digest.
    /// The host separately qualifies any new parent against its selected base.
    /// The detached result has no file digest until encoded and decoded again.
    /// A refusal changes neither source store, files nor the borrowed carrier.
    pub fn with_retained_inputs_from(
        mut self,
        input: &Checkpoint,
        limits: CheckpointLimits,
    ) -> Result<Self, String> {
        if self.artifact().id != input.artifact().id
            || self.artifact().domain != input.artifact().domain
        {
            return Err("retained input belongs to another source/domain".into());
        }
        if let (Some(current), Some(selected)) = (&self.source_meta, &input.source_meta) {
            if current != selected {
                return Err("retained source-entry metadata conflicts with current carrier".into());
            }
        }
        // Bound source serialization and both entry sets before any retained
        // byte cloning or validation scratch. Counting does not allocate output.
        let source_bytes = source_size(&self, limits)?;
        source_size(input, limits)?;
        resources::validate(&self.resources, limits)?;
        resources::validate(&input.resources, limits)?;
        let mut entries = self
            .resources
            .len()
            .checked_add(1)
            .ok_or("entry overflow")?;
        let mut total = 128usize;
        let meta = self.source_meta.as_ref().or(input.source_meta.as_ref());
        codec::charge(
            &mut total,
            snapshot::SOURCE_KEY,
            snapshot::SOURCE_KIND,
            source_bytes,
            meta.map_or(0, Vec::len),
            limits.file_bytes,
        )?;
        for resource in &self.resources {
            codec::charge(
                &mut total,
                &resource.key,
                &resource.kind,
                resource.bytes.len(),
                resource.meta.as_ref().map_or(0, Vec::len),
                limits.file_bytes,
            )?;
        }
        for resource in &input.resources {
            if let Some(old) = self.resources.iter().find(|r| r.key == resource.key) {
                if old != resource {
                    return Err("retained resource payload/kind/metadata conflicts".into());
                }
                continue;
            }
            entries = entries.checked_add(1).ok_or("entry overflow")?;
            if entries > limits.entries {
                return Err("retained input union exceeds entry budget".into());
            }
            codec::charge(
                &mut total,
                &resource.key,
                &resource.kind,
                resource.bytes.len(),
                resource.meta.as_ref().map_or(0, Vec::len),
                limits.file_bytes,
            )?;
        }
        snapshot::validate_record(&self.record, limits)?;
        snapshot::validate_record(&input.record, limits)?;
        // All fallible admission has finished. No source/dependency edits are
        // inferred here; the canonical encoder validates the eventual package.
        self.resources
            .try_reserve_exact(entries - 1 - self.resources.len())
            .map_err(|_| "retained carrier allocation refused")?;
        for resource in &input.resources {
            if !self.resources.iter().any(|r| r.key == resource.key) {
                self.resources.push(resource.clone());
            }
        }
        if self.source_meta.is_none() {
            self.source_meta = input.source_meta.clone();
        }
        self.file_digest = None;
        Ok(self)
    }
}
