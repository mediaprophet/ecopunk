# Native growing reference producer

Source-only handoff for the existing Poet native backend. The six example modules
are saved under `C:/github/Poet/native/examples/growing_reference/`; root owns the
Cargo example registration and all builds/runs. No Cargo, GPU, GUI, OS credentials
or human acceptance was exercised by the source author during this handoff.

The example uses `NativeProject`, `SessionEditorHost` and `EditorHost`, with the same
canonical source/authority/checkpoint/authoring backend as the native interface.
Public, non-production test seeds from `native_project_tests` sign an explicitly
scoped SoftwareAgent fixture admission and fresh controller challenges. The author
has contributor membership; only the distinct reviewer has an explicit source
review grant. These public fixture keys confer no real-host trust or human role.
They are never emitted into retained outputs or the HMC.

Root can run `growing_reference produce EXISTING_EMPTY_OUTPUT PROFILE ASSET_SOURCE_ROOT`
with absolute paths and `PROFILE` one of `film`, `open_world_game`, `cartoon`.
It submits the master and selected preset, then performs actual parameter, material
and transform edits through `EditorHost.authoring_edit`. The exact accepted source
is `source_cells.join("\n\n")` UTF-8, matching the native source-to-asset digest.
It saves a source checkpoint, installs compiled `.10d` plus native Q42 through
`compile_checkpoint`, and retains the raw master/preset/source/lineage as HMC
resources. Each native replacement uses the previous expected file SHA256.

The author records measured software execution wall minutes rounded down with
unknown compensation status, then requests exact-source review. Source authorship
before this run is excluded from that effort. A new process reopens the durable
journal and HMC, proves fresh reviewer key possession, explicitly reconstructs the
accepted execution and checks exact deterministic `.10d` equality. All images use
the decoded HMC `.10d`, normals, UV0, material ranges and stable semantic part IDs.
The actual native PortalGpu/Etalon path produces four 1024 inspection PNGs under
synthetic neutral probe/sun lighting. This inspection size is not a master ceiling.
No separate renderer, authored geometry kernel or document owner is introduced.

Review remains pending by default. After independent visual/source inspection,
root may separately invoke `review OUTPUT EVIDENCE_FILENAME accepted` (or
`changes-requested`/`rejected`). This repeats technical checks, binds the supplied
bounded evidence file by digest and records only a SoftwareAgent source decision.
Supplied prose is not automatically verified as human or visual acceptance.
Reviewing source does not approve the complete HMC, rights, consumer mapping,
film/game/cartoon package or quality gates. All those claims remain incomplete
under the production dossier and catalogue obligations.

Budgets are explicitly one million compiled vertices, six million indices,
256 native source dispatches and 128 MiB retained-value accounting; they do not
silently simplify film geometry. Source/output writes use create-new operations;
an interrupted run's evidence is preserved and another run requires a fresh empty
output directory. No output is written to the canonical Poet executable path.

`save-receipt.json` pins actual source files. Later execution receipts must be
assessed separately; source presence is not evidence that this producer has run.
