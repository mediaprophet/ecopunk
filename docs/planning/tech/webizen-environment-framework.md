# Controlled framework, Webizen launcher and harmonised WASM applications

**Status:** specification draft v0.2 · **Updated / inspected:** 2026-10-24.
This records Timothy's clarification: **Aether**, his Dioxus fork at
`C:/github/rust-app-framework`, is the controlled framework for evolving the **Webizen Desktop
native environment**. That environment provides a **launcher and native host
services**. Poet is the unified authoring application. Harmonised WASM versions
use native environment capabilities where required.

**Chora**, the Servo-derived engine at `C:/github/chora`, remains a separate
code base with its own licensing/provenance boundary. The
[Chora engine contract](chora-engine-boundary.md) governs generic integration,
host-owned adapters and source/feature/distribution qualification.

**Etalon**, `C:/github/etalon`, is the developed wgpu fork supplying graphics and
compute mechanisms. Its [capability handoff](etalon-capability-handoff.md) records
concrete local extensions, existing Qualia consumer paths and source/version
reconciliation; do not infer its capabilities from unmodified upstream wgpu.

This corrects earlier planning that treated WASM as only legacy compatibility
or treated a separate desktop imitation as the destination. No source migration,
framework/host changes, build or runtime qualification is performed here.

Companions: [architecture](../architecture.md), [execution modes](development-modes.md),
[Poet/Craft consolidation](poet-craft-consolidation.md),
[production toolchain](assets/production-toolchain-spec.md).

## 1. Product and ownership boundaries

| Layer / repository | Required responsibility |
|---|---|
| Aether, controlled Dioxus fork, `rust-app-framework` | Shared application components/reactivity, layout/input/accessibility, native and WASM rendering adapters, platform integration seams and controlled framework evolution. Preserve upstream and Qualia-specific provenance. |
| Chora engine, separate `chora` repository | Servo-derived web/spatial document engine; independent source/workspace/license provenance and generic integration surfaces. Host adapters remain outside it. Exact role/topology requires qualification; it is excluded from Craft extraction. |
| Etalon, developed wgpu fork, `etalon` | Graphics/compute API, validation/HAL/compiler and extended resource/graph/tensor/render/WASM mechanisms. Qualify actual resolved source/features and UI/engine device interoperability while preserving owning-layer policy. |
| Webizen Desktop native environment, `webizen-desktop` and qualified extracted host libraries | Launcher, native app/window/session lifecycle, service hosting/discovery, authorized platform/device/file/GPU mechanisms and connection to QualiaDB. Evolve useful existing services rather than discarding the environment's purpose. |
| Existing `webizen-os-shell` source | Candidate launcher/shared UI/host-service implementation material to reconcile with the native environment. Its current visual shell is not automatically the approved product or a requirement to imitate an OS desktop. Final code location must avoid two competing environments. |
| Poet, `poet` | One authoring/composition interface with integrated Craft tools, content surfaces, inspector, graph/behaviour/package/claim workflows. Native and WASM are projections of this application, not separate authoring products. |
| Craft, proposed `craft` | Extracted reusable domain engines, document models, codecs and adapters supplying Poet. Standalone Craft app shells remain migration references. |
| QualiaDB and custom dependencies | Authoritative graph/identifiers/rules/rights/formats, native capability services, computation, simulation, rendering/audio and qualified execution. The application framework/launcher must not grow a parallel authority/state engine. |

“Webizen Desktop” names the intended native environment and launcher role.
It does not require retaining the unsuccessful existing desktop UX or its
implementation choices. Recover its useful behaviour/services and harmonise
them through the controlled framework. The launcher hosts/accesses applications;
it does not absorb Poet's editing responsibilities into a generic dashboard.

## 2. Inspected source: what exists and what remains unqualified

| Source | Observation / implication |
|---|---|
| `rust-app-framework/README.md`, root `Cargo.toml` | Describes the fork as Aether / Qualia application framework. Workspace declares `0.8.0-alpha.3` and includes native, desktop, web and `refract`. These are source/doc facts, not performance or end-to-end product qualification. |
| `rust-app-framework/packages/desktop/Cargo.toml` | Declares a **WebView renderer**, with `dioxus-interpreter-js`, Wry and Tao. Its “desktop” name does not establish the requested JS/V8-independent native rendering path. |
| `rust-app-framework/packages/native/{Cargo.toml,src/lib.rs}` | Blitz/AnyRender/Vello/winit path, custom-widget features, native renderer/window and device-handle surfaces exist. Candidate for native UI/specialised GPU integration; actual launcher/Poet integration, feature coverage and no-JS/V8 dependency closure need qualification. |
| `rust-app-framework/packages/web/Cargo.toml` | WASM/web-sys renderer with interpreter/wasm-bindgen dependencies exists. Separate browser infrastructure from authored application policy; do not label a browser-hosted projection as proof that the native path has no JS/V8 dependency. |
| `rust-app-framework/Cargo.toml` native dependency declarations | Includes Blitz/AnyRender and `wgpu 30.0.1`. Compatibility with the owning Qualia/etalon device/resource contracts must be checked, not inferred from a matching family name or version number. |
| `webizen-desktop/Cargo.toml`, README | Current Tauri host with Qualia/Vibe/runtime/render/audio dependencies and WebView2 bindings. Useful native services exist; adopting the fork/native renderer is not demonstrated by this manifest. |
| `webizen-os-shell/Cargo.toml`, README | Declares registry Dioxus `=0.8.0-alpha.1`, desktop/web targets and optional embedded engine; README explicitly says Windows desktop uses WebView2. No direct fork source declaration appears in the inspected manifest. Actual resolved dependency provenance still needs a complete lock/config/graph audit. |
| `webizen-os-shell/src/{components/stage_host.rs,stages/poet.rs}` | Stage routing and a descriptive Poet stage exist; they do not establish that the actual Poet editor is mounted. |
| `webizen-os-shell/crates/{webizen-host,webizen-ui}` | Toolkit-independent host and command/context/menu model source homes exist, both using `qualia-app-contract`. Reuse and qualify these contracts before proposing duplicates or new homes. |
| `chora/{LICENSE,AGENTS.md,Cargo.toml}`, engine and C-API source | Declares MPL-2.0 and generic embedding/C ABI boundary; prohibits Qualia/Webizen dependencies and implementation imports inside Chora. Chora-branded types/wrappers coexist with older package/export identifiers. Current embedded profile enables JS JIT; separate boundary and actual integration/profile closure need qualification. |

The fork README also describes webview GPU flags and an injected JS limits shim.
Those belong to that backend's implementation; they are not evidence that the
native backend requires the same mechanisms or that its rendering is qualified.
No GPU/security flags were changed and no framework claims were measured here.

## 3. One harmonised application, explicit execution modes

| Mode | Where application work executes | Native environment requirement |
|---|---|---|
| Native application | Shared app model/UI over the controlled native framework; engine/domain libraries and services through qualified native APIs. | Webizen provides launcher and required platform/service lifecycle; app definitions remain distinct from service processes. |
| WASM local operation | Same authored command/state/schema semantics compiled to the qualified WASM profile; only capabilities actually supported there execute locally. | None for operations declared local. Missing native functionality must not silently change meaning or be advertised as available. |
| WASM with native capabilities | WASM presentation/local work plus explicit jobs/queries/actions on the authorized running native service provider. | Required services must be installed/running, compatible and authorized; otherwise the affected operation is unavailable/held with a useful explanation. |

WASM is a supported target, including native-assisted use. It is not required
to reproduce every platform capability inside a browser, nor must every local
WASM operation depend on a running native environment. Dependency is declared
per capability. Native-assisted operation may work offline with an authorized
local host; “WASM” does not itself mean cloud-dependent.

Application rules and orchestration remain Vibe/native Rust over shared
contracts. No JavaScript business-logic, graph, physics or editor-state fallback
is introduced. Standard-browser deployment may have generated platform binding
machinery; its exact dependency and code scope must be declared and qualified
through the controlled fork. This user clarification authorizes the WASM target,
not an ungoverned second application implementation. A promised completely
JS-free browser artifact requires its own verified framework/host path; the
inspected web package does not establish it.

The native workflow must retain its JS/Node/V8-independent execution requirement.
Selecting a Cargo `desktop` feature cannot stand in for that proof. Use and
qualify the native renderer, or record the precise missing native capability
for upstream work rather than quietly selecting a webview dependency.

## 4. Launcher and native service contract

The native environment must support the following without becoming an imitation
of an unrelated OS desktop:

- Discover and present installed/admitted applications, their availability and
  relevant existing documents/sessions; launch or focus the actual application.
- Restore the correct context/session and expose meaningful running/loading/
  unavailable/failure states. A placeholder information card is not an application.
- Start/adopt compatible required services with explicit ownership, readiness,
  health and resource state; keep an externally owned engine's lifetime distinct
  from the launcher window and from a document view.
- Handle minimize/close/tray/quit/restart according to declared lifecycle rules,
  preserving work and leases. Closing one view must not destroy its document or
  stop another application's/shared external service.
- Expose authorized platform mechanisms through toolkit-independent host APIs,
  keeping app-specific editing and semantic policy with their owning layers.

Concrete launcher visual design remains open. The required behaviour is a
coherent launch/access environment, not eleven duplicated Craft app launchers,
a second Poet interface, or decorative status indicators disconnected from services.

The [high-grade/civics-experience contract](high-grade-production-and-civics-experience.md)
also defines the environment's experiential purpose. Qualify continuity from an
identified place/subject through knowledge and provenance inspection, scale/
representation changes, native Poet editing, checkpointed package revision and
bounded scenario comparison. High-grade Ecopunk outputs and reviewed game-demo
derivatives must support this path. A working launcher alone cannot establish
that integrated creative/exploratory experience; layout and interaction design
remain open while identifiers/state/service ownership stay explicit.

## 5. Shared native/WASM capability contract

The existing app/host/Vibe contracts must be reviewed for these conceptual
fields and operations. These are required semantics, not invented live APIs.

| Contract area | Required semantics |
|---|---|
| Provider discovery/identifiers | Identify the intended environment/provider, supported protocol, service versions and readiness. A reachable localhost port alone is not authorization or provider identifiers. |
| Capability description | Stable callable ID/schema, effects, local/native execution location, required/optional status, resource/authority needs, supported targets and limitations. |
| Session and principal context | App/document/entity/part/session IDs, authority/grants and scope, expected revision and state-owner identifiers. Revalidate on execution, not only when drawing the button. |
| Request/result and jobs | Typed bounded inputs/results or authorized payload handles, progress, cancellation, timeout, stale-result handling and replay/idempotency policy. Native-only work remains an explicit job/service action. |
| State authority | One authoritative writer/transaction policy per document/session. WASM mirrors/projections/caches do not become independent truths. Transfer/branch ownership explicitly when needed. |
| Data/resource transfer | Verified HMC/Q42 resources and bounded native handles/chunks with units/format/version/rights preserved; no graph-text parsing or base64 frame polling as the interactive render design. |
| Connection lifecycle | Absent/incompatible/unauthorized/connecting/ready/degraded/disconnected/revoked distinctions; reconnect resynchronizes revisions/capabilities before work resumes. |
| Permission and transport binding | Origin/application/provider pairing and scoped authorization for browser-to-native access; exact transport/authentication is to be qualified. No blanket CORS/native filesystem/device access just because a page is local. |
| Persistence and recovery | Distinguish in-memory browser state, pending proposals, native committed state, exported checkpoints and restart recovery. Disconnection cannot falsely acknowledge an uncommitted mutation. |
| Resource and service lifetime | Budget/lease ownership, backpressure, unload/dispose, device loss and owned-service shutdown; closing a browser tab does not orphan jobs or kill unrelated native services. |

When a native dependency is absent, preserve the current admitted content and
allow unaffected local functions. Do not simulate success, silently switch to a
different model, launch unapproved substitutes, or replay side effects on
reconnect. Mutating proposals created while disconnected retain their base
revision and require explicit reconciliation/admission; they are not automatic
committed native writes.

## 6. Controlled framework evolution and harmonisation

| Requirement ID | Requirement / acceptance |
|---|---|
| REQ-HOST-001 | Webizen Desktop MUST provide the intended native launcher/host environment through the controlled framework, preserving useful existing services and replacing the failed UX through reviewed design. Real app launch/focus/session restoration and service readiness must be demonstrated. |
| REQ-HOST-002 | Poet MUST remain the unified authoring interface; launcher, authoring and reusable Craft libraries have distinct responsibilities over shared contracts. Launching a descriptive stage or separate Craft desktop is not integration. |
| REQ-HOST-003 | Native and WASM MUST share versioned application semantics, identifiers, commands, state schemas, design/accessibility conventions and feature definitions. Per-target adapters may differ; copied editor rules/independent state engines are prohibited. |
| REQ-HOST-004 | WASM MUST declare native dependencies per capability and diagnose absent/incompatible/denied providers. Qualify browser-local and native-assisted cases independently. |
| REQ-HOST-005 | Framework consumers MUST resolve to an identified compatible fork release/revision and coherent Dioxus-family versions/features. A crate name/version alone is not evidence of the controlled source. Audit direct and transitive resolution, locks and local overrides. |
| REQ-HOST-006 | Native renderer/backend and browser host dependencies MUST be explicit. Qualify native no-JS/V8 operation separately from webview/browser projections; framework marketing or a `desktop` feature is insufficient. |
| REQ-HOST-007 | Shared UI/engine/GPU/audio integration MUST have qualified device/resource ownership, revision, layout/input/accessibility and lifetime contracts. Avoid duplicated adapters/graphs and incompatible device handles. |
| REQ-HOST-008 | Browser-to-native capability calls MUST enforce provider identifiers, scoped authority, target/revision validation, bounded jobs and auditable outcomes through the real native admission path. |
| REQ-HOST-009 | Connection loss, cancellation, restart, device/service failure and framework upgrades MUST retain user data and truthful state; stale results cannot mutate replacements or falsely commit. |
| REQ-HOST-010 | Updating the controlled framework/host MUST preserve a compatibility matrix, source provenance, staged validation, migration/rollback and known limitations across launcher, Poet, Craft adapters and WASM. Local source presence is not qualification. |
| REQ-HOST-011 | Completion MUST record the exact fork/build/backend/app/service/schema/artifact versions, actual execution modes and passed/open cases. Independent review is required for the launcher/Poet experience; no placeholder or process-only success. |

Use the fork as the controlled development home. Generic framework changes
belong there; reusable engine capability changes belong in the owning custom
library; host lifecycle/mechanisms belong to Webizen host services; product
components and editing behaviour belong to launcher/Poet layers. Retain one
coherent shared contract/component foundation rather than one private version
per app. `refract` may be qualified for relevant paths, not assumed to prove
all UI or application work has zero allocation.

Aether and Chora have different source/license identifiers. Generic Chora engine
changes stay in Chora; its host adapters stay outside it. Do not consolidate the
engine into Aether/Craft or import host/domain code into it. Source separation
and qualified interface/distribution obligations apply independently of the
runtime topology eventually selected. See CHORA-01–05 and engine register G84.

## 7. Enablement and qualification packets

All packets remain open and require separate implementation allocation.

| Packet | Deliverable | Acceptance evidence |
|---|---|---|
| ENV-01 | Framework/host/app source and dependency matrix | Exact current revisions, locks/features/platforms and real fork resolution; isolate registry/fork and desktop/native/web differences without overwriting working sources. |
| ENV-02 | Native renderer, shared UI and GPU/engine integration seam | Real launcher/Poet widgets plus specialised Qualia content on the qualified native backend; correct input, resize, scale, accessibility, device loss and lifetime; no hidden JS/V8 requirement. |
| ENV-03 | Launcher UX and useful-service reconciliation | Preserve existing services; actual app launch/focus/session restore, health/status, tray/close/quit and service-ownership behaviour. Review the intended launcher, not an OS-desktop imitation. |
| ENV-04 | Canonical capability/session/job/authority contracts | Versioned typed local/native requests, results, data handles, expected revisions and one state writer; reuse existing contracts and Vibe dispatch with explicit scope. |
| ENV-05 | Shared Poet native and WASM projections | Same identified document, commands and editing behaviour; actual native and WASM artifact/runtime evidence, with declared target limitations and lossless state handoff. |
| ENV-06 | WASM/native provider binding and loss/recovery | Test absent/wrong-version/unauthorized/revoked provider, disconnect during work, timeout/cancel/stale response and reconnect; preserve content and prevent duplicate effects. |
| ENV-07 | Controlled updates and full compatibility qualification | Staged fork/host/app updates, recovery and data/schema compatibility across native, WASM-local and native-assisted modes on declared hardware; record failures, review and exact versions. |

These requirements extend the Poet/Craft migration and the model-production
chain; they do not remove the HMC graph/representation or per-model completion
obligations. See engine register G81–G83 for upstream allocation.

## 8. Open decisions

Exact fork release and reproducible pinning policy; native backend/renderer
capability gaps; placement/reuse of existing `webizen-host` and `webizen-ui`;
launcher visual/interaction design and host cutover; transport/provider pairing;
required versus optional native capabilities per WASM operation; durable ownership
handoff and update compatibility rules. The framework fork, native launcher role,
unified Poet interface and harmonised native-assisted WASM direction are established.
