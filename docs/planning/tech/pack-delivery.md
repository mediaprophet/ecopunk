# Pack delivery — verified acquisition service (proposed contract)

**Status:** draft v0.1 · **proposed service contract** · explicit downloads
are a first-class mode **alongside** streaming — one governed, verified cache
serves both; downloads are not a second engine.

Driven by game-demo remediation R3 (findings F5/F10) — see
[game-demo-remediation.md](game-demo-remediation.md); worker orchestration in
[webizen-execution.md](webizen-execution.md). Distribution identity/addressing
rules live in [durability-and-distribution.md](durability-and-distribution.md);
node roles in [network-nodes.md](network-nodes.md); artefact formats in
[pipeline.md](pipeline.md).

---

## 1. Delivery modes

| Mode | Semantics |
|---|---|
| **Stream now** | fetch only the extents needed for what is visible/needed now |
| **Download selected** | install a pack, profile, or declared dependency closure for offline/later use — explicit, user-visible |
| **Hybrid** | visible-first streaming, then allowed background install of the selected scope |
| **Import** | bring in an approved local/offline pack (file, removable media) |

Rules:

- Download scope is declared: **full immutable bundle** *or* a **selected
  profile + dependency closure**. A partial/range read is **never** labelled a
  complete pack.
- Ontology/logic packs must match a compatible engine/profile version; rights,
  scopes, and executable capabilities are validated **before** activation.
- An optional LLM model pack is an explicit, heavyweight, exceptional
  admission — never auto-downloaded.
- Model weights are stored separately from editable world/save data.
- A download helper fixes acquisition only — it does **not** repair
  CPU/animation/render deficits, and optional streaming supplements are not an
  online requirement.

## 2. Format facts that bound the design (verified, do not blur)

- **Q42 `BIDX`** (object hash → contiguous block interval, `q42/volume/index.rs`)
  and the **HMC `QBDL` footer entry index** (`BundleEntry{key,kind,offset,
  length,sha256,meta}`) are *different* indexes. HMC entries are not "BIDX
  sections"; `.10d` decode may still require the full entry even when ranges
  are fetchable.
- Native `HttpRangeSource` (`q42/volume/range.rs:148ff`) is `cfg !wasm32` —
  its existence is **not** browser proof. The ad-hoc `serve.mjs` used in
  diagnosis sends `200` whole-file with no `Range` support; whether a deployed
  carrier honours `Range`/`206` must be checked per deployment — do not claim
  GitHub Pages (or any host) supports it currently.
- The existing QBDL whole-file CRC and `BundleReader.parse` verification
  **cannot be silently skipped**: an upstream range reader must authenticate a
  committed trusted index/manifest and the fetched entries, emitting its own
  receipts — not fake whole-file verification. A raw range response carries no
  full-digest proof; CID proofs exist only on the specific partial-CAR path.
- Identity is a trusted name + version bound to immutable digests — **not**
  HTTP aliases alone; policy gates apply for cache/egress restrictions
  (lane obligations travel with the bytes).
- Deployed carriers must be checked per deployment for `Range`/`206`
  behaviour, exact `Content-Range`, total length, consistent
  identity/encoding bytes, and the CORS-exposed headers the reader needs —
  support is an advertised, verified property of that host, not assumed.

## 3. Closures — what "downloaded" actually means

Three distinct states; none may borrow the status of another:

| Closure | Meaning |
|---|---|
| **Ready-to-play scene** | the validated subset needed to activate the *chosen* scene — a scene may enter ready while the chosen install remains pending |
| **Installed chosen profile** | the selected profile + dependency closure durable on device |
| **Bundle-complete** | the entire immutable bundle verified — never claimed for partial extents |

An **offline closure** additionally requires: a compatible application shell +
WASM binaries + Vibe modules/schema/action catalogs, the required ontology
references, mesh+texture+rig/clip/material dependencies, and
identity/licence/provenance/auth-policy metadata. **Saving the content pack
alone is not enough for browser offline startup** — if offline reload is
claimed, the shell must be cached via the host/engine service as part of the
closure. Install commit is atomic and separate from the scene GPU swap; save
migration applies only across compatible versions.

## 4. Proposed bounded acquisition sequence

resolve identity + profile/deps → scoped authorization/admission → cache
lookup → fetch missing extents → verify → stage durable output → publish
completion **atomically** (only when the required verified closure is done)
→ decode/upload the selected working set → scene-generation-specific
first-present.

Verification is three separate questions, never merged: **(a)** the received
bytes are exactly the requested extents (bounds correct); **(b)** per-entry
digest/chunk proof integrity holds; **(c)** authorization permits the access.
CRC is integrity, not authenticity; the expected index/digest is
authenticated via a signed or explicitly trusted pinned root manifest —
a bare hash alone is not authority.

- Reuse the OPFS adapter/cache APIs after qualification; quotas and persistent
  storage are best-effort vs cache eviction; native uses the same contract
  over file backing.
- Avoid the current failure shape: no whole pack in the JS/WASM heap and no
  chunk-list + contiguous-copy duplication.
- Journal tracks **received vs verified coverage** bound to an immutable
  version: received partial extents stay staged-untrusted until the complete
  entry hash or a verified chunk proof lands; resume discards a corrupt
  extent only and never merges across versions — a version mismatch starts a
  separate stage. ETag/`If-Range` may corroborate a version optionally, but
  digests are canonical.
- Staged content is **never executable until admitted**. Interrupted success
  restarts or cancels without deleting user saves.
- Content-addressed dedup with scope-per-access (no inferring a global public
  cache); coalesce identical authorized requests. Cached rights are checked
  **on each read, including offline** — revocation knowledge is only as fresh
  as the last sync; say so, never promise magically fresh authorization.
  Pin installed/selected assets; eviction is explicit and never loses user
  data.
- **Installation ≠ GPU residency** — decode/upload is a separate stage.
- Preflight storage space + dependency total before admission. Name the
  actual storage state honestly — **cache-only** (evictable) vs **durable
  installed** vs **offline-ready** (the exact selected closure); browsers
  offer no guaranteed persistence, so quota-refused or unpersisted storage is
  reported as honest readiness, not hidden.
- Updates stage alongside the last good version; validated versions switch
  atomically and roll back without clobbering campaign saves. A portable
  install manifest must be exportable independent of DNS.

## 5. Loader & scene lifecycle (UI contract — F10 fix shape)

Two **distinct** state machines sharing one load generation:

| Job state machine | Scene state machine |
|---|---|
| queued · fetching · verifying · staging · installed · failed · paused · cancelled | resolving · preparing · awaiting-first-present · ready · failed · degraded |

- A new style/pack request **cancels the old scoped job or coalesces
  references** to it; stale progress, errors, and delayed completions are
  ignored — never hide or reset the current loader for a stale event.
- **One RAF owner** — no tick-polling running a second simulation alongside
  the normal frame loop.
- First-frame receipt is tied to the *new* scene generation with a bounded
  real-elapsed deadline, and first-frame does **not** wait on optional
  installs; loader visibility follows lifecycle events, not per-byte
  progress; accessible reduced-motion and throttled live announcements;
  focus moves only on explicit open/close of the loader — never a
  `root.focus` on each progress update.
- Bytes progress only where length is known per stage; otherwise
  indeterminate. Download 100% ≠ render-ready — GPU/decode continues.
  Optional downloads are non-modal after play-ready.
- Required failures are typed: retry / offline / fall back to last-good where
  authorized. Optional failures are non-blocking with honest degradation.
  Diagnostics preserve the actual validation/network/renderer error — never a
  generic "connection issue".
- Network and job timeouts are bounded; retries bounded; the completion timer
  is generation-guarded.

## 6. Acceptance cases (exact)

206 responses: correct `Content-Range`, truncated body, invalid bounds, wrong
size, `416` · server `200` whole-file refused for bounded paths unless an
explicit admitted finite-download mode applies · unknown total length ·
immutable version changes mid-resume · bad hash, forged index, missing
texture · quota full, private mode, eviction · offline required-closure cache
hit and honest missing-dependency failure · restart midway reusing verified
extents · same pack from two clients coalesced under authorization · A/B style
switches arriving out of order · cancel/error with stale timer not hiding the
current loader · no second tick owner · render first-present distinct from
download completion · warm cache with no full redownload/recompile ·
accessibility: focus management and reduced motion honoured · selected shell
offline reload works · missing dependency / expired scope refused or
degraded honestly · forged signed/pinned index rejected · an explicitly
chosen quality `0` is preserved (no silent upgrade to a default) · duplicate
job completion is idempotent — committed once.

## 7. Open questions & decisions

1. Pack ABI version for the range reader contract (committed index format).
2. Browser storage qualification: OPFS vs Cache API per deployment; eviction
   behaviour under quota pressure.
3. Service-worker isolation for deployments that need it vs in-page loader.
4. Target budgets per profile/deployment — requires approval plus empirical
   justification once a baseline exists (remediation evidence checklist).
