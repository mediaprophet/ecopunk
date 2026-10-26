# Round 3 — format and SDK foundation implementation

Authorized 2026-10-25: continue the foundation blockers, use scoped agents, save
and verify changes. Asset production is on hold until the applicable ecosystem
readiness gates qualify. This round does not create assets or confer acceptance.

| Job | Assignment | Concrete change and boundary |
|---|---|---|
| R3-GEO | gpt-6.1-sol, high | Opt-in developmental M64 georaster revision 3 with exact persisted origin, spacing, datum/epoch declarations, source extent, row direction, edge and nodata policy. Preserve existing v1/v2 reads, default version constant and legacy writer output. Canonical checked Core reader and geospatial compiler; no claimed datum transformation or authority. |
| R3-FRAMES | gpt-6.1-sol, high | Canonical caller-buffered NRM1/FRM1 decoding, complete preflight before output writes, explicit cold allocation budgets and native consumer wiring. Preserve old signatures and typed MID2 selection. |
| R3-PRESERVE | gpt-6.1-sol, high | Explicit structural opaque-section preservation API with bounds, CRC, budgets and transactional refusal. Keep strict semantic interpretation unchanged; opaque bytes never establish understood or authorized semantics. |
| R3-INVENTORY | gpt-6-luna, medium | Read-only source-pinned inventory of remaining gates and installed targets; completed inventory is separate from implementation evidence. |
| R3-POET | Reused gpt-6.1-sol geospatial worker, high | Typed read-only M64 inspection through existing native Poet package controls and the same authorized host/session chain. Bind source, package and resource digests; invalidate stale jobs. Poet-only targets preserve the Core freeze. Explicit fixed-scratch inspection limits are an operator profile, not an asset quality ceiling. |
| Integration/review | Root supervisor | Independently review frozen candidates, apply hash-guarded exact sibling targets through permission review, run one serial actual-source native conformance suite and reconcile saved-source results. |

Workers own disjoint staging trees under this directory. Root owns shared routing
edits and the conformance runner. No concurrent Cargo invocations. Staging is not
application; authored tests are not executed results. Every installed file must
have its prior and candidate digest recorded. A denied installation stays staged
until an authorized exact-target request is approved.

Compatibility contracts precede implementation. New M64 revision 3 is opt-in and
developmental; it does not announce a released standard or silently relabel old
files. Unknown `.10d` payloads can be preserved only under an explicit operation
contract; normal renderers must not infer support from a structural parse.

The existing terrain M64 and growing-station `.10d`/HMC remain immutable regression
inputs. No model claims, human reviews, grants, enrolments or graph authority are
created by these checks. Native tests, WASM compilation and actual WASM execution
are separate evidence. No GUI/Computer Use, credentials, network, publication,
commits, payments or production exports are part of this dispatch.

FND-01 through FND-06 remain open until their full requirements are evidenced.
This round reports exact subcases passed, failed or unrun and the next owning
layer/action. Future round counts require findings rather than a fixed promise.
