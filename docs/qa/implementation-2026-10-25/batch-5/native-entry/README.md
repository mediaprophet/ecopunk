# RUN10 saved native project operator backend

The native entry backend is saved in Poet's `native/src/graphical/native_entry/`
under `runtime.rs`, `files.rs`, `actions.rs` and `create.rs`. The supervising
worker owns its native UI, operator lane, types, CLI route and checks. This
record describes saved implementation and its limits; it asserts no graphical
acceptance or OS credential-store execution by the implementation worker.

See the concise [native project operator walkthrough](operator-workflow.md) for the saved UI route and its qualification limits.

`Runtime::open` reads bounded public configuration only. Every app opening starts
unadopted, without a signer or source. Existing projects require an operator to
confirm the displayed public authority-key SHA-256 fingerprint. Adoption opens
the existing protected authority journal and its signed admission, without HMC
IO. Current profile login opens the strict OS-backed native vault, verifies its
controller against its public profile locator, finds the controller association
in the pinned signed admission and proves a fresh one-use session challenge.
Only then does `NativeProject::open` authorize actual HMC reopening and compare
the retained source revision.

`CreateProject` is an explicit local-root creation action, not adoption of an
external project or proof of legal identity. It derives separate controller and
project-authority keys from the strict OS vault, signs a canonical initial local
steward admission, creates an empty editable Studio artifact and saves a genuine
`working.hmc` through the authenticated native checkpoint/audit path. It writes
the new public `project.json` create-new. It does not supply a fabricated model,
master or review. Existing public projects cannot be overwritten by creation.
Human capacity is an explicit local operator assertion; real contributor human
verification remains a separately evidenced steward decision.

Profiles are plain bounded lowercase locators in a pinned `profiles` directory.
Their JSON contains a requested capacity and controller public key, never a role,
private key or live session. The concrete Windows credential owner has no
plaintext fallback and serializes profile secret creation across processes.
Metadata files are bounded regular non-reparse files and use create-new writes.
No CLI secret, principal, steward or review-grant argument is accepted by the
backend. Configuration changes retire the session and require explicit reopening.

Enrollment submission proves the selected profile key without logging it in or
assigning stewardship/review. Only the current opaque steward permission exposes
pending requests and member projections. Approval, distinct review grants and
revocation require the pinned project-authority signer from the currently
authenticated profile. Human requests additionally require explicit steward
verification evidence. Signed admission changes retire all old sessions.
Steward revocation is held for a separate authority rotation/recovery procedure.

The runtime retains one actual NativeProject/source/execution service across
profile logout/login and signed admission changes. Each login receives a fresh
native worker/lease bound through the shared SessionSlot. Logout, expired or
revoked authority, reconciliation errors and unavailable sessions join/retire
the old worker and remove its lease. Per-profile private draft buffers remain
separate from canonical accepted source. No second roster or authority database
is introduced.

Saved file/route tests operate only on isolated public JSON and paths; they do
not call the OS vault, launch a window, adopt a project, authenticate a real
person or read package source. The core enrollment tests use explicitly labelled
software agents and isolated encrypted fixture keys, including a fresh-process
reopen test. Rustfmt passed the backend source. Cargo/build/test outcomes belong
to the supervising evidence receipts and must not be inferred from this note.

The current Geometry operator window uses the existing bounded geometry/preview
profile; it is not qualification of complete high-grade production authoring.
Graphical contributor/reviewer usability, actual OS credential persistence,
human enrollment and agreement adoption, semantic whole-HMC/rights/resource
review, accepted editable masters and complete film/game/cartoon embodiments
remain open. The existing local-journal hostile-writer/rollback and Windows
power-loss directory durability limits remain. IP-03 C1–C8 and all 120 production
packets remain open.

