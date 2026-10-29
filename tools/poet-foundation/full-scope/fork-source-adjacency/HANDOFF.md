# Scoped botanical junction: adjacency and integrity handoff

Status: source inspection and design only, no implementation or tests executed. This is one optional starting slice, not a master-quality ceiling or a complete biological asset.

## Existing behavior

`poet/native/src/graphical/authoring/botanical/recipe.rs` admits a single rooted, unique-keyed, parent-before-child graph, validates finite bounded metre coordinates, and refuses short parent edges. `botanical_graph_host.rs` builds parent indices in source order and obtains radii for the adopted graph prefix from QualiaDB `pipe_model_radii`; the colonization suffix is an unadopted proposal. These checks establish graph structure, not 3D clearance or surface correspondence.

QualiaDB `tapered_branch_field.rs` evaluates the supplied segments in order with a left-associated smooth union over *all* segments. It does not consult organ identifiers or graph adjacency. A closed, one-component, vertex-link-manifold mesh receipt does not prove the resulting junctions correspond to the intended graph: spatially crossing or approaching graph-unrelated segments may merge. Edge-source pairs identify sample-grid edges, not organs. Preserve the source graph, parent keys, all field/grid parameters and ordering; do not mislabel sample IDs as organ provenance.

## Bounded starting slice

A separately named explicit single-junction profile can admit four adopted organs: root endpoint, central child, and two endpoint children of the central node. The exact parent relationships must be checked by identity and all three field segments must use the central organ's source position and pipe radius. This yields exactly three source edges, each pair sharing the same central organ, so there are no graph-unrelated edge pairs in the field. Repeated/coincident endpoints and zero/poorly conditioned axes must refuse. Orient vectors *outward from the central node* for any explicit angular ambiguity screen; opposite collinear vectors may be a valid trunk continuation and should not be rejected merely for collinearity. Any finite-precision angle tolerance is a screen, not an exact predicate or a proof that arms remain distinct after blending and sampling.

This restriction is only an opt-in bounded capability. It does not solve larger trees, branch shape quality, UVs, normals, intersections, lineage per surface element, or HMC/review workflow; a one-component triangle manifold may still be a blob or hide an arm. Preserve v1/v2; do not silently switch old source or create biological defaults. A diagnostic generalized global-union preview must be honestly identified as unqualified and must not be promoted as proof of intended graph junctions.

## General graphs

A later general path needs bounded checks that distinguish intended adjoining segments from spatial contact of unrelated segments. It also needs a policy for adjacent arms that overlap again away from their common junction, tolerance and sampling ambiguity, a controlled cost budget, and clear source attribution. Existing `point_segment_distance_sq_3d` is not a 3D segment-to-segment clearance certificate: endpoint-only tests can miss two segments crossing at their interiors, and its ordinary floating-point projection has conditioning limitations. AABB rejection may safely narrow candidates only if conservative outward rounding is accounted for; overlap is inconclusive. Do not declare general spatial separation from index topology or a few samples.

## Atomic edit and resource contract

Field/grid units and bounds, blend, axis and projection thresholds, minimum triangle area and explicit ordering belong in the authored or host-selected profile as appropriate; host independently enforces sample-by-segment work, extraction buffer/probe limits, topology work and source/preview limits. Use checked arithmetic for combined work and scratch, and retain source untouched on every failure, including late numeric or probe exhaustion. Derive vertices and triangles in scratch; validate the prior projection against the exact prior recipe; refuse replacement if authored UV0, atlas, normals, skin or unknown geometry streams cannot be mapped. Preserve all other part fields and maintain deterministic reconstruction. A saved graph is not a claim that every output surface point has organ provenance. Contributor/reviewer and fresh HMC reopen paths still need native execution evidence.

## Inspected source pins

- Poet `recipe.rs`: a13bb0aba6327a2ccb3d28a749c0ee1d6acd2e15ac965409f388cfb2735eab7d.
- Poet `source_edit.rs`: fcd18a1c2a280fbc4d1beadc00c3ce988f65d8f22fae4eaa271dbba5f77a2afc.
- Poet `botanical_graph_host.rs`: b360de5701e2620540419a79483d2efb0081175249c229683bd01e7cef8cad85.
- QualiaDB `tapered_branch_field.rs`: 5a78f146f37ddb5f001dc089cee0fda545d39cc777e7e4f9f404e174eadc4058.
- QualiaDB `indexed_tetra_surface.rs`: 3c062ebd250ceb45f3c8b76add5576e069ccc553dc626554ce0526298b9fbe12.
- QualiaDB `surface_topology_qualification.rs`: c5a1810e8bb2940351e7cba5247a061e8a34d7c98aebd8ba94b9199bf8650604 (pre-budgeted variant; re-pin before implementation).

A repository-wide targeted symbol search for 3D segment-to-segment distance returned no direct matching public owner; this is not a semantic audit of every specialized library or proof one does not exist.

## Transaction and replay seam (later trace)

A safe source-profile promotion cannot charge a fresh limit independently per call. `authoring/controller.rs::edit_assembly` calls `edit_source` before `self.submit`; the former regenerates the prior recipe to check geometry drift, then evaluates the candidate. `controller.rs::submit` subsequently calls `host.begin_submission`, resetting its work counter before executing the source, and calls `botanical_session::validate` afterward. That validation reparses the resulting botanical binding and regenerates the accepted geometry again. On post-execution validation failure the stored accepted source is preserved, but the controller marks reconciliation required because session/effects may already have advanced; it is not an automatic full rollback. Execution rebuild also evaluates saved cells through a total host allowance, then calls the same botanical validation.

At present `botanical_session::validate` receives an immutable host; so does authoring inspection. It can read the initial `botanical_limits` but cannot debit actual `remaining_work`. A v3 implementation must thread a bounded operation ledger through pre-submit prior/candidate generation, source submission and post-submit validation without resetting charges, while preserving rebuild's aggregate work limits. A separately admitted inspection may use its own explicit finite ledger. Source replay also needs a common accounting rule for Vibe work and native field/mesh checks. The extractor's present interface accepts a maximum probe count but does not return probes used, so a safe composition must reserve the full supplied cap unless that API gains compatible actual-consumption reporting. Checked peak scratch must account for simultaneously live sample points, grid, worst-case extractor vertices/triangles/edge-source/table, topology scratch, retained prior meshes, serialized source and session values, not just final mesh size. Use fallible bounded allocations and keep every intermediate unpublished until success.

The separately staged `botanical-operation-budget/policy.rs` is a design prototype and does not yet derive all those charges or integrate with Poet. In view of these open transaction and accounting contracts, no four-organ Poet promotion candidate was staged: a superficial profile would bypass the existing host and replay budgets. Required tests include exact-budget and one-below refusal across the entire edit/submit, revalidation and rebuild; verify stored source and untouched authored streams on refusal, the explicit reconciliation state after post-execution failure, stale revision behavior, V1/V2 compatibility, and successful HMC fresh reopen and review. Those tests are not executed evidence.
