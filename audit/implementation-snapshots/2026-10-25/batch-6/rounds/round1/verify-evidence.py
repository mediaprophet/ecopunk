"""Read-only reconciliation of Round 1 files and deliberately open model claims."""
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[6]
QA = Path(__file__).resolve().parent
DOSSIER = ROOT / "docs/production/community-grounds-growing-station"
OUTPUT = ROOT / "assets/reference/community-grounds-growing-station/outputs/round1-film-006"


def read(path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


claims = read(DOSSIER / "claims.json")
template_root = ROOT / "docs/planning/tech/assets/model-production-template"
template = read(template_root / "claims.template.json")
assert digest(template_root / "deliverables.json") == claims["catalogue"]["sha256"]
assert [r["deliverable_id"] for r in claims["claims"]] == [
    r["deliverable_id"] for r in template["claims"]
]
rows = claims["claims"] + claims["subject_specific_claims"]
assert len(claims["claims"]) == 193 and len(claims["subject_specific_claims"]) == 8
assert len({r["deliverable_id"] for r in rows}) == 201
assert all(r["assessment"]["applicability"] == "unassessed" for r in rows)
assert all(r["assessment"]["claim_status"] == "unassessed" for r in rows)
assert all(not r["cases"] for r in rows)  # Required expansion remains incomplete.
assert {p["id"] for p in claims["profiles"]} == {"film", "open_world_game", "cartoon"}
assert all(p["status"] == "unassessed" for p in claims["profiles"])
assert set(read(DOSSIER / "open-ids.json")["open_claim_ids"]) == {
    r["deliverable_id"] for r in rows
}

index = read(DOSSIER / "evidence-index.json")
for artifact in index["artifacts"]:
    path = ROOT / artifact["path"]
    assert path.stat().st_size == artifact["bytes"], str(path)
    assert digest(path) == artifact["sha256"], str(path)

run = read(OUTPUT / "run.json")
receipt = read(OUTPUT / "render-receipt.json")
assert run["hmc_sha256"] == digest(OUTPUT / "hmc/growing-station.hmc")
assert run["compiled_sha256"] == digest(OUTPUT / "growing-station.10d")
assert run["accepted_source_sha256"] == digest(OUTPUT / "accepted-source.vibe")
assert receipt["checks"]["compiled_sha256"] == run["compiled_sha256"]
assert receipt["checks"]["parts"] == 199
assert len(receipt["images"]) == 4
for image in receipt["images"]:
    assert digest(OUTPUT / image["file"]) == image["sha256"]
    assert image["width"] == image["height"] == 1024

counts = {}
for filename, expected in [("catalog-tests.log", 6), ("native-final-tests.log", 42),
                           ("codec-tests.log", 4)]:
    result = re.search(r"test result: ok\. (\d+) passed; 0 failed", (QA / filename).read_text())
    assert result and int(result.group(1)) == expected, filename
    counts[filename] = expected
print(json.dumps({"result": "pass", "artifact_hashes_checked": len(index["artifacts"]),
                  "open_base_claims": 193, "open_subject_claims": 8,
                  "expanded_cases": "incomplete", "profile_acceptance": "unassessed",
                  "native_images": 4, "selected_checks": counts,
                  "model_complete": False, "human_acceptance": False}, indent=2))
