"""Consolidate selected actual passes without counting failed or zero-selection runs."""
import hashlib
import json
from pathlib import Path
import re
from datetime import datetime, timezone

from run_check import OUTPUT, snapshot

CASES = ["vibe-repaired-package-attempt4", "runtime-probes-attempt3",
         "core-publication-executed", "vibe-tooling-repaired", "runtime-staging",
         "poet-native-repaired-attempt3", "poet-native-gpu-executed"]


def main():
    destination = OUTPUT / "verified-summary.json"
    if destination.exists():
        raise FileExistsError("retain the existing summary")
    results = []
    records = {}
    for case in CASES:
        record_path, log_path = OUTPUT / (case + ".json"), OUTPUT / (case + ".log")
        record = json.loads(record_path.read_text(encoding="utf-8"))
        records[case] = record
        raw = log_path.read_bytes()
        if hashlib.sha256(raw).hexdigest() != record["log_sha256"]:
            raise ValueError(f"log changed: {case}")
        log = raw.decode("utf-8", errors="replace")
        summaries = re.findall(r"^test result: .*", log, re.MULTILINE)
        passed = sum(int(n) for s in summaries for n in re.findall(r"(\d+) passed;", s))
        failed = sum(int(n) for s in summaries for n in re.findall(r"(\d+) failed;", s))
        ignored = sum(int(n) for s in summaries for n in re.findall(r"(\d+) ignored;", s))
        warnings = len(re.findall(r"^warning(?:\[[^\]]+\])?:", log, re.MULTILINE))
        if (record.get("exit_code", record.get("test_exit_code")) != 0
                or record.get("compile_exit_code", 0) != 0 or not record["source_unchanged"]
                or not passed or failed or warnings):
            raise ValueError(f"case does not qualify: {case}")
        results.append(dict(case=case, passed=passed, failed=failed, ignored=ignored,
                            compiler_warning_lines=warnings, record=record_path.name))

    current = snapshot()
    if records["poet-native-gpu-executed"]["after"] != current:
        raise ValueError("source boundary changed after final native verification")
    if records["core-publication-executed"]["after"]["qualiaDB"] != current["qualiaDB"]:
        raise ValueError("Core inputs changed after store verification")
    vibe = records["vibe-repaired-package-attempt4"]["after"]["qualiaDB"]
    current_vibe = current["qualiaDB"]
    for key in ("head", "manifests_and_config"):
        if vibe[key] != current_vibe[key]:
            raise ValueError("Vibe source/config boundary changed")
    only_vibe = lambda rows: [r for r in rows if r["path"].startswith("crates/vibe/")]
    if only_vibe(vibe["dirty"]) != only_vibe(current_vibe["dirty"]):
        raise ValueError("Vibe source changed after package verification")
    if records["vibe-tooling-repaired"]["after"]["vibe-script"] != current["vibe-script"]:
        raise ValueError("tooling source changed")

    artifacts = []
    for path in sorted((OUTPUT / "gpu-evidence").iterdir()):
        data = path.read_bytes()
        artifacts.append(dict(path=str(path), bytes=len(data), sha256=hashlib.sha256(data).hexdigest()))
    evidence_dir = OUTPUT / "gpu-evidence"
    if (evidence_dir / "parent-final-gpu.png").read_bytes() != (evidence_dir / "child-reopened-gpu.png").read_bytes():
        raise ValueError("fresh-process PNG differs from the retained final output")
    result = dict(created_utc=datetime.now(timezone.utc).isoformat(), state="selected_software_repairs_verified",
        results=results, total_passed=sum(r["passed"] for r in results),
        ordinary_native_ignored=13, separately_executed_gpu_workflows=1,
        core_cases_filtered=9864, source_boundary=current, gpu_artifacts=artifacts,
        final_parent_and_child_png_identical=True, human_acceptance=False,
        production_ready=False, asset_production="HOLD", new_production_assets=0,
        scope="Selected software regression and headless GPU qualification; not all platform/provider or model acceptance")
    destination.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({k: result[k] for k in ("state", "total_passed", "ordinary_native_ignored",
        "separately_executed_gpu_workflows", "final_parent_and_child_png_identical", "asset_production")}))


if __name__ == "__main__":
    main()
