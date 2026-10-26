# POET-TEXTURE-W2 domain handoff

Staged complete files only under this directory, matching canonical paths in `handoff.json`. Canonical files and existing `tests.rs` were not edited. The manifest pins the three exact canonical preimages and four staged outputs. The new `effect_tests.rs` has no prior canonical file.

## Existing operation and exact change

Read canonical Poet `native/src/graphical/surfaces/detail/{recipe.rs,render.rs,mod.rs,tests.rs}` and current Qualia Core `hypermedia_authoring/image.rs` ImageLayer API and apply_filter body. Read frozen CONTRACT, existing work/report.md, AGENTS, architecture/development-mode opening requirements. Core source-over/composite/encoder owner and prior trace remain as documented in work/report.md; no replacement math or renderer was added. Current Core filter body pin is recorded below.

Added strict typed effects and public validate_effects helper (root-requested API addition). Paint v1 requires absent/empty effects; Paint v2 permits up to eight. Sepia is finite 0..1 even disabled; Grayscale/Invert have no intensity field. Default/empty field omission preserves legacy source-expression serialization. Procedural schema and behavior unchanged.

Paint rendering composites existing original + authored layers once, copies result into one temporary Core ImageLayer, then applies enabled actual Core filters in list order. The temporary is not attached to the source document; Core PNG/digest logic is unchanged. No filter mathematics was copied. Empty effects take the exact legacy flatten path. Every authored step including disabled is charged a pixel pass, plus two initialization/copy passes; one additional RGBA layer is charged in working memory. Recipe validation/budget happens before base/resource work or allocation.

## Authored tests, not execution evidence

Six test functions all include texture_effect. They cover independent explicit RGBA vectors for the three filters (Sepia 0, 0.5, 1), noncommuting order, disable/remove, recipe/base immutability, post-paint compositing, existing layers retained, canonical PNG decode/digest repeatability, pinned legacy expression, v1 empty acceptance/v1 effects refusal/unknown versions, actual Vibe source roundtrip including disabled state, strict unknown kinds/fields/missing fields, nonfinite/out-of-range disabled parameters, max8, size, disabled work charge and exact memory/output/work bounds. Budget refusal with absent base confirms admission precedes base/render work. They do not claim GPU, GUI or human acceptance.

No Cargo, builds, tests, formatter, GUI, provider calls or asset production ran. Read-only `git diff --no-index` returns exit 1 for the expected staged differences. Root must integrate and verify serially. Root owns legacy struct-literal mechanical additions; existing domain tests.rs remains untouched here.

## Limits

Existing opaque imported sRGB profile stays in force; alpha checks verify opaque alpha preservation only. Encoded-sample effects do not establish ICC/linear-light grading. HMC dependency closure remains existing resolution owner plus integration tests. No models/packages completed and no production assets made. No unresolved schema ambiguity was found.

Core inspected image.rs SHA-256: 6D492B8DB1E1A78E2F43BD95BC0A9FD2B7DA9DCC054FCA7EAEAE8E789F0AAB49 (ImageLayer fields/new, apply_filter lines 224-299).

## Root serial attempt 1 / strict Serde repair

Root reported actual exit101 (266 passed, 2 failed, 9 ignored). Both failures exposed unknown-field acceptance on internally tagged unit variants. See serde-repair.md and updated handoff.json for the bounded recipe.rs-only repair; initial handoff saved as handoff-before-serde-repair.json. Earlier unrun wording describes the initial worker handoff, not root's subsequent execution. The repair candidate remains unrun.
