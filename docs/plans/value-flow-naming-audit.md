# Audit: `value_flow` naming — collision review & migration plan

**Status:** audit complete · **migration executed 2026-10-17** · **Repos scanned:**
`C:\github\qualiaDB` (28 crates, core-ontologies, docs), `C:\github\ecopunk`
(source + planning corpus)

**Trigger:** "Value Flows" (`vf:`) is an existing external project/product —
the REA-derived economic vocabulary. QualiaDB's obligation-economics module
shares its name, which invites confusion and entanglement neither project
wants. This audit inventories every surface carrying the name, classifies the
rename risk per surface, and proposes the migration.

**Key finding up front:** the name lives in *Rust module paths, an invoke-mode
string, and UI/doc labels* — **not in the ontology layer.** `pc:` and `spc:`
(`ns.webcivics.net`) mint their own vocabulary; no `.n3` file imports `vf:` or
REA classes. The collision is a naming one, not a schema one — which makes it
mechanically simple but spread across many files.

---

## 1. Surface inventory — qualiaDB

### 1A. Code surfaces (mechanical, compile-checked)

| File | What carries the name |
|---|---|
| `crates/qualia-core-db/src/modalities/value_flow.rs` | **The module itself.** File name, module doc ("§23 … value flow"), two prose uses ("a value flow thermodynamically viable"), test names. Public fns (`commons_cost`, `royalty`, `pool_after`, `is_commons_discharged`, `outstanding`, `ancestor_royalty`, `royalty_tree_total`, `convert_currency`, `apply_tax`, `liquidity_*`, `usury_*`, `check_usury`, `carriage_fee_from_meters`, `accumulate_meter_to_pool`) — **function names are clean**; only the module path + prose need renaming |
| `modalities/mod.rs:137–138` | `pub mod value_flow;` + re-export line |
| `bundle/cbor_ld_manifest.rs:119–136` | `modalities::value_flow::{commons_cost, is_commons_discharged, outstanding}` inside `CommonsObligation::{is_discharged, outstanding_obligation}` |
| `q42/asset_envelope/licence.rs:293` | `value_flow::is_commons_discharged` |
| `services/swarm/mod.rs:38` + `services/swarm/settlement.rs:14,19` | doc ref + `value_flow::{commons_cost, eroi_viable}` imports |
| `poet_host/invoke/econ/finance_ext.rs:4,15` | `value_flow::convert_currency` |
| `governance/coordination.rs:20` | `value_flow::{check_usury, UsuryError, USURY_OVERAGE_PERCENT_DEFAULT}` |
| `modalities/DIRECTORY_INDEX.md:616` | index entry — regenerate via the directory-indexer skill |

### 1B. Invoke/UI surface (the breaking part — external callers)

| File | What carries the name |
|---|---|
| `poet_host/invoke/logic/governance_workbench.rs:17,30` | **`"value_flow"` invoke-mode string** — the dispatch arm external callers reach through VibeScript/poet |
| `poet_host/invoke/logic/governance_workbench_tests.rs:12–15` | test asserting `mode: "value_flow"` |
| `crates/poet/src/browser/logic_workbench.rs:90,193` | registry entries `("value_flow", "Value Flow", "💰")` and `("value_flow", "Value Flow / Commons")` |
| `crates/poet/src/browser/logic_workbench/dispatch.rs:32` | label→mode map `"Value Flow / Commons" => "value_flow"` |
| `crates/poet/src/browser/logic_workbench/descriptions.rs:89` | `"value_flow" =>` description arm |
| `crates/poet/src/browser/logic_workbench/p1_governance.rs` | `build_value_flow_panel` / `wire_value_flow_panel` fns + `make_tool_panel(document, "value_flow", …)` DOM id |
| `crates/poet/src/browser/logic_workbench/request_governance.rs:19,28` | `"mode": "value_flow"` request payloads |
| `docs/js/modality-engine.js:464–466` | `{id:'value-flow', name:'Value-Flow / Commons'}` — the JS twin registry |
| `docs/poet-app/poet-ui-*.wasm` | compiled binary embedding the strings — **rebuild artifact, not hand-edited** |

### 1C. Doc surface (prose only)

`CHANGELOG.md:259,332` · `qualiadb-website-update.md:34,112` ·
`docs/QUALIA_CORE_DB_ROADMAP_TRACKER_2026-10-13.md:416` ·
`docs/QUALIA_STUBS_AND_GAME_DEMO_ADOPTION_TRACKER_2026-10-12.md:249` ·
`docs/vibescript-full-impl-PLAN.md:845` ·
`docs/manuals/standards/qualia-decentralized-network-fabric/implementation-conformance.md:47` ·
`docs/manuals/standards/qualia-decentralized-network-fabric/source-and-current-stack-review.md:334` ·
`audit-docs/FUNCTIONS.md`, `audit-docs/modules/crates_poet.md`,
`audit-docs/modules/crates_qualia-core-db.md` (generated — regenerate)

### 1D. Deliberately clean / do-not-touch

- `core-ontologies/*.n3` — `pc:`/`spc:` self-minted; **no `vf:` usage**. The
  single exception: `selfhood.n3:66` comment citing "GoodRelations /
  ValueFlows / schema.org" as an external extension point — a legitimate
  reference *to* the third-party project; annotate or drop, but it isn't the
  collision.
- `licence_class = "SocialPermissiveCommons"` (`cbor_ld_manifest.rs:143`) —
  persisted in CBOR manifests but carries no `value_flow` name. **Leave as is**
  — renaming would be a wire-format break with zero naming benefit.
- `PermissiveRoutingLane::EnforcePermissiveCommons` (`governance/cell_graph.rs`)
  — lane name unaffected.

## 2. Surface inventory — ecopunk

**Code: zero hits** — `crates/ecopunk-terrain` never touches it. All 33 matches
are planning docs referencing upstream machinery:

| File | Hits |
|---|---|
| `docs/QualiaDB Capability Reference.md` | 10 (lines 331, 420, 897, 915, 1013, 1025, 1368, 1375, 1385, 1444) |
| `docs/plans/hypermedia-lifecycle-implementation-plan.md` | 8 |
| `docs/planning/tech/commons-packaging.md` | 3 |
| `docs/planning/tech/durability-and-distribution.md` | 3 |
| `docs/planning/tech/capability-spotlights.md` | 2 |
| `docs/planning/tech/{cooperative-layer, engine-requirements, engine-capabilities, engine-comparison, wasm-edge, map-layers}.md` | 1 each |
| `docs/QualiaDB Verified Inventory.md` | 1 |

`map-layers.md:110` also says "REA value-flow" as a general phrase — same sweep.

## 3. What the module actually is (drives the new name)

Not a generic "value flow" engine — the **economics of the commons**:
audited production cost + capped ROI = bounded obligation; agent-category
royalties accumulate into a pool; pool ≥ cost → **discharged, freed globally**.
Plus royalty-tree upstream distribution, currency/tax shunts, liquidity ODE,
E-ROI thermodynamic floor, usury ceiling, and the QDNF meter binding that
prices joules/bytes/compute directly. Resources always carry cost — time at
minimum, usually far more — so the name should cover *economics*, not just
accounting.

**Decision (2026-10-17, revised same-day): `socioeconomics`** — the user's
final pick, and the strongest fit once the social dimension is made explicit:

- **The distinguishing feature is social.** What separates this module from
  ordinary ledger arithmetic is that obligation, royalty and discharge are
  *keyed to agent category* — natural person, corporate person, non-profit,
  artificial agent under an accountable human principal (SOC-AX-14: every
  agent act resolves to a human root). That is socioeconomics by definition:
  economics embedded in social relations and standing, not generic
  flow-of-value.
- **Already canonical vocabulary.** Tensor10D `w = 4` is the socioeconomic
  domain; `soc:` is the sociosphere namespace; `data-sources/socioeconomic.md`
  exists. The modality name joins a term family the corpus already owns —
  and "socioeconomics" is a generic discipline word no project can claim,
  closing the collision cleanly.
- **Adjacent-but-distinct senses are namespaced.** `stat:` socioeconomic
  (ABS/SEEA official statistics — data *about* populations) is a different
  sense from this modality (obligation *evaluation*); `modalities::` vs
  `stat:` keeps them apart — noted once in docs to prevent conflation.
- The `net/qdnf/economics/` adjacency reads correctly under either name:
  meters *measure* resources, socioeconomics *prices* them between agents.

Rejected: `commons_ledger` (undersells scope — a cost model, not a store) ·
`commons_economics` (accurate but names the substrate, not the
agent-social dimension that distinguishes it) · `obligation_ledger`
(`economics/obligation.rs` adjacency) · `commons_recompense`,
`fair_compensation`, `discharge_pool` (all narrower than actual scope).

**Rename map:** `modalities/value_flow.rs` → `modalities/socioeconomics.rs`
· `mod value_flow` → `mod socioeconomics` · invoke mode `"value_flow"` →
`"socioeconomics"` · UI labels "Value Flow (/ Commons)" → "Socioeconomics" ·
JS `id:'value-flow'` → `'socioeconomics'` · panel fns
`build/wire_value_flow_panel` → `build/wire_socioeconomics_panel` ·
fn names unchanged (already clean) · prose "value flow" → "socioeconomics"
(or "obligation flow" where the generic flow phrase is meant).

## 4. Migration plan

```
Phase A — upstream rename (compile-checked, one commit) — EXECUTED
  1. git mv modalities/value_flow.rs → socioeconomics.rs; module doc/prose swept
  2. modalities/mod.rs: pub mod + re-export updated
  3. All call sites fixed (cbor_ld_manifest ×4, asset_envelope/licence, swarm×2,
     finance_ext, coordination, governance_workbench, computational_economics
     comment, lib.rs capability op)
  4. Invoke mode: HARD CUT — "value_flow" → "socioeconomics" (no external
     callers exist; pre-release mode string, all callers in-repo)
  5. cargo test -p qualia-core-db — run post-migration; failures unrelated to
     the rename are pre-existing (dirty worktree noted at migration start)
  + q42:ValueFlowConfigurationShape → q42:SocioeconomicsConfigurationShape
    (logic_modalities_shacl.rs — SHACL term renamed; no persisted instances)

Phase B — UI/JS surface — EXECUTED
  6. poet workbench: registry tuples, dispatch map, descriptions, panel fns,
     DOM/button/editor/results ids, request modes ("value-flow*" →
     "socioeconomics*"), capability map, command palette, tracker
  7. docs/js/modality-engine.js registry entry updated
  8. docs/poet-app wasm — NOT rebuilt here; regenerate with the normal build

Phase C — docs corpus (both repos) — EXECUTED
  9. qualiaDB: CHANGELOG (+rename entry), trackers ×2, vibescript plan, QDNF
     conformance/review, shacl-extensions.html, audit-docs ×3 hand-synced,
     modalities/DIRECTORY_INDEX line, qualiadb-website-update.md
 10. ecopunk: 11 doc files — prose sweep complete
 11. CHANGELOG entry added under Unreleased → Changed
```

**Residuals (expected):** `selfhood.n3:66` external ValueFlows citation
(intentional); `.audit/site/index.html` generated artifact (regenerates);
`docs/plans/qualia-stubs-and-game-demo-adoption-plan-2026-10-12.md` — 2 stale
lines in a **gitignored** file (left untouched); `docs/poet-app/*.wasm` binary
(rebuild).

**Verification:** `cargo test -p qualia-core-db` green · `rg -i 'value.?flow|valueflow'` returns only (a) the `selfhood.n3` external-reference comment and (b) deliberate historical mentions (CHANGELOG rename note) · workbench smoke test in poet UI.

## 5. Risk notes

- **Invoke-mode string is the only true API break.** Everything else is
  internal code or prose. If no external VibeScript callers exist yet, a
  hard cut beats a deprecation shim — the mode is invoked by name from JS.
- **No persisted-data migration needed** — no `.q42`/`.hmc`/CBOR field carries
  the name; nothing stored on disk references it.
- **`economics/obligation.rs` namespace adjacency** — whatever name is chosen,
  check it doesn't read as a sibling of `economics::obligation` (a different
  concept: project-obligation ownership). `commons_ledger` avoids the clash.
- **Generated docs** (`audit-docs/`, `DIRECTORY_INDEX.md`, wasm bundle) are
  regenerated, not edited.

## 6. Open questions

1. ~~The name~~ — **decided: `socioeconomics`** (§3).
2. ~~Invoke-mode hard-cut vs one-release alias~~ — **hard cut executed**; all
   callers were in-repo (poet workbench + modality-engine.js).
3. ~~Keep or drop the `selfhood.n3` comment~~ — **kept**; a legitimate citation
   of the external project as an extension point.
