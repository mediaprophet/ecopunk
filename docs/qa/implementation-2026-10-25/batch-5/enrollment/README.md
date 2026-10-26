# RUN10 native controller enrollment and signing

Saved source adds a concrete native signer and bounded enrollment lifecycle to
QualiaDB's existing `poet_host::native_authority` owner. It uses the existing
identity KeyVault, contextual Ed25519 adapter, OS entropy and Sanctuary
XChaCha20-Poly1305 wrapping primitives. It does not create a parallel project,
membership, contribution or payment database. Enrollment records are retained in
the existing locked native authority journal.

## Existing source findings and retained secret owner

`qualia-client-core/src/identity_plane/person.rs` has a sealed PersonPrincipal,
but its cold fallback writes `person_wrap.key` as a plaintext secret. The existing
`identity/key_vault.rs::load_or_generate` can write a plaintext `keystore.bin` and
`KeyVault::new` uses a deterministic fixture seed. Neither supplies production
enrollment authority through this new adapter.

`KeyVault::open_native_secure(profile_path, create)` explicitly builds the
concrete Windows Credential Manager backend with Local persistence. It bypasses
the replaceable global credential store, verifies stored persistence and secret
readback, refuses legacy plaintext vault files and fails on credential errors.
Creating a profile must follow an explicit operator gesture. Non-Windows native
credential backends remain explicitly unsupported in this slice. A private,
nonserialized provenance bit distinguishes this owner from fixture/legacy
vaults; `NativeSigner::from_vault` requires that bit and an unlocked vault.
No real OS credentials were created, read or tested by the enrollment worker.
The strict owner requires an absolute profile path and serializes its complete
read/create/readback transaction using a regular non-reparse no-delete-sharing
per-profile native lock. Credential locators use the canonical profile path;
concurrent create attempts cannot silently replace one another's new key.
Core unit-test compilation explicitly disables both OS entry construction in the
legacy helper and the strict OS opener. Legacy unit tests consequently exercise
their existing isolated temporary-file fallback; they do not qualify the actual
OS provider. This corrects a newly identified safety issue in those tests and
does not claim that earlier full suites avoided the old helper or remove any
existing credentials.

`NativeSigner::create_sealed` and `open_sealed` also provide explicit retained
encrypted key files under wrapping material supplied by an already authenticated
native secret owner. Their fixed 76-byte format is `QNS1` plus the existing AEAD
wrapped seed, with fixed versioned AAD. Creation is create-new; reopening rejects
links, unexpected lengths, appended bytes, plaintext, wrong wrapping keys and
corruption. Secret buffers are zeroized; the signer implements neither Debug,
Serialize nor Deserialize. Its public key is safe to display. The isolated tests
use known test wrapping material only, never a user's OS credential store.
These encrypted-file APIs and their regression module require the explicit
`sanctuary-crypto` feature, matching the owning engine primitives' existing
capability boundary. Without that feature the encrypted-file constructors are
unavailable; no substitute constructor fabricates success. The concrete native
OS-vault signer, session proofs and admission signing remain available without
the sealing feature. The supervising Poet native dependency and encrypted-key
test profile select `sanctuary-crypto` explicitly.

## Native host API and authority boundary

- `NativeSigner::from_vault(&vault, profile_label)` derives a purpose-separated
  retained controller from the strict native vault. `public_key`, `prove_session`
  and `sign_admission` use the engine's contextual signing implementation.
- `NativeAuthority::enrollment_snapshot()` gives the current signed host record.
  This protected metadata is host-readable before login; it is not an
  authenticated partition-rights gate and must not become a transport authority
  constructor. `EnrollmentRequest::signed` binds project, current admission
  sequence/digest, nonce, subject, controller, capacity, requested membership,
  qualified identifiers and all validity windows into a possession proof.
- `submit_enrollment` verifies the proof and current binding, then retains a
  pending request. Candidates may request contributor or observer membership;
  requested steward authority or embedded review grants are rejected. Replay,
  duplicate subject/controller, identifier duplication, wrong project, stale
  snapshot and expired requests fail closed. A key proof proves possession; it
  does not prove a natural person's legal identity.
- `enrollment_permission(&session)` returns an opaque in-memory permission only
  for a currently authenticated, admitted steward. `list_enrollments`,
  `approve_enrollment`, `decline_enrollment`, `revoke_enrollment` and
  `grant_enrollment_review` require that current permission. Approval requires
  explicit decision evidence. Human capacity additionally requires a separately
  supplied operator verification reference; software agents cannot be labelled
  human by the tests. Review authority is a separate explicit signed grant.
- Approval, revocation and review grants require a concrete signer whose public
  key equals the independently pinned project authority key. They advance and
  sign the existing AdmissionSnapshot, append it to the existing journal, and
  invalidate all live sessions and permissions. Operators must authenticate
  afresh after these changes. Declined/revoked requests remain visible.
- `open_retained_current(root, anchor, bounds)` reads only the protected journal
  metadata before login. `open_retained(root, anchor, document, bounds)` also
  compares a supplied actual source revision. Both create a fresh owner
  incarnation and restore no session or pending login challenge. After login,
  the host must authorize actual HMC reopening and compare its source revision.

The native host still owns explicit project creation, displayed authority-key
pinning/adoption, operator profile selection and real human verification. A
generated project, signed fixture, configuration field, HMC label, CLI argument
or MCP request cannot establish those decisions. This engine adapter does not
automatically enroll anyone or adopt a project. `sign_admission` creates signed
bytes; host pinning and adoption remain separate actions.

## Evidence and open scope

`before.json` records observed pre-edit hashes of existing authority routing,
admission validation and key-vault source. `after.json` pins the saved source
inventory. `lock-receipt.json` records both exact Cargo.lock transitions; only the
qualia-core-db dependency blocks gain two already locked credential dependencies.
No package versions or source pins were resolved or changed by that script.

The worker ran rustfmt successfully, with `skip_children=true`, on its owned
Rust modules. Cargo execution belongs to the supervising serial check runner;
no build or test pass is asserted here. Saved regression source covers sealed
key reopen/corruption/plaintext/wrong wrapping material, nonsecure vault denial,
explicit steward approval, signed snapshots, fresh-process encrypted-key and
admission reopening with fresh login, forged/stale/wrong-project/expired proofs,
replay/duplicate identifiers/controllers, self-assigned review/steward rejection,
human-capacity-without-verification denial, foreign/stale permissions, distinct
review grants, decline persistence and revocation. Every successfully admitted
test actor is explicitly `SoftwareAgent`.

The local journal's existing hostile-writer/rollback and Windows power-loss
directory durability limitations remain. OS credential-store persistence is a
concrete source integration, not an executed credential-store qualification in
this session. Graphical usability and real human enrollment/adoption require
operator work. Source review remains source-only; it is not whole-HMC semantic,
rights, resource or model acceptance. Complete Ecopunk masters, required film,
open-world-game/cartoon embodiments and reviewed game-demo derivatives remain
incomplete; IP-03 C1–C8 and all 120 production packets remain open.
