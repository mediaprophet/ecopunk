# Final bounded root capability and surface wiring recheck

**Source-review disposition: no remaining blocker in this reviewed scope.**
The exact combined install manifest is SHA-256
`0ee679a2e1024e909a3bb4cdb9f1db21d64808a631ff62dcf5a1145e82b220e8`.
All 53 staged file hashes matched it when checked. Selected rechecked inputs and
their hashes are retained in `final-root-pins.json`. No builds, tests, source
evaluation, native kernel/GPU execution, GUI or live writes occurred here.

## R6-SURF-ROOT-01 source closure

The new `native_geometry_grants(false)` returns the prior six-entry host list,
including invocation/extrusion/swung/profile sweep/UV sphere/torus. It does not
include revolution. Only `native_geometry_grants(true)` appends the new exact
`ComputationalGeometry.revolve_profile` identifier. `project_config` enables that
branch only when trusted process environment `POET_NATIVE_REVOLVE_PROFILE` equals
the exact string `1`; absent, other or non-Unicode values keep the legacy list.
This applies equally to retained project attachment and new project creation.
The prior unconditional retained-project elevation is therefore removed in
source. The pure helper test checks the legacy list prefix and single appended
grant; it was not executed by this reviewer.

The diagnostic/file-selected entry still requires an explicit known target grant
in its trusted configuration. Source capability requests, the native actual
target check and the cooperative actor/session grants remain separate gates.
This repair is process-local host opt-in, not a persisted project capability
profile or an automatic adoption of new grants by retained projects.

## Exact UI/source/DTO recheck

The formatted control callbacks still capture displayed SourceRef values for
surface/profile/CAD/TRS/material/parameter/structure edits and package binding.
The same source is carried into the Action; native dispatch compares it to the
current origin before worker admission. Surface controls remain selected-part
and loaded-source guarded. The source inspector exposes the matching leaf names
`surface`, `uv_transform`, `uv_projection` and the mutation adapter emits those
same Vibe record members.

Surface native commands use the strict leaf SurfaceDraft DTO, the four-field
canonical UV wire DTO with validated conversion, and the bounded planar
projection DTO. Payload command strings, Query/Invoke categories and native
scopes match their decode counterparts. No raw source expression was substituted
for these typed authoring arguments. Source mutations preserve unrelated fields;
identity UV reset no longer blocks geometry replacement solely due to field
presence, while real topology-bound attributes remain conservatively refused.

The final manifest includes the optional PNG retention leaf revision
`a13384470dea99e86a2724bec2abfbf8cf3da5b51e31c96e86a1c3e6fcc11a61`.
Its future retained images identify direct native GPU calibration readbacks,
separate from authored workflow and human acceptance. No images were produced
or inspected during this source review.

Root remains responsible for actual compiler/test/GPU receipts and their source
pins. Native usability, complete HMC/model targets and IP-03/FND acceptance are
not established by this review. Assets remain HOLD.
