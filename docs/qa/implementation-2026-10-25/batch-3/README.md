# Native cooperative authoring, persistence and rendering — batch 3

This is a bounded implementation record. Exceptional Ecopunk masters, complete
species/thing embodiments, reviewed film/game/cartoon derivatives and the
game-demo outcome remain unaccepted. IP-03 C1–C8 and all 120 production packets
remain open. No Computer Use, commits, publication or payments were performed.

The [governing plan](../../../planning/implementation-plan.md) retains the
production outcome. This batch advances the native tools required to deliver it.

| Implemented scope | Owning native code | Evidence and limits |
| --- | --- | --- |
| Project authorisation policy | `qualia-cooperative-core::authoring` | [Policy](authorisation/README.md): current native principal, qualified identifier association, admitted membership, document revision/digest and explicit review grant. Thirteen focused tests passed. No production authentication provider is invented. |
| One UI/MCP authoring session | Poet `graphical::{cooperation,shared_host,mcp}` | [Gateway](cooperative-gateway/README.md), [shared host](shared-host/README.md): one owned editor/session, separate policy gates, bounded nonblocking access, source stamps and native publication stamps. Fixture authority does not establish human acceptance. |
| MCP transport | `qualia-mcp::authoring` | [Protocol](mcp/README.md): sixteen focused tests passed. Thin native stdio/lifecycle route avoids the legacy receipt fallback and enables no JavaScript/V8 runtime. Unbound access denies source and mutations. |
| Pre-promotion output admission | Canonical Vibe `SourceSession` | [Actual session run](session-validation/actual-session-tests.json): nine tests passed, including rejected output preserving values/source/revision. Previously attempted host effects are not rolled back. Language stays pinned at 0.1. |
| Durable source/history HMC | Core `studio_store::checkpoint`, Poet `checkpoints` | [Persistence](hmc/README.md): canonical HMC reader/writer, exact cells, private undo/redo history, integrity/budget checks, fresh incarnation, absent-only activation and expected-file persistence. Six checkpoint, ten journal and five stamped-store cases passed in the first canonical core run. |
| Resource-preservation guard | Poet checkpoint service | A source-only replacement refuses any file containing additional resources. It cannot silently discard meshes, graphs, textures or unknown entries. Full resource-preserving package editing remains required. |
| Material authoring | Poet Vibe material records and native controls | [Materials](materials/README.md): explicit linear factors, canonical material validation, typed material-only edits and persistent source. Texture/HDR/physical-material production and visual acceptance remain open. |
| Operator save/reopen | Poet diagnostic entry factory | [Operator route](diagnostic-checkpoints/README.md): configured native checkpoint directory, Save in Poet, filename-based fresh reopen and explicit reconstruction. Clearly labelled local diagnostic; no cooperative membership or grants come from a file. |

## Demonstrated rendering faults and remedies

The actual native readback test found that an unbound metallic-roughness sample
multiplied every authored metallic factor by zero. The correction makes the
missing map a neutral multiplier. [Post-correction GPU evidence](native-build/material-pixels-1.json)
records actual changed pixels: base colour 295, roughness 211, metallic 295 and
emissive 295 at 64×64. This proves those factors affect this native render; it
does not qualify a production model, lighting design or material appearance.

The broad core run also reproduced an AO limitation. Linear BA16 depth across
the native 15,000-unit range quantised near geometry in approximately 0.229-unit
steps. [Isolated evidence and repair](materials/ao-qualification/precision/README.md)
replace this transient internal format with tagged logarithmic depth, updating
the packer, reconstruction and bilateral shader consumer. The 15 km range,
background sentinel and byte budget remain explicit. Five CPU/oracle cases and
three candidate shaders passed their focused checks; final applied-source GPU
qualification is recorded in the run receipts when completed. This is a near
detail improvement, not a universal far-distance precision claim.

The CPU culling failure had a different cause: its test projection placed the
near plane at ten units and excluded the nearby fixtures. The
[test-only correction](core-cull-fix/README.md) preserves production culling and
the original draw-count assertions. It is not a claimed production culling fix.

## Exact recovery versus derivative conversion

The first integrated native restart test incorrectly demanded bit equality
between raw master geometry and decoded `.10d` output. The native codec uses
u16 coordinates per mesh AABB. The corrected oracle separately requires:

- Exact accepted source cells and bit-exact raw geometry/index witnesses across
  two fresh native processes, with no saved grant or automatic evaluation.
- Exact material records and indices, and measured derived-position error within
  the codec's documented per-axis `extent / 65535` bound.
- Another actual material edit and expected-digest save after reconstruction.

[Initial measurement](hmc/restart-initial-quantization-analysis.json) preserves
the failed assertion and observed errors. The codec bound is not a blanket
film/game/cartoon acceptance threshold. Each production dossier still needs its
own conversion-loss and retained-master claims.

## Coherent checks and remaining work

[Latest immutable source snapshot](latest-source-snapshot.json) resolves the
guarded source transition chain. Applied sources, focused harnesses, actual
consumer runs and human acceptance are separate claims. The saved
[source audit](final-source-audit.txt) verified 94 current source files across
120 applied transitions against the pinned `e4e2e18c8cba17113fab1f7b53e0055da459ab0b1c69d824ae248e66aa2d3bfb`
snapshot: all current hashes, receipt digests and latest-per-target mappings
matched.

The source receipts record 27 phase-2 QualiaDB repairs, followed by three
chemistry constructor files and one WGSL emitter header/documentation follow-up.
Two phase-2 files were then updated by those later receipts; the latest-source
snapshot pins the final versions and preserves each contiguous hash transition.
An earlier sibling apply was rejected by automatic approval under the repository
boundary. The user later explicitly authorized writes to QualiaDB and Etalon;
the 27-file phase-2 set and constructor/header follow-ups were then saved to
QualiaDB and hash-verified. No Etalon source defect was established.
The recorded WGSL failures name unknown consumer-side `enable` extensions
(`wgpu_cooperative_matrix` and `wgpu_ray_query`) rejected by selected Naga
validation; this batch contains no Etalon source writes and makes no Etalon
readiness claim.

The first unfiltered canonical core run completed with **9,712 passed, 43 failed,
three ignored**. [Triage](core-failure-triage.txt) preserves those historical
failures, including the observed Windows socket access denial and the earlier
CML failure. The later full selected-feature run below passed against the saved
current-source snapshot without reproducing outstanding failures.

### Current saved-run receipts

The full selected-feature Core run passed: **9,782 passed, 0 failed, 3 ignored**
in 428.89 seconds, bound to source snapshot
`e4e2e18c8cba17113fab1f7b53e0055da459ab0b1c69d824ae248e66aa2d3bfb`.
[Saved receipt](core-final-saved-tests.json). The earlier failing receipts
remain preserved as historical evidence; no outstanding Core failures were
reproduced in this current-source run.

Current native integration, material GPU readback, WASM and library receipts
also passed against the same source snapshot. The native checks recorded 62
library tests and 15 CLI tests passing. The actual material-edit/readback test
changed 295 base-colour, 211 roughness, 295 metallic and 295 emissive pixels at
64×64. [Receipt](native-build/material-pixels-final.json) binds the test to the
saved snapshot and identifies the test binary as
`C:\github\ecopunk\target\debug\deps\poet_cli-0723ae997384f051.exe`
(SHA256 `c8afcb652f43f26a401f7bea58bd9f5ebbed1df0af08f71bfbe7315e418f048a`).
The separate [directly linked CLI artifact](native-build/direct-cli/e4e2e18c8cba/artifact.json)
records `C:\github\ecopunk\target\debug\poet-batch3-e4e2e18c8cba.exe`
(SHA256 `03e7bb285df584e668ce4587a31d1814f08ed68932f7e1e60e638a09b8128aed`);
its [checks](native-build/direct-cli/e4e2e18c8cba/checks.json) passed help,
no-window source check, strict JSON-RPC framing and unbound source-read denial.
No GUI was launched and these checks do not claim human or production operator
acceptance. The [WASM check](native-build/wasm-saved-final.json) and
[native library build](native-build/library-saved-final.json) also passed.

The refresh script derives each current status from these saved receipts and
their source-snapshot bindings. The tests establish selected Core, native
library/CLI, actual pixel-readback and compile-time WASM outcomes for this
snapshot. They do not establish a complete cooperative authoring workflow,
full resource-preserving species/thing packages, accepted high-grade models,
reviewed derivatives or human acceptance.

The successful selected-feature suite, actual native pixel output, direct CLI
checks, library build and WASM compile close only those scoped checks. Full
cooperative authoring usability, complete resource-preserving embodiments,
high-grade accepted content, reviewed derivatives and human review remain open.

The next dependencies are:

1. Bind the gateway to verified native authentication, qualified identifier
   associations and admitted project memberships from the existing QDNF/Wellfair
   owners. Replace the historical hardcoded `current_user` path; never bootstrap
   a principal from MCP JSON, CLI text or a restored HMC. Persist current document
   metadata and contribution/review evidence through the canonical owner.
2. Complete resource-preserving HMC package editing and full graph/knowledge,
   editable-master and provenance retention. A source checkpoint is not the
   complete species/thing embodiment.
3. Qualify the native contributor/reviewer workflow and required Craft operators
   on actual reference source. The headless fixture results do not establish
   graphical usability or human review.
4. Advance calibrated texture/material/lighting authoring and finish the defined
   high-grade organism, person, vehicle, community system and place work. Keep
   the full first-outcome census and every dossier obligation open.
5. Produce explicit, reviewed game-demo derivatives from those accepted masters.
   Payment rails remain supportable later work.

Native preview controls use Aether and the selected Qualia/Etalon dependency
closure. Chora remains separate and untouched. The saved QualiaDB session
handoff is recorded in the [guarded handoff receipt](engine-handoff-saved.json).
No blanket claim is made about all local fork features, browser/WASM parity or
untested feature combinations.
