# Independent focused review: Poet M64 inspection candidate

Reviewed on 2026-10-25 by the separately assigned frames worker. Frozen
`poet-inspection/target-manifest.json` SHA-256:
`06646bd4d4577ea6ea610120972a019521c54d87bbe3934cc34c29f81a136e6b`.
All 15 candidate files matched their manifest hashes during this review.
The inspector contract and README were read together with the staged adapters,
service, worker, display predicates and authored security/freshness tests.
Unchanged authority, native owner, path/resource-policy and canonical checkpoint
load implementations were inspected read-only to trace the complete boundary.

No blocking correctness or authorization finding was identified within this
bounded read-only selected-resource inspection scope. This review makes no claim
about graphical usability, actual operator cooperation, human acceptance,
FND-05 or IP-03 C1–C8. Root separately owns UI/API integration review and actual
native verification. No candidate code, live sibling source or asset was edited;
no build, test, GUI/Computer Use, credential or network action ran here.

## Authorization and source binding

The new `AuthorizedEditorHost::inspect_package_resource` invokes the existing
`with_authorized` with the exact expected SourceRef and `AuthoringAction::ReadSource`.
The unchanged gateway authenticates against the current host-owned project,
document and source; verifies the actual Studio source's identifiers/domain,
Lamport revision, bounded source digest and exact stamp; and checks source
currency again after the read. The same native owner transaction guard spans
the operation. No serialized actor, request role, package metadata or inspection
result creates a permit.

`SessionEditorHost`, `SharedEditorHost` and `ResourceImportHost` delegate the
new method to that existing chain. SharedEditorHost retains one canonical
editor lock; it does not create another source/session/authorization owner.
CheckpointEditorHost checks the expected source before and after the service
read and requires the opaque source stamp to remain current. The trait default
refuses unsupported host profiles; there is no new successful unbound default.

The current source digest authorizes reading the associated document. A selected
HMC remains a separately identified saved snapshot: its entire SHA-256 binds its
saved source and selected resource together, and the result labels the saved
source revision explicitly. Inspection does not claim that saved historical
source bytes equal current accepted source bytes or that reading the snapshot
activates its source. This distinction agrees with the staged contract.

## Resource, package and path consistency

`checkpoints/inspection.rs::inspect_resource` validates bounded filename/key
intent, uses the existing host-owned path resolver and canonical bounded HMC
load, checks trusted source identifier/domain association, exact expected package
digest, immutable-release pin, selected key and resource digest before decoding.
Only recognized M64/E64 magic reaches the canonical M64Reader. Other resources
remain unsupported/unassessed; no alternate decoder or payload reinterpretation
was added.

After deriving details, the first carrier is dropped. The service resolves the
same host path and performs another canonical bounded load, source association,
release-pin and package-digest check before returning. Filename validation
rejects directory/path syntax and device names, and existing root canonical pins
and regular-file/link refusals remain in use. A changed package observed during
this scan is refused; selected resource bytes cannot silently follow another
digest. The operation does not edit or activate source, package bytes, resources
or opaque metadata.

As the contract states, this double read binds an inspected snapshot. It is not
perpetual disk freshness, protection against every malicious filesystem race,
cross-store atomicity or an external anti-rollback oracle. No stronger guarantee
was inferred from it.

## Delayed result and display freshness

`jobs.rs::execute` validates target SourceRef against origin and the admitted
starting Context before dispatch. After the operation, it obtains and rechecks
the current authoritative snapshot. Inspection details are retained only when
that final snapshot's complete Context equals origin and publication remains
current; this explicitly excludes read-only results following successor source,
session or actor contexts. Write results retain their existing separate successor
contract.

Before installation, `completion_is_current` rechecks current actor publication,
the requesting view's Context, final snapshot publication, worker latest Context
and current source stamp. The UI installs inspection only after that predicate.
The display predicate additionally requires matching Context, current source/
execution/authority publication, selected filename, package digest, resource key
and resource digest against the currently displayed inventory. Beginning another
selected inspection clears previous details. Clearing/refreshing inventory or
changing filename prevents an old result from being rendered under another
selection. These are render/publication checks; the review does not establish
instantaneous erasure of already displayed pixels or human GUI acceptance.

The authored worker test exercises a source replacement during an in-flight
inspection and withholds its details. It also checks a changed session revision
and retired execution epoch at publication. The authority test checks unbound,
revoked and stale-source refusal before the inner inspector dispatch. These are
appropriate focused cases to run; this reviewer did not execute them.

## Resource limits and evidence precision

The native action supplies a fixed 64-entry `M64PayloadInterval` stack array,
64 chunks, 262,144 records and 16,781,504 selected-payload bytes. That byte cap
includes the v3 header, 64 directory entries and padded raw records, and is
clamped to the configured checkpoint file-byte cap. M64 file/chunk/record
admission remains canonical. Early byte/chunk/scratch diagnostics report the
required quantity and limit; canonical early record refusal states that the
requirement exceeds the record cap without inventing an exact total.

Complete HMC loading is separately bounded by existing CheckpointLimits. It can
read/allocate more than the selected payload's 16 MiB profile and is performed
twice sequentially, with the first carrier dropped first. The README/contract
state that distinction; no whole-operation zeroheap, exact peak-memory or time
budget claim is made. Started inspection remains bounded and non-interruptible;
queued/window-abandoned work follows the existing worker behavior and delayed
publication guards.

No source-presence fact in this review is an executed test result. The six new
CPU tests, native compile/name-resolution/Dioxus integration, real operator use,
WASM interoperability and broader source-to-output cooperation still require
their separately recorded evidence. Complete model/package claims and all asset
production remain outside this slice; assets stay on hold.
