"""Build the existing Ecopunk WASM application into an isolated review package."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
EVIDENCE = ROOT / "docs/qa/implementation-2026-10-29/species-production-round1"
TARGET = ROOT / "target/demo-wasm"
PACKAGE = ROOT / "target/species-review-pkg"
spec = importlib.util.spec_from_file_location(
    "serial_check", ROOT / "tools/poet-foundation/biosphere-shared/serial_check.py"
)
serial = importlib.util.module_from_spec(spec)
spec.loader.exec_module(serial)


def main():
    command = ["wasm-pack", "build", str(ROOT / "crates/ecopunk-wasm"),
               "--target", "web", "--out-dir", str(PACKAGE),
               "--release", "--no-opt", "--", "--locked", "--offline"]
    serial.require_start_space(ROOT)
    lease = serial.acquire(serial.LEASE, "species-review-round1/root-wasm", command)
    if lease is None:
        return 75
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    attempt = 1
    while (EVIDENCE / f"wasm-build-{attempt:02d}.json").exists():
        attempt += 1
    log_path = EVIDENCE / f"wasm-build-{attempt:02d}.log"
    record_path = EVIDENCE / f"wasm-build-{attempt:02d}.json"
    env = os.environ.copy()
    settings = dict(CARGO_BUILD_JOBS="2", CARGO_PROFILE_DEV_DEBUG="0",
                    CARGO_PROFILE_RELEASE_DEBUG="0", CARGO_INCREMENTAL="0",
                    CARGO_TARGET_DIR=str(TARGET))
    env.update(settings)
    record = dict(command=command, cwd=str(ROOT), start_utc=serial.now_utc(),
                  target=str(TARGET), package=str(PACKAGE), log=str(log_path),
                  environment_overrides=settings, scope="WASM review build; acceptance pending")
    record_path.write_text(json.dumps(record, indent=2) + "\n")
    with log_path.open("w", encoding="utf-8") as log:
        child = subprocess.Popen(command, cwd=ROOT, env=env,
                                 stdout=log, stderr=subprocess.STDOUT)
        lease["child_pid"] = child.pid
        serial.write_lease(serial.LEASE, lease)
        print(f"Owned WASM build PID {child.pid}; log: {log_path}", flush=True)
        code = serial.wait_with_disk_budget(child, ROOT)
    record.update(end_utc=serial.now_utc(), exit_code=code)
    record_path.write_text(json.dumps(record, indent=2) + "\n")
    if not serial.release(serial.LEASE, lease["lease_uuid"]):
        raise RuntimeError("Owned build exited but lease changed; retained")
    print(f"WASM build exit code: {code}", flush=True)
    return code


if __name__ == "__main__":
    sys.exit(main())
