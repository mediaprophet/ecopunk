# Wave 2: native Poet texture/effect editing

2026-10-26. Bounded implementation under IP-03 / PA-03, PA-06, PA-11, PA-12, PC-06 and TOOL-06. The selected software path now has [native GPU and actual mounted-component qualification](gpu-qualification/README.md): **271 library tests passed, seven explicit native GPU tests passed, and the current native executable rebuilt successfully**. Sixteen current canonical files are pinned by that follow-up receipt. Asset production **HOLD**; this is not a passed whole-product/model gate. The original CPU implementation/evidence is preserved below.

## Selected increment

Extend existing imported Paint/detail recipes with up to eight ordered post-composite effects: Grayscale, Invert and Sepia. Core `hypermedia_authoring::image::apply_filter` owns the actual operations; existing source-over, canonical PNG encode/decode and HMC resource owners remain in use. Native controls retain paint layers/strokes and original image bytes; effects can be added, changed, toggled, reordered and removed. Version 1 empty/absent effect recipes retain their serialized expression; nonempty effects require explicit Paint version 2. Procedural version 1 is unchanged.

Admitted MCP `poet_texture_effects_edit` edits only effects/version over the existing Paint recipe through `AuthoringEdit::TextureDetail`. `poet_texture_effects_inspect` reports current recipe, base and resolved output digests. Native selected-surface inspection reports the same actual output digest. These are identifiers and lineage witnesses, not permission or human approval.

The supported working profile remains opaque square imported sRGB base colour, side 8–1024, encoded sample processing and existing bounded layers/strokes. This is not a complete compositor, RAW development, linear-light/ICC grading, continuous animation or a biological/medical quality claim.

## Source, dispatch and review

- [Frozen contract and ownership](../../../../tools/poet-foundation/texture-effects/CONTRACT.md), [four-wave plan](../../../planning/poet-authoring-swarm-phases.md) and [current register](../../../planning/biosphere-work-packets.json).
- [Bounded reuse trace](../../../../tools/poet-foundation/texture-effects/work/report.md): existing Qualia API/body pins, Photo/Effect workflow references and limits. No new image mathematics or copied external app host.
- [Domain handoff](../../../../tools/poet-foundation/texture-effects/work/domain/handoff.json), [controls handoff](../../../../tools/poet-foundation/texture-effects/work/controls/handoff.json) and [integration tests](../../../../tools/poet-foundation/texture-effects/work/integration-tests/handoff.json): disjoint staged ownership and actual source hashes.
- [Independent root-glue review](../../../../tools/poet-foundation/texture-effects/work/domain/integration-review.md): found valid-shaped failure context-consumption mismatch; root corrected the staged adapter to consume after parameter preflight even when expected-context admission fails. Existing jobs fixture gained the new optional output digest field. The review is a source check, not executed qualification.

Root reviewed the domain admission/render/tests, reactive draft helpers/controls, signed integration/recovery cases and new adapters before assembling fourteen preimage-pinned source candidates. Existing version-1 Rust fixtures receive only an empty effect field. Runtime checks subsequently exposed Serde's extra-field acceptance on internally tagged unit variants; a strict empty-struct decoding adapter now preserves the public unit API and serialized schema while rejecting those fields. The refusal tests were retained; a foreign-context retry assertion was also added. No manifests, lockfiles, upstream engine source, external shell, JS/Boa, new state store or production assets are changed by this install.

## Explicit future functionality

The [deferred inventory](../../../../tools/poet-foundation/texture-effects/work/deferred-functionality.md) and [machine-readable cases](../../../../tools/poet-foundation/texture-effects/work/deferred-functionality.json) track all confirmed stubs/ignored fields in this bounded Core image audit: four no-op filters, nine ignored non-Normal blend modes, layer masks and placement offsets. Each has its owning symbol, remedy and acceptance requirements. They remain open work, not silently successful operations. Brightness, Contrast and EdgeDetect are implemented source branches but are not qualified native Wave 2 controls. This is not a repository-wide no-op audit.

## Evidence and remaining gates

- [Installed source receipt](applied-source.json), [initial install](applied-source.initial.json) and [final manifest](../../../../tools/poet-foundation/texture-effects/work/integration/install-manifest.json): exact preimages and current canonical hashes; byte backups retained.
- [First full check](check-attempt-1.log): exit 101, 266 passed, two failed, nine ignored. Both failures caught the strict-decoder defect; it was corrected in production code.
- [Initial full CPU qualification](check-attempt-2.log): exit 0, 268 passed, zero failed, nine ignored. All 15 initial domain/control/integration cases passed; previous material-motion/source/history regressions also passed.
- [Initial native executable build](native-build.log): exit 0, locked/offline native-graphical build. [Initial verification](verification.json) pins its fourteen-file source set, then-built executable and diagnostic outputs. [Current GPU/component verification](gpu-qualification/verification.json) supersedes this source/binary snapshot. No native window was launched.
- [Retained diagnostic outputs](diagnostic/README.md): actual original, filtered and fresh-process reopened PNG resources and a digest witness. These are synthetic engineering calibration only.
- [Manual verification route](try-it.md): implemented labels and project launch command; not evidence that a person has used or accepted the current window.

Executed cases cover known filter pixels/order/disable, strict version/parameter/budget refusal, retained invalid control text, signed native/MCP source edits, history, original/derived HMC closure and metadata, and fresh-process reopen/rebind. Raw test logs include each of the nine ignored GPU cases; these were not converted into passes.

Actual native-window layout/accessibility and human usability, production-scene visual quality, separate human/reviewer and biological/domain acceptance, simultaneous distinct-principal cooperation and whole IP-03 C1–C8 remain unassessed. The selected 64px native DX12 effect-rendering/recovery path and mounted-component event cases passed as described in the follow-up. Film/game/cartoon species packages are not produced or declared complete. No commits, publication or payments.
