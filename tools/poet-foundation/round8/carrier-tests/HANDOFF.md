# Public native source-carrier regression candidate

Staged and unrun, 2026-10-26. Depends on the metadata worker's approved Core
`VerifiedSourceCarrier`/`Checkpoint::with_source_carrier` and native EditorHost
integration. No sibling writes or Cargo/build/test commands ran in this worker.

`manifest.json` pins the current Poet `native/Cargo.toml` before image, its new
explicit `foundation_source_carrier` route requiring `native-graphical`, and the
new external `native/tests/foundation_source_carrier.rs` candidate. The test file
is 485 rustfmt lines; `rustfmt --check --edition 2021` passed. It uses public
Poet/Core services against Poet's actual native dependency closure. Private Core
unit cases are not counted or substituted for these consumer cases.

The three public cases are:

* Native bind of an opaque-metadata source HMC; real Vibe transform, explicit
  rebuild and geometry compilation; fresh output, stale destination preimage
  refusal, changed material/new-key compilation into the existing output, SaveAs,
  and fresh service/interpreter bind/rebuild/compilation. Every output checks the
  entire current SOURCE payload/journal/parent against public stamped capture,
  original whole-HMC archive bytes/kind/metadata and original inert resource.
  Original disk bytes remain exact and origin archives do not recursively wrap.
* Read compatibility and fail-closed source-meta encoding/legacy copy in both
  directions, foreign-source rejection, entry/source/file budgets, and nonzero
  HMC flags with a repaired CRC so the flags policy is actually reached.
* Exact resource conflicts for payload/kind/metadata/absence, unchanged detached
  inputs after refusal, generic protected-origin Add/ReplaceBytes/Remove refusal,
  and origin payload/kind/metadata/key-case corruption rejection through canonical
  decode.

Synthetic fixtures intentionally contain source JSON whitespace and opaque CBOR
metadata; these are not invented valid signatures or human approvals. The native
preview provider validates actual assembly output and surface bindings but
deliberately renders no pixels. The fresh service/interpreter sequence reuses the
current stamped store; it is not a fresh-process source activation claim. The
separate source-safety restart test covers its own fresh-process HMC case.

The supervisor owns review, guarded installation after current commands exit,
then the `foundation_source_carrier` consumer target. No product readiness,
foundation, full graph/rights lineage, model quality or human acceptance follows
from these staged cases.
