from pathlib import Path
import hashlib
import json
import sys

root = Path(__file__).parent
planning = root.parents[2] / "planning"
tech = planning / "tech"

live = int(sys.argv[1]) if len(sys.argv) > 1 else 0
assert 0 <= live <= 4


def read(path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


def write(path, value):
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def requirements(record):
    for pin in record.get("supporting_requirement_documents", []):
        pin["sha256"] = sha(tech / pin["path"])
    record["implementation_plan"]["version"] = "0.6"
    record["implementation_plan"]["sha256"] = sha(tech / record["implementation_plan"]["path"])


def current_source_digest():
    pointer_path = root / "latest-source-snapshot.json"
    if not pointer_path.exists():
        return None
    pointer = read(pointer_path)
    snapshot_path = Path(pointer["path"])
    if not snapshot_path.exists() or sha(snapshot_path) != pointer.get("sha256"):
        return None
    return pointer.get("sha256")


def receipt_state(relative_path, expected_source_digest):
    """Assess a final saved JSON receipt without inferring results from logs."""
    path = root / relative_path
    result = {
        "path": relative_path.as_posix() if isinstance(relative_path, Path) else str(relative_path).replace("\\", "/"),
        "status": "pending",
        "exit_code": None,
        "source_binding": "unassessed",
        "reason": None,
    }
    if not path.exists():
        result["reason"] = "final saved JSON receipt is not present"
        return result
    result["sha256"] = sha(path)
    try:
        payload = read(path)
    except (OSError, ValueError, json.JSONDecodeError) as error:
        result["reason"] = f"receipt is not readable JSON: {error}"
        return result

    code = payload.get("exit_code")
    result["exit_code"] = code if isinstance(code, int) else None
    source_pin = payload.get("input_snapshot") or payload.get("source_snapshot")
    if source_pin and expected_source_digest:
        pinned_digest = str(source_pin.get("sha256", "")).lower()
        if pinned_digest == expected_source_digest.lower():
            result["source_binding"] = "matches latest saved source snapshot"
        else:
            result["source_binding"] = "does not match latest saved source snapshot"
    elif source_pin:
        result["source_binding"] = "latest source snapshot unavailable"
    else:
        result["source_binding"] = "receipt does not record an input source snapshot"

    if result["exit_code"] is None:
        result["reason"] = "receipt has no integer exit_code; no test result inferred"
        return result
    if result["exit_code"] != 0:
        result["status"] = "failed"
        result["reason"] = "saved receipt reports a non-zero exit code"
        return result
    if result["source_binding"] != "matches latest saved source snapshot":
        result["reason"] = "successful exit is not bound to the latest saved source snapshot"
        return result
    result["status"] = "passed"
    result["reason"] = "saved zero exit code is bound to the latest saved source snapshot"
    return result


def combined_state(receipts):
    states = [receipt["status"] for receipt in receipts]
    if any(state == "failed" for state in states):
        return "failed"
    if states and all(state == "passed" for state in states):
        return "passed"
    return "pending"


def milestone_status(receipts):
    state = combined_state(receipts)
    return {
        "passed": "bounded_applied_final_source_tests_passed",
        "failed": "bounded_applied_final_source_tests_failed",
        "pending": "bounded_applied_final_source_tests_pending",
    }[state]


version = "2026-10-25.1"
queue_path = planning / "implementation-queue.json"
queue = read(queue_path)
assert len(queue["jobs"]) == 120 and all(job["packet_status"] == "open" for job in queue["jobs"])
queue["version"] = version
batch_ref = "../qa/implementation-2026-10-25/batch-3/README.md"
if batch_ref not in queue["bounded_execution_refs"]:
    queue["bounded_execution_refs"].append(batch_ref)
for job in queue["jobs"]:
    if job["id"] in ["ECO-034", "ECO-044", "ECO-045", "ECO-050"]:
        job["dispatch_status"] = (
            "RUN-08 scoped native policy/MCP/source-HMC/material/render integration; "
            "production native authentication, complete resource retention, contributor/reviewer "
            "and high-grade acceptance remain open; see batch-3 evidence"
        )
write(queue_path, queue)

template_path = tech / "ecopunk-production-claims.template.json"
template = read(template_path)
assert len(template["packets"]) == 120
assert all(packet["status"] == "open" and not packet["milestone_results"] for packet in template["packets"])
template["version"] = version
requirements(template)
write(template_path, template)

actual_path = tech / "ecopunk-production-claims.json"
actual = read(actual_path)
assert len(actual["packets"]) == 120 and all(packet["status"] == "open" for packet in actual["packets"])
actual["version"] = version
requirements(actual)
actual["source_template"]["version"] = version
actual["source_template"]["sha256"] = sha(template_path)
ref = "../../qa/implementation-2026-10-25/batch-3/"
source_digest = current_source_digest()

final_receipt_paths = {
    "core": "core-final-saved-tests.json",
    "native": "native-build/tests-saved-final.json",
    "material_pixels": "native-build/material-pixels-final.json",
    "wasm": "native-build/wasm-saved-final.json",
    "library": "native-build/library-saved-final.json",
}
final_receipts = {
    name: receipt_state(path, source_digest)
    for name, path in final_receipt_paths.items()
}

milestones = [
    (
        "RUN-08-AUTH",
        ["ECO-034", "ECO-044", "ECO-050"],
        "Host-bound canonical project policy, qualified identifier associations, revision checks and native publication guards",
        [
            "authorisation/verification.json",
            "latest-source-snapshot.json",
            "final-source-audit.json",
            "native-build/direct-cli/e4e2e18c8cba/artifact.json",
            "native-build/direct-cli/e4e2e18c8cba/checks.json",
        ],
        ["core", "native"],
        "Verified native authentication/admission adapter and durable document/contribution/review commit; real contributor/reviewer production acceptance",
    ),
    (
        "RUN-08-MCP",
        ["ECO-034", "ECO-044"],
        "Thin lifecycle/framing transport and shared canonical UI/MCP host, with unbound denial and action-specific admission",
        [
            "mcp/verification.json",
            "shared-host/verification.json",
            "native-build/direct-cli/e4e2e18c8cba/artifact.json",
            "native-build/direct-cli/e4e2e18c8cba/checks.json",
        ],
        ["core", "native"],
        "Production host bootstrap, operator coverage and native/WASM/session/lifecycle acceptance beyond fixtures",
    ),
    (
        "RUN-08-HMC",
        ["ECO-034", "ECO-044", "ECO-050"],
        "Durable exact source/private journal HMC, fresh authorised activation and measured derived-geometry precision",
        ["hmc/verification.json", "hmc/restart-initial-quantization-analysis.json"],
        ["core", "native"],
        "Complete embodiment graph/resources/masters retention and resource-preserving package editing; diagnostic checkpoints are not full packages",
    ),
    (
        "RUN-08-MATERIAL",
        ["ECO-044", "ECO-045", "ECO-050"],
        "Native explicit factor material editing with pre-promotion validation and source-bound actual GPU readback",
        ["session-validation/actual-session-tests.json"],
        ["core", "native", "material_pixels"],
        "Final current-revision GPU proof, texture/HDR/physical material authoring, calibrated lighting and human acceptance of actual model masters",
    ),
    (
        "RUN-08-RENDER",
        ["ECO-044", "ECO-045"],
        "Demonstrated metallic suppression correction and tagged near-detail AO depth repair with discovered consumers updated",
        ["materials/ao-qualification/precision/verification.json", "materials/ao-qualification/precision/independent-review.json"],
        ["core", "native", "material_pixels", "library"],
        "Full cooperative authoring usability, complete resource-preserving embodiments, high-grade accepted content, reviewed derivatives and human acceptance",
    ),
]

for identifier, packet_ids, scope, prior_evidence, required_names, remaining in milestones:
    required = [final_receipts[name] for name in required_names]
    evidence_paths = ["README.md"] + prior_evidence
    evidence_paths += [receipt["path"] for receipt in final_receipts.values() if (root / receipt["path"]).exists()]
    evidence_paths = list(dict.fromkeys(evidence_paths))
    available = [name for name in evidence_paths if (root / name).exists()]
    pins = [dict(path=ref + name, sha256=sha(root / name)) for name in available]
    item = dict(
        id=identifier,
        scope=scope,
        status=milestone_status(required),
        evidence_refs=[ref + name for name in available],
        evidence_pins=pins,
        final_receipt_statuses=[dict(receipt) for receipt in required],
        pending_receipt_paths=[receipt["path"] for receipt in required if receipt["status"] == "pending"],
        remaining=remaining,
        human_acceptance_claimed=False,
    )
    for packet in actual["packets"]:
        if packet["id"] in packet_ids:
            packet["milestone_results"] = [
                result for result in packet["milestone_results"] if result["id"] != identifier
            ] + [item.copy()]

summary = actual["execution_summary"]
summary["batch"] = "2026-10-24/25 bounded implementation batches 1–3"
summary["batch_3_evidence"] = ref + "README.md"
summary["live_builds"] = live
summary["remaining_priority"] = (
    "Verified native project authentication/admission and evidence commit; complete HMC resource preservation; "
    "actual cooperative reference production, exceptional editable masters and reviewed game-demo derivatives"
)
summary["model_or_language_release_complete"] = False
summary["human_acceptance_claimed"] = False
summary["batch_3_final_receipts"] = list(final_receipts.values())
summary["batch_3_native_run"] = combined_state([final_receipts["core"], final_receipts["native"]])
summary["batch_3_material_readback"] = final_receipts["material_pixels"]["status"]
summary["batch_3_wasm_run"] = final_receipts["wasm"]["status"]
summary["batch_3_library_run"] = final_receipts["library"]["status"]

for name in [
    "README.md",
    "latest-source-snapshot.json",
    "final-source-audit.json",
    "final-source-audit.txt",
    "engine-handoff-saved.json",
]:
    path = ref + name
    pin = next((pin for pin in actual["supporting_requirement_documents"] if pin["path"] == path), None)
    if pin is None:
        actual["supporting_requirement_documents"].append(dict(path=path, sha256=sha(root / name)))
    else:
        pin["sha256"] = sha(root / name)
write(actual_path, actual)

print(json.dumps({
    "status": "Refreshed scoped evidence; all 120 packets remain open and starter remains blank",
    "live_builds": live,
    "final_receipts": {name: receipt["status"] for name, receipt in final_receipts.items()},
    "native_current_status": summary["batch_3_native_run"],
}, ensure_ascii=False))
