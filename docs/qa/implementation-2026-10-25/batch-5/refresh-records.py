"""Refresh RUN10 evidence from completed saved receipts; never runs Cargo or edits source.

Usage after the supervisor announces final results:
  python refresh-records.py --final-results-confirmed
"""
from pathlib import Path
import argparse
import hashlib
import json

root = Path(__file__).parent
planning = root.parents[2] / "planning"
tech = planning / "tech"
queue_path = planning / "implementation-queue.json"
plan_path = planning / "implementation-plan.md"
PLAN_SHA256 = "62303E796003C13779F04F041AC0047BEA641FDE02428E3A4670F084B88E65E2"
VERSION = "2026-10-25.3"
PACKETS = {"ECO-034", "ECO-044", "ECO-050"}
BATCH_REF = "../../qa/implementation-2026-10-25/batch-5/"

parser = argparse.ArgumentParser()
parser.add_argument("--final-results-confirmed", action="store_true",
                    help="operator confirms supervisor has finished all intended RUN10 checks")
args = parser.parse_args()
if not args.final_results_confirmed:
    raise SystemExit("wait for the supervisor's final results, then pass --final-results-confirmed")


def read(path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


def write(path, value):
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


if sha(plan_path).upper() != PLAN_SHA256 or "implementation plan v0.6" not in plan_path.read_text(encoding="utf-8-sig").lower():
    raise SystemExit("governing plan changed; reconcile RUN10 before refreshing")
pointer = read(root / "latest-source-snapshot.json")
snapshot_path = Path(pointer["path"])
if not snapshot_path.exists() or sha(snapshot_path).lower() != pointer["sha256"].lower():
    raise SystemExit("latest source snapshot is absent or pointer digest is invalid")
snapshot_digest = pointer["sha256"].lower()
snapshot = read(snapshot_path)
if not isinstance(snapshot.get("sources"), list) or len(snapshot["sources"]) != 166:
    raise SystemExit("latest RUN10 source snapshot must contain the reviewed 166 inputs")
for source in snapshot["sources"]:
    path = Path(source["path"])
    if not path.is_file() or sha(path) != source["sha256"]:
        raise SystemExit("live source differs from final receipt snapshot: " + str(path))


def checked_receipt(relative, optional=False):
    result = {
        "path": relative,
        "status": "not_run" if optional else "pending",
        "exit_code": None,
        "source_unchanged": None,
        "source_binding": "unassessed",
        "reason": "optional receipt absent" if optional else "final receipt absent",
    }
    path = root / relative
    if not path.exists():
        return result
    result["sha256"] = sha(path)
    try:
        payload = read(path)
    except (OSError, ValueError, json.JSONDecodeError) as error:
        result.update(status="failed", reason=f"receipt is not valid JSON: {error}")
        return result
    if not isinstance(payload, dict):
        result.update(status="failed", reason="receipt root must be an object")
        return result
    code = payload.get("exit_code")
    unchanged = payload.get("source_unchanged")
    pin = payload.get("input_snapshot")
    pin_digest = pin.get("sha256") if isinstance(pin, dict) else None
    result["exit_code"] = code if isinstance(code, int) and not isinstance(code, bool) else None
    result["source_unchanged"] = unchanged if isinstance(unchanged, bool) else None
    result["source_binding"] = (
        "matches latest source snapshot" if isinstance(pin_digest, str) and pin_digest.lower() == snapshot_digest
        else "does not match latest source snapshot" if isinstance(pin_digest, str)
        else "receipt has no input snapshot digest"
    )
    if result["exit_code"] != 0:
        result.update(status="failed" if result["exit_code"] is not None else "pending",
                      reason="exit_code is nonzero or absent")
    elif unchanged is not True:
        result.update(status="failed", reason="source_unchanged is not exactly true")
    elif result["source_binding"] != "matches latest source snapshot":
        result.update(status="failed", reason="receipt is not bound to the current snapshot")
    else:
        result.update(status="passed", reason="zero exit, unchanged sources and exact current snapshot pin")
    return result


receipts = {
    "checkpoint": checked_receipt("checkpoint-final.json"),
    "fresh_process": checked_receipt("core-fresh-process-corrected.json"),
    "core": checked_receipt("core-final.json"),
    "native": checked_receipt("native-final.json"),
    "library": checked_receipt("library-final.json"),
    "wasm": checked_receipt("wasm-final.json"),
    "gpu": checked_receipt("gpu-final.json", optional=True),
}

# Optional public-config-only CLI proof. It must not be mistaken for authority,
# credential, source-read or GUI evidence.
cli_rel = "direct-cli/native-project-check.json"
cli_path = root / cli_rel
if cli_path.exists():
    cli = checked_receipt(cli_rel, optional=True)
    if cli["status"] == "passed":
        payload = read(cli_path)
        if not all(payload.get(key) is value for key, value in {
            "public_config_only": True,
            "source_files_opened": False,
            "os_credential_accessed": False,
            "adoption_created": False,
            "gui_launched": False,
        }.items()):
            cli.update(status="failed", reason="CLI proof does not establish the required public-config-only boundary")
    receipts["cli"] = cli
else:
    receipts["cli"] = {
        "path": cli_rel, "status": "not_run", "exit_code": None,
        "source_unchanged": None, "source_binding": "unassessed",
        "reason": "optional public-config-only CLI proof absent",
    }

required_names = ("checkpoint", "fresh_process", "core", "native", "library", "wasm")
required = [receipts[name] for name in required_names]
if any(item["status"] == "failed" for item in required):
    milestone_status = "bounded_RUN10_final_source_checks_failed"
elif all(item["status"] == "passed" for item in required):
    milestone_status = "bounded_RUN10_final_source_checks_passed"
else:
    milestone_status = "bounded_RUN10_final_source_checks_pending"

queue = read(queue_path)
template_path = tech / "ecopunk-production-claims.template.json"
actual_path = tech / "ecopunk-production-claims.json"
template = read(template_path)
actual = read(actual_path)
if len(queue.get("jobs", [])) != 120 or any(job.get("packet_status") != "open" for job in queue["jobs"]):
    raise SystemExit("queue is not all-open with 120 jobs; refuse refresh")
for name, record in (("starter template", template), ("actual ledger", actual)):
    if len(record.get("packets", [])) != 120 or any(packet.get("status") != "open" for packet in record["packets"]):
        raise SystemExit(f"{name} is not all-open with 120 packets; refuse refresh")
if any(packet.get("status") != "open" or packet.get("milestone_results") for packet in template["packets"]):
    raise SystemExit("starter template is not blank/all-open; refuse refresh")

queue["version"] = VERSION
queue_ref = "../qa/implementation-2026-10-25/batch-5/README.md"
queue.setdefault("bounded_execution_refs", [])
if queue_ref not in queue["bounded_execution_refs"]:
    queue["bounded_execution_refs"].append(queue_ref)


def pin_docs(record):
    record.setdefault("supporting_requirement_documents", [])
    paths = [
        "README.md", "latest-source-snapshot.json", "source-snapshots/" + snapshot_path.name,
        "enrollment/README.md", "native-entry/README.md", "native-entry/operator-workflow.md", "workflow/README.md",
        "review/README.md", "review/reviewed-inputs.json", "inventory/README.md",
        "native-check-initial.json", "native-check-integrated.json", "poet-lock-reconciled.json",
        "network-sandbox-proof.json", "network-native-proof.json", "network-restricted-results/core-final.json",
    ]
    for rel in paths:
        path = root / rel
        if not path.exists():
            continue
        ref = BATCH_REF + rel
        pin = next((entry for entry in record["supporting_requirement_documents"] if entry.get("path") == ref), None)
        if pin is None:
            record["supporting_requirement_documents"].append({"path": ref, "sha256": sha(path)})
        else:
            pin["sha256"] = sha(path)
    plan_ref = next((entry for entry in record["supporting_requirement_documents"] if entry.get("path") == "../implementation-plan.md"), None)
    if plan_ref is not None:
        plan_ref["sha256"] = sha(plan_path)
    queue_ref_pin = next((entry for entry in record["supporting_requirement_documents"] if entry.get("path") == "../implementation-queue.json"), None)
    if queue_ref_pin is not None:
        queue_ref_pin["sha256"] = sha(queue_path)


pin_docs(template)
template["version"] = VERSION
pin_docs(actual)
actual["version"] = VERSION
evidence_files = [
    "README.md", "latest-source-snapshot.json", "source-snapshots/" + snapshot_path.name,
    "enrollment/README.md", "native-entry/README.md", "native-entry/operator-workflow.md", "workflow/README.md",
    "review/README.md", "inventory/README.md",
    "network-sandbox-proof.json", "network-native-proof.json", "network-restricted-results/core-final.json",
]
for item in receipts.values():
    if (root / item["path"]).exists():
        evidence_files.append(item["path"])
evidence_files = list(dict.fromkeys(rel for rel in evidence_files if (root / rel).exists()))
evidence_refs = [BATCH_REF + rel for rel in evidence_files]
evidence_pins = [{"path": ref, "sha256": sha(root / ref.removeprefix(BATCH_REF))} for ref in evidence_refs]
record = {
    "id": "RUN-10-NATIVE-ENROLLMENT-WORKFLOW",
    "scope": "Saved native enrollment/profile/adoption entry and cooperative Poet source workflow; final source-bound target checks only.",
    "status": milestone_status,
    "evidence_refs": evidence_refs,
    "evidence_pins": evidence_pins,
    "final_receipt_statuses": [dict(item) for item in receipts.values()],
    "pending_receipt_paths": [item["path"] for item in receipts.values() if item["status"] in {"pending", "not_run"}],
    "remaining": "Actual native OS credential-provider execution/persistence; adopted external trust anchors; real human enrollment and accepted contributor/reviewer GUI work; fresh-process HMC/source recovery after login. The focused fresh-process key-possession/login proof and low-level checkpoint subprocess recovery/private undo-redo fixture both passed. They do not qualify actual native operator/profile authenticated session-to-HMC/source recovery after restart or OS-provider persistence. Complete semantic HMC/model dossiers and reviewed film/game/cartoon derivatives also remain open.",
    "human_acceptance_claimed": False,
    "model_completion_claimed": False,
    "production_ready_claimed": False,
}
for packet in actual["packets"]:
    if packet.get("id") in PACKETS:
        previous = [entry for entry in packet.get("milestone_results", []) if entry.get("id") != record["id"]]
        packet["milestone_results"] = previous + [dict(record)]
        packet["status"] = "open"
summary = actual.setdefault("execution_summary", {})
summary["batch_5_evidence"] = BATCH_REF + "README.md"
summary["batch_5_receipts"] = list(receipts.values())
summary["batch_5_status"] = milestone_status
summary["human_acceptance_claimed"] = False
summary["model_completion_claimed"] = False
summary["production_ready_claimed"] = False
summary["all_120_packets_open"] = True

write(queue_path, queue)
for record_doc in (template, actual):
    for pin in record_doc.get("supporting_requirement_documents", []):
        if pin.get("path") == "../implementation-queue.json":
            pin["sha256"] = sha(queue_path)
write(template_path, template)
actual["source_template"] = actual.get("source_template", {})
actual["source_template"]["version"] = VERSION
actual["source_template"]["sha256"] = sha(template_path)
write(actual_path, actual)
report = {
    "status": milestone_status,
    "snapshot_sha256": snapshot_digest,
    "required_receipts": {name: receipts[name]["status"] for name in required_names},
    "optional_receipts": {name: receipts[name]["status"] for name in ("gpu", "cli")},
    "queue_packets_open": sum(job.get("packet_status") == "open" for job in queue["jobs"]),
    "ledger_packets_open": sum(packet.get("status") == "open" for packet in actual["packets"]),
    "template_blank": all(packet.get("status") == "open" and not packet.get("milestone_results") for packet in template["packets"]),
    "human_acceptance_claimed": False,
    "model_completion_claimed": False,
    "production_ready_claimed": False,
}
write(root / "refresh-console.json", report)
print(json.dumps(report, ensure_ascii=False, indent=2))




