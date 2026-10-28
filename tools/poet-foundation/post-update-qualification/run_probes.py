"""Record the external Rust qualification probes against a freshly built Vibe rlib."""
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
from datetime import datetime, timezone

from run_check import OUTPUT, ROOT, snapshot


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    case, library_arg = sys.argv[1:]
    if not re.fullmatch(r"[a-z0-9-]+", case):
        raise ValueError("invalid case")
    if (ROOT / "target/coordination/native-check.lock").exists():
        raise RuntimeError("Cargo lease is held; run probes after that check completes")
    library = Path(library_arg).resolve()
    deps = (ROOT / "target/debug/deps").resolve()
    if library.parent != deps or not library.name.startswith("libvibe-") or library.suffix != ".rlib":
        raise ValueError("expected the fresh Vibe rlib in the shared target")
    source = Path(__file__).with_name("runtime_boundary_probes.rs")
    executable = ROOT / "target/qualification" / (case + ".exe")
    executable.parent.mkdir(parents=True, exist_ok=True)
    record_path, log_path = OUTPUT / (case + ".json"), OUTPUT / (case + ".log")
    if record_path.exists() or log_path.exists():
        raise FileExistsError("retain earlier evidence; use a new case")
    record = {"case": case, "started_utc": datetime.now(timezone.utc).isoformat(),
        "before": snapshot(), "source": str(source), "source_sha256": sha(source),
        "vibe_library": str(library), "vibe_library_sha256": sha(library),
        "scope": "Synthetic in-process runtime boundaries; no human delegation or external effects",
        "human_acceptance": False, "production_authorised": False}
    record["compile_command"] = ["rustc", "--edition=2021", "--test", str(source),
        "--extern", "vibe=" + str(library), "-L", "dependency=" + str(deps), "-o", str(executable)]
    record_path.write_text(json.dumps(record, indent=2) + "\n", encoding="utf-8")
    with log_path.open("wb") as log:
        result = subprocess.run(record["compile_command"], stdout=log, stderr=subprocess.STDOUT)
        record["compile_exit_code"] = result.returncode
        if result.returncode == 0:
            record["run_command"] = [str(executable), "--nocapture"]
            result = subprocess.run(record["run_command"], stdout=log, stderr=subprocess.STDOUT)
            record["test_exit_code"] = result.returncode
    record["after"] = snapshot()
    record["source_unchanged"] = (record["before"] == record["after"]
        and record["source_sha256"] == sha(source) and record["vibe_library_sha256"] == sha(library))
    record["log_sha256"] = sha(log_path)
    record["summaries"] = re.findall(r"test result: .*", log_path.read_text(encoding="utf-8", errors="replace"))
    record["finished_utc"] = datetime.now(timezone.utc).isoformat()
    record_path.write_text(json.dumps(record, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({k: record.get(k) for k in ("case", "compile_exit_code", "test_exit_code", "source_unchanged", "summaries")}))
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())
