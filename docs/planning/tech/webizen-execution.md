# Webizen execution — governed worker orchestration (proposed)

**Status:** draft v0.1 · **proposed architecture** · the Webizen/Sentinel
guardian exists; a browser worker supervisor is **not demonstrated or
qualified in the inspected game path**. This doc proposes the contract — it
is not a claim of demonstrated capability.

Driven by the game-demo remediation package R4 (and R6 for the optional LLM
lane) — see [game-demo-remediation.md](game-demo-remediation.md). Pack
acquisition is out of scope here: [pack-delivery.md](pack-delivery.md).

---

## 1. Terminology & existing substrate

- **Webizen is the Sentinel — the existing guardian**, not a replacement
  component. Rename is accepted terminology per
  `qualiaDB/docs/manuals/adr/004-sentinel-to-webizen-rebrand.md`.
- The QDNF implementation notes state it plainly
  (`qdnf-imp/README.md:44-53`): *"Webizen is the Sentinel. Its 42 MiB table
  allocation does not establish one arena per cell or complete-pass
  enforcement"* — and *"ordinary cells have an admitted ceiling at most
  512 MiB"* with aggregate host reservations; exceptional LLM-class execution
  stays explicitly classified and isolated. **These are the existing native
  spec — not a default size for new browser workers.** Browser admission needs
  per-instance *and* aggregate budgets, with GPU residency counted.
- `governance/coordination.rs` carries authorization, resource accounting, and
  circuit breakers — the admission **mechanics are confirmed in source**; the
  full guard path has not been executed end-to-end, and a browser governed
  pool is not demonstrated in the inspected game path.
- Existing schedulers are not a qualified browser pool:
  `platform/local_scheduler.rs` is `cfg !wasm32` (native only);
  `qualia-client-core/src/local_job_scheduler.rs` is client-side.
- `docs/js/vibescript-bench-worker.js` demonstrates a Vibe worker as an
  **independent instance** — evidence for separate-memory workers, not for a
  governed supervisor.
- Capability detection: `docs/js/browser-capability.js:180-213` exposes
  `hardwareConcurrency` (a hint — logical processors, privacy-limited), SIMD,
  a threads feature probe (**availability ≠ artifact-activated threads**), and
  GPU hints. Native `net/host_topology.rs:139ff` uses `sysinfo`. `deviceMemory`
  is best-effort and not presently detected. Do **not** auto-spawn one worker
  per core, do not require hardware-fingerprint export — capability info stays
  local and minimal.

## 2. Governance model

Human-authorized rules → **Webizen admission** → restricted orchestrator
(optional curator) → workers.

- Human beings are the roots of authority; organization channels delegate.
  Machine output is a *proposal*, never ratification — same ladder as the
  review workflow.
- The **orchestrator** admits, schedules, and terminates within granted
  budgets; the **curator** (optional) sequences/lanes work — it sees scoped
  job descriptors, not raw private corpora, and can neither mint grants nor
  override review or budgets.
- No privilege expansion: a worker holds exactly the capability references in
  its job descriptor; aggregate admission can be refused even when each
  individual request would fit.

## 3. Logical lanes (lanes ≠ workers)

Intended priority, highest first:

| Lane | Contents | Admission intent |
|---|---|---|
| Interactive presentation | frame-adjacent work, input response | protected — frame and input are never starved |
| Authoritative sim/logic | essential world simulation and rules | guaranteed progress; some work validates before dependent presentation |
| Latency-needed queries & visible assets | promoted queries, verified-visible assets | next after essentials; speculative asset work stays after verified-visible |
| Background ontology / prefetch | reasoning, indexing, speculative fetch | bounded progress — not strict starvation (see fairness rule) |
| LLM (optional) | NPC/NL proposals | capped; user-requested NL outranks speculative chatter but stays below essentials |

**Fairness rule:** lanes are not blindly strict-priority — background lanes
get bounded progress without starving authoritative validation, and
starvation of lower lanes is bounded, not permanent.

## 4. Worker model

- **Persistent workers as independent instances**: separate memories; shared
  *compiled module* acceptable; inputs via transferable buffers. SharedArrayBuffer
  + atomics optional — used only where isolated and capable; low-core and
  no-shared-memory deployments must work (game-demo WASM already reports
  `ArrayBuffer shared:false`).
- **Single authoritative writer + versioned snapshots.** Side-effectful logic
  is never duplicated or sharded; workers produce proposals/results, one
  writer commits.
- Host keeps DOM, input, and the main thread. **The first worker split keeps
  the current renderer on the main thread** and offloads verification,
  decoding, and ontology work. A **renderer worker is conditional** on
  OffscreenCanvas — adoption must qualify the GPU worker API surface, device
  lifecycle, residency, and recovery, not just availability — and the current
  public binding is `HtmlCanvasElement`, an upstream change is required; it
  is not a mandatory launch assumption.
- Long-running WASM cannot receive cancellation until it yields: jobs run in
  small resumable quanta; the supervisor must avoid making a worker
  synchronously critical to any frame.
- CPU and GPU admission are coupled in reality — a separate worker still
  competes for the same GPU and RAM. Inference prefill/decode run under
  capped, cooperative scheduling. There is no browser OS priority or
  affinity — adaptation uses frame time, latency, memory, backpressure, and
  power signals where observable. A request-specific foreground NL exchange
  may outrank speculative chatter, never essential work.

## 5. Proposed descriptors (not an existing API)

**Job descriptor:** job ID · world/request generation · capability/authority
reference · input digests or scoped handles · lane and dependencies ·
budget/deadline/cancellation token · allowed result destination.

**Result receipt:** job/result identity · source/request generations ·
version · status · verification + resource receipt (what it actually
consumed). The policy revision is **rechecked on receipt/commit**; results
are idempotent — committed at most once — and late/stale results never write
the graph.

## 6. Honest boundary — what this does not claim

- No end-to-end browser governed pool is demonstrated in the inspected game
  path; §1 lists the mechanics confirmed in source and their scope — "exists"
  means confirmed in code, not that a full guard path was executed.
- The 42 MiB pass requirement and ≤512 MiB ordinary-cell ceiling are existing
  native spec quoted for discipline, **not** promised browser worker sizes.
- Threads/SIMD probes describe availability on the host, not that our artifacts
  activate or are permitted to use them.
- Browser scheduling has no OS-level priority, affinity, or real-time class —
  all adaptation is best-effort within the page's cooperative event loop.
- Lanes are a scheduling contract, not a promise that work lands on distinct
  OS threads; lane semantics must hold even when everything shares one thread.

## 7. Tests (from R4 / R6 acceptance)

Low-core and no-shared-memory operation · cancellation mid-quantum · worker
crash isolation with state preserved and no duplicate commit · backpressure ·
stale-version results rejected · revoked/expired/unauthorized scope refused
**before** data exposure · priority+deadline ordering honoured · limited
total GPU/CPU admissions against the essential reserve · aggregate admission
refusal · no privilege expansion · (R6) capped prefill/decode, fallback
interaction without model, stale/cancelled replies discarded, no eviction of
essential reserves, no unauthorized disclosure.

## 8. Open questions

1. Which browser bindings are actually supported today for worker-hosted
   engine calls (WASM surface vs Vibe host)?
2. Queue reuse: can an existing priority/queue component be qualified, or does
   the F9-style ordering issue require a fresh implementation?
3. Memory accounting for shared compiled modules and cached artifacts across
   instances — how are they attributed to per-instance vs aggregate budgets?
4. Approved device budgets: what envelope may a browser session claim, and who
   approves it?
