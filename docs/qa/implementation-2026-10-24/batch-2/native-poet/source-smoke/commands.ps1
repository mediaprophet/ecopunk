# Historical exact executed commands; not an automatic rerun request.
$env:CARGO_PROFILE_DEV_DEBUG='0'
$env:CARGO_PROFILE_DEV_INCREMENTAL='false'
$env:PYTHON3='C:/Users/tim_h/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe'
$env:PYTHONDONTWRITEBYTECODE='1'
cargo build --offline --locked --manifest-path C:/github/ecopunk/docs/qa/implementation-2026-10-24/batch-2/native-poet/source-smoke/Cargo.toml --features native-graphical --target-dir C:/github/ecopunk/target
C:/github/ecopunk/target/debug/poet-source-smoke.exe
