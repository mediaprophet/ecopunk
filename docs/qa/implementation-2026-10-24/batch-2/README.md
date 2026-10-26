# Canonical language and native authoring implementation

Started 2026-10-24; continued 2026-10-25 (Australia/Sydney). This batch advances the native source-authoring prerequisites for
exceptional Ecopunk masters. It does not qualify a complete model, HMC embodiment,
VibeScript 0.2 release, native graphical workflow or IP-03 C1–C8 acceptance.

## Applied source changes

| Job | Canonical change | Evidence and scope |
| --- | --- | --- |
| RUN-05A | Vibe cell parser consumes the entire input and preserves trailing lexical errors. | [Guarded source map](parser/apply-manifest.json), [original failure](parser/baseline.log), [five applied regression passes](parser/applied.log). |
| RUN-05B | Canonical transitive effect checking, owned evaluation context and persistent checked sessions; native Poet REPL uses that session. | [Frozen 24-target package](runtime/frozen-package.json), [independent review](runtime-review.md). Applied after all original/candidate guards passed. Canonical integration results are recorded separately from private verification. |
| RUN-06B | Existing Studio source store restores actual source on undo/redo and rejects stale revision commands atomically. | [Four-target source map](native-3d/apply-manifest.json), [test receipt](native-3d/verification.json), [scope and outstanding seams](native-3d/README.md). Applied source hashes are recorded separately; the staging map remains historical. |

RUN-05B retains the language and host version at 0.1. The checker follows
transitive calls, aliases and callbacks, fails closed on unresolved calls and
checks budget/material expressions. Existing unsafe host examples were corrected
or retained as negative cases rather than exempted from checking. The session
retains typed/mutable bindings, functions and import context between submissions;
only the newly submitted statements execute. Grants belong to the host and cannot
be acquired by source declarations. Native Poet defaults to no grants and accepts
explicit repeatable local `--grant` arguments.

Checked candidates are bound to their session and revision. Failed submissions
do not promote local state. External host effects cannot be rolled back by that
rule: runtime failures retain diagnostics and require host reconciliation where
applicable. Definitions are additive; replacement/live reload, durable shared
checkpoints, project membership and async jobs remain separate work.

Source ingress and expression/call depth limits reject excessive nesting before
or during execution. Private verification includes 1,278 test passes and five
isolated default-stack boundary probes, including combined calls and expression
depth. These are bounded runtime checks, not an allocator-hard-limit or process
sandbox claim. A pre-existing sub-microsecond timing assertion failed in the
original canonical baseline; its failure and explicitly qualified functional
baseline are retained in `vibe-before-checker*.log`.

Actual applied-source verification is complete in the
[target receipt](runtime/actual-verification.json): the functional Vibe suite
passed **1,232** tests with only that timing case filtered; session/Poet embedding
passed **45**, actual Poet CLI tests passed **15**, and five isolated boundary
processes passed. The first unfiltered actual Vibe run retained **972 passes and
one timing failure** (1,100 ns against <1,000 ns). Direct default/granted native
CLI transcripts and the installed `wasm32-unknown-unknown` library check passed.
These counts describe distinct suites with overlapping coverage and are not
summed into a unique-case total. [Applied source pins](applied-sources.json)
record all 30 initial canonical/Poet/journal changes.

RUN-06B preserves the existing store and public artifact shape. Source snapshots
live beside the artifact in its existing locked entry. Native host callers can
select larger journal budgets for high-grade editable sources; arbitrary script
properties do not constitute authoritative rights or contribution records.
Focused journal tests passed 10/10. The wider mounted suite passed 16 with one
pre-existing inline-CML import failure, reproduced against original source.
These tests use real cached core/Vibe dependencies; current whole-core consumer
compilation is a separate check. The actual native Ecopunk consumer subsequently
passed against these applied sources in **24m 21s**; the controlled Aether native
CPU UI profile passed in **13m 52s** with its lockfile unchanged. See the
[consumer receipt](consumer-checks.json). These checks precede the separate
stamped-store addendum and do not constitute full core tests or graphical use.
Source-driven loft/material regeneration is a
test fixture, not an installed production editor or accepted model.

## Next authoring requirement

The reviewed RUN-07A store addendum has subsequently been applied. It adds opaque
process-local incarnation/revision stamps, atomic create-if-absent and fallible
detached-candidate edits. Stamped edit/undo/redo paths reject identical visible
replacement that erased private history; numeric legacy paths remain available
without that guarantee. Five new stamped and ten prior journal tests passed
against real cached dependencies. The [store review](native-3d-review.md) records
the defect, correction and exact guards. This intentionally supersedes two of
the initial applied-source hashes; frozen staging and initial check receipts
remain historical records. Current native integration qualification is separate.

RUN-07A connects an additive native Poet module to Aether's actual native launch
path and the real Qualia/Etalon geometry/material preview. Existing Poet
Tool-Chest/Manifold concepts are the UI recovery reference. Chora remains separate.
Native dependency resolution, compilation and a real window/render must be
verified before describing that module as usable.

The [15-target package](native-poet/apply-manifest.json) has now been applied to
the actual Poet workspace after [independent review](native-poet-review.md).
It adds optional `native-graphical` / `poet studio` integration; the selected
Aether UI uses native CPU rendering and the geometry preview uses actual Etalon
30 through Qualia. The [CLI-specific dependency record](native-poet/native-dependency-closure.json)
records 604 normal/build packages and the exact local/Git sources. Broad workspace
metadata is explicitly an over-approximation and is not used to claim the
CLI's active renderer. [Lock changes](native-poet/lock-delta.json) disclose the
browser-family alignment and other resolver version-set shifts. Browser source
execution remains a separate qualification.

The editor shares the canonical source store and interpreter. Explicit commands
run on one bounded native worker; stamped source, session epoch and completion
checks withhold late or misbound previews. Geometry invocation rechecks actual
operation grants at the host. Undo restores source; an explicit, budgeted
read-only geometry-profile rebuild reconstructs accepted execution state and
retires old completion epochs. It does not replay general external effects or
implement durable checkpoint recovery. The initial output is an editable geometry
diagnostic with caller-specified factor material and SDR preview, not material
source authoring or an accepted organism/assembly. The HMC helper is unmounted
pending colour/texture/integrity qualification.

The actual native-feature test build is in progress. Source-byte verification
currently covers [46 files across 51 reviewed transitions](integrated-sources.json);
this and [applied target verification](native-poet/applied-verification.json) are
separate from coherent tests, regular-binary linkage and window/render evidence.

The production path must replace placeholder Studio/CAD/Scene receipts with
actual operators, bind output to the accepted source revision, and preserve
editable sources through the real binary compiler/HMC writer. Existing
`StudioArtifact` JSON exporters and `Scene.render` planning records do not prove
those operations. After this, demonstrate contributor/reviewer source editing,
conflict recovery, checkpoint/reopen and source-to-HMC-to-game-demo use against
the complete model dossiers. All 120 broad production packets remain open;
payment rails remain later support work.
