# Native effect-contract repairs

Staged only. Five failing core cases were legacy fixtures that contradicted the
canonical pinned 0.1 checker. No parser, effect classification, transitive checker
or language version is weakened by this change.

* `on tick` is Hot. Both old tick fixtures indirectly returned an External
  publisher. One case now proves an actual numeric Hot hook/sibling call returns
  42; the publish-on-tick case proves E200 and no publication. A separate external
  `on pulse:message` hook proves actual authorized publishing, missing source
  request denial, actual native grant denial and native topic allowlist denial.
* `hid.poll` consumes native input and is External. Positive queued/empty polling
  now uses an explicit effect function. Pure-cell and missing native grants deny
  without draining the real queued event.
* Native Ed25519 generation/signing changes or uses the private vault and is
  External. Positive functions declare that effect and use explicit host-held
  grants. The roundtrip still generates a real native key, signs, verifies true,
  and rejects tampered content. Missing/revoked native grants deny. The adjacent
  pre-existing missing-key case now reaches native E702 with an effect function,
  rather than accidentally passing because an earlier E200 masked the vault test.

The public directory-backed `poet_host::scoped_effects::ScopedEffects` borrows the
existing PoetSnapshot and a bounded explicit Rust-held grant list. It checks
`pulse.publish`, `hid.poll`, `crypto.generate_key` and `crypto.sign` before native
delegation. Unknown, duplicate and oversized grant sets fail closed. Graph/aura
methods deny; unimplemented Host defaults remain unchanged and are not advertised
by this narrow profile. Pure verification keeps its pinned classification and
returns validity without exposing a seed or granting signing authority. There is
no new state database, source-derived grant, default all-capability grant, principal
bootstrap, private-key export, source fallback or automatic execution retry.

The native owner supplies current grant state and keeps the borrowed adapter
inside its admission transaction. This adapter is not a new authentication or
revocation provider. Legacy raw trusted-Rust PoetSnapshot APIs stay compatible;
this patch does not claim every historical caller is automatically scoped.
The new cases separately prove SourceSession admission and native-host admission:
source `requires` alone cannot replace either actual grant boundary.

## Scoped decomposition

`poet_host/mod.rs` is already oversized. The tracked decomposition here extracts
only the five targeted cases plus the adjacent masked missing-key case into
`scoped_effects/tests.rs`; all original function names are retained. Original
source blocks and new module mapping are in `extracted-test-history.json`. No
unrelated tests are extracted, skipped or removed. The owning file gains only one
module route; new implementation and tests are focused files below 500 lines.
This is not a claim that the broader host module decomposition is complete.

## Verification

The exact proposed adapter/tests compile against cached actual canonical Vibe and
core libraries through `harness/tests.rs`, without rebuilding core or Cargo. The
final focused execution passes 11 tests, including real native HID/vault/pulse
state and both permission/effect denial layers. `verification.json` pins the actual
cached libraries and commands. This is an actual-source candidate plus cached
native-state qualification; the parent's guarded application and coherent current
source integration build remain distinct checks.

All changes are staged with old/candidate SHA256 guards in `apply-manifest.json`.
No sibling edit, provider call, network publication, UI/GPU launch or payment was
performed by this worker. Detached snapshots journal pulse results locally; no
attached live daemon was used. These repairs do not close authenticated
cooperative readiness, IP-03 C1–C8, all 120 production packets or any complete
film/game/cartoon model or master/consumer outcome.
