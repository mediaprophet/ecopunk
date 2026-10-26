"""Read-only saved-source/evidence checks; writes only its own audit receipt."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib
import json

root = Path(__file__).resolve().parent
workspace = root.parents[3]


def read(path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


pointer = read(root / "latest-source-snapshot.json")
assert sha(pointer["path"]) == pointer["sha256"]
snapshot = read(Path(pointer["path"]))
assert len(snapshot["sources"]) == 166
for pin in snapshot["sources"]:
    assert sha(pin["path"]) == pin["sha256"], pin["path"]
review = read(root / "review/reviewed-inputs.json")
assert review["supervisor_snapshot"]["sha256"] == pointer["sha256"]
assert len(review["files"]) == 53
for pin in review["files"]:
    assert sha(pin["path"]) == pin["sha256"], pin["path"]
names = ["checkpoint-final.json", "core-fresh-process-corrected.json", "core-final.json", "native-final.json",
         "library-final.json", "wasm-final.json", "gpu-final.json",
         "direct-cli/native-project-check.json"]
for name in names:
    receipt = read(root / name)
    assert receipt["exit_code"] == 0 and receipt["source_unchanged"] is True, name
    assert receipt["input_snapshot"]["sha256"] == pointer["sha256"], name
assert read(root / "core-final.json")["execution_boundary"] == "native"
denied = read(root / "network-sandbox-proof.json")
allowed = read(root / "network-native-proof.json")
assert denied["exit_code"] != 0 and allowed["exit_code"] == 0
assert denied["sha256"].lower() == allowed["sha256"].lower() == sha(allowed["binary"])
assert denied["arguments"] == allowed["arguments"]
for boundary, proof in (("sandbox", denied), ("native", allowed)):
    assert proof["boundary"] == boundary and proof["source_unchanged"] is True
    assert proof["input_snapshot"]["sha256"] == pointer["sha256"]
    assert proof["log_sha256"].lower() == sha(root / ("network-" + boundary + "-proof.log"))
checkpoint_log = (root / "checkpoint-final.log").read_text(encoding="utf-8-sig")
for case in ("real_deny_delete_handle_release_allows_bounded_retry",
             "permanent_deny_delete_refusal_preserves_destination_and_cleans_staging",
             "changed_expected_bytes_stop_retry_before_replacement"):
    assert case + " ... ok" in checkpoint_log, case
gpu_log = (root / "gpu-final.log").read_text(encoding="utf-8-sig")
assert "running 1 test" in gpu_log and "test result: ok. 1 passed; 0 failed;" in gpu_log
assert "accepted_material_edits_change_native_gpu_pixels" in gpu_log
cli = root / "direct-cli" / pointer["sha256"][:12]
checks, artifact = read(cli / "checks.json"), read(cli / "artifact.json")
assert checks["exit_code"] == 0 and checks["gui_launched"] is False
assert sha(artifact["path"]) == artifact["sha256"] == checks["artifact_sha256"]
public = read(root / "direct-cli/native-project-check.json")
assert public["artifact_sha256"] == artifact["sha256"]
tech = workspace / "docs/planning/tech"
actual = read(tech / "ecopunk-production-claims.json")
template = read(tech / "ecopunk-production-claims.template.json")
queue = read(workspace / "docs/planning/implementation-queue.json")
assert len(queue["jobs"]) == 120 and all(p["packet_status"] == "open" for p in queue["jobs"])
assert len(actual["packets"]) == 120 and all(p["status"] == "open" for p in actual["packets"])
assert len(template["packets"]) == 120
assert all(p["status"] == "open" and not p["milestone_results"] for p in template["packets"])
summary = actual["execution_summary"]
assert summary["live_builds"] == 0
assert summary["human_acceptance_claimed"] is False
assert summary["model_completion_claimed"] is False and summary["production_ready_claimed"] is False
assert all(item["status"] == "passed" for item in summary["batch_5_receipts"])
for record in (actual, template):
    for pin in record.get("supporting_requirement_documents", []):
        if pin["path"].startswith("../../qa/implementation-2026-10-25/batch-5/"):
            assert sha(tech / pin["path"]) == pin["sha256"], pin["path"]
assert actual["source_template"]["sha256"] == sha(tech / "ecopunk-production-claims.template.json")
catalogue = tech / "assets/model-production-template/deliverables.json"
assert sha(catalogue) == "c0cb76151bf5d0776afaae4d59d7427efb65633bf359d3f4c2bda83c361c91cb"
result = dict(checked_utc=datetime.now(timezone.utc).isoformat(), source_snapshot=pointer,
              source_inputs_verified=166, reviewed_inputs_verified=53,
              receipts=[dict(path=n, sha256=sha(root / n), exit_code=0) for n in names],
              cli_artifact=artifact, live_builds=0, production_packets_open=120,
              verification_scripts=[dict(path=str(p), sha256=sha(p)) for p in
                                    sorted(list(root.glob("*.py")) + list(root.glob("*.ps1")) +
                                           [root / "direct-cli/link_and_check.py"])],
              starter_milestones=0, human_acceptance_claimed=False,
              model_completion_claimed=False, production_ready_claimed=False)
(root / "final-verification.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
print(json.dumps(dict(status="saved-source/evidence/open-ledger checks passed",
                     sources=166, reviewed=53, packets_open=120)))
