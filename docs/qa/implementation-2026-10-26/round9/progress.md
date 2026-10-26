# Native equipment authoring — implemented and exercised

2026-10-26. **A native equipment-authoring trial is implemented and built. Production/model acceptance remains incomplete.** No Computer Use or human acceptance is claimed. The earlier time/round forecast was unreliable; this report replaces a new forecast with saved capabilities, executable results and remaining limits.

## Delivered capabilities

| Track | Saved native behavior | Allocation |
|---|---|---|
| Retained viewport | Orbit, pan, zoom, framing, exact compiled-triangle/part picking, selection-linked inspector and an x-ray wireframe selection overlay. GPU device and scene uploads are retained across view changes. | gpt-6.1-sol, high reasoning: renderer/camera/picking integration; independent source review |
| Source-owned layers | Enable layers for the existing recipe, then regenerate dimensions while preserving material/TRS overrides. State is ordinary editable Vibe source, persisted through Studio/HMC; unsupported correspondence refuses without source promotion. | gpt-6.1-sol, high reasoning: source generation/admission/recovery; independent source review |
| Operator controls | Native contributor/reviewer context, exact-revision pending requests versus historical decisions; viewport gestures and lighting controls use the existing authorized worker/intent path. | gpt-6-luna, medium reasoning: UI/projection assembly; root review and executable checks |
| Integration and qualification | Native owner/host forwarding, atomic guard hooks, current-revision form checks, profile wiring, actual reference qualification, installation and verification. | Root: integration/review/check execution |

The concurrency limit is four active agents including root; there were three implementation sub-agents. Higher-cost work was kept to renderer/runtime/source correctness. No provider spend, real enrollment/grants, money movement, commit, push or publication occurred.

## Actual reference result

The exact existing 15,690-byte master (`0.2`, SHA-256 `8aac87e9f98dc305ea85d45f2ebae19aba5fbd9323c165f160c83bb46ce89a63`) was used without reducing tessellation or changing production source. A native offscreen case:

1. Evaluated the original source and admitted all **199 parts**.
2. Enabled source layers, changed cabinet material and station TRS, then changed width to **2.8 m**. It checked retained overrides and increased compiled x-extent, not merely changed labels.
3. Rendered, framed and picked the actual hatch from the compiled geometry.
4. Compiled an HMC, then activated it in a fresh process, opened held, explicitly rebuilt, and checked source history and regenerated compiled-container digest.

Observed output: **239,952 vertices, 79,984 triangles, five source cells**, a **5,391,416-byte diagnostic HMC**, and two retained companion resources on reopen. The compiled `.10d` digest is `84dce18579ec38a4c4f7e4f902219531391e3635aa2b70aad0d02f22cb073104`.

The measured full write-side workflow was **75.065 s** and the fresh-process recovery side **65.859 s**. These aggregate timings include source, geometry, rendering, checks and packaging; they are not per-gesture latency, FPS or peak-memory measurements. Large-workload responsiveness still needs operation-level profiling. Renderer reuse tests establish device/upload call counts, not universal performance superiority.

Retained, inspectable evidence:

* [Diagnostic HMC](final/reference-output/diagnostic-equipment.hmc) — editable source/history plus compiled resources; a software fixture, not an accepted release.
* [Source-cell history](final/reference-output/accepted-source.vibe) — concatenated for inspection. Reopen the HMC to preserve cell boundaries; do not treat this concatenation as one equivalent REPL submission.
* [Measured write result](final/reference-output/observed.json), [fresh reopen result](final/reference-output/reopened.json), [command and source pins](final/reference-result.json).
* [Whole-equipment render](final/reference-output/r9-reference-frame-all.png), [picked hatch](final/reference-output/r9-reference-picked-hatch.png), [fresh-process render](final/reference-output/r9-reference-reopened.png).

The first diagnostic was visibly underlit. Preview lighting/exposure controls now use the existing renderer APIs; the studio viewing preset makes the same geometry easier to inspect. Lighting is explicitly view-only, not a change to world physics, material source, or an adopted scene-lighting recipe. This is not a film-grade artistic acceptance.

## Concrete blockers found and fixed

* The reference exceeds the native application's former **200,000-vertex** preview limit. A host-selected equipment profile now supports the actual source across creation/reopen, preview and rebuild. `POET_NATIVE_EQUIPMENT_PROFILE=1` selects 1m preview vertices, 6m indices, 128 MiB retained-value allowance, 256 MiB viewport working budget and bounded rebuild work. Legacy defaults remain available; source/HMC data cannot select its own limits or grants.
* The actual reference exceeded the source-layer snapshot's fixed 64 MiB cold-copy admission. Snapshot census now precedes allocation and uses the host-selected budget; native routing preserves the prior 64 MiB operation default and admits 128 MiB with the equipment profile. These are declared representation/work allowances, not measured RSS figures. No quality reduction was used to pass.
* The new guard envelope initially conflicted with strict spatial admission. The adapter now recognizes only the declared reserved guard shape; relational validation still precedes native source promotion. General unknown root/part properties remain refused. Opaque raw-source preservation is tested separately from native rendering admission.
* Native component interpolation/property/borrow integration errors were repaired and retested. Stale local part/parameter forms cannot silently apply against a newer revision; explicit reload/reselection is required.

Historical failed attempts remain in `initial/`, `repaired/`, `corrected/` and `actual/`. A lighting-panel-only CPU test selection matched zero tests; the runner correctly refused to count it as passing. The panel is compiled, while its backend validation and actual GPU lighting behavior have executed tests. No extra mirroring test was added to manufacture a green count.

## Reused infrastructure and executed checks

This work uses QualiaDB's canonical Vibe `SourceSession`, Studio source/journal, HMC checkpoint codec, compiled `.10d`/Q42 path, native authority/contribution/review services, camera/projection and ray/triangle kernels, surface/material resolution, and Etalon-backed `PortalGpu`. Aether supplies native layout and event data. There is no replacement interpreter, mesh engine, billing system or JavaScript/V8 runtime added to the native product.

The [final verifier](saved-evidence-verification.json) checks **58 current canonical source/test targets**, their staged images and installation receipts, command/log/source pins, retained artifacts, and unchanged master/terrain/older fixture hashes. It records **127 distinct selected passing test names** across actual reference, source-layer/recovery, viewport/input, three native GPU cases, worker/intents, cooperative projection/workflow, checkpoints/carriers and host-profile checks. Some registered process-child entrypoints do no work outside their parent phase; parent cases require their actual output. Counts do not imply human acceptance.

The final native binary was rebuilt after the inspector-context correction. Existing Core/Etalon infrastructure was consumed without new Core/Etalon implementation in this round. Prior SDK WASM compilation is not restated as a new run or WASM runtime qualification.

## Open the saved result

Run [open-equipment-trial.ps1](../../../../tools/poet-foundation/round9/open-equipment-trial.ps1) yourself. With no argument it opens the retained HMC in **local diagnostic** mode. Click **Rebuild execution session**, then inspect/refresh, choose **Neutral studio**, and use orbit/pan/zoom/picking. Inspect the assembly to populate part controls, change a material or parameter, and save to a new HMC filename.

For an existing admitted cooperative project, pass `-ProjectRoot 'C:\path\to\project'`; the same built application then uses its native enrollment/authorization and contributor/reviewer workflow. The script was saved, not launched through Computer Use. Actual window behavior and operator usability remain unverified in this task.

## Remaining production work

FND-01–06 and IP-03 C1–C8 are not declared passed as whole gates. Usable human contributor/reviewer acceptance, shared native brief/task integration, full graph/rights/derivation continuity, required advanced geometry/rig operations, topology-bound UV/detail remapping, representative performance and declared runtime interoperability remain open. Alpha/wind picks and depth-correct highlighting are not qualified; the overlay shows x-ray edges, including occluded edges. View lighting is not yet an authored/persisted scene-lighting contract.

The [diagnostic ledger](../../../production/community-grounds-growing-station/claims-round9-diagnostic.json) retains all **193 catalogue + eight subject-specific obligations**, pinned to catalogue `2026-10-24.2` / `c0cb76151bf5d0776afaae4d59d7427efb65633bf359d3f4c2bda83c361c91cb`. All remain unassessed. [Original ledger](../../../production/community-grounds-growing-station/claims.json), [inventories](../../../production/community-grounds-growing-station/inventories.md) and [all open IDs](../../../production/community-grounds-growing-station/open-ids.json) remain available. Film, open-world game and cartoon packages remain incomplete. **Asset production remains HOLD; this is a retained engineering fixture and a native-authoring trial, not mission completion.**
