# Independent review: Poet capability adaptation guide

**Final document-only pass · 2026-10-26.** Reviewed `docs/planning/tech/poet-capability-adaptation-guide.md` against the assigned EffectCraft UI inventory, current Poet/Craft and Aether specifications, and the saved EffectCraft review index. No source implementation bodies were reopened, no builds/tests/runtime actions were performed, and no guide, plan, or source was edited by this review. The source-admission and engine-authoring reports are now present in the review directory; their implementation findings below are reported as the guide's and index's bounded source conclusions, not independently re-audited here.

## Prior documentation findings resolved

| Prior finding | Current location | Resolution |
|---|---|---|
| Guide linked a missing EffectCraft review `README.md`. | Guide lines 110–111; `docs/qa/implementation-2026-10-26/effectcraft-review/README.md` | Resolved. The index now exists and identifies the separate source-admission, engine-route, UI, and ecosystem-example scopes and limits. It expressly says no native compositor, product, or model acceptance occurred. |
| “Complete EMF” was ambiguous with Enhanced Metafile. | Guide line 104 | Resolved. It now says “complete physical/spectral representation,” distinguishing the intended physical/spectral meaning from VectorCraft's Enhanced Metafile (EMF). |
| Guide did not link the eight governing UI categories. | Guide lines 88–92 | Resolved. It links directly to the UI matrix and says context-menu/dialog detail and accessibility remain unassessed. |

## Current assessment

The guide remains consistent with the Poet-first product boundary. It treats source applications as inputs to human/agent requirements, preserves existing Poet functions, and does not require a bulk move to `C:/github/craft` or a separate compositor product. It distinguishes Construct as observer/agent scope from a project/composition (lines 22–25, 63, 98); the worked composition is still explicitly a **future qualification case**, not an implementation or acceptance claim (lines 113–138). The source-example section calls its examples patterns to adapt and says tests were not run (lines 35–57).

It preserves the human/agent operation: direct integrated controls, an admitted agent using the same typed operation against the same document/revision, and a distinct reviewer (lines 124–133). These are required cases rather than claims that the complete workflow exists. Its VibeScript guidance retains Vibe as the authored language, records Boa in the full source host, treats a lower non-JS `Session` as only a candidate seam, and explicitly rejects Boa/JS as the required native tier (lines 68–71, 101). The Aether boundary treats the Tauri/egui application surface as source behavior rather than the native authority target (lines 50, 67, 102); the GPU row calls CPU handoff a proposal and native GPU interoperability measured work (line 103), consistent with the Etalon and framework contracts.

## Current bounded source findings recorded by the guide

The new “Additional source findings” section (lines 59–92) makes its evidence level explicit: linked routes/manifests are source findings and were not executed or integrated into Poet (lines 61–66). It records the following limitations and candidate adaptation seams:

- Lower `Session` has no default expression host, accelerator, or exporter; the full source host installs Boa. The lower profile is a candidate seam, not proof of full-host compatibility (lines 68–71).
- `.ecproj` stores project/view and project replacement clears session history; the guide does not treat save/reopen as cooperative recovery (lines 72–75).
- `execute_checked` enablement and a default-unconfined MCP path are not native principal/revision admission; generated parameter descriptions are not strict schemas (lines 76–79).
- File-service injection alone does not qualify direct filesystem paths across plugin/export operations (lines 80–82).
- A selected export producer can return a cancellation marker, but full queue/MCP acknowledgement, cleanup and late-result publication remain unreviewed (lines 83–86). This is consistent with the UI inventory's narrower `renderQueue.stop` source-route observation; neither review claims successful, acknowledged cancellation.

These are attributed to the guide and the now-saved bounded source reports. This document review does not confirm their implementation details or expand their tested scope. In particular, no cancellation contract should be presented as qualified until the full queue/MCP result path and cleanup are reviewed and accepted.

The linked UI inventory remains appropriately bounded: command, panel, gesture, shortcut and selected view routes were inspected; context menus and dialogs were not body-audited; accessibility remains unknown. The guide links that distinction directly at lines 88–92. Keyboard/menu source presence is not accessibility or native interaction evidence.

## Acceptance and remaining status

The guide says it performed no source extraction, native integration, build, or acceptance (lines 3–6). Its new source findings also say they were not executed or integrated (lines 61–66). The worked case and reviewer/undo/reopen/error conditions are prospective requirements, not passes. The review index states no native compositor, product, or model acceptance; IP-03 C1–C8 and foundation gates remain open. Nothing reviewed here supports full-suite readiness, accepted production output, or a completed native cooperative authoring path.

No unresolved contradiction was found in the guide's current Construct/Manifold referents, Vibe/Boa distinction, Aether/Etalon boundary, source-app reuse direction, or cancellation qualification. The remaining UI unknowns are stated explicitly and must remain open until separately reviewed.
