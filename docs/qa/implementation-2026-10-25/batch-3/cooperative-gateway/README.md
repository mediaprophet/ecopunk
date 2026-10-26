# Native cooperative gateway proposal

**Staged, not applied or qualified.** Parent owns native module routing,
`qualia-cooperative-core`/`sha2`/`serde_json` dependencies, controller updates,
actual coherent compilation and the production native authority provider.
No production principal or membership is bootstrapped here. No Computer Use,
sibling write, paid call, commit or publish was performed by this worker.

`cooperation::AuthorityOwner: Send + Sync` borrows a current native authority
view under its owner lock through an entire operation. `UnboundAuthorityOwner`
always denies. The `Scope` contains host-supplied project/document/source/domain
bindings and deliberately cannot be deserialized. These strings alone do not
authenticate anyone or authorize a write. The trusted provider must verify actual
native authentication, current identifier associations, admitted memberships,
review grants and revocation before constructing the view. It must preserve the
owner transaction and recheck time/generation at commit as appropriate.

`with_authorized(owner, scope, expected_source, action, limits, operation)` first
authorizes against the provider's current policy view, then checks actual stored
source identifiers/domain, opaque StoreStamp, revision and digest before invoking
the one-shot native operation. The operation receives the borrowed view and
consumable current `AuthoringPermit`, suitable for canonical contribution/review
record construction. Historical records cannot recreate a permit. The API
never takes a caller-supplied principal, grant, authority clock or generation.
The native current generation enters the canonical policy request.

`AuthorizedEditorHost` wraps the canonical host before either UI or MCP obtains
it. Snapshot/check/source-cell read use ReadSource; submit uses ExecuteSource;
material editing uses MaterialEdit; undo, redo, rebuild and checkpoint save use
their corresponding actions. Feature availability is a pure public catalog
query. Explicit source stamps remain checked by both this gateway and the
canonical controller. The controller retains responsibility for read-only replay,
source/session reconciliation, jobs and stale frame rejection.

The provider must call its callback exactly once. Never-call providers deny;
duplicate callbacks never execute the operation twice. An error or duplicate
callback after an operation is reported as potentially committed and requiring
reconciliation, with no retry or rollback claim. The provider must update its
admitted document metadata and atomically record required source/contribution/
review evidence after writes before another operation. This adapter does not
implement a cross-store transaction: failing to update metadata causes subsequent
actions to fail closed on digest/revision mismatch. Actual native transaction and
durable cooperative evidence wiring remain required.

## Source digest domain and bounds

`source_digest` hashes UTF-8 serde_json bytes of the tuple:

```text
("poet.source-authority.v1", id, did_q42, domain, title, routing_lane,
 sensitivity, context_did_hash, nodes, properties)
```

Node/property maps are native BTreeMaps, so ordering is deterministic. Lamport
revision is separately compared to current DocumentRevision. History and redo
are not current authored state and are excluded. Every native source-cell key
must form the existing contiguous identified sequence and contain `text`.
This is a hash projection for admission, not a new source/HMC file format or
authority record. The native owner uses this same helper when binding its
current source state; it supplies the admitted author subject separately.

Caller-selected limits bound scope text, node/cell scans and serialized digest
bytes; encoding streams into SHA256 without an output byte allocation. The
upstream StoreStamp API clones a stored artifact before gateway inspection, so
the native source owner's existing admission/journal memory bounds remain
necessary. This wrapper does not claim to independently bound that preexisting
snapshot allocation or impose a master quality ceiling.

## Verification status

Six staged tests use actual canonical source/controller/policy APIs with clearly
labelled fixture native principals. They cover default denial, observer write
denial, accepted canonical source edits with explicit native metadata refresh,
foreign source/digest/generation/revocation rejection, callback cardinality,
self-review denial and hash/source-cell bounds. No test creates a fake frame.
They are **not yet compiled/executed**, pending the parent's updated actual core,
Vibe and cooperative artifacts and coherent native build. Rustfmt passed for the
two proposal files. Both are below 500 lines. No authenticated human principal, native window,
MCP client, human review or complete HMC/production acceptance is established.

IP-03 C1–C8, the complete model packages and all broad production packets remain
open. Film, open-world game and cartoon master qualification remain incomplete.

The parent applied the original two-file gateway proposal separately. The original
manifest/candidate bytes remain the base version. Subsequent two-test MCP/native
protocol integration additions are isolated in [followup](followup/apply-manifest.json),
guarded against the parent-applied tests.rs digest and absent new mcp_tests.rs.
That follow-up remains unexecuted pending the parent's coherent native build.
