# Material motion: independent staged integration review

2026-10-26 · reviewer `/root/poet_full_authoring` · source review only, before coordinator installation/checks.

No blocking issue found in the selected integration bodies. This supports coordinator installation and serial qualification of the bounded workflow; it is not compilation, execution, GUI acceptance, HMC reopen evidence or a general Poet readiness claim. The reviewer authored the separate material-motion model; this review covers root/peer integration rather than independently qualifying that model. No source edits, Cargo, tests, GUI, exports or provider calls were performed during this review.

The native route preserves the existing operation owner. `native_dispatch.rs:43–53,65–73` refuses an action/source mismatch, retains the existing explicitly local diagnostic branch, and encodes authenticated material motion through the original intent bus. `poet_workspace/native_commands.rs:3–9,106–165` strictly decodes the typed edit, classifies it as Invoke with `Studio.edit` provenance scope, validates before constructing `Action::AuthoringEdit`, and takes its target from admitted context. The scope string is provenance, not an independently invented permission grant. The current intent envelope rejects client provenance/foreign targets and checks current authority before existing jobs dispatch (`C:/github/poet/native/src/graphical/poet_workspace/intents.rs:75–89,203–281`).

Both UI and MCP reach `AuthorizedEditorHost::authoring_edit`, which uses the existing `AuthoringAction::ExecuteSource` authorization/write wrapper (`cooperation/editor.rs:168–188`, `cooperation/mod.rs:410–432`). They do not establish principals or grants from request fields. Material motion deliberately follows existing authored-assembly edits, rather than the separate legacy `material_edit` action (`cooperation/editor.rs:316–323`). Current jobs reject stale origin source, execution epoch, authority, or a different target before calling the host (`jobs.rs:286–307`); completion publication checks its admitted origin and accepted successor (`jobs.rs:450–471`). No additional animation host grant is required by these adapters: the frozen helper samples existing Rust easing and submits ordinary material/data source.

`authoring/controller.rs:6–23` populates tracks from the actual current `poet_material_motion` binding after expected-source and execution checks. The empty vector in `authoring/source.rs:134–139` is only the generic view constructor; the canonical controller replaces it before returning this authoring view. `authoring/mod.rs:13–16,65–77` registers the modules and typed view/edit field. The helper call at `controller.rs:74–82` matches the frozen helper argument order, and `:67–72` refuses an existing immutable motion binding before source submission. The existing mutable `poet_parts` requirement remains at `:30–34`.

The controller returns through its ordinary `submit` at `:93`, with expected-source and retired-execution checks in the existing owner (`C:/github/poet/native/src/graphical/controller.rs:93–98,339–370`). It uses the native validated session and journal commit path, including reconciliation on failure; no parallel motion store/session was introduced. Source replay, undo/rebuild and HMC persistence still need coordinator execution evidence. Inspecting these callers does not prove successful replay or reopen.

MCP advertises typed bounds and strict nested request fields (`mcp/material_motion.rs:5–31`). Inspect is read-only and rechecks authority and source before returning tracks plus actual factors (`:39–55`). Edit validates before context consumption, resolves a connection-local current snapshot, then clears context before an attempted mutation and passes its exact SourceRef into the authorized host (`:57–77`). `mcp/mod.rs:79–111` also compares session revision, epoch and reconciliation state; `:192–218` qualifies receipts after the operation. A well-formed attempt that fails later requires a fresh read, as existing mutation tools do. Tool routing is limited to the two declared names; it does not expose the child caller as a generic tunnel (`mcp/mod.rs:247–256`).

The UI mount passes the displayed SourceRef through existing `send(..., true)` (`authoring/controls.rs:240–242,302`). `send` checks busy state and preserves an unsubmitted source draft before queuing, captures the source draft, checks completion currency, and preserves newer text entered during a request (`:57–143`). The enclosing fieldset disables interaction while busy (`:255–256`). A callback from an old render still faces the native source/context match and host checks; UI freshness is not relied on for authority.

The peer form retains its loaded SourceRef, part and material identity; `current` checks all three (`material_motion_controls.rs:53–95`). Selection/source changes do not silently reload local keys/fields. `ready` requires a fresh inspection with the same identity, and stale forms explain the explicit reload requirement (`:189–225`). Key edits validate a candidate before replacing retained state (`:126–154`); loading/reloading fields has explicit replacement labels (`:210–219,248–262`). Saving blocks uncommitted key fields and empty tracks; sampling/removal of accepted state blocks local dirty keys/fields (`:285–337`). The playhead is an explicit sample position; changing it alone appropriately does not mark the stored key draft dirty. Controls disclose integer milliseconds, linear factors, shared materials and the absence of continuous playback (`:208–209`). These are source/UI-state observations, not accessibility or current-window execution evidence.

No additional source change is requested by this review. Coordinator qualification should retain the existing narrow checks for a native edit and MCP operation sharing one admitted project, stale/denied/immutable refusal without accepted-source changes, source-draft and local-key retention across refresh, and undo/rebuild plus HMC reopen preserving full keys. Native rendering, actual human usability, host grant execution and checkpoint recovery remain unassessed here. Existing cancellation is explicitly unsupported by the original intent bus (`intents.rs:292–299`); this synchronous bounded source edit adds no claim of cancellable animation jobs or playback.

## Inspected scope and pins

Paths below are relative to `C:/github/ecopunk/tools/poet-foundation/motion-integration/work/integration/native/src/graphical/`. Ranges identify selected bodies, not a semantic audit of every legacy operation in the files. Whole-file SHA256 values were computed from the current staged files after coordinator formatting; relevant changed bodies were checked again after that formatting.

| File | Selected read ranges | SHA256 |
|---|---|---|
| `authoring/controller.rs` | 1–94; prior compile body 95–166 only as unchanged context | `8102e2cc5c931481f0ae2533c4c54bfca08aee44312677cb8bde815a69505f52` |
| `authoring/controls.rs` | 1–143,145–305 | `34d7be3cc660adbd7f3161404a1a7000f8b06f102d2479a893adcffc8248618e` |
| `poet_workspace/native_commands.rs` | 1–190 before formatting; changed/current 104–167 rechecked | `10b5cf9fb3e2ed50a62e348c9d7b59113c50d08e662b1f0ee55f0363a3e0fa20` |
| `native_dispatch.rs` | 1–80; new material-motion arm/current markers rechecked | `492532e99ece2df15c9cbd280d07034c9be8ce02bdd753a8a17c15a9237a3781` |
| `mcp/material_motion.rs` | whole 1–78 | `1573f858739a22e23f609d1d22cd88b8677e232e88045cea7ebf5e75cf84b3cb` |
| `mcp/mod.rs` | 1–159,160–345; new registration/routing markers rechecked | `e669dddf57310b9c54a28ebaca6e60b39d2ffa3776776b76d028e6b68110fe2e` |
| `authoring/material_motion_controls.rs` | whole 1–419; embedded tests read, not run | `2da7f9ffc599b8dfef018555171740105faa918a5495c50f236e99f865a2cf6b` |
| `authoring/mod.rs` | 10–22,63–82 | `4a3c6ec81db38ca0e239c3cee2c80f127671b72713305110bb273e74c0376318` |
| `authoring/source.rs` | 130–146; targeted view/variant discovery | `534896e93841e3b49e2c3cac07029aa1de84802621f79f2b8c30374a60a06314` |

The reviewed control file is byte-identical to `C:/github/ecopunk/tools/poet-foundation/motion-integration/work/controls/material_motion_controls.rs` at the hash above. This does not authorize installation over an unchecked canonical preimage.

Additional current canonical caller scope is rooted at `C:/github/poet/native/src/graphical/`:

| File | Read ranges | SHA256 |
|---|---|---|
| `cooperation/editor.rs` | 1–70,160–205,300–345 | `b34e76f6a0288a1b12936904af9796168ac6757628a6b68eacf6308ddf93798d` |
| `cooperation/mod.rs` | 410–436 | `8e538ad7a7b1017a01fddcb86b9847f80000e6a270610d2f4d34c07c17166970` |
| `controller.rs` | 90–106,335–401; targeted method discovery | `99fbbd923b9410266ca7b3fc4dac3e8fb1f55e2d4d571739ac416ddc39f7f8d3` |
| `jobs.rs` | 272–332,450–476 | `9a3c0c79741c14def521e3d1b00ffdad622f53dcb5bb8776d539faad23a2ab88` |
| `poet_workspace/intents.rs` | 70–103,203–301 | `f1d9ff820e82eee35c1c22beb0c00abd86650a630ead3394acaaca44db6a31f5` |

Existing material/source-layer generation was checked at `authoring/source.rs:189–269` with targeted later-branch discovery, reusing its prior focused study; no replacement source generator or parallel grant contract is proposed. The model module and its tests remain covered by their separate staging handoff, not independent evidence from this integration review.
