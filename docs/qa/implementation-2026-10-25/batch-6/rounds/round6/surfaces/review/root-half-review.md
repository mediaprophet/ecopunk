# Root integration half — independent source review

Reviewed the eleven requested integration candidate files and two live read-only
native-entry references. Exact inspected SHA-256 values are in
`root-half-pins.json`. No builds, tests, native execution, GUI, credentials or live
writes were performed. Surface leaf candidates were not modified in this review.

## Concrete issue reported

**R6-SURF-ROOT-01 — do not automatically extend retained native project grants.**
The reviewed `native_entry/create.rs::project_config` unconditionally appended
`ComputationalGeometry.revolve_profile`. Despite the file name, existing
`native_entry/runtime.rs` calls that same function while logging into a retained
`PublicProject` (around line 231). `PublicProject` has no persisted geometry grant
profile. Consequently an old project acquired the newly introduced target grant
without explicit trusted host opt-in. The earlier profile_sweep/uv_sphere/torus
entries already existed in this function; only revolution is new in this diff.

Root accepted the finding and specified a repair: retain the prior default grant
list, admit revolution only with explicit `POET_NATIVE_REVOLVE_PROFILE=1` trusted
host opt-in, and test a pure geometry-grant selection helper. Existing source
request and actual target checks still apply. This report pins the pre-repair
candidate; repair/typecheck/test evidence must be recorded separately. Persisted
admitted capability profiles remain later work, not an outcome of this repair.

## Other scoped conclusions

- PartView/source field names match the surface leaf API: `surface`,
  `uv_transform`, `uv_projection`; source mutations use the exact Vibe member
  names and full canonical visual surface updates. Identifier/name and unrelated
  part/material fields remain member references. UV projection removal writes
  null; identity UV transformation is reversible and no longer blocks geometry
  replacement merely because the field exists. Real topology-bound geometry
  fields, active projection and nonidentity part transforms still refuse
  unsupported topology replacement.
- New commands have strict tagged ingress. Surface intent deserialization uses
  the leaf strict typed DTO and common material validation, UV transform converts
  through the validated canonical wire DTO, projection calls its bounded source
  admission, and geometry recipes validate before native dispatch. No source
  expression is accepted in place of a typed recipe/material command.
- New native payload names match command decode, ActionType and scope. Surface,
  part-UV transform and projection are Invoke/Studio.edit; trusted package binding
  is Query/Studio.inspect with a SHA-guarded request. Kernel execution remains
  behind source invocation requests plus the actual GeometryHost target grant.
- Source-changing control callbacks capture the displayed SourceRef per render.
  `native_dispatch` refuses a captured target that differs from its current
  origin; the worker separately checks current source/session/execution epoch and
  authority before execution. This closes the previously reported dynamic-source
  callback rebasing concern. Surface leaf controls additionally pin loaded fields
  to inspected source/selected part. Existing draft preservation remains in the
  shared send path.
- Binding command names/request shape agree across controls, native intent,
  action and worker routing. The worker checks cancellation before binding and
  reports epoch retirement. Detailed resource-path/authority ownership is being
  reviewed separately; this review does not duplicate that half or infer its
  execution.

No additional blocking source issue was identified within this requested half.
This is source review only. Compilation, negative effect/grant tests, queued stale
event tests, authority/resource epoch tests, source-to-output persistence and
human native usability remain separate evidence. Assets remain HOLD; all model
targets and IP-03/FND acceptance remain incomplete.
