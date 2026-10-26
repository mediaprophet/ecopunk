# UV edit preflight repair — independent bounded review

**No blocking source finding in the two-file repair.** Exact before, candidate
and current live-preimage guards matched
[uv-edit-preflight-reviewed-pins.json](uv-edit-preflight-reviewed-pins.json).
The repair manifest SHA-256 is
`3622b3eb0ee7f459575a0a23c06eff2a8cf8bd90cd3048d50670e5bdd3dc4419`.
The reviewer used read-only source/diff/hash inspection, wrote only this review
record, and ran no Cargo, test, export, GPU or live sibling mutation command.

The root-reported authoring run exposed that evaluating the incompatible planar
proposal before output rejection could leave the execution held, preventing the
subsequent explicit atlas removal in the unchanged regression. The repair adds
an immutable `Inventory::preflight_uv_edit(&AuthoringEdit)` over the actual
current part records and invokes it in `edit_source` immediately after inventory
construction, before candidate source generation/evaluation.

The early check refuses a non-null planar proposal or nonidentity UV affine
proposal while the current part has an active atlas. Enabling/replacing an atlas
also refuses an existing non-null planar projection or a nonidentity current
UV transform, parsed through the existing strict transform owner. Unknown part
selection refuses through the same inventory. Non-UV edits pass through without
new behavior. Explicit atlas removal, projection removal and identity affine
reset remain possible; no authored value is changed implicitly.

Both records and edit values are borrowed. This check performs no evaluation,
host invocation, journal mutation, source rewrite, session promotion or effect.
All unchanged part/geometry/material attributes still flow through the existing
reference-preserving source generator. Final geometry/atlas/material/output
admission is unchanged and remains necessary; this early check does not certify
topology, resources, new recipe validity or unrelated current part fields.

The caller remains the canonical `edit_assembly` route: expected source and
reconciliation/mutability checks precede source generation, and authorized
wrappers still own the current actor/session policy. Returning an error before
`submit` avoids a new candidate evaluation; it does not clear an already held
execution or bypass required rebuild/reconciliation. A raw Vibe source mutation
still receives the unchanged final output rules rather than silently acquiring
this UI/domain preflight as a replacement validator.

The two diffs contain only the one early call and the focused pure compatibility
method. No assertions were changed or weakened. The existing authoring regression
should now receive the incompatible-proposal error before candidate evaluation,
then continue its explicit removal and exact source-geometry restoration checks.
That outcome requires an actual rerun and is **not** claimed by this review.
The original failed run remains evidence; the active GPU run belongs to its
original frozen source until it exits, as directed by the supervisor.
