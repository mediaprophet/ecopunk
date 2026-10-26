# Round 5 independent typed-resource and routing source review

Reviewer: `round5_authoring`, 2026-10-25. Reviewed the separate typed worker's
candidate and root's common integration/workflow; this is not independent
acceptance of this reviewer's own authoring implementation. No Cargo build,
test, UI action, live sibling write, asset edit or credential operation was run.

**Source disposition: no remaining blocker found in this bounded recheck.**
Execution and native usability remain unqualified. All eight typed staged files
matched `typed/manifest.json`; all nineteen integration staged files matched
`integration/target-manifest.json` at the final check. Temporary integration
manifest drift observed during formatting was resolved before this disposition.

## Findings closed by saved source changes

1. MID2 declarations were previously checked only when FieldSidecar existed.
   Final `typed_10d/payload.rs` checks every resolver declaration first and
   refuses non-FieldSidecar or empty bindings independently of sidecar presence.
   The new
   `typed_10d_no_sidecar_material_mesh_and_empty_declarations_refuse_without_hmc_mutation`
   case constructs the otherwise format-valid Materials, QuantizedMesh and empty
   bindings, requires payload refusal, and requires unchanged persisted HMC.
   This is authored test evidence, not an executed pass.
2. Root's native workflow fixture previously included an unsupported part
   `annotation`. The final admitted fixture uses the actual strict assembly
   schema. Source-only extension preservation remains a separate test scope.
3. Final `native_dispatch::run` checks every explicit Action source against its
   admitted origin before serialization, including typed/resource/authoring
   commands. The native workflow now requires a stale-Action/fresh-origin call
   to fail before its subsequent compile, preventing silent retargeting.

## Reviewed ownership and preservation paths

Typed inspection enters the existing `with_authorized(ReadSource)` path. Typed
writes enter `write_package`, whose existing `with_current_package` callback
holds the current package owner and whose inner authorization requires
SaveCheckpoint against the actual document/source stamp. Session/shared/package
decorators forward to that same owner. No request/file creates authority.

Service reads bind actual source, saved source, exact package and resource
digests. Selected metadata and unknown dependent schemas conservatively refuse
direct editing. MAT4 must regenerate byte-exactly before changing declared
visual factors; other material factors, geometry, frames/scientific payloads and
unrelated resources use canonical preservation paths. Scientific declarations
are structural and do not establish units, physical accuracy or source rights.

Packages containing Q42 conservatively refuse direct `.10d` factor replacement.
The root workflow instead creates real geometry/material/containment source
revisions, compiles `.10d` plus Q42, inspects typed output, requires unchanged
package bytes after direct-edit refusal, and exercises separate explicit
software review and durable reopen. The retained recipe, source/compiled digest
facts and package-review lineage are distinct from full part/design/instance
graph qualification, which remains open.

Jobs withhold typed inspection unless the original source/session context is
current; UI additionally binds its panel to the exact resource request. Saved
derivative completion clears package inventory. Same-source package-review
publication invalidation is covered by the root workflow's authored assertions.
The `foundation_fiel` Cargo target includes the actual three canonical checked
FIEL cases through public APIs in Poet's dependency closure; this review did not
compile or execute that wrapper.

## Final critical source pins (SHA-256)

Paths below are relative to the Round 5 directory. Complete file sets and
before pins remain in the two manifests.

| Staged path | SHA-256 |
|---|---|
| `typed/after/poet/native/src/graphical/checkpoints/typed_10d/payload.rs` | `4cfc3042b433444b888594ea04d8533a140bce626e0f5d30ba8fc70e1669ebe7` |
| `typed/after/poet/native/src/graphical/checkpoints/typed_10d/tests/admission.rs` | `390d1ddc556abe3b0b287e696b9b2e0f1a853206e0b20f32b52c80d3c385f92b` |
| `typed/after/poet/native/src/graphical/checkpoints/typed_10d/service.rs` | `3335d736b670497133399359f14196610aeea857b812a0d9f910433ff9320b6d` |
| `integration/after/native/src/graphical/cooperation/editor.rs` | `d0bf74cc7b78c184e45d092594dcbc13199f02d70db3a6f434eccf7df28ebd24` |
| `integration/after/native/src/graphical/jobs.rs` | `911fb274aa7a41ef0a1e5690b16d914377f69db6e927d2b02d3a3a9e5b8c589d` |
| `integration/after/native/src/graphical/package_typed_10d_view.rs` | `a3cc96e1e0e3911f41d283d289debfc49fe5dd1b275b6f1bd70b19216c110e95` |
| `integration/after/native/src/graphical/native_dispatch.rs` | `97e8ba9eb1a53dfdcbfd9c8bf3a9ff9cc0b36eda0b704de370a2ff287a900964` |
| `integration/after/native/src/graphical/native_project_spatial_tests.rs` | `304372bcfe7b2e5adf6dd2931bbdf433188fd493e3321d49f2ad13a5b3c3d4cf` |
| `integration/after/native/tests/foundation_fiel.rs` | `fadafc5bff9bc257458a6eac2f25e00a186a375dbf0627783e5140690ff1d59c` |

Remaining: supervisor compiler/tests against installed exact hashes, actual
native interaction, representative resources and contributor/reviewer usability.
FND-01–06 and IP-03 C1–C8 remain open within their wider scope. Assets stay HOLD;
film, open-world game and cartoon packages remain incomplete.
