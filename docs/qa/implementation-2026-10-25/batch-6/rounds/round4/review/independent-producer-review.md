# Independent producer and native package projection review

2026-10-25. Read-only source review; no source edits, builds, tests, GUI,
credentials or asset operations. This review does not claim executed behavior.
The staged Core policy was reviewed separately by the producer worker; the two
supervisor-requested Core source-history/target-isolation corrections are in the
updated [target manifest](target-manifest.json). A later supervisor-requested
publication guard and qualified prior-contribution projection bring that Core
stage to 29 targets, patch SHA-256
`21b66bed150a53a3b49cdd52f30bb49c07f0a298b51d1d90ce7b572eb6848009`.

## Findings requiring resolution

**P1 — package workflow publication misses same-source package successors.**
In `integration/after/native/src/graphical/package_workflow/service.rs:54`, the
raw owner guard is dropped after a valid current package inspection. The later
check verifies authority generation/source, but a resource-only HMC save can
change package bytes without changing either. `jobs.rs:276` and `:298` check
source/session context again; `package_review_controls.rs:40` additionally
compares the view with its already loaded inventory digest. A concurrent MCP
save can leave all those comparisons true while the selected on-disk HMC has
advanced. Old decisions can then be displayed as current until inventory refresh.
Decision commit is protected by Core and the direct callback; this finding
concerns publication/consumption of the displayed review state. Add an exact
package freshness guard/token updated by every native save and rechecked at
publication/consume, or a fresh bounded canonical file check with equivalent
ownership. Retain the explicit external-writer moment-in-time limitation.

**P2 — the producer drops its synthetic source declaration when saving.**
`producer/after/native/src/graphical/package_georaster_create_view.rs:45` requires
the literal `synthetic constant grid; unqualified`. That declaration is absent
from `GeoRasterCreateRequest`; `checkpoints/georaster_edit/producer.rs:155` saves
the added M64 with `meta: None`. Reloaded package consumers therefore receive
the numeric authored frame/samples without the only explicit synthetic/observed
status declaration. A user-selected key is not provenance. Persist the typed
source declaration/status and applicable evidence in the canonical resource
metadata/source descriptor. Keep synthetic, supplied sample-array observation
and unknown provenance distinct; do not label all typed API input synthetic.

Both findings were sent to the supervisor immediately. These are source-review
findings, not executed reproductions. Recheck corrected hashes before closing.

The supervisor subsequently requested a Core nonblocking package-publication
guard. That method and its public integration regression are staged in the
current manifest; the test remains unrun. Closure of P1 also requires the root's
opaque Poet stamp, job withholding/UI checks and real-HMC successor regression.

## Inspected behavior with no additional blocker found

The 19-target producer stage uses public canonical M64 recovery/compile/check
APIs. Sample count, shape, caller output/chunk/record/scratch and working-byte
bounds precede fallible allocation. Edit regenerates the original elevation
payload and requires exact byte equality before changing it; richer fields,
unknown layouts and aliases are refused rather than discarded.

Create/edit route through `AuthorizedEditorHost::write_package`, the current
SaveCheckpoint/native owner and the exact source stamp. `CheckpointService`
uses the existing `save_with_resource_edits` carrier path: verified disk/retained
base, full resource copy, explicit Add/ReplaceBytes, bounded encode/decode, final
source/destination guards and canonical save. Unrelated resource bytes and
metadata are retained by that existing path. Actual preservation qualification
still depends on the supervisor's tests.

Cancellation is checked before admission/codec work and immediately before the
existing save operation. The already-started HMC capture/encode/install operation
has no mid-save cancellation contract; Worker documentation explicitly discloses
that started effects cannot be forcibly cancelled. Do not claim otherwise.

The package workflow callback acquires the raw shared owner after the native
authority and retains it in an outer variable until the authority call returns,
including journal append. It calls raw snapshot/package inventory methods and
does not recursively call AuthorizedEditorHost/native authority. The actual
NativeProject composition puts the raw canonical/checkpoint/import decorators
inside SharedEditorHost and AuthorizedEditorHost outside it.

Native grant/revoke actions obtain `enrollment_permission` from the current
session and use the existing protected `authority_signer`. Core rechecks current
stewardship/signature and invalidates sessions on signed admission commit. The
source-review action remains a separate API; no implicit package authority or
automatic human status was found in the inspected routes.

## Source pins for the findings

| Inspected file | SHA-256 |
|---|---|
| `integration/after/native/src/graphical/package_workflow/service.rs` | `fc3586bfbf8f61cce5feb36bae56c7c127fc19cbb0c88f51a6dd33fda7a89b3e` |
| `integration/after/native/src/graphical/jobs.rs` | `f7d583a040b3238022c9cb4746c61bac42c878555947c17f88133ee099c4f567` |
| `producer/after/native/src/graphical/checkpoints/georaster_edit/producer.rs` | `127c3b5a9e8983e6e73cb5c9498c1866d22c5efe2ded2bea4ff356e7d4d1970a` |

Assets remain on hold. Native operator/human acceptance and full HMC/model
semantic, rights, visual and scientific acceptance remain open.

## Corrected-source recheck

The supervisor requested a final source recheck after both repairs. **P1 and P2
are addressed in the inspected candidate sources; execution remains unverified.**
No further source blocker was found within this bounded review.

For P1, `PackageWorkflowView` now carries a private nonserialized native stamp
with the real authority/session/generation/exact package target. Its current
check also binds the source reference; jobs withhold stale views and the native
panel repeats the stamp check during display. Core's nonblocking current audit
check invalidates same-source package successors. Raw editor guards remain
retained through the authoritative operation. The staged real-HMC integration
test explicitly checks a separate authorized frontend's same-source edit
invalidates an old accepted software review, omits that decision under successor
bytes and refuses externally corrupted bytes on refresh. This supports the
required test design, not a pass claim. External writes remain detected by
bounded file verification on refresh/consume rather than continuous monitoring.

For P2, `GeoRasterCreateRequest.source` now requires a typed Synthetic, Provided
or Unknown declaration, with bounded description/reference and an explicit
observation-claim flag. Claims of observations require Provided plus a supplied
reference and remain Unverified. The saved resource metadata is bounded standard
CBOR containing that declaration and the actual original payload SHA-256. It is
an inert app-owned source annotation, not a scientific provenance attestation.
The native synthetic form retains its declaration and optional reference.

The subsequent semantic repair also prevents retaining misleading selected
metadata across sample edits. Only absent metadata or a complete bounded typed
Unverified Synthetic, non-observational annotation is editable; unknown fields,
trailing bytes, zero origins and Provided/Unknown/observational/dependent selected
metadata are refused. The origin digest describes original source payload;
current edited digest and package lineage remain separate. Other opaque resource
metadata retains its existing exact-copy path. Staged tests cover fresh reopen,
origin declaration retention and selected-metadata refusal; none were run here.

| Rechecked candidate | SHA-256 |
|---|---|
| `integration/after/native/src/graphical/package_workflow/mod.rs` | `cd150c98e0a267b2d8da3a8829416061d9b218f42736d0e1bddab9a1c7fd91dd` |
| `integration/after/native/src/graphical/jobs.rs` | `2d9cd6dc2d39976d8cb2b8067bdbe0ef9db57e80f88cdb4efceb552528eac1e5` |
| `integration/after/native/src/graphical/package_review_controls.rs` | `182cc75c942012b5c1e158a8f04568ac4a189233f624133144700aabd71dd624` |
| `integration/after/native/src/graphical/native_project_package_tests.rs` | `45907e0592258bcfe36c5ce10f8befaae36ba889f6fa0dec7b1ad8772991badb` |
| `producer/after/native/src/graphical/checkpoints/georaster_edit/source.rs` | `0fe448cda647975bbae4f886e4260704b39a4faa53e123dad533cd4a250d06a6` |
| `producer/after/native/src/graphical/checkpoints/georaster_edit/producer.rs` | `d30bf0ca276bda7bc8d041b58e14802a93aaeb4330abe2a705dc143d7974874b` |
