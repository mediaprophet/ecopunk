# Round 8 public source-carrier test review

Reviewed 2026-10-26 by the metadata implementation worker, independently of the
public-test author. Read-only review; no test/source changes, builds, runtime
commands, credentials or provider calls were performed by this reviewer.

Result: **no blocking finding in the inspected test candidates**. This is source
review, not an executed pass. The supervisor owns actual-source compilation,
execution, source pins and receipts.

## Reviewed inputs

- `tools/poet-foundation/round8/carrier-tests/manifest.json`: original public
  `native/tests/foundation_source_carrier.rs`, 485 lines, SHA-256
  `2b2810a986a74794db200ce457a71bf2fa43d6c77bf52060b6a3574a4e2d3ed9`, plus its
  explicit native-feature Cargo test route.
- `tools/poet-foundation/round8/carrier-restart/manifest.json`: thin module-route
  update of that public file, 487 lines, SHA-256
  `37edf0add629976a532ae64806e4151e459f48dc27dc907550fc62bf9a0feda7`, and new
  `native/tests/foundation_source_carrier/restart.rs`, 149 lines, SHA-256
  `b08497f9f32facedad5e8146a0dc2b562618b364ab11de7eae87983498319313`.

All three inspected file hashes matched their manifests. The restart manifest
before hash matches the original public-test after hash; the two-line module
route points to the companion in a subdirectory, avoiding an accidental second
Cargo integration-test crate.

## Public consumer assessment

The primary case uses actual public Poet/Core APIs: canonical Vibe submission,
source-bearing HMC binding, transform editing, explicit rebuild, fresh native
compilation, material editing and new-key replacement compilation, stale-disk
preimage refusal, SaveAs and a fresh service/interpreter. It checks real part
transform/material values and existence of actual compiled 10d output.

Its `assert_origin` compares the entire CURRENT SOURCE payload, including full
journal and host-qualified parent, against a fresh public stamped capture. The
metadata-bearing original intentionally has noncanonical source whitespace;
whole-HMC archive equality therefore proves preservation rather than a
reconstructed JSON approximation. It separately checks outer archive kind and
absent metadata, current source metadata absence, original resource bytes and
opaque metadata, all prior resources through replacement, no recursive origin
wrapping and unchanged original disk bytes.

Negative cases exercise both legacy-copy directions and encode after parent
change, foreign source, file/source/entry budgets, declared flags with repaired
CRC, payload/kind/metadata/metadata-absence conflicts, protected origin edits,
and canonical decode rejection of altered origin payload/kind/metadata/key
case. Refusals compare retained detached or installed bytes where relevant.
The same-store fresh-service sequence is correctly described as such; it is
not offered as fresh-process source activation evidence.

## Fresh-process companion assessment

The parent produces its HMC through actual public bind/transform/material/
compile routes, then launches the current test executable with the exact child
name `source_carrier_restart::source_carrier_restart_child`. That name matches
the declared module and child function. The parent retains its TempDir until
child exit, checks child success, then requires the child's expected output
file, its parent digest and unchanged original source file. Thus an accidental
zero-test successful child would still fail the parent at the missing output.

The child asserts its Studio source store is empty, uses native
`prepare_reopen` and `activate`, retains that same service, opens a held native
interpreter, and proves authoring inspection is unavailable before explicit
rebuild. It then checks actual transform/material output, exact unchanged
accepted artifact, all three accepted source cells, native SaveAs, exact current
source/journal/parent and original whole-HMC archive. It also requires only one
origin archive. These checks distinguish real cold source activation from
same-process reopen, and implicit source replay from explicit reconstruction.

The bare child test intentionally returns without the parent environment. A
passing bare child is not recovery evidence; the substantive parent case must
execute and pass. Both handoffs state this limitation.

## Qualification limits

The fixtures are synthetic and exercise CPU source, package and authoring
services without a GPU, graphical operator session, real human enrollment or
review, source-rights adoption or licence interpretation. The no-pixel provider
still validates real assemblies and surface bindings. The tests do not certify
full graph/rights derivation, native UI usability, model quality, FND gates or
IP-03 C1–C8. Asset production remains on hold.
