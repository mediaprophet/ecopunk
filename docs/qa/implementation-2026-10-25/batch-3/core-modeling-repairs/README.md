# Core modeling test contract repairs

This folder contains three full staged source candidates for the scoped Core modeling test repairs. The default `Material::new()` fixture assigns `youngs_modulus = 200000` and `yield_strength = 250`, which is not a consistent SI steel fixture when dimensions and force are labeled in metres and newtons. New mechanical and vibration cases now use a separate explicit SI fixture: 0.1 m × 0.1 m cross-section, 2 m length, 200 GPa Young's modulus, 250 MPa yield strength, 7850 kg/m³ density, and loads in newtons. Existing tests using the prior default fixture remain unchanged.

The chemical assertions pin only the implemented registered group-contribution equations and fail-closed initialization/unknown-model behavior. They do not claim empirical chemical accuracy.

`apply-manifest.json` contains guarded target hashes. The candidates remain staged and have not been copied into the sibling QualiaDB repository. No Cargo build or test execution was performed; only `rustfmt` was used on the staged candidates.
