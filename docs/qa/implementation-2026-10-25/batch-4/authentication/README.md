# Native authentication/admission integration — saved source

Scope: canonical native project admission/session/authority journal and Poet
adapter. Actual source is saved under
`C:/github/qualiaDB/crates/qualia-core-db/src/poet_host/native_authority/`,
`C:/github/poet/native/src/graphical/cooperation/`, and the bounded native project
integration test. The engine and Poet module routes were updated without changing
unrelated source. No commits, pushes, GUI operations or paid calls occurred.

The [contract](contract.md) defines the adopted trust-anchor inputs, signed record,
native one-use proof, current QDNF local execution permit, qualified identifiers,
source/package evidence distinction, and storage/power-loss qualification limits.
It does not assert any real human was enrolled or any model/package accepted.

Implemented behavior:

- Signed, bounded, monotonically replaced project admission binds canonical
  project/membership/identifier records, explicit review grants and admitted
  controller keys/capacities. Weak Ed25519 keys are rejected through the existing
  dependency's public validation API.
- Fresh OS-nonce challenge verification precedes local QDNF grant installation.
  Opaque sessions are never serialized/restored or derived from source, MCP,
  `Identity.current_user`, CLI values or HMC content.
- Current project/subject/identifier/membership/review/session windows and
  revocation govern every action; nonblocking publication refuses busy/stale
  authority. Blocking transactions sample time after acquiring the owner lock.
- An OS-locked bounded native journal retains exact document successors,
  contribution/source-review records, and separate package actor/expected/result
  SHA-256 audit records. Predictable package audit capacity is checked before the
  file operation. Persistence failure after source/package mutation requires
  reconciliation and withholds publication.
- Poet gates source, preflight, resource inventory/import/edit and HMC save;
  verified fresh-process HMC activation delegates existing checkpoint admission.

Executed by this worker: Rust formatting/parser checks against the actual saved
owned source passed. Twelve canonical focused tests and one signed native
project/Vibe/Studio/HMC/MCP integration test were added; **this worker ran no Cargo
builds or tests** because the supervisor owns serialized integration execution.
The supervisor's execution evidence must be consulted for actual test results.

The integration fixture uses actual native Ed25519 signing/verification, OS nonce,
canonical Vibe geometry/material evaluation, shared UI/MCP source edits, HMC
resource retention/import/edit, durable package audit, signed revocation and a
distinct software-agent review. Its preview provider explicitly declines pixels.
It is not artist/contributor/reviewer GUI or human acceptance evidence.

Preservation/evidence: [initial digests](before.json),
[before/after save transitions](save-receipt.json),
[saved source manifest](saved-source-manifest.json). `candidate/` retains the exact
owned candidates, not a claim they replace the actual repository source. The
native-project main implementation is supervisor owned; this worker added only
its bounded test file and test route.

Remaining product work includes actual adopted project trust anchors and current
signed admissions, real contributor/reviewer key enrollment/signing integration,
usable graphical cooperative acceptance, complete HMC/model embodiments and
reviewed downstream derivatives. No complete model/package is declared here.
