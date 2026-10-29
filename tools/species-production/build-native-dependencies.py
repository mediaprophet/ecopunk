"""Coordinator-owned, isolated native dependency build for species review round 1.

Reuses the existing cooperative lease and disk guard. Does not qualify assets.
"""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
WORKSPACE = ROOT.parent / "qualiaDB"
TARGET = ROOT / "target/species-production-native"
EVIDENCE = ROOT / "docs/qa/implementation-2026-10-29/species-production-round1"
spec = importlib.util.spec_from_file_location(
    "serial_check", ROOT / "tools/poet-foundation/biosphere-shared/serial_check.py"
)
serial = importlib.util.module_from_spec(spec)
spec.loader.exec_module(serial)


def main():
    command = [
        "cargo", "build", "--locked", "--offline", "-p", "qualia-core-db",
        "--lib", "--no-default-features", "--features",
        "gpu-runtime,gpu-native-dx12,gpu-native-portable,wgsl-forge",
        "--target-dir", str(TARGET), "-j", "2",
    ]
    serial.require_start_space(ROOT)
    lease = serial.acquire(serial.LEASE, "species-review-round1/root", command)
    if lease is None:
        return 75
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    attempt = 1
    while (EVIDENCE / f"native-build-attempt-{attempt:02d}.json").exists():
        attempt += 1
    log_path = EVIDENCE / f"native-build-attempt-{attempt:02d}.log"
    record_path = EVIDENCE / f"native-build-attempt-{attempt:02d}.json"
    env = os.environ.copy()
    env.update(CARGO_BUILD_JOBS="2", CARGO_PROFILE_DEV_DEBUG="0",
               CARGO_PROFILE_DEV_INCREMENTAL="false", CARGO_INCREMENTAL="0")
    record = {"command": command, "cwd": str(WORKSPACE),
              "start_utc": serial.now_utc(), "target": str(TARGET),
              "log": str(log_path),
              "scope": "native dependency build only; no asset acceptance",
              "environment_overrides": {k: env[k] for k in (
                  "CARGO_BUILD_JOBS", "CARGO_PROFILE_DEV_DEBUG",
                  "CARGO_PROFILE_DEV_INCREMENTAL", "CARGO_INCREMENTAL")}}
    record_path.write_text(json.dumps(record, indent=2) + "\n")
    with log_path.open("w", encoding="utf-8") as log:
        child = subprocess.Popen(command, cwd=WORKSPACE, env=env,
                                 stdout=log, stderr=subprocess.STDOUT)
        lease["child_pid"] = child.pid
        serial.write_lease(serial.LEASE, lease)
        print(f"Owned Cargo PID {child.pid}; log: {log.name}", flush=True)
        code = serial.wait_with_disk_budget(child, ROOT)
    record.update(end_utc=serial.now_utc(), exit_code=code)
    record_path.write_text(json.dumps(record, indent=2) + "\n")
    if not serial.release(serial.LEASE, lease["lease_uuid"]):
        raise RuntimeError("Owned build exited but lease changed; lease retained")
    print(f"Native dependency build exit code: {code}", flush=True)
    return code


if __name__ == "__main__":
    sys.exit(main())
