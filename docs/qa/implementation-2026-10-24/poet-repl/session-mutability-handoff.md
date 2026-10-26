# RUN-03 follow-up: persistent mutable bindings remain broken

**2026-10-24 — upstream defect evidence, not a session acceptance pass.** This
follow-up leaves the reviewed/applied REPL patch, its 10-test result and its
pre-application source guards unchanged. The new probe is separate from those
regressions. It also does not implement a Poet-local checker or bypass either
mutability or effects checks.

The supervisor's actual native CLI transcript exposed a binding-context defect:
`let mut scale: f64 = 1.0;` succeeds, but the next submission `scale = 2.0;`
returns E701 as though `scale` were immutable. The isolated follow-up probe
reproduces that failure while confirming `env.mutables` still contains `scale`.
The runtime has retained the metadata; program checking does not use it.

## Separate, reproducible observations

[harness/tests/session_mutability_findings.rs](harness/tests/session_mutability_findings.rs)
is compiled unchanged against the recorded original and the exact applied
candidate. Its tests assert the currently observed defect so that its evidence
is reproducible; **passing these assertions does not pass persistent mutability**.

| Input sequence | Required session outcome | Observed outcome |
|---|---|---|
| `let radius: f64 = 2.0;` then `radius * radius` | 4.0 | 4.0; ordinary value reads persist. |
| `let mut scale: f64 = 1.0;` then `scale = 2.0;` | Successful mutable assignment; `scale` becomes 2.0. | Both versions reject the update and leave value 1.0 despite retained mutable metadata. Current returns E701; original fallback masks that checker failure as E001, `unexpected tokens after cell expression`. |
| Then `radius * radius * scale` | 8.0 after the intended assignment. | 4.0, confirming the failed assignment left the old value. |
| One submission `let mut scale: f64 = 1.0; scale = 2.0; scale;` | 2.0 | 2.0; parser, same-program checking and runtime assignment work for this case. |

Commands run from `C:/github/ecopunk`:

```powershell
cargo test --manifest-path docs/qa/implementation-2026-10-24/poet-repl/harness/Cargo.toml --offline --locked --test session_mutability_findings -- --nocapture
cargo test --manifest-path docs/qa/implementation-2026-10-24/poet-repl/harness/Cargo.toml --offline --locked --features baseline --test session_mutability_findings -- --nocapture
```

The initial current-candidate run exited 0 with **2 defect/control observations
reproduced**; see [session-mutability-current.log](session-mutability-current.log).
The first baseline run exposed an incorrect probe expectation: it expected E701
but observed E001 after the original shell's fallback. Its exit 101 and assertion
mismatch are preserved in
[session-mutability-baseline.log](session-mutability-baseline.log).
The refined probe explicitly distinguishes those shell diagnostics and checks
that the shared canonical program checker returns E701. Final results are
recorded separately in
[session-mutability-current-confirmed.log](session-mutability-current-confirmed.log)
and [session-mutability-baseline-confirmed.log](session-mutability-baseline-confirmed.log).
Both final runs exited 0 with 2/2 observations reproduced each. This confirms the
assignment rejection is pre-existing: current reports E701, baseline reports the
fallback E001, and both keep the old value. `rustfmt --check` on the final probe
also exited 0.

Source checks after the supervisor's authorized application confirmed:

- Applied `C:/github/poet/native/src/repl.rs` and staged candidate SHA256:
  `78F1B4C3640B2630B5760B03706078540B4D093010E26691CB44D3AE5C1AD2A3`.
- Preserved original SHA256:
  `2E95F923A5E4E89F0DE1AE47960A5EC136A6F396CF7A7CEF0CC9798284B6BD19`.
- Initial probe SHA256 before the explicit baseline diagnostic distinction:
  `737746CC29C75C60D79621F73C0AB39437FBAC956CDE701F12C978421997864B`.
- Final probe SHA256 with the explicit baseline/current diagnostic distinction:
  `4DCC9FCDEAC9A5E4C8C5FEE802AD511C570C8F9EC9D8E457D0CA2DD128113668`.

These isolated tests are additional diagnostics. They do not replace the
supervisor's actual CLI transcript and do not infer success from the REPL
process's normal exit code after printing an error.

## Canonical API requirement for RUN-05

`vibe/src/check.rs::check_program_all` (lines 35–43) creates new type and
mutability collections for each invocation: `let mut env = HashMap::new()` and
`let mut mutables = HashSet::new()`. Its `Stmt::Let` checker records mutability
only while checking that same Program; its `Stmt::Assign` branch (lines 427–445)
rejects a target absent from the fresh set. Public `check_program` wraps this
function, and `check_program_with_vocab` wraps `check_program`; neither accepts
an existing session checking context.

The original REPL reaches that checker through `load_program`, then discards its
E701 and retries cell parsing, returning E001 instead. The applied
candidate reaches it directly after parse-mode selection. Both paths therefore
have the same cross-submission loss of checker context. `Env` retains runtime
values and `mutables`; `eval/stmt.rs` inserts the mutable name on declaration
(lines 31–47) and consults it again during runtime assignment (lines 73–92).
This evidence identifies a missing canonical checked-session interface, not
permission to skip checking or weaken immutable bindings.

RUN-05 needs a public canonical checking/session API that accepts prior accepted
binding declarations and their types, mutability, scope and source/revision
provenance, together with resolved function/import/effect/capability context.
It must return an admitted candidate context without changing the accepted
session on parse/type/effect/authority failure. Runtime values and checker
metadata must advance together under that accepted context. Merely seeding a
Poet-local HashSet from `env.mutables` would duplicate policy and omit the other
required declaration/type/effect/source semantics; it is not implemented here.

Required acceptance extends VIBE-04 / VBC-008/019/020/021 and ECO-034/044:

- A mutable top-level session binding can be assigned from later submissions,
  and subsequent calculations use the new value.
- An immutable binding still rejects assignment, preserving its accepted value.
- Function parameters/locals, shadowing and block scopes follow pinned 0.1
  decisions and do not leak into the session's mutable-binding permissions.
- Invalid declarations/replacements and failed activation retain the accepted
  checking context and values; no checking/evaluation fallback replays effects.
- Persistent functions/imports use the canonical resolved effect/capability
  checking required by the earlier transitive-effect probe.

Persistent mutable bindings and functions remain open; neither a positive
read-only expression transcript nor the narrow double-dispatch fix closes the
native usable/cooperative authoring gates.
