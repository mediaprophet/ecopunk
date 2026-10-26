# Round 8 parameter regeneration loss prevention

2026-10-26. **Staged, not installed or executed.** This is bounded loss prevention,
not a complete nondestructive editing stack or foundation-gate acceptance.
Asset production remains HOLD. No production sources, assets or sibling repositories
were written; no Cargo command, build, test, provider call, commit or publication ran.

## Evidence and reproduced-case proposal

Inspected actual Poet source before staging:

* `native/src/graphical/authoring/source.rs:167–217` constructs a Parameter cell ending
  with `poet_parts = poet_rebuild_parts(poet_parameters)`.
* `authoring/assembly_source.rs` preserves edited fields in `poet_parts` by reference;
  UV atlas/detail/material/part/parent edits do not amend the rebuild function.
* `authoring/controller.rs:41–53` submits the generated cell into the existing
  `SourceSession`, making a replacement the accepted next source revision.
* `authoring/tests.rs` previously accepted Parameter after transform/material edits,
  checked the regenerated translation, and used undo to recover material edits;
  it did not require the parameter revision to retain them.

Therefore the source contains a direct loss path. No runtime reproduction was
executed by this worker. Proposed baseline case: submit the synthetic procedural
fixture; edit shared material and part transform (then atlas/detail/structure in
separate cases); apply a parameter; compare every field and accepted cell history.
The former generator replaces the mutated parts wholesale. The staged cases demand
pre-execution refusal and unchanged source/session/history instead.

The first evidence/options dispatch preceded implementation. The supervisor approved
option B as bounded loss prevention. Option A, a durable modifier stack with
qualified topology/UV/detail dependency remapping, remains open and is not simulated
with hidden command replay.

## Chosen contract

An author explicitly initializes a source-owned mutable `poet_parameter_parts`
baseline before downstream edits. The host permits a parameter operation only when
the full current `poet_parts` value equals that baseline. It then evaluates the
rebuild once into the baseline and assigns `poet_parts` from it in the same canonical
cell. SourceSession retains and checks both bindings; the existing Studio journal
and HMC source history retain the cell. There is no host-side shadow document,
automatic baseline repair, private interpreter, dialect/version change or replay.

Any downstream difference, including unknown extension fields, material, detail,
atlas, geometry, parent, translation, part addition/removal or ordering, refuses
before `submit`. Refusal explains the retained work and the next safe source action.
Even equal baseline/current values refuse active atlas/detail/projection,
nonidentity part UV transforms and geometry beyond vertices/triangles: their
topology and brush/seam mapping cannot be qualified by this control. Existing
source-owned transform/material values remain possible; a later native edit causes
a difference and holds further regeneration.

The baseline is an explicit authoring opt-in, not proof of an arbitrary manually
assigned value's provenance. Resetting it onto edited parts is not a semantic merge.
Arbitrary direct Vibe source can still explicitly replace data; this guard governs
the native Parameter intent and its source generator.

The iterative comparator admits one million values, 64 pending traversal frames
and 1 MiB of record-key/string text, with bounded traversal storage. It compares
primitive types and f64 bits exactly; unknown domain variants refuse. Existing
assembly admission retains its 256-part limit and canonical source/retained-value
budgets remain active. The baseline consumes additional retained-value capacity;
no allocator or full-workload memory qualification is asserted.

## Concrete migration and usable sequence

1. Save the current accepted source/history and complete resource carrier under an
   explicit new checkpoint name using the current native save route. Keep the
   original package and any unsent text. Do not replace an accepted master.
2. For a fresh procedural source, initialize its bindings in this order, before any
   native transform/material/structure/UV/detail edits:

   ```vibe
   let mut poet_parameters={offset:0.0,channels:6};
   let mut poet_parameter_parts=poet_rebuild_parts(poet_parameters);
   let mut poet_parts=poet_parameter_parts;
   ({parts:poet_parts});
   ```

   The existing canonical rebuild function must already be in source. This is
   ordinary pinned Vibe 0.1, not a released dialect extension.
3. Inspect accepted output. Change parameters repeatedly while still at the
   recorded procedural base. Each operation updates both source-owned bindings
   once. Parameter type/precision and untouched parameter values remain retained.
4. Then apply material, structure, UV and detail edits. A later Parameter action
   refuses rather than replacing them. Undo the relevant source revisions and
   explicitly rebuild if returning to the original procedural base is intended.
5. To migrate already edited legacy source, first incorporate required edits into
   the rebuild function and verify the resulting explicit source revision, or make
   a separate derivation from the saved original. Initialize the baseline only in
   that reviewed recipe setup. **Do not merely assign the edited `poet_parts` into
   the baseline.** Active atlas/detail/richer geometry still requires a future
   qualified regeneration operation; it is intentionally refused here.

Compatibility change: legacy recipes with no explicit baseline remain readable,
saveable and editable through other native part controls, but native Parameter
regeneration now refuses with the initialization procedure. A supplied immutable
baseline refuses before candidate evaluation. This deliberately replaces the
previously unsafe overwrite behavior; it does not alter the 0.1 language.

## Stage and checks

The [manifest](../../../../tools/poet-foundation/round8/edit-safety/manifest.json)
pins five actual before images and eight candidate after images under
`tools/poet-foundation/round8/edit-safety/`. The three new files have no original
target. Every live target still matched its before hash/absence at manifest creation.
Only the supervisor may compose and apply the reviewed images, guarded by hashes.

| Target | Purpose |
|---|---|
| `authoring/parameter_safety.rs` | Focused 105-line iterative comparison and conservative preflight |
| `authoring/source.rs` | Baseline-aware generator, fail-closed legacy wrapper, one rebuild/assignment cell |
| `authoring/controller.rs` | Current SourceSession values and baseline mutability; no core changes |
| `authoring/mod.rs` | Thin module/test registration |
| `authoring/tests.rs`, `structure_tests.rs` | Explicit fixture baseline and deliberate overwrite-to-refusal expectation; original coverage retained |
| `authoring/parameter_safety_tests.rs` | Positive repeated edits, source/value/history refusal, native undo and resource bounds |
| `authoring/parameter_safety_restart_tests.rs` | Synthetic fresh-process HMC save/reopen, held open, explicit rebuild, refusal and undo/regeneration |

`rustfmt --check --edition 2021 --config skip_children=true` passed for the three
new files and the three implementation/module routes. Existing test files retain
their original formatting to avoid unrelated rewrites. A no-index diff inspection
confirmed narrow existing-file changes. New files are 105, 359 and 155 lines.
No compilation or execution evidence is claimed.

Supervisor verification selections after review/application: the
`authoring::parameter_safety_tests` group, the exact parent
`authoring::parameter_safety_restart_tests::parameter_safety_and_recipe_history_survive_fresh_process_hmc_reopen`,
the existing authoring and structure selections, and relevant UV/detail native
workflow regressions. The restart child is dispatched by its parent; invoking it
without phase environment does no recovery work. All proposed cases remain unrun.

## Integration risks and remaining work

The root owns native UI changes. Existing `authoring/controls.rs::send` checks for
unsent text before dispatch, but copies an edit reply's snapshot into draft/buffer
even when `reply.operation_succeeded` is false. Jobs may include an unchanged
snapshot on refusal. The root was notified: refusal must not overwrite text typed
while a request is in flight. Gate draft replacement on successful operation and
unchanged draft/context; preserve local field drafts with explicit status. Native
guard preservation is not evidence that this UI case has passed.

Suggested parameter-help text: “Parameters regenerate the recorded procedural
base. Set up poet_parameter_parts in the recipe before part/surface edits. Later
edits are preserved by refusing regeneration; merge them explicitly into source or
create a separate derivation. UV/detail remapping is not yet qualified.”

No full nondestructive modifier stack, automatic topology/UV/brush remap, native
human usability, shader/cache fidelity, representative memory/latency or whole
cooperative production claim is made. Existing source/epoch guards invalidate old
inspection context after a successful accepted parameter revision; this stage adds
no cache that could retain an atlas from a different topology. Full modifier/source
layer design and qualification remain required for growing-equipment authoring.

FND-01–06, IP-03 C1–C8 and broad ECO/TOOL requirements remain open. No model or
package was changed or accepted; film, open-world game and cartoon production
remain incomplete, with all 193 catalogue and eight subject-specific claims
unassessed in the unchanged growing-station dossier.
