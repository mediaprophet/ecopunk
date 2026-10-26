# Round 8 verified source carrier — independent review

2026-10-26. Reviewer: edit-safety worker. **No remaining blocking finding in the
frozen v2 candidate.** This is static source review, not compile, execution,
human acceptance or complete HMC/rights-lineage qualification. The reviewer made
no metadata candidate or sibling-source edits and ran no Cargo/build/test.

Reviewed canonical `before/` and `after/` images listed in
`tools/poet-foundation/round8/metadata/manifest.json`, candidate
`round8-source-carrier-v2`. All 20 before/after image hashes matched. The reviewed
manifest SHA-256 is
`b0093cf6960bf2f151bd220d68d97f69d8469e851969de136822c48762a0d66d`.
Transitional `core/` and `poet/` staging files and the rejected first candidate
are not the frozen installation contract.

## Findings resolved before freeze

1. **Legacy source-metadata stripping:** the first revised legacy helper refused
   a metadata-bearing base but could clear metadata from a metadata-bearing self
   when copying a metadata-free base. It now refuses if either side has source
   metadata; staged coverage exercises both directions. The encode guard cannot
   be bypassed by that legacy operation.
2. **Mutable digest-qualified origins:** initially the carrier generated a correct
   archive, but ordinary resource editing/encoding could retain an old digest key
   over changed bytes. The reserved namespace now requires the exact lowercase
   SHA-256 locator, `hmc` kind, absent outer metadata and matching payload digest.
   Decode and resource validation enforce it before retained payload cloning;
   generic Add/Replace/ReplaceBytes/Remove requests in that namespace refuse.
   Origin creation stays within the verified carrier path. Nested archive content
   is not recursively activated or decoded by this integrity check.
3. **Source-free snapshot compatibility:** native source-carrier discovery now
   returns None when no Studio SOURCE entry exists; existing ordinary bounded
   retained-input processing remains the fallback. Actual SOURCE-bearing inputs
   still require canonical checkpoint validation and matching artifact/domain.

These concerns were sent directly to the worker and supervisor before freezing.
The worker implemented the fixes; this reviewer re-read their frozen images.

## Contract and route assessment

Core owns `VerifiedSourceCarrier` construction through canonical decode. It keeps
the exact original HMC bytes and SHA-256 alongside its inert decoded checkpoint.
When opaque source-entry metadata is present, union onto a current capture adds
the complete original HMC as one digest-qualified inert resource. Its source JSON,
original key, opaque metadata, index and other container context remain inside
the original bytes. The metadata is never copied onto the fresh current SOURCE.
Even a parent-only or JSON-whitespace change therefore cannot silently change
the payload to which those opaque bytes refer.

`with_source_carrier` checks matching artifact and domain, requires current source
metadata absent, and changes only retained resources/file-digest state. It does
not substitute the input record, journal or parent. The native service selects
the verified destination/retained parent, captures the current stamped source,
unions base and bound input carriers, applies explicit ordinary edits, encodes,
then checks the current stamp and existing file preimage before atomic save.
The same textual identifier or content hash grants no authority.

Current source JSON is counted against source/file limits without a serialization
allocation before carrier payload cloning. Existing and incoming resource sets
are validated, exact overlaps deduplicated, and aggregate current-source,
resource, metadata, entry and whole-archive costs admitted before cloning the
union. Different payload/kind/metadata, including metadata absence, refuses.
The new source-owned archive is charged in full. This is bounded cold allocation
admission, not a measured process-memory ceiling or cancellation benchmark.

The native EditorHost seam is additive. Session/shared/package/entry decorators
forward it through their existing owners; the authorized decorator retains its
current ReadSource guard. Canonical discovery checks the exact expected source
on both sides of inert decoding. Held execution can retain inputs without
implicitly rebuilding; compilation retains its current reconstructed-publication
gate. Opening/activation retains the original carrier and leaves execution held
for a separate explicit native rebuild.

Fresh compile and Save use the carrier directly. Existing-package compilation
skips incoming retained origins/resources when forming generic ResourceEdit
requests, then unions them through the final trusted carrier path. This avoids
both protected-origin edit refusal and silent omission. Changed compiled output
still requires a fresh explicit asset key, retaining prior .10d/Q42 resources.
SaveAs retains the prior carrier; metadata-free current packages carry existing
origin archives without recursively wrapping the current package.

## Verification still required

The candidate's eight Core unit cases and migrated resource cases remain unrun
by this reviewer. The separately staged public consumer target at
`tools/poet-foundation/round8/carrier-tests/manifest.json` uses actual native
EditorHost bind/edit/rebuild/compile/save routes plus public Core conflict,
budget, flags and archive-integrity cases. This reviewer authored those consumer
cases; their review and execution belong to the supervisor, not to an invented
independent passing-test claim.

The compatibility change is deliberate: metadata-bearing old checkpoints remain
readable, while direct encode and legacy copying now refuse until callers make
a fresh current capture and retain the verified original. Existing consumer and
fresh-process reopen tests must qualify that migration. Current source/opaque
resource preservation is separate from adopted graph derivation semantics,
signature interpretation, rights/author acceptance and complete embodiment.

No assets or model claims changed. FND-01–06, IP-03 C1–C8 and film/open-world
game/cartoon package acceptance remain open.
