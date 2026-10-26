# Native format SDK conformance runner

This runner compiles the actual QualiaDB Core and geospatial libraries. It points
directly to their canonical public integration tests and adds consumer tests for
the existing M64 fixture and HMC packaging. It contains no replacement codec,
renderer, production asset generator or authority implementation.

The initial lockfile is seeded from the qualified Poet workspace, then reconciled
offline. Use `cargo test --manifest-path tools/format-sdk-conformance/Cargo.toml
--locked --offline --target-dir target` from the Ecopunk root.

Results qualify the declared native serialization/SDK cases. They do not establish
geographic truth, scientific validity, cryptographic authenticity, native Poet
usability, WASM parity, model completion or human acceptance. The stored terrain
file remains unchanged; tests build serialization vectors only in memory.
