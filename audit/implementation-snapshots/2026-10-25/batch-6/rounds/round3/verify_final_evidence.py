"""Read-only final source, actual-case and artifact reconciliation; no builds."""
import hashlib
import json
from pathlib import Path
import re

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[5]


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


expected = {}
for name in ["applied-source.json", "poet-inspection/applied-source.json",
             "poet-inspection/repair/applied-source.json",
             "poet-inspection/repair-2/applied-source.json"]:
    for row in read(HERE / name):
        key = str(Path(row["target"])).lower()
        if key in expected:
            assert row["before_sha256"] == expected[key]["after_sha256"], key
        expected[key] = row
assert len(expected) == 44
for row in expected.values():
    assert sha(row["target"]) == row["after_sha256"], row["target"]

receipts = {}
for mode in ["native", "wasm", "sdk-wasm", "poet-tests", "poet-epoch", "poet-build"]:
    row = read(HERE / (mode + "-result.json"))
    assert row["exit_code"] == 0 and row["source_unchanged"], mode
    assert not row["source_changes"], mode
    assert sha(row["log"]) == row["log_sha256"], mode
    assert sha(row["source_snapshot"]) == row["source_snapshot_sha256"], mode
    if mode in ["poet-tests", "poet-epoch"]:
        assert row["test_selection_ok"], mode
    receipts[mode] = row


def counts(mode):
    log = Path(receipts[mode]["log"]).read_text(encoding="utf-8-sig")
    rows = [tuple(map(int, x)) for x in re.findall(
        r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored", log)]
    assert rows and all(failed == 0 and ignored == 0 for _, failed, ignored in rows)
    return sum(passed for passed, _, _ in rows)


assert counts("native") == 56
assert counts("poet-tests") == 6
assert counts("poet-epoch") == 1
required = [
    "graphical::checkpoints::inspection::tests::canonical_v3_and_legacy_inspection_are_distinct",
    "graphical::checkpoints::inspection::tests::corruption_unknown_payload_and_budgets_never_fallback",
    "graphical::checkpoints::inspection::tests::selected_hmc_digests_and_unknown_metadata_remain_exact",
    "graphical::checkpoints::inspection::tests::native_service_preserves_immutable_pin_and_path_refusals",
    "graphical::cooperation::tests::selected_resource_inspection_requires_current_read_authority_and_source",
    "graphical::jobs::tests::resource_inspection_late_source_and_session_completions_are_withheld",
]
poet_log = Path(receipts["poet-tests"]["log"]).read_text()
for name in required:
    assert "test " + name + " ... ok" in poet_log, name

assets = read(HERE.parent / "round2/status-final.json")["preserved_assets"]
for row in assets:
    path = ROOT / row["path"]
    assert sha(path) == row["sha256"] and path.stat().st_size == row["bytes"]
source = ROOT / "assets/reference/community-grounds-growing-station/master.vibe"
assert sha(source) == "8aac87e9f98dc305ea85d45f2ebae19aba5fbd9323c165f160c83bb46ce89a63"
binary = ROOT / "target/debug/poet.exe"
with binary.open("rb") as handle:
    assert handle.read(2) == b"MZ"
result = dict(status="passed", kind="read_only_final_reconciliation",
    distinct_installed_targets=44, historical_test_repairs=2,
    all_latest_installed_hashes_match=True, native_sdk_tests=56,
    native_poet_inspection_tests=6, existing_native_lifecycle_tests=1,
    unique_tests_in_this_pass=63, all_six_inspector_case_names_verified=True,
    qualifying_commands_exit_zero_and_source_unchanged=True,
    wasm_frontend_compile="passed; does not compile revised SDK",
    wasm_sdk_portal_compile="passed; runtime interoperability untested",
    preserved_regression_artifacts=3, reference_source_unchanged=True,
    native_poet_binary=dict(path=str(binary), bytes=binary.stat().st_size, sha256=sha(binary)),
    asset_production="on_hold", foundation_readiness="not_qualified",
    gui_verification="not_run", human_acceptance=False,
    builds_or_tests_executed_by_this_verifier=False)
print(json.dumps(result, indent=2))
