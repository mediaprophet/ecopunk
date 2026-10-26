# Exact compiler-input retention repair

Two staged native Poet targets only, with exact live preimages in `before/` and
replacements in `after/`, pinned by [manifest.json](manifest.json). The root
confirmed `authoring/package.rs` had no separate changed integration candidate;
its live source is the approved preimage. The original eight-target detail
manifest and stage are unchanged. No Cargo, live sibling write, UI or asset work.
Rustfmt passed; this repair has not been compiled or executed.

`CheckpointService::save_with_resource_edits` retains its signature and behavior
by delegating to new crate-private `save_with_retained_inputs` with empty inputs.
The new method has the same arguments plus `Vec<core::ResourceData>`. Ordinary
resource edits still require `ExpectedFile::Sha256`; trusted bound-input
retention also works on an absent destination. Source capture, retained/disk/
release carriers, parent/source/path/file guards and the canonical atomic write
remain in the same owner.

The crate-private `CheckpointService::merge_retained_inputs` adds only absent
complete records through `Checkpoint::apply_resource_edits`, retaining canonical
reserved-source, graph, aggregate byte/count and dependency validation. Same-key
records must agree in full bytes, kind and opaque metadata; conflicting duplicate
input declarations also refuse. It does not infer authority from equality. The
root promises `EditorHost::retained_surface_inputs(expected)` returns exact
verified **all non-source entries** from the authorized bound immutable HMC,
under current source/epoch/read-authority checks.

`authoring/package.rs` retrieves those inputs before compiling. Fresh save calls
the new trusted method with compiled assets and complete inputs in one native
transaction. Existing-destination planning first merges the complete inputs into
a detached canonical capture, then encodes/decodes compiled assets through Core.
Its inventory refuses payload/kind changes as before, and additionally refuses
opaque metadata digest/length changes before guarded destination edits. Existing
records use `ReplaceBytes` only after that exact metadata comparison, retaining
destination metadata. Canonical same-key asset collisions remain refusals; use
explicit fresh derivative keys.

Focused native-consumer regression proposal for the root's workflow target:

1. Build/bind an HMC containing an original base image with nonempty opaque
   metadata, an unused image, and another non-source opaque resource, each with
   distinct verified payloads/kinds/metadata. Author a paint or procedural detail
   recipe while retaining original material bindings.
2. Compile to an **absent** checkpoint, load it with Core, and compare all three
   full `ResourceData` records exactly. Assert new derived map dependencies and
   new native-source revision/witness are also present. Reopen, remove the recipe,
   rebuild using original bindings, save a new derivative key, and reopen again.
3. On an existing destination, supply same-key identical payload/MIME but
   different opaque metadata from the bound snapshot. Assert refusal and exact
   unchanged destination SHA and resource inventory.
4. Assert identical retained declarations deduplicate, conflicting full-record
   duplicates refuse, reserved native-source input refuses, and aggregate entry/
   byte exhaustion refuses before writing. Verify the legacy caller resource-edit
   API still refuses nonempty edits for `ExpectedFile::Absent`.

These are proposed executions, not fabricated passing evidence. Root owns the
promised trait/wrappers, retained-entry extraction, all final integration tests
and manifest composition. Asset production and all broad completion gates remain
held/open.
