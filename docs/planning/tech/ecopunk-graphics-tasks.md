# Ecopunk graphics remediation tasks

**Governing order:** [the consolidated implementation plan](../implementation-plan.md)
now places usable cooperative native authoring and exceptional masters on the
critical path. This document retains detailed defect acceptance; its older
delivery sequence is subordinate to IP-00–09 and the IP-03 operational gate.

Created 24 October 2026 from the [evidence-based graphics audit](ecopunk-graphics-audit-2026-10-24.md). All implementation tasks below are **open**. Priorities and owners are proposed work allocation, not estimates or an approved production schedule.

**Current execution index:** the [120-packet production backlog](ecopunk-production-backlog.md)
integrates these tasks with the full authoring/model/stack/host requirements.
Its sequence supersedes the narrower delivery order below; S/A/I/P/E/Q IDs and
their acceptance remain traceable. [Etalon](etalon-capability-handoff.md) is the
developed local wgpu fork; reconcile existing capabilities and receipts before
declaring upstream gaps. Supported harmonised WASM and native-assisted delivery
are required under the [framework contract](webizen-environment-framework.md);
the JS/V8-independent requirement below applies to the native workflow.

**Specification-first correction, 2026-10-24:** Timothy's clarification governs
this backlog. Settle the native-stack, complete-species-HMC and made-world
contracts below before further graphics implementation. The original browser-led
delivery order is superseded; its observed defects remain evidence of legacy debt.

**Outcome:** native QualiaDB/custom-stack authoring and use of complete species
and made-world HMC embodiments, with multiple representations and a convincing
habitat/specimen presentation. No JavaScript, Node.js or V8 dependency may be
required for that workflow. Native capability development is an explicit project
purpose alongside content quality.

P0 = correctness or review blocker; P1 = required representative qualification;
P2 = expansion. These implementation priorities apply after the relevant
specification contracts are settled. They do not authorize upstream edits.

## Specification work first

- [ ] **S01 · Specify the native end-to-end workflow.** Review [architecture](../architecture.md) REQ-NAT-001–006: Vibe/native authoring, codecs, graph query, admission, operation and presentation; identify the owner/public contract for each seam. **Done:** no required JS/V8/Node step remains in the specified workflow; missing capabilities have an upstream packet, not a substitute implementation.
- [ ] **S02 · Specify complete species HMC embodiments.** Review [species-embodiment-spec.md](assets/species-embodiment-spec.md): full available graph corpus at an identified revision, completeness reconciliation, rights partitions, interpretation closure and selective resource use. **Done:** the contract accounts for all species knowledge held, whole/part identifiers, representations, applicable variants and offline dependencies; it is not limited to mesh traits.
- [ ] **S03 · Specify made-world things and systems.** Review [made-world.md](../made-world.md): design/type versus instance, BOM/as-built assembly, ports/connections, operation, state, failure/repair and provenance. **Done:** a vehicle and a workshop/services composition can be expressed without treating natural organisms or people as manufactured contents.
- [ ] **S04 · Reconcile the shared family schema and native APIs.** Review [asset-package-spec.md](assets/asset-package-spec.md) with upstream owners; bind logical groups to existing codecs and public capabilities. **Done:** versioned metadata/graph/variant/admission contracts and compatibility cases are concrete enough to allocate upstream implementation; G71–G74 identify remaining work. Specification review is distinct from runtime qualification.
- [ ] **S05 · Enable the native authoring suite and per-model completion gate.** Review [production-toolchain-spec.md](assets/production-toolchain-spec.md) TOOL-01–12 and [model-completion-claims.md](assets/model-completion-claims.md). Every model gets the [193-deliverable template](assets/model-production-template/README.md), expanded inventories and evidence-backed claims for film/game/cartoon and all applicable movement/behaviour. **Done:** G75–G78 have owned, versioned interface/qualification packets; no installed app name or unchecked claim is mistaken for complete model production or enforced release validation.

## First delivery sequence

1. Complete S01–S04 and allocate the required stack work through the upstream register.
2. Establish native artifact/execution evidence (Q01); qualify shared codecs, graph access, HMC selection and complete-root admission before calling the workflow delivered.
3. Repair the native source/manifest/package path (P01–P03), then author the representative species and artefact resources through that path. Keep their graph completeness and representation quality as separate acceptance obligations.
4. Compose the habitat and specimen views; finish materials, camera/UI, animation, water and LOD using qualified native capabilities. I01's specific browser upload error is legacy work if that projection is retained.
5. Present native fixed-camera and motion evidence for human review (Q03). Extend the reviewed family while retaining its complete available knowledge and resource inventory. The master backlog separately gates required native/WASM-local/native-assisted coverage; browser texture and presentation defects cannot be waived as legacy merely because native output passes.

## Models, materials and environment art

- [ ] **A01 · P0 · Define the two actual visual targets.** Owner: art direction. Dependencies: none. Create an Ecopunk world reference board and a separate scientific-lab reference board, drawing on the existing T3/T1 direction. Include full-scene composition, tree silhouette, animal proportion, surface closeups and light. Deliver a short list of visible criteria and rejected failure examples from this audit. **Done:** a reviewer can compare an in-engine image to a concrete target; “PBR”, “high-resolution” and “digital twin” are not used as acceptance criteria.

- [ ] **A02 · P1 · Rebuild a hero tree at organism and walk distances.** Owner: botanical artist / procedural author. Dependencies: A01; integration uses P03. Select the habitat species explicitly. Author trunk taper, root flare, branch unions, primary/secondary/twig hierarchy, characteristic asymmetry and open crown structure. Replace the near-view opaque canopy shell with plausible leaf-bearing shoots; retain aggregate representations only where a distant LOD justifies them. **Done:** a label-free silhouette, neutral-light turntable, trunk closeup and habitat view all pass review; the crown has deliberate gaps and leaf/branch scale is plausible.

- [ ] **A03 · P1 · Finish species-specific bark and foliage materials.** Owner: materials artist. Dependencies: A02, I01. Author bark colour/roughness/relief, branch-to-trunk transitions, leaf upper/lower surfaces, midrib and edge shape, restrained colour variation, and sun-facing/backlit appearance. Set texel density from the inspection distance. Use geometry leaves or masked cards deliberately; test the chosen path. **Done:** bark, leaf and wood read as different surfaces; no exposed UV seams, giant veins, repeated colour bands or visibly solid crown cores at the approved distance.

- [ ] **A04 · P1 · Derive a small tree/understorey family.** Owner: vegetation artist / ecology reviewer. Dependencies: accepted A02/A03. Produce distinct age/shape variants, one shrub, grasses/sedges appropriate to the slice, and deadwood/litter. Preserve species identifiers and use habitat constraints rather than modulo selection. **Done:** variation changes branching and placement meaningfully; closeups retain identifiers and the stand avoids a repeating silhouette rhythm. Keep broader taxon production queued until this family is accepted.

- [ ] **A05 · P1 · Rebuild the kangaroo hero model.** Owner: creature artist. Dependencies: A01. Establish orthographic proportions and landmarks before detail: skull/muzzle, ears, shoulder girdle, pelvis, haunch, hock, long feet, forelimbs, tail base and taper. Create coherent deformation topology and separate eyes/nose/claws/coat materials. **Done:** recognisable without its label in side/front/three-quarter views, credible beside the rover/person, and clean in neutral light and the habitat. Increasing SDF sampling alone does not satisfy this task.

- [ ] **A06 · P1 · Author and integrate animal motion.** Owner: rigger / animator. Dependencies: A05, P03; engine integration through E04 if needed. Deliver neutral pose, idle/breathing, feeding, locomotion and transitions; review feet and tail contact, balance, body compression, ears and secondary motion. Establish each clip's intended speed. **Done:** no sliding, disconnected joints, obvious ground penetration or frozen lighting during turns; provide a short continuous native-runtime capture and reduced-motion alternative.

- [ ] **A07 · P1 · Finish rover and ecologist as scale anchors.** Owner: hard-surface / character artists. Dependencies: A01, I06. Rover: wheel/tyre shape, wheel wells, body bevels, glazing thickness, lights and suspension/contact. Ecologist: human proportions, hands/feet, face readability, clothing and carried equipment. Split paint/metal/rubber/glass/fabric/skin materials. **Done:** believable silhouettes and distinct light responses at gameplay distance; wheels and feet meet the terrain during motion. Reusing pebble texture across both assets is removed.

- [ ] **A08 · P1 · Recompose one habitat foreground.** Owner: environment artist / geospatial integrator. Dependencies: A01; tree/animal work proceeds alongside. Design foreground, middle distance and backdrop; terrain breaks, path width/camber, shoreline transitions, rock exposure, grass clusters, litter and deadwood. Choose either evidenced place-specific terrain or an explicitly illustrative study environment. **Done:** the default view has a clear subject and route; no conspicuous bare surface expanses, oversized repeated tufts, arbitrary rock obstacles or obvious scene boundary. A road beneath a decorative boulder is a failed composition.

- [ ] **A09 · P1 · Author a coherent ground and rock material set.** Owner: materials/environment artist. Dependencies: A08, I01. Separate soil, path, bedrock, loose rock, litter, vegetation and wet surfaces; combine broad masks with local detail and controlled repetition. Model distinctive rock shapes rather than uniformly rounded pebbles for every geology. **Done:** material changes correspond to landform and moisture, with convincing scale at walking height and no stretched texture dominating slopes.

- [ ] **A10 · P1 · Finish shore and water appearance.** Owner: environment/technical artist. Dependencies: A08/A09, I03; E05 if its fixture fails. Author bank profile, shallow/deep colour response, exposed/wet stones, submerged detail and appropriate surface motion. **Done:** water meets the bank, retains readable depth and believable opacity, and avoids a uniform cyan sheet or shoreline seam across the approved camera path.

- [ ] **A11 · P1 · Rebuild botanical lab presentation and organ detail.** Owner: botanical/scientific illustrator. Dependencies: A01, I04, P03. Finish the whole-organism view and separate stem cross-section, flower, fruit, leaf and tissue study assets. Make tissue boundaries, cut surfaces, thickness, veins and floral structures legible. Use explicit magnification/layout changes for microstructures. **Done:** the default view contains the complete subject, cutaway/explode controls reveal the named structure, colour is explanatory, and a specialist review distinguishes schematic simplification from asserted species anatomy.

- [ ] **A12 · P1 · Rebuild the vertebrate lab around one reviewed animal.** Owner: anatomy modeler / specialist reviewer. Dependencies: A05, I04. Align skeleton, muscle and exterior to one landmark set; correct joints, muscle attachments and layer thicknesses; provide intact, cutaway and exploded views with purposeful framing. **Done:** all three layers fit coherently, landmarks can be selected/highlighted, and the animal remains identifiable. A readable schematic may ship as T1; do not call generalized archetype geometry a verified species reconstruction.

- [ ] **A13 · P2 · Produce derived LODs and species/place expansion.** Owner: asset team. Dependencies: accepted hero family, Q02, E03/E04 as needed. Derive LODs from the approved masters with distance-based silhouettes, foliage coverage, material and rig compatibility. Extend to wombat, platypus, eagle and remaining taxa; make each place recognisable through landform and ecology. **Done:** transitions preserve volume and identifiers in motion, each species has review evidence, and collection counts refer to qualified render assets separately from semantic catalog entries.

## Native integration and presentation

- [ ] **I01 · P0 · Correct texture-role admission.** Owner: Ecopunk integration. Evidence: G01. Upload the four current colour maps using the colour role; introduce explicit per-resource role declarations for future normal/data/mask maps. **Done:** a material fixture shows all declared textures resident under the exact semantics requested by materials; the selected native materials bind the correct maps; the specific legacy browser role bug is corrected only if that projection is retained. Add a focused role/residency regression; do not merely update the displayed material count.

- [ ] **I02 · P0 · Surface material and renderer failures.** Owner: Ecopunk integration, with E01. Evidence: G01/G02. Handle texture upload return values and errors, distinguish required from intentionally optional maps, and expose missing bindings in diagnostics. Handle init failure explicitly. **Done:** invalid digest, wrong role and unsupported backend produce an actionable receipt; normal startup shows the actual bound materials. Preserve verified loading and deliberate fallback policies.

- [ ] **I03 · P0 · Calibrate one shared look configuration.** Owner: technical art / integration. Evidence: G02. Use a neutral grey/colour/roughness fixture and a representative branch, then isolate direct light, ambient/IBL, exposure, AO, shadow and postprocessing contributions. Record a shared native look configuration; compare any separately scoped projection only against identified matching inputs. **Done:** pale bark and white paint retain form, dark surfaces retain detail, contact is readable, and matching-path captures explain any remaining differences. Investigate a shader change only through E02 if configuration and inputs do not explain the failure.

- [ ] **I04 · P0 · Fit the camera to each subject and state.** Owner: viewer/integration. Evidence: G09. Make a single camera owner; fit bounds to the unobscured viewing region, store per-scale views and useful selected-part focus, and account for exploded extents. Keep inspect and walking camera behaviour distinct. **Done:** all five default scales fit on screen, selected microstructure is readable, resize preserves the subject, and first render after selection does not override the fitted view.

- [ ] **I05 · P1 · Give the scene priority in the interface.** Owner: interaction designer / viewer developer. Evidence: G09. Collapse diagnostics, use responsive drawers, simplify headings, provide a clean inspection view, and synchronize active scene/scale/callout state. Project selected pins onto geometry or visibly isolate/highlight the relevant part. **Done:** desktop, narrow-window and keyboard use leave a useful view; no cropped controls or stale vertebrate labels after returning to the world; explanatory detail is available on demand.

- [ ] **I06 · P1 · Unify physical and display transforms.** Owner: world integration. Evidence: G08. Document and implement one world-to-render conversion for terrain, camera, actors, water and shadows. Remove arbitrary displacement compression/clamping from the accepted exploration path, or explicitly identify the view as a bounded schematic. **Done:** a known metre displacement, rover dimensions, human height and tree size agree in the same rendered scene; camera near/far and contact tests pass through movement and site changes.

- [ ] **I07 · P1 · Stop rebuilding for selection and idle motion.** Owner: integration. Evidence: G12. Separate pin selection from mesh generation; cache immutable compiled assets; coalesce parameter edits; update only changed actors and geometry. Separate cold authoring from frame execution. **Done:** selecting a pin causes zero compiles, an idle actor causes zero position uploads, and slider gestures remain responsive with final-state correctness. Measure allocations and timings; use existing engine APIs before adding new ones.

## Asset pipeline and reproducibility

- [ ] **P01 · P0 · Repair model-set source and manifest consistency.** Owner: asset pipeline. Evidence: G10. Recover or replace the missing documented build command, identify intended authoritative files, regenerate hashes/byte counts from those outputs, and correct the stale README. **Done:** all 124 references resolve with matching digests and lengths, a clean rebuild is documented, and its output is reproducible. Do not regenerate manifest hashes blindly over unexplained replacements.

- [ ] **P02 · P0 · Define a reviewed source asset package.** Owner: asset pipeline / artists. Dependencies: A01, P01. Each hero keeps editable source/recipe, references/licence, units/pivot, topology/UVs, materials, rig/clips, variants, LODs and output digests with review status. **Done:** another author can reopen and modify it; approved appearance is linked to a specific compiled artifact. A sealed file alone is not source recoverability or art approval.

- [ ] **P03 · P1 · Make the demo consume the assets being reviewed.** Owner: Ecopunk integration / asset pipeline. Evidence: G11. Bind stable entity/part IDs to the selected admitted asset package; use that HMC embodiment in the native specimen studio, habitat and renders, including its complete graph and selected representation resources. Retain runtime procedural construction only where it is a declared requirement. **Done:** changing one authored asset and rebuilding its package updates all relevant views; the live viewer reports the exact variant/digest. Include material/rig dependencies and preserve semantic selection through LOD changes.

## Engine work — qualify existing capabilities first

- [ ] **E01 · P0 · Qualify texture binding diagnostics.** Owner: QualiaPortal. Dependencies: I01/I02 fixture. Verify existing residency/deferral receipts reach this host; extend them only where missing. **Done:** the host can distinguish absent, wrong-role, decoded, resident and intentionally deferred textures, without replacing integrity verification or silently calling a fallback fully textured.

- [ ] **E02 · P1 · Qualify colour, material and contact rendering.** Owner: renderer. Dependencies: Q01, I03. Versioned native fixtures cover colour-space conversion, tone mapping, material maps, shadow bias, AO and camera scale. **Done:** each difference is explained or fixed with a minimal regression; captures and actual settings accompany results. Allocate missing generic capabilities to the owning dependency with a concrete public contract and acceptance fixture.

- [ ] **E03 · P1 · Qualify the real foliage path.** Owner: renderer/vegetation integration. Dependencies: A02/A03. Test authored branch assets for two-sided normals/light, chosen mask/geometry representation, mip coverage, shadowing, wind attachment and distance transitions. **Done:** leaf density survives mip/LOD changes without obvious shimmer, halos, disappearing crowns or detached leaves; unsupported features have explicit fallback and target coverage. Add generic support only for observed gaps.

- [ ] **E04 · P1 · Qualify scene instances and animation updates.** Owner: renderer / host integration. Dependencies: Q02, A06, I07. Exercise existing instancing, culling, skinning and LOD with separately identifiable scene objects; integrate stable actor transforms rather than deforming a monolithic world unnecessarily. **Done:** measured draw/upload/CPU costs improve on the fixture; normals, tangents, bounds, shadows and picking remain correct. The current engine already updates dirty normals—preserve that correctness.

- [ ] **E05 · P1 · Qualify water/depth behaviour.** Owner: renderer. Dependencies: A10's fixture, I06. Check shallow/deep depth response, opaque/transparent ordering, shore intersections, reflections and camera movement on supported backends. **Done:** isolate host scale/material issues from shader faults, fix demonstrated faults, and retain a native regression scene. Additional water simulation is not a prerequisite for convincing still water.

## Qualification and completion

- [ ] **Q01 · P0 · Establish a versioned visual baseline.** Owner: QA/build integration. Dependencies: none. Record Ecopunk + engine/dependency identifiers, native binary/package hashes, host/adapter/backend, viewport/display scale, camera, settings, HMC root/representation and accepted capabilities. Capture default world, walk, tree/animal inspection and five lab scales. **Done:** another native machine can reproduce the exact inputs; legacy browser evidence is explicitly separate. Existing browser screenshots remain historical appearance evidence, not native conformance or a reproducible performance baseline.

- [ ] **Q02 · P1 · Measure the cost of the proposed slice.** Owner: performance engineering. Dependencies: Q01 and initial authored slice. Separate startup generation/decode/upload/first-present from steady-state CPU/GPU costs; collect frame-time distributions, allocations, draw counts, texture/mesh memory, uploads and LOD/cull counts on representative hardware. **Done:** agree device-specific budgets from the measurements, then assign optimization tasks to observed bottlenecks. Do not assert 60 FPS or a universal polygon budget without evidence.

- [ ] **Q03 · P1 · Human acceptance of the complete slice.** Owner: art reviewer + subject specialist + QA. Dependencies: P1 art/integration work; relevant engine fixtures qualified; Q01/Q02. Review matched before/after native-runtime frames, neutral-light turntables and continuous motion; examine specimen interactions and responsive UI. **Done:** explicit visual approval is recorded, scientific claims have an independent review status, runtime/dependency checks pass, and known limitations are visible. Compilation, checksums, counts or an automated image score cannot close this task.

## Completion rules

Every closed task links to its output, input revision and the relevant acceptance evidence. An engine task may close as **existing capability qualified, no engine change required** with a passing fixture. A model task closes only when its intended appearance and use are reviewed. New styles, distant regions and catalog breadth remain planned work; they do not displace the unfinished representative scene.
