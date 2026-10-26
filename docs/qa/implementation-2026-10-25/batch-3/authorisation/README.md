# Bounded cooperative authoring authority foundation

Staged on 2026-10-25 for independent review and guarded application. This is a
policy/evidence implementation in `qualia-cooperative-core`, not an installed
production authentication provider, membership database, payment mechanism or
IP-03 C1–C8 completion claim. No sibling source was changed by this worker.

`apply-manifest.json` pins one minimal `lib.rs` routing addition and five new
directory-backed module files. Null original hashes require target absence.
`source-pins.json` pins the actual owners inspected. The harness imports actual
canonical Project, ProjectMembership and Contribution source, compiling the
proposed module without a full `qualia-core-db` build.

## Public integration contract

`authorize(&HostAuthorityView, &AuthoringRequest) -> Result<AuthoringPermit,
AuthoringError>` borrows native owner state. The request carries only an action,
exact target `DocumentRevision` and expected authority generation. The view must
come from trusted Rust host code after authentication, identifier association and
project/review admission checks. These view/principal/admission types deliberately
do not implement Deserialize. Do not construct them from source, MCP arguments,
`--principal-id`, checkpoint records or the local social roster.

The native owner holds its current-state lock/transaction while authorizing and
performing an operation. It supplies current time, current source state and a
generation changed on every relevant session/identifier/membership/review change.
Reauthorize every action, including save, reopen, contribution and review. Never
cache the borrowed permit, use source-supplied time as the host clock, or release
the owner lock before source/evidence commit. Asynchronous work must recheck at
commit. The pure policy cannot verify an external host's clock, proof or locking.

`DocumentRevision` binds project, document, editable source identifier, source
revision, SHA256 bytes and admitted last-author referent. The host computes these
from actual current source. None is a StoreStamp, authentication grant or universal
identifier. Reopening a checkpoint never reinstalls saved authority. The current
target and candidate checkpoint target both need admission by the integration
owner before activation; this policy's ReopenCheckpoint checks its supplied exact
current target and cannot inspect an independent package by itself.

Covered actions: ReadSource, ExecuteSource, MaterialEdit, Undo, Redo, Rebuild,
SaveCheckpoint, ReopenCheckpoint, ContributionRecord, ReviewDecision. Direct
authenticated membership is required. Observers can read; review requires both
membership and an explicit current document grant. Steward does not imply review.
Unknown authors and self-review deny. Future/expired/revoked membership,
identifier or review authority deny. Ambiguous matching admissions/associations
deny. A separate delegated-agent adapter remains open; generic
`agency_delegation::delegation_permits` is preserved and is not repurposed as
project authentication.

`AuthoringPermit::contribution_record(...)` consumes a ContributionRecord permit,
checks the current author, and returns canonical Contribution plus exact source,
qualified identifier association, member/admission/authentication references,
actor kind and generation. Effort is whole minutes; capital is not recorded by
this API, no mixed-unit valuation runs, and privacy defaults to Private. WorkStatus
and its evidence reference are an attributed assertion, not verified settlement.
An optional predecessor must belong to the same project/contributor/identifier.
Legacy contribution IDs/API names and `urn:wellfair:*` helpers stay unchanged.

`AuthoringPermit::review_record(...)` consumes a ReviewDecision permit and binds
the actual decision/rationale/evidence to the exact source and admitted reviewer.
Actor kind remains explicit: a software agent decision is not a human acceptance
gate. Missing decision evidence/rationale fails. Deserialize reconstructs historical
records only; it never recreates a permit. The native cooperative persistence
owner still must atomically persist/deduplicate contribution/review records with
source revisions and preserve predecessor provenance. This worker did not invent
a second Poet-local store or claim that persistence wiring was completed.

## Native provider investigation and concrete remaining work

* `poet_host/invoke/governance/capability.rs::current_user` returns a hardcoded
  `principal_default`. It cannot authenticate the invoker.
* `qualia-client-core::project_collab` explicitly stores an always-available local
  JSON social roster. It is not project admission evidence.
* `qualia-client-core::wellfair::api::coop` journals ProjectMembership and
  Contribution through the existing vault/owner path. Its public typed add method
  alone is not proof of authorized admission. `WebizenHostApi::new` receives owner
  and author DID strings plus a signing key; those strings do not prove their key
  mapping. Preserve the canonical owner rather than copying its store into Poet.
* `qualia-client-core::handshake::verify_response` verifies a signature against the
  response's embedded key; `responder_is` compares DID strings. They need a separately
  trusted key-to-identifier association plus fresh nonce/session lifecycle before
  they can supply an authenticated project member.
* `net::qdnf::authority::AuthorityOwner` owns bounded generation-bearing grants,
  revocation and ExecutionPermit issuance. It is the existing native authority
  seam to reuse; a digest binding or installed local grant does not itself prove
  project membership or an authenticated person association.

Required host adapter: use an actual authenticated local/remote native session
with verified key/issuer associations, resolve admitted cooperative journal
membership/review decisions, and borrow them under the owning lock into this
view. Bind that provider at native Poet/MCP startup; unbound stdio defaults to
denial. No turnkey inspected provider already performs all these checks. Source
and signed-journal verification, trusted admission/revocation policy, durable
cooperative evidence commit and production provider wiring remain integration
work. The Rust fixtures are explicitly test principals, not credentials for real
people or permission to synthesize their acceptance.

## Executed verification and limits

Command from `C:/github/ecopunk`:

```powershell
cargo test --manifest-path docs/qa/implementation-2026-10-25/batch-3/authorisation/harness/Cargo.toml --offline --locked
```

Final result: 13 passed, 0 failed. The first run had 11 passes; new tests then
qualified ambiguous revoked identifier associations and preferred canonical
high-resolution membership time. Final compilation emitted cache hard-link
warnings and fell back to copying; compilation and tests exited 0. No production
authentication, native UI, MCP integration, durable cooperative journal, HMC activation,
human acceptance, payment or full-stack conformance was exercised by these tests.
`verification.json` records this bounded result. Parent owns independent review,
guarded application and coherent actual integration checks.

The policy allocates no heap and iterates caller-bounded slices. Record construction
checks text and conservative byte budgets before allocating owned evidence; an
actual encoder also needs a limited writer. This is cold authoring state outside
SlgArena and changes no Quin/ABI layout. No zero-allocation measurement was run.

IP-03 C1–C8, all 120 production packets and complete master/HMC/game-demo output
remain open. No film/open-world game/cartoon model completion is asserted.
