# Independent root integration review — UV, source and job routing

Status: **Freeze 2 source review complete; no remaining blocking source finding
in the assigned scope.** All three findings below were corrected and reinspected.
The exact install manifest SHA-256 is
`87c893608a76cda2c1abb38bc6484a58a3ea63c0820d415a720282867b7f2605`.
[root-integration-reviewed-pins.json](root-integration-reviewed-pins.json) records
26 assigned root source/test files and verifies candidate/live-preimage guards
for all 58 manifest entries. Hash verification of the remaining entries is not
a technical review of other workers' scopes. Root stage stayed read-only.
No Cargo commands, test runs or production-asset actions were performed.
The earlier [Freeze 1 pins](root-integration-reviewed-pins-freeze1.json) are
retained separately. Freeze 2's three changed assigned files were reinspected:
`surfaces/inspection.rs`, `authoring/atlas_detail_tests.rs` and the native project
UV/detail workflow. Inspection scratch uses `max(raw,split)` counts, and its
combined preview decode allowance includes other retained geometry/resources.
The new negative assertions' required Debug bound is present because the frozen
SurfaceInspection now derives Debug; an initial concern based on the old header
was withdrawn after a fresh read. This observation is not actual compilation.

## Remedy reinspection

- The root's revised DraftCursor stores actual full Context equality plus selected
  part and advances a checked monotonic token when either changes. This closes
  finding 1 without relying on Debug identity or discarding the local draft.
- Atlas admission now divides the host work-unit allowance by the number of
  active atlas recipes before all preflights, then supplies the same fraction to
  every apply call. This is conservative but supplies an aggregate allowance
  without copying the canonical estimator formula; finding 3 is closed at source
  level for each assembly admission. Whole-operation cancellation remains limited.
- Revised inspection borrows only the selected part and solves it once; it no
  longer clones/reconstructs the complete assembly. Prospective seam cardinality
  now refuses before inserting beyond the edge limit. The common borrowed
  `assembly/part_input.rs` preflight checks current geometry fields/counts,
  normal/UV values and active-atlas planar/affine conflicts before selected
  resolution. Duplicate selected identifiers also refuse. This handles scalar
  source mutations rather than assuming every retained binding was admitted.
  Its returned counts now enforce the 32,768 diagnostic-triangle and 64 MiB
  scratch ceilings before resolution/mesh allocation. Finding 2 is closed.
- The worker also appends an explicit selected-surface-context-changed diagnostic
  when a read succeeded but its pixels/edges are withheld at final publication;
  successor snapshots no longer imply unqualified successful pixel delivery.
- Two actual-worker payload publication regressions are staged separately in
  [job-regression](../job-regression/HANDOFF.md), guarded by its exact preimages
  and manifest. They include post-admission source/session/epoch successors and
  a positive current-payload control followed by session/rebuild invalidation.
  Rustfmt passed; these tests remain unexecuted. Root's frozen copies differ
  in line endings only under the text diff and are separately pinned with the
  actual final install candidate hashes.

## Original findings sent to root — all corrected above

1. **P1 — loaded local surface drafts omit actor/session/epoch context.**
   `authoring/surface_tools.rs` constructs its token from source reference and
   selected part only. If a successor actor/session/epoch selects the same source
   and part, equal accepted recipes leave both UV and texture-detail drafts ready.
   The actual dispatch does reauthorize the successor, so this is stale local
   draft mixing rather than an authorization bypass. Store the full `jobs::Context`
   with the selected part, or derive a UI generation from `Context` equality and
   include it in the token. Formatting `SessionEpoch` with Debug is insufficient:
   its derived representation displays the live AtomicBool value, while epoch
   equality deliberately uses Arc identity. Regress a successor actor/epoch with
   unchanged source/part/recipe and require explicit reload before apply.

2. **P1 — selected surface inspection clones and reconstructs the whole assembly
   before aggregate geometry admission.** `surfaces/inspection.rs` clones raw
   `parts` into a new assembly value, then resolves every detail and solves every
   atlas through `admit_assembly_with_materials`; it later solves the selected
   atlas again. The clone occurs before aggregate geometry preflight. A valid
   scalar-returning source cell can mutate `poet_parts` without returning an
   assembly, so current source/session state alone does not prove those parts
   satisfy geometry limits. Add a borrowed shape/size admission before any clone,
   and derive the selected-part diagnostic once using borrowed selected source
   and the accepted source/resource context. Reuse admitted derived data when
   whole-assembly validation is required. This also avoids unrelated expensive
   detail/atlas work for a selected-part inspection.
   The seam-edge BTreeSet also currently checks the 65,536-edge cap only after
   collecting all edges; the 32,768-triangle inspection cap can produce 98,304
   unique edges. Enforce prospective cardinality before growing past the edge
   cap, or conservatively admit the complete possible edge/scratch count first.

3. **P2 — atlas work is bounded per part but not charged across the assembly.**
   `assembly/admission.rs` correctly adds worst-case expanded vertex/index counts,
   but every atlas receives a fresh default 100M work-unit allowance. With many
   permitted parts, the total native operation can admit billions of work units;
   the new explicit inspection compounds that by reconstructing all parts.
   Introduce a shared canonical cost estimator and aggregate host-selected work
   admission before the first solve, as detail resolution already does for pixel
   visits. A conservative assembly-level ceiling or a clearly narrower total
   admitted scope is required before claiming a caller-bounded whole operation.
   Cancellation still uses false callbacks inside assembly/inspection; disclose
   its whole-operation latency unless the worker cancellation token is routed.

## Checked source contracts without a new finding

- Geometry admission retains the exact supported `vertices`, `triangles`,
  `normals`, `uv0` schema and rejects tangent, skin, morph, additional UV or other
  extension streams before narrowing. Atlas preflight charges the maximum
  three-corner-per-triangle expansion in aggregate output vertex counts before
  mesh cloning. The normal stream maps through the same corner source map as
  positions; missing normals continue through the existing flat-normal path.
- Active atlas admission requires null/absent planar projection and an identity
  part UV affine transform. It does not silently override authored values.
  Source editing writes only `uv_atlas`; raw geometry/topology/authored UV0 and
  unrelated fields remain by reference. Geometry replacement refuses while an
  atlas is active. Compilation and native preview feed the same adapter output
  into canonical assembly UV0 and the existing `.10d` UV section path.
- Texture detail edits use the existing material-sharing model, updating the
  retained part-level recipe on every part with the selected material identifier;
  they do not silently replace raw geometry or original map bindings. Required
  derived maps/resources come from the resolution owner rather than a UI cache.
- `CanonicalEditorHost::inspect_surface` checks the expected source and live
  execution epoch before reading the retained session. Shared and checkpoint
  wrappers forward the same expected reference; the session wrapper selects the
  current opaque session; the cooperative wrapper invokes existing ReadSource
  authorization rather than trusting a client principal.
- Native command decoding keeps inspect/query versus edit/invoke scopes distinct,
  checks bounded part identifiers, and serializes recipes through strict schemas.
  `native_dispatch` refuses a command target that differs from its admitted origin.
- The worker checks origin source, session, execution epoch and authorization
  before dispatch. Read-only surface data is withheld unless the final snapshot
  context equals the request origin. UI delivery repeats completion freshness and
  filters selected part and actual source; cached inspection display checks full
  Context equality and publication freshness. This prevents late pixel/edge
  disclosure into a successor source/actor context. These are source-level
  observations pending actual stale-context regression execution.
- UV inspection draws actual delivered triangle UVs for the current selected
  source; the atlas summary identifies local-metres-before-TRS density and local
  distortion extrema. Raw edges are source-index pairs for explicit seam edits.
  The diagram is a bounded diagnostic, not an accepted production output.

## Remaining qualification limits

The frozen unit-tile diagnostic now uses bounded parametric segment clipping
before pixel conversion; the previous endpoint-clamping limitation is corrected.
Reserved chart gutters are integer layout reservations; filtering, texture-edge
dilation and mip padding remain separate work. No native UI/human review,
whole-workload budget measurement, durable HMC round trip or gate completion is
established by this source review. Assets remain on hold.

The new root authoring and cooperative workflow definitions were read for their
bounded scope. They exercise original native intents, expected-source refusal,
recipe edits, planar/affine atlas conflict refusal, actual inspection payloads,
retained source recipes, resource/unknown-metadata closure, Q42 source/output
witnesses and reopen flow with explicit software actors. They are unrun evidence
definitions. Freeze 2 adds the two requested coverage definitions: a scalar
mutation can be admitted without returning an assembly, but inspection then
refuses the unsupported skin stream and, separately, an active atlas with a
nonidentity affine transform; the workflow reads the actual serialized UV01
section and compares the first flat-normal part's corner UVs with canonical
AtlasApplied output. Source part order is preserved by the existing assembly
owner, so the comparison addresses the intended serialized part. Both additions
remain unrun. They close the missing test-definition gaps, not their execution
or native acceptance requirements.
