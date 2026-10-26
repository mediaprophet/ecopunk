# Adapting source capabilities into Poet: requirements and existing-code guide

**Revision:** 2026-10-26. **Status:** source-backed adaptation specification and
agent handoff. No source extraction, native integration, build or acceptance is
performed by this document. EffectCraft at
`C:/github/storytold/effectcraft-main` is the additional source under review.

The source tools were supplied to accelerate existing QualiaDB/Webizen/Poet
requirements. Begin with the required human/agent operation and an existing
implementation example; select reusable source code to serve that operation.
Preserve useful editing behaviour without importing an independent application's
world model, navigation, authority or runtime by default. This guide extends the
[comprehensive review](poet-comprehensive-authoring-review.md), PC/TOOL and the
single [implementation plan](../implementation-plan.md).

## Existing requirements come first

Earlier local specifications already define these outcomes:

| Existing reference | Requirement to preserve | Evidence limit |
|---|---|---|
| `qualiaDB/docs/poet/03_STUDIO_CREATIVE_SPEC.md`, POET-STU-001–009 | Audio channel strips/transport, scene hierarchy and gizmos, material/light controls, 10D projection, Dual Studio source/preview, shader Forge and spatial audio. | Specification intent, not a current FPS, precision, execution or usability pass. Map individual cases to current source and receipts. |
| `qualiaDB/docs/work-in-progress/POET_UPDATED_IMPLEMENTATION_PLAN_2026-09-05.md`, ST-C/D/E | Domain engines, shared typed command/context/session contracts, toolboxes/tool chains/contextual actions, authorized host services, native/WASM workers and bounded media transfer. | Earlier ST source-home/UI wording must be reconciled with subsequent native/Aether/Etalon/Poet decisions. A planned path is not an installed adapter. |
| `qualiaDB/docs/manuals/adr/0012-construct-is-the-distributable-composition.md` | Construct is the observer/agent's coherent scope; Manifolds are lenses, containers are structures, Workspace is its live hardware projection. | A composition, source app or project is not automatically a Construct. Keep those referents distinct. |
| `qualiaDB/docs/manuals/standards/poet-motion-contract.md` | Layout/Stage/Timeline are aspects; state is not communicated only by motion; reduced motion and honest commit/error states. | Chrome animation constraints are not a prohibition on authored film/keyframe animation. Editorial time, simulation time and UI motion remain distinct. |
| `qualiaDB/docs/work-in-progress/ONE_POET_PRODUCT_CUT_WIP.md` | One human product, useful direct human paths and shared native/WASM semantics; agents assist rather than replace human operation. | Work-in-progress lexicon/history is a recovery reference. Do not create another navigation system or infer live capability from a label. |

The older architecture/master catalogues also cover project delivery, knowledge,
agreement, communication, personal records and host administration. Adding a
compositor must not narrow Poet to a media suite or overwrite those functions.
Retain exact legacy source/API spellings when citing them; current framework
vocabulary remains **identifiers**, and current native no-JS/V8, Chora separation,
rights and human-review requirements govern conflicting historical wording.

## Concrete implementation examples to follow

These are real selected source excerpts. They are patterns to adapt after semantic
review, not claims that all document domains already implement them. Exact ranges
and source hashes are in the [example coverage](../../qa/implementation-2026-10-26/effectcraft-review/ecosystem-examples.coverage.json).

| Existing implementation | What an agent should follow | What must be adapted or refused |
|---|---|---|
| `poet/native/src/graphical/native_project.rs:184–230` | `NativeProject` owns a shared editor host. `connect` admits an authorized session; `mcp_backend` and `launch` obtain projections from that same owner. | One authoritative **domain** session per document; a rich composition/document is not forced into the current spatial editor's source-value layout. Domain adapter/session support is still required. |
| `poet/native/src/graphical/native_dispatch.rs:13–135` | Compare supplied expected source with the admitted origin; convert selected typed UI actions into existing Poet payloads. Retain the explicitly labelled local diagnostic path separately. | Do not dispatch a foreign/stale target or add a generic unchecked command tunnel. New domain operations need an explicit versioned mapping, not only a renamed toolbar. |
| `poet/native/src/graphical/poet_workspace/intents.rs:203–301` | Reject client-supplied provenance, foreign context/target and action mismatch; run actual native jobs against current admitted context; publish actual operation result. | A client actor string or MCP caller is not authentication. Current cancellation is explicitly unsupported; never return completed cancellation from an unchecked/default method. |
| `poet/native/src/graphical/cooperation/editor.rs:101–145,172–193` | Validate typed resource bounds and expected package digest; authoring edits use the native write path with expected source and `AuthoringAction::ExecuteSource`. | EffectCraft selection enablement is not this authority gate. Preserve contributor/reviewer authority and revision checks before mutating its domain session. |
| `qualiaDB/crates/qualia-app-contract/src/adapter.rs` | Reuse existing adapter/command/target/session/gesture/job contracts and reconcile them with current Poet IntentBus types. | Default revision zero, available status, Null/empty snapshot, successful restore/cancel/close are not functional implementations. Admit only demonstrated supported methods; do not create a second DTO crate. |
| `poet/native/src/graphical/authoring/source_layers/snapshot.rs:1–130` | Caller-selected structural-byte budget, checked counting before clones, useful refusal preserving accepted work. | This model counts structural values, not measured RSS. Use a domain-appropriate representation and budget; don't copy a 64 MiB constant as every medium's ceiling or clone whole large compositions per preview. |
| `qualiaDB/crates/qualia-core-db/src/poet_host/invoke/studio_checkpoint/native_io.rs:1–160` | Bounded native/inert reads, verified source carrier and expected-destination guarded staged persistence. Rust module path is `studio_store::checkpoint`; physical source is `studio_checkpoint`. | Existing persistence does not automatically serialize an EffectCraft project or authorize executing loaded expressions. Preserve domain source, opaque data/resources/graph and exact expected-file semantics through the selected existing package profile. |
| `webizen-desktop/src/commands/vibe_host.rs:69–159` | Existing parse/check/catalog mechanism demonstrates source-text operations and native capability invocation. Recover useful services instead of inventing a new scripting host. | This legacy Tauri wrapper is not the native Aether projection or an authority exemplar. The sampled invoke converts invalid JSON to Null; do not copy that input behaviour as an admission contract. |

Existing test source provides useful independent cases to adapt:
`native_project_tests.rs::signed_native_project_shares_actual_edits_hmc_resources_and_revokes_ui_mcp`,
`poet_workspace/tests.rs::foreign_target_context_client_provenance_unknown_and_oversize_are_rejected`
and `revocation_refuses_dispatch_and_cancel_never_claims_cancelled`.
These tests were not run in this review. Their names, selected bodies and earlier
scoped receipts are not proof of a compositor or all-domain workflow.

## Additional source findings: actual admission decisions

The [engine trace](../../qa/implementation-2026-10-26/effectcraft-review/engine-authoring.md)
follows two connected source routes, including `comp.new`, `layer.newSolid`,
`effect.apply`, `prop.addKey`, frame rendering and project save/reopen. The
[source admission](../../qa/implementation-2026-10-26/effectcraft-review/source-admission.md)
records 38 manifests and actual dependency pins. These are source findings;
none was executed or integrated into Poet in this pass.

- Lower `Session` construction has no expression host, accelerator or exporter
  installed by default. The full source host installs Boa expressions/script
  facilities unconditionally. A non-JS profile is therefore a candidate lower
  seam to qualify, not evidence the whole host is compatible.
- Domain edit/history/revision code is reusable, but `.ecproj` stores project and
  view; `replace_project` clears session history. Preserve Poet's durable
  source/history/authority/resource records explicitly. A reopened snapshot
  cannot be presented as complete cooperative recovery.
- `execute_checked` checks command enablement, not an admitted principal or
  expected native source revision. The sampled MCP backend defaults to no path
  confinement; generated parameter descriptions are not strict typed schemas.
  Reuse current native guards and strictly validate the selected operation.
- File services have ambient defaults and some paths use direct filesystem
  calls. Injecting only a read/write service is not enough to qualify every
  plugin/export path. Route required effects through authorized host mechanisms.
- The selected export producer can observe callback cancellation and return a
  cancelled marker. Full queue/MCP acknowledgement, cleanup and late-result
  publication remain unreviewed. Preserve the current honest refusal until the
  exact producer/result path is qualified.

The [UI inventory and governing-category matrix](../../qa/implementation-2026-10-26/effectcraft-review/ui-functions.md#governing-ui-category-coverage)
cover commands, panels, gestures, shortcuts, context menus, dialogs, views and
accessibility separately from feature groups. Context-menu/dialog details and
accessibility remain unassessed; useful keyboard/menu references alone do not
close them. The additional source does not erase the original eleven families.

## Differences the adaptation must reconcile

| Source-app concept | Required ecosystem meaning and adaptation |
|---|---|
| Application-local project/composition/layer IDs | Preserve local keys and map to qualified identifiers for subjects, documents, instances, resources and parts. No equality/authority inference from matching strings. A shot/layer view of a wallaby does not become the organism itself. |
| Composition/layer/property model and history | Keep an authoritative domain model/session with editable tracks, keys, effects and history. Bind it to project/Q42 provenance and cross-document references. Do not flatten it into pixels or create a second UI-only copy. Cross-domain global atomic undo is not assumed. |
| Selection-enabled command registry | Preserve catalogues, menus/gestures and precise arguments; wrap actual execution in native capability/authority/revision admission. Availability and principal authorization are different checks. |
| JavaScript expressions/scripts | Vibe remains the authored application/REPL language. Inventory source expression semantics and pin an explicit compatibility/migration profile. Preserve original source inertly; unsupported expression-bearing imports remain held or report explicit permitted loss. Never silently drop animation or introduce Boa/JS as the required native execution tier. |
| egui app shell and standalone control transport | Recover useful panel/tool/gesture/accessibility semantics into existing Poet composition and Aether surfaces. Existing transport is a source reference; connect agents to the same governed operation as users. No hidden independent authority or auto-enabled local control server. |
| Upstream GPU/device and render/export services | Review actual selected features/dependency closure against developed Etalon and canonical Qualia GPU owners. Initial CPU frame/bounded binary handoff can be a proposed integration seam; no per-frame base64/PNG polling or invented GPU pointers. Native GPU interoperability remains measured work. |
| Editorial color/time/frames and 3D composition | Preserve explicit frame/sample/rational time, color/alpha and source resolution. Compositing cameras/lights are not organism anatomy, a physics model or a complete physical/spectral representation. Keep scientific spectral/cue state and simulation data with their actual owners. |
| `.ecproj` save and linked media | Retain editable project/resources, provenance and recognized opaque data in an HMC embodiment under an explicitly supported profile. JSON project save alone does not close HMC/Q42 completeness, streaming, contributor review or game-demo admission. |
| Plugins/export jobs | Source scripting/plugin facilities and queued jobs do not establish sandboxing, cancellation, file access or safe installation. Use reviewed host capabilities, declared resources and exact actual job outcomes; unqualified external execution remains held. |
| Branding/licences/updates/accounts | Preserve source notices and dependency/asset provenance; examine restricted brand exceptions. Product controls belong to Poet. Do not bring independent branding, updater, account or provider authority across with the engine. |

These are required reconciliation points, not proof every source limitation is
already repaired. The detailed [EffectCraft review](../../qa/implementation-2026-10-26/effectcraft-review/README.md)
separates metadata, actual engine/caller bodies, UI behaviour and unknowns.

## Worked adaptation brief: one editable composition in Poet

This is a **future qualification case**, not a new asset or implementation pass:

1. Use the existing admitted Poet project/observer context. Open an identified
   composition document in an appropriate Manifold/container; show its actual
   source revision and capability state. Preserve other open surfaces.
2. From an explicit small native-supported profile, create/open a composition,
   add a layer, set a property/keyframe and apply one supported effect through
   the actual domain commands. Avoid JS expressions for this first case; reject
   or hold unsupported source instead of silently altering it.
3. The person uses integrated timeline/inspector/Tool Chest controls. An admitted
   agent discovers and proposes/executes the same typed operation against the
   same document/revision. Both inspect the actual changed model and preview.
4. Undo/redo returns exact declared editable state. Save/reopen through the
   existing approved package/resource path preserves keys/effects, linked source,
   relevant unknown data and Q42/rights/contribution records. Do not claim that a
   standalone source-project save already proves this integration.
5. A distinct admitted reviewer inspects the exact source and output revision;
   reject/revise/accept decisions bind to it. Actual human/domain acceptance is
   recorded only when performed. A generated review record is insufficient.
6. Refuse denied/revoked/stale/foreign/oversized/corrupt work without changing the
   accepted document. Cover failed render/export, interrupted save, restart,
   unsupported script/plugin, and honest unacknowledged cancellation states.

Pin expected model values/time/color/source/resource IDs and actual artifacts.
The next bounded contract job is existing **PA-R06**, consuming PA-R01/shared
contract decisions and the EffectCraft findings; PA-R02 consumes UI mappings.
This is not a request to audit all repositories again or build a separate
compositor app. Required domain/adapter/UI changes become exact staged patches
only when implementation is resumed under appropriate source permissions.

## Agent assignment contract

Every extraction/integration assignment must include:

- Existing product requirement IDs, chosen human and agent operation, and its
  current source/caller/test/prior-evidence map.
- One existing ecosystem example above, with exact pinned source/ranges, plus
  what it proves and what must change for this domain.
- Exact source/app/host/QualiaDB owner boundaries and compatibility decisions;
  explicit unsupported/loss cases and no duplicate numerical/graph/codec engine.
- Exact disjoint file/symbol ownership, dependency preimages and persistent
  handoff. A coordinator integrates shared registries/manifests/controller code.
- Observed source-model/preview/undo/save/reopen/failure expectations, actual
  evidence levels and review requirements; command counts are not acceptance.

Current prepared jobs are in the [central register](../biosphere-work-packets.json).
Research jobs remain source-read-only with separate reports. Current scope does
not activate builds, source relocation, providers, model production or sibling
writes. Payment rails remain later work; source rights/contributions start now.
