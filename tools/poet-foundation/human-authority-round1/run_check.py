"""Record one coordinator-owned serial Cargo check and its source boundary."""
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from datetime import datetime, timezone

ROOT = Path("C:/github/ecopunk")
OUTPUT = ROOT / "docs/qa/implementation-2026-10-27/human-authority-round1"
REPOS = ("qualiaDB", "poet", "etalon", "rust-app-framework", "vibe-script")


def git(root, *args):
    result = subprocess.run(["git", "-C", str(root), *args], capture_output=True)
    if result.returncode:
        raise RuntimeError(result.stderr.decode("utf-8", errors="replace"))
    return result.stdout


def snapshot():
    result = {}
    for name in REPOS:
        root = Path("C:/github") / name
        status = git(root, "status", "--porcelain=v1", "-z", "-uall")
        rows, dirty, i = status.decode("utf-8").split("\0"), [], 0
        while i < len(rows) and rows[i]:
            row = rows[i]
            i += 1
            entry = {"status": row[:2], "path": row[3:]}
            if "R" in row[:2] or "C" in row[:2]:
                entry["previous_path"] = rows[i]
                i += 1
            path = root / entry["path"]
            entry["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None
            dirty.append(entry)
        locks = {}
        for rel in ("Cargo.toml", "Cargo.lock", ".cargo/config.toml"):
            path = root / rel
            if path.is_file():
                locks[rel] = hashlib.sha256(path.read_bytes()).hexdigest()
        result[name] = {"head": git(root, "rev-parse", "HEAD").decode().strip(),
                        "dirty": dirty, "manifests_and_config": locks}
    return result


def main():
    case, repo, *command = sys.argv[1:]
    if not re.fullmatch(r"[a-z0-9-]+", case) or repo not in REPOS:
        raise ValueError("invalid case or repository")
    OUTPUT.mkdir(parents=True, exist_ok=True)
    record_path, log_path = OUTPUT / (case + ".json"), OUTPUT / (case + ".log")
    if record_path.exists() or log_path.exists():
        raise FileExistsError("case evidence already exists; select a distinct attempt name")
    record = {"case": case, "cwd": "C:/github/" + repo, "command": command,
              "started_utc": datetime.now(timezone.utc).isoformat(), "before": snapshot(),
              "human_acceptance": False, "production_authorised": False,
              "environment_overrides": {k: os.environ[k] for k in
                  ("RUSTFLAGS", "RUSTUP_TOOLCHAIN", "CARGO_BUILD_JOBS", "CARGO_TARGET_DIR",
                   "POET_TEXTURE_EFFECT_GPU_EVIDENCE_DIR") if k in os.environ}}
    record_path.write_text(json.dumps(record, indent=2) + "\n", encoding="utf-8")
    lease = ROOT / "tools/poet-foundation/biosphere-shared/serial_check.py"
    with log_path.open("wb") as log:
        completed = subprocess.run([sys.executable, str(lease), "--owner", "human-authority-" + case,
                                    "--", *command], cwd=record["cwd"], stdout=log, stderr=subprocess.STDOUT)
    record.update({"exit_code": completed.returncode,
                   "finished_utc": datetime.now(timezone.utc).isoformat(), "after": snapshot()})
    log_bytes = log_path.read_bytes()
    record["log_sha256"] = hashlib.sha256(log_bytes).hexdigest()
    record["summaries"] = re.findall(r"test result: .*", log_bytes.decode("utf-8", errors="replace"))
    record["source_unchanged"] = record["before"] == record["after"]
    record["warning_count"] = len(re.findall(
        r"^warning(?:\[[^\]]+\])?:", log_bytes.decode("utf-8", errors="replace"), re.MULTILINE))
    record["passed_tests"] = sum(int(n) for summary in record["summaries"]
                                 for n in re.findall(r"\b(\d+) passed;", summary))
    # This is a QA recording guard, not the native product/model acceptance gate.
    # A successful Cargo process selecting zero tests cannot qualify a test case.
    record["qualification_accepted"] = (completed.returncode == 0 and record["source_unchanged"]
        and record["warning_count"] == 0 and ("test" not in command or record["passed_tests"] > 0))
    record_path.write_text(json.dumps(record, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({k: record[k] for k in ("case", "exit_code", "source_unchanged",
        "warning_count", "qualification_accepted", "summaries")}), flush=True)
    return completed.returncode if completed.returncode else (0 if record["qualification_accepted"] else 2)


if __name__ == "__main__":
    sys.exit(main())
