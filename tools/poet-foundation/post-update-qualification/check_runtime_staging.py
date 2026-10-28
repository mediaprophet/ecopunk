"""Run the build-script staging regressions without another Cargo build."""
import hashlib
import json
from pathlib import Path
import subprocess
from datetime import datetime, timezone

from run_check import OUTPUT, ROOT


def main():
    if (ROOT / "target/coordination/native-check.lock").exists():
        raise RuntimeError("Cargo lease held")
    source = Path("C:/github/qualiaDB/crates/qualia-core-db/build.rs")
    sha = lambda: hashlib.sha256(source.read_bytes()).hexdigest()
    executable = ROOT / "target/qualification/core-runtime-staging-tests.exe"
    executable.parent.mkdir(parents=True, exist_ok=True)
    log_path, record_path = OUTPUT / "runtime-staging.log", OUTPUT / "runtime-staging.json"
    if log_path.exists() or record_path.exists():
        raise FileExistsError("retain the earlier run")
    record = {"source": str(source), "before_sha256": sha(),
        "started_utc": datetime.now(timezone.utc).isoformat(),
        "compile_command": ["rustc", "--edition=2021", "--test", "-D", "warnings", str(source), "-o", str(executable)],
        "human_acceptance": False}
    with log_path.open("wb") as log:
        result = subprocess.run(record["compile_command"], stdout=log, stderr=subprocess.STDOUT)
        record["compile_exit_code"] = result.returncode
        if result.returncode == 0:
            record["test_command"] = [str(executable), "--nocapture"]
            result = subprocess.run(record["test_command"], stdout=log, stderr=subprocess.STDOUT)
            record["test_exit_code"] = result.returncode
    record["after_sha256"] = sha()
    record["source_unchanged"] = record["before_sha256"] == record["after_sha256"]
    record["log_sha256"] = hashlib.sha256(log_path.read_bytes()).hexdigest()
    record_path.write_text(json.dumps(record, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(record))
    return result.returncode


if __name__ == "__main__":
    raise SystemExit(main())
