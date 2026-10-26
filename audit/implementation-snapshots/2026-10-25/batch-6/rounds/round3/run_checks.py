"""Serial actual-source checks with command/log and local source-drift receipts."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[5]
QUALIA = Path("C:/github/qualiaDB")


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def snapshot():
    files = set()
    for base in [QUALIA / "crates/qualia-core-db", QUALIA / "crates/qualia-geospatial",
                 ROOT / "tools/format-sdk-conformance", Path("C:/github/poet/wasm")]:
        for extension in ["*.rs", "*.wgsl", "*.toml"]:
            files.update(base.rglob(extension))
    for base in [QUALIA, ROOT / "tools/format-sdk-conformance", Path("C:/github/poet")]:
        for name in ["Cargo.toml", "Cargo.lock"]:
            path = base / name
            if path.is_file():
                files.add(path)
    return {str(path): sha(path) for path in sorted(files)}


mode = sys.argv[1]
assert mode in ["native", "wasm"]
if mode == "native":
    command = ["cargo", "test", "--manifest-path", str(ROOT / "tools/format-sdk-conformance/Cargo.toml"),
               "--locked", "--offline", "--target-dir", str(ROOT / "target"), "--tests"]
else:
    command = ["cargo", "check", "--offline", "--locked", "--manifest-path",
               "C:/github/poet/Cargo.toml", "-p", "poet", "--lib", "--target",
               "wasm32-unknown-unknown", "--target-dir", str(ROOT / "target")]
before = snapshot()
(HERE / (mode + "-source-before.json")).write_text(json.dumps(before, indent=2), encoding="utf-8")
started = datetime.datetime.now(datetime.timezone.utc).isoformat()
clock = time.monotonic()
environment = os.environ.copy()
environment["CARGO_BUILD_JOBS"] = "2"
log_path = HERE / (mode + "-conformance.log")
with log_path.open("w", encoding="utf-8") as log:
    process = subprocess.Popen(command, cwd=ROOT, env=environment, stdout=subprocess.PIPE,
                               stderr=subprocess.STDOUT, text=True, encoding="utf-8", errors="replace")
    for line in process.stdout:
        log.write(line)
        log.flush()
        print(line, end="", flush=True)
    exit_code = process.wait()
after = snapshot()
changes = [path for path in sorted(set(before) | set(after)) if before.get(path) != after.get(path)]
receipt = dict(command=command, working_directory=str(ROOT), exit_code=exit_code,
    started_utc=started, finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    elapsed_seconds=round(time.monotonic()-clock, 3),
    source_snapshot=str(HERE / (mode + "-source-before.json")),
    source_snapshot_sha256=sha(HERE / (mode + "-source-before.json")),
    recorded_local_source_files=len(before), source_changes=changes, source_unchanged=not changes,
    log=str(log_path), log_sha256=sha(log_path),
    boundary="Local Rust/WGSL/manifests/locks source set only; dependency caches are selected by recorded locks. "
             "Native test scope and WASM compilation are distinct; no runtime WASM/GUI/acceptance claim.")
(HERE / (mode + "-result.json")).write_text(json.dumps(receipt, indent=2), encoding="utf-8")
print(json.dumps(receipt), flush=True)
sys.exit(exit_code if exit_code else int(bool(changes)))
