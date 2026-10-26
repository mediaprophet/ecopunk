# Bounded native authentication and adopted project admission

This implementation reuses canonical cooperative project/membership/identifier/review
types and the engine's contextual Ed25519 verifier and OS entropy. It introduces a
versioned signed admission descriptor because the existing Wellfair record envelope
does not authenticate its asserted membership or bind its subject to a session key.

A native host explicitly pins an adopted project authority's project identifier,
qualified issuer identifier and Ed25519 public key. A descriptor signed by that key
binds its sequence, validity/revocation, canonical project, admitted subjects and
capacities, controller public keys, qualified identifier associations, memberships
and explicit document review grants. Exact JSON field order is the Rust schema's
order; SHA-256 of bounded serialization is signed using the existing contextual
Ed25519 API and the `qualia.native-project-admission.v1` context. This is an
application-specific signed admission record, not a new credential standard or a
claim of adopted agreement/legal identity.

The owner creates a fresh one-use OS nonce. A controller proves possession over a
digest binding owner incarnation, adopted descriptor digest/sequence, project,
subject, controller key and expiry using `qualia.native-authoring-session.v1`.
Only after verification does the owner install the canonical QDNF local execution
grant. This is local authenticated authoring; it does not claim a QDNF network
handshake occurred. Key possession does not prove a natural person's legal status.
Principal capacity is explicitly qualified by project admission; no production
human enrollment is asserted by fixture tests.

Current membership, identifier, review, descriptor and session windows/revocation
are checked on every admission and publication. Signed descriptor replacement must
advance sequence and invalidates sessions. Saved bytes never restore a session.
The native transaction serializes authoring and successor metadata/evidence writes;
post-mutation persistence failure blocks publication pending reconciliation. It
cannot promise atomicity across source and authority files. The append-only bounded
authority journal uses create-new numbered entries and syncs before promotion;
reopening verifies the entire retained chain. Host trust anchors, adopted admission
signatures and contributor/reviewer possession proofs remain required real inputs.
No CLI/MCP/source/HMC/current_user value supplies them.

The journal's OS ownership lock serializes native owners. Its hash chain checks
ordinary sequence/corruption consistency, not hostile-storage authenticity or
rollback resistance. A privileged journal writer could rehash unsigned evidence
or delete a contribution/review-only tail. The host must protect this directory;
there is no external authenticated latest-head service. Current pinned source and
signed admission catch regressions only in those fields. Windows file contents
are synced, but directory-entry survival after machine power loss is unqualified.

Package write audit records separately bind the native actor/session admission,
project/document/source revision, expected HMC SHA-256 and resulting HMC SHA-256,
size, filename and time. They commit in the same native authority transaction as
the actual write. `ReviewRecord` and `record_review` review editable source only;
they never attest resources, a complete HMC or a model/package acceptance gate.
