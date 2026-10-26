# Empty molecule constructor and direct-predictor conformance

Staged followup to the reviewed modeling fixture repair. Three full candidates
are guarded against the current actual Core sources in `apply-manifest.json`.
No sibling source was changed and no Cargo rebuild was run by this worker.

`Molecule::new()` now implements its documented empty contract: empty formula,
atoms, bonds and coordinates. The legacy `mol_1` local record key and all public
field types remain unchanged. `MolecularProperties::new()` zero-initializes its
four fields as unmeasured placeholders, rather than claiming methane mass or an
electronic energy. This is constructor conformance, not chemical accuracy.
Other property types and their existing defaults are outside this bounded fix.

`PropertyPredictor::predict_from_molecule` now invokes its existing empty-atoms
validator before calculating descriptors or returning any model intercept.
Empty and formula-only input return `ValidationError`; the existing uninitialized
valid-water case still returns `NotImplemented`. No model coefficient, reference
value, unit contract or empty-input policy has been loosened.

All `Molecule::new` callers across Core/crates were audited; see
`callsite-audit.json`. The unsupported-element quantum and carbon property
fixtures explicitly build carbon with formula C, one atom and one coordinate.
The current positive MD fixture is already explicit Ar4; its negative fixture
now uses the genuinely empty constructor. Water explicitly contains only OHH,
formula H2O and exactly three matching coordinates. MCP atom-list branches
replace atoms/coordinates; its formula-only fallback now honestly fails existing
facade validation. The separate manually constructed SMILES placeholder remains
an unqualified pre-existing limitation and is not repaired or validated here.

The focused direct-rustc harness includes the **exact candidate production**
`types.rs` and `properties.rs`, with actual cached Core supplying only unchanged
supporting enum/error/metrics types and cached serde derives. Five tests passed:
constructor shape, unmeasured defaults, exact staged carbon helper structure,
registered-model water equations including unchanged density 0.962427, and
empty/formula-only direct prediction denial. This avoids pretending cached Core
contains the new constructor. Parent must run coherent actual-source facade,
quantum and MD checks after guarded application. Initial harness selection failed
before compilation because Core's target kind is rlib/cdylib rather than lib;
selection was corrected to accept exact Cargo rlib artifacts.

Ownership review: existing `properties.rs` (807 lines) receives one validation
call and focused regression/fixture updates; existing tests.rs (674) receives
only explicit fixture construction. No new lifecycle or expanded model schema is
introduced into those files. The constructor owner types.rs stays at 322 lines.
This explicitly bounded compatibility repair does not claim full chemistry,
model/master/package acceptance or completion of IP-03 C1-C8/production packets.
