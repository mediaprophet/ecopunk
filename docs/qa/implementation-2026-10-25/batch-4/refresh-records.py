"""Refresh RUN09 bounded implementation evidence from saved receipts only.

Intentionally do not run this script until root records final receipts. The script
never runs Cargo and preserves the template's blank, all-open milestone rows.
"""
from pathlib import Path
import hashlib
import json

root = Path(__file__).parent
planning = root.parents[2] / "planning"
tech = planning / "tech"
queue_path = planning / "implementation-queue.json"
plan_path = planning / "implementation-plan.md"
PLAN_SHA256 = "62303e796003C13779F04F041AC0047BEA641FDE02428E3A4670F084B88E65E2"
VERSION = "2026-10-25.2"


def read(path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


def write(path, value):
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


# This batch is pinned to implementation plan v0.6. Stop before writes if the
# governing plan has changed since this script was prepared.
if sha(plan_path).lower() != PLAN_SHA256.lower():
    raise SystemExit("implementation plan changed; reconcile RUN09 before refreshing")
if "implementation plan v0.6" not in plan_path.read_text(encoding="utf-8-sig").lower():
    raise SystemExit("implementation plan is not v0.6")

pointer = read(root / "latest-source-snapshot.json")
snapshot_path = Path(pointer["path"])
if not snapshot_path.exists() or sha(snapshot_path).lower() != pointer["sha256"].lower():
    raise SystemExit("latest source snapshot is absent or its pointer digest is invalid")
source_digest = pointer["sha256"].lower()
snapshot = read(snapshot_path)
if not isinstance(snapshot.get("sources"), list) or len(snapshot["sources"]) != 127:
    raise SystemExit("latest source snapshot must contain the reviewed 127 source inputs")

review_manifest = read(root / "review" / "reviewed-inputs.json")
review_inputs = review_manifest.get("inputs", [])
source_hashes = {item.get("path", "").lower(): item.get("sha256", "").lower() for item in snapshot["sources"]}
if len(review_inputs) != 51 or any(
    source_hashes.get(item.get("path", "").lower()) != item.get("sha256", "").lower()
    for item in review_inputs
):
    raise SystemExit("the 51 reviewed inputs do not exactly match the frozen source snapshot")

cli_dir = root / "direct-cli" / "130ad0c42d5e"
cli_checks = read(cli_dir / "checks.json")
cli_artifact = read(cli_dir / "artifact.json")
required_cli_flags = (
    "help",
    "diagnostic_check_no_evaluation_or_window",
    "mcp_stdout_strict_jsonrpc",
    "mcp_unbound_source_read_denied",
)
if (
    cli_checks.get("exit_code") != 0
    or cli_checks.get("artifact_sha256", "").lower() != cli_artifact.get("sha256", "").lower()
    or cli_checks.get("production_authority_supplied") is not False
    or cli_checks.get("gui_launched") is not False
    or not all(cli_checks.get(flag) is True for flag in required_cli_flags)
):
    raise SystemExit("direct CLI proof is incomplete or not a passing no-authority/headless check")


def receipt_state(name, optional=False):
    path = root / name
    result = {
        "path": name,
        "status": "pending",
        "exit_code": None,
        "source_binding": "unassessed",
        "source_unchanged": None,
        "reason": None,
    }
    if not path.exists():
        result["status"] = "not_run" if optional else "pending"
        result["reason"] = "optional receipt absent" if optional else "final saved JSON receipt is not present"
        return result
    result["sha256"] = sha(path)
    try:
        payload = read(path)
    except (OSError, ValueError, json.JSONDecodeError) as error:
        result["reason"] = f"receipt is not readable JSON: {error}"
        return result
    if not isinstance(payload, dict):
        result["reason"] = "receipt JSON root is not an object"
        return result

    code = payload.get("exit_code")
    result["exit_code"] = code if isinstance(code, int) and not isinstance(code, bool) else None
    unchanged = payload.get("source_unchanged")
    result["source_unchanged"] = unchanged if isinstance(unchanged, bool) else None
    pin = payload.get("input_snapshot")
    pinned_digest = pin.get("sha256") if isinstance(pin, dict) else None
    if isinstance(pinned_digest, str) and pinned_digest.lower() == source_digest:
        result["source_binding"] = "matches latest saved source snapshot"
    elif isinstance(pinned_digest, str):
        result["source_binding"] = "does not match latest saved source snapshot"
    else:
        result["source_binding"] = "receipt does not record an input snapshot digest"

    if result["exit_code"] is None:
        result["reason"] = "receipt has no integer exit_code; no test result inferred"
    elif result["exit_code"] != 0:
        result["status"] = "failed"
        result["reason"] = "saved receipt reports a non-zero exit code"
    elif result["source_unchanged"] is not True:
        result["reason"] = "receipt does not prove source_unchanged is exactly true"
    elif result["source_binding"] != "matches latest saved source snapshot":
        result["reason"] = "successful exit is not bound to the latest saved source snapshot"
    else:
        result["status"] = "passed"
        result["reason"] = "zero exit, unchanged sources and exact latest snapshot digest are recorded"
    return result


receipts = {
    "core": receipt_state("core-final.json"),
    "native": receipt_state("native-final.json"),
    "wasm": receipt_state("wasm-final.json"),
    "library": receipt_state("library-final.json"),
    "gpu": receipt_state("gpu-final.json", optional=True),
}


def combined(items):
    states = [item["status"] for item in items]
    if "failed" in states:
        return "bounded_applied_final_source_checks_failed"
    if states and all(state == "passed" for state in states):
        return "bounded_applied_final_source_checks_passed"
    return "bounded_applied_final_source_checks_pending"


def pin_record_documents(record):
    for pin in record.get("supporting_requirement_documents", []):
        candidate = (tech / pin["path"]).resolve()
        if candidate.exists():
            pin["sha256"] = sha(candidate)
    plan = record.get("implementation_plan", {})
    plan["version"] = "0.6"
    plan["sha256"] = sha(plan_path)


queue = read(queue_path)
if len(queue.get("jobs", [])) != 120 or any(job.get("packet_status") != "open" for job in queue["jobs"]):
    raise SystemExit("queue no longer has all 120 packets open; reconcile before refresh")
queue["version"] = VERSION
queue_ref = "../qa/implementation-2026-10-25/batch-4/README.md"
queue.setdefault("bounded_execution_refs", [])
if queue_ref not in queue["bounded_execution_refs"]:
    queue["bounded_execution_refs"].append(queue_ref)
for job in queue["jobs"]:
    if job.get("id") in {"ECO-034", "ECO-044", "ECO-050"}:
        job["dispatch_status"] = (
            "RUN-09 source integration and resource-preserving HMC editing; final source-bound "
            "checks, controller-backed project adoption, real contributor/reviewer acceptance, "
            "semantic complete embodiments and all production outcomes remain open; see batch-4 evidence"
        )

# Keep the starter template blank. Only refresh provenance pins and version.
template_path = tech / "ecopunk-production-claims.template.json"
template = read(template_path)
if len(template.get("packets", [])) != 120 or any(
    packet.get("status") != "open" or packet.get("milestone_results")
    for packet in template["packets"]
):
    raise SystemExit("starter template is not blank/all-open; refusing to change it")
template["version"] = VERSION
pin_record_documents(template)

actual_path = tech / "ecopunk-production-claims.json"
actual = read(actual_path)
if len(actual.get("packets", [])) != 120 or any(packet.get("status") != "open" for packet in actual["packets"]):
    raise SystemExit("actual ledger no longer has all 120 packets open")
actual["version"] = VERSION
pin_record_documents(actual)
actual["source_template"]["version"] = VERSION
actual["source_template"]["sha256"] = sha(template_path)

batch_ref = "../../qa/implementation-2026-10-25/batch-4/"

def pin_batch_evidence(record):
    record.setdefault("supporting_requirement_documents", [])
    for relative in ["README.md", "latest-source-snapshot.json", "authentication/contract.md", "authentication/save-receipt.json", "hmc/source-transitions.json", "hmc/checks.json", "hmc/README.md", "authentication/README.md", "review/README.md", "review/reviewed-inputs.json", "native-check-corrected.json", "direct-cli/130ad0c42d5e/checks.json", "direct-cli/130ad0c42d5e/artifact.json", "engine-handoff-saved.json"]:
        evidence_path = root / relative
        if not evidence_path.exists():
            continue
        relative_path = batch_ref + relative
        pin = next((p for p in record["supporting_requirement_documents"] if p.get("path") == relative_path), None)
        if pin is None:
            record["supporting_requirement_documents"].append(dict(path=relative_path, sha256=sha(evidence_path)))
        else:
            pin["sha256"] = sha(evidence_path)

pin_batch_evidence(template)
pin_batch_evidence(actual)

milestones = [
    {
        "id": "RUN-09-AUTH-HMC",
        "scope": "Native project-root authority integration and resource-preserving HMC checkpoint editing, bounded to applied-source tests/builds.",
        "required": [receipts["core"], receipts["native"]],
        "remaining": "Authenticated controller-backed principal and project adoption proof in a real contributor/reviewer workflow; durable contribution/review evidence; complete semantic HMC/model dossiers; human acceptance and all production derivatives.",
        "evidence": ["README.md", "inventory/README.md", "authentication/save-receipt.json", "hmc/source-before.json", "hmc/source-after.json", "hmc/source-transitions.json", "hmc/checks.json", "hmc/README.md", "authentication/README.md", "review/README.md", "review/reviewed-inputs.json", "latest-source-snapshot.json", "native-check-corrected.json", "direct-cli/130ad0c42d5e/checks.json", "direct-cli/130ad0c42d5e/artifact.json", "engine-handoff-saved.json"],
    },
    {
        "id": "RUN-09-TARGETS",
        "scope": "Saved WASM and native library checks bound to the latest unchanged source snapshot.",
        "required": [receipts["wasm"], receipts["library"]],
        "remaining": "These target checks do not establish native login/adoption proof, cooperative production acceptance, complete HMC semantics, model acceptance, or human review.",
        "evidence": ["README.md", "latest-source-snapshot.json"],
    },
]

for milestone in milestones:
    available = [batch_ref + item for item in milestone["evidence"] if (root / item).exists()]
    # Include only final receipts actually present; no absent or inferred result is pinned.
    for result in receipts.values():
        if (root / result["path"]).exists():
            available.append(batch_ref + result["path"])
    available = list(dict.fromkeys(available))
    required = milestone["required"]
    record = {
        "id": milestone["id"],
        "scope": milestone["scope"],
        "status": combined(required),
        "evidence_refs": available,
        "evidence_pins": [dict(path=ref, sha256=sha(root / ref.removeprefix(batch_ref))) for ref in available],
        "final_receipt_statuses": [dict(item) for item in required],
        "pending_receipt_paths": [item["path"] for item in required if item["status"] == "pending"],
        "remaining": milestone["remaining"],
        "human_acceptance_claimed": False,
        "model_completion_claimed": False,
    }
    for packet in actual["packets"]:
        if packet.get("id") in {"ECO-034", "ECO-044", "ECO-050"}:
            previous = [result for result in packet.get("milestone_results", []) if result.get("id") != record["id"]]
            packet["milestone_results"] = previous + [dict(record)]

summary = actual.setdefault("execution_summary", {})
summary["batch"] = "2026-10-24/25 bounded implementation batches 1–4"
summary["batch_4_evidence"] = batch_ref + "README.md"
summary["live_builds"] = 0
summary["batch_4_live_builds"] = 0
summary["batch_4_receipts"] = list(receipts.values())
summary["batch_4_auth_hmc_status"] = combined([receipts["core"], receipts["native"]])
summary["batch_4_target_status"] = combined([receipts["wasm"], receipts["library"]])
summary["model_or_language_release_complete"] = False
summary["human_acceptance_claimed"] = False
summary["remaining_priority"] = (
    "Controller-backed project authentication/adoption and actual cooperative contributor/reviewer work; "
    "complete semantic HMC/model dossiers, accepted editable masters and reviewed film/game/cartoon derivatives"
)

# Persist only after all validation and record construction succeeded. Queue
# writes change its pinned hash, so refresh those pins before writing the starter
# template and then refresh the actual ledger's template hash.
write(queue_path, queue)
for record in (template, actual):
    for pin in record.get("supporting_requirement_documents", []):
        if pin.get("path") == "../implementation-queue.json":
            pin["sha256"] = sha(queue_path)
write(template_path, template)
actual["source_template"]["version"] = VERSION
actual["source_template"]["sha256"] = sha(template_path)
write(actual_path, actual)
report = {
    "status": "RUN09 evidence refreshed; all 120 packets remain open and starter template remains blank",
    "plan_version": "0.6",
    "plan_sha256": sha(plan_path),
    "receipts": {key: value["status"] for key, value in receipts.items()},
    "auth_hmc": combined([receipts["core"], receipts["native"]]),
    "targets": combined([receipts["wasm"], receipts["library"]]),
    "gpu": receipts["gpu"]["status"],
    "direct_cli": "passed",
    "live_builds": 0,
    "packets_open": 120,
    "template_blank": all(packet.get("status") == "open" and not packet.get("milestone_results") for packet in template["packets"]),
    "reviewed_inputs": len(review_inputs),
    "review_mismatches": 0,
}
write(root / "refresh-console.json", report)
print(json.dumps(report, ensure_ascii=False, indent=2))
