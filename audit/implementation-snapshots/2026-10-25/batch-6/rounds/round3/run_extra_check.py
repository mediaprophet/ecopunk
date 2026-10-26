"""Additional serial consumer checks; no GUI, credentials or asset generation."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time
import tempfile
import re

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[5]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def snapshot():
    files = set()
    for base in [Path("C:/github/qualiaDB/crates/qualia-core-db"),
                 Path("C:/github/qualiaDB/crates/qualia-geospatial"),
                 Path("C:/github/poet/native"), Path("C:/github/poet/crates"),
                 ROOT / "tools/format-sdk-wasm-check"]:
        for ext in ["*.rs", "*.wgsl", "*.toml", "Cargo.lock"]:
            files.update(base.rglob(ext))
    for base in [Path("C:/github/poet"), Path("C:/github/qualiaDB")]:
        for name in ["Cargo.toml", "Cargo.lock"]:
            if (base / name).exists():
                files.add(base / name)
    return {str(p): sha(p) for p in sorted(files)}


mode = sys.argv[1]
assert mode in ["sdk-wasm", "poet-tests", "poet-epoch", "poet-build"]
if mode == "sdk-wasm":
    command = ["cargo", "check", "--manifest-path", str(ROOT / "tools/format-sdk-wasm-check/Cargo.toml"),
               "--locked", "--offline", "--target", "wasm32-unknown-unknown"]
else:
    command = ["cargo", "test" if mode in ["poet-tests", "poet-epoch"] else "build", "--manifest-path",
               "C:/github/poet/Cargo.toml", "-p", "poet-cli", "--features", "native-graphical",
               "--locked", "--offline"]
    command += ["--lib"] if mode in ["poet-tests", "poet-epoch"] else ["--bin", "poet"]
command += ["--target-dir", str(ROOT / "target")]
if mode == "poet-tests":
    command += ["inspection", "--", "--test-threads=1", "--nocapture"]
elif mode == "poet-epoch":
    command += ["actual_source_successor_and_retired_rebuild_epoch_publish_into_the_requesting_view", "--", "--test-threads=1", "--nocapture"]
before = snapshot()
source_path = HERE / (mode + "-source-before.json")
source_path.write_text(json.dumps(before, indent=2), encoding="utf-8")
started = datetime.datetime.now(datetime.timezone.utc).isoformat()
clock = time.monotonic()
environment = os.environ.copy()
environment["CARGO_BUILD_JOBS"] = "2"
environment["CARGO_PROFILE_DEV_DEBUG"] = "0"
environment["CARGO_PROFILE_TEST_DEBUG"] = "0"
environment["CARGO_PROFILE_DEV_INCREMENTAL"] = "false"
environment["CARGO_PROFILE_TEST_INCREMENTAL"] = "false"
scratch = tempfile.TemporaryDirectory(prefix="poet-sdk-check-", dir=ROOT / "target")
scratch_path = Path(scratch.name).resolve()
assert scratch_path.is_relative_to((ROOT / "target").resolve())
environment["TEMP"] = str(scratch_path)
environment["TMP"] = str(scratch_path)
log_path = HERE / (mode + ".log")
with log_path.open("w", encoding="utf-8") as log:
    process = subprocess.Popen(command, cwd=ROOT, env=environment, stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT, text=True, encoding="utf-8", errors="replace")
    for line in process.stdout:
        log.write(line)
        log.flush()
        print(line, end="", flush=True)
    exit_code = process.wait()
after = snapshot()
changes = [p for p in sorted(set(before) | set(after)) if before.get(p) != after.get(p)]
test_selection_ok = True
if mode in ["poet-tests", "poet-epoch"]:
    counts = re.findall(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored", log_path.read_text())
    minimum = 6 if mode == "poet-tests" else 1
    test_selection_ok = bool(counts) and sum(int(row[0]) for row in counts) >= minimum
receipt = dict(command=command, working_directory=str(ROOT), exit_code=exit_code,
    build_environment={k: environment[k] for k in ["CARGO_BUILD_JOBS", "CARGO_PROFILE_DEV_DEBUG",
        "CARGO_PROFILE_TEST_DEBUG", "CARGO_PROFILE_DEV_INCREMENTAL", "CARGO_PROFILE_TEST_INCREMENTAL"]},
    started_utc=started, finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    elapsed_seconds=round(time.monotonic()-clock, 3), source_snapshot=str(source_path),
    source_snapshot_sha256=sha(source_path), recorded_local_source_files=len(before),
    source_changes=changes, source_unchanged=not changes, log=str(log_path), log_sha256=sha(log_path),
    test_selection_ok=test_selection_ok, temporary_scratch=str(scratch_path),
    boundary="Recorded local Core/geospatial/Poet source and manifests/locks; not every transitive dependency source. "
             "Compilation is not runtime WASM, GUI, human review or readiness acceptance.")
(HERE / (mode + "-result.json")).write_text(json.dumps(receipt, indent=2), encoding="utf-8")
print(json.dumps(receipt), flush=True)
# RAII cleanup is restricted to the verified uniquely owned workspace child.
assert scratch_path.is_relative_to((ROOT / "target").resolve())
scratch.cleanup()
sys.exit(exit_code if exit_code else int(bool(changes) or not test_selection_ok))
