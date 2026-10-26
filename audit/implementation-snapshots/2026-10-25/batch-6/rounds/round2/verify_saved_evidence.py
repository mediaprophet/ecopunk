"""Read-only reconciliation of saved SDK evidence; does not run/build code."""
import hashlib
import json
from pathlib import Path
import re

ROUND = Path(__file__).resolve().parent
ROOT = ROUND.parents[5]


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def read(relative):
    return json.loads((ROUND / relative).read_text(encoding="utf-8-sig"))


receipt = read("applied-source.json")
corrections = read("evidence/doc-corrections/manifest.json")
by_target = {str(Path(row["target"])): row for row in corrections}
installed = []
for row in receipt:
    correction = by_target.get(str(Path(row["path"])))
    expected = correction["after_sha256"] if correction else row["after_sha256"]
    if correction:
        assert correction["before_sha256"] == row["after_sha256"]
    assert digest(row["path"]) == expected, row["path"]
    installed.append({"path": row["path"], "current_sha256": expected,
                      "post_test_documentation_change": bool(correction)})

for row in corrections:
    name = Path(row["target"]).name
    marker = b"pub mod axis_role;" if name == "mod.rs" else b"use bytemuck::{"
    before = (ROUND / "evidence/doc-corrections" / (name + ".before")).read_bytes()
    after = Path(row["target"]).read_bytes()
    assert hashlib.sha256(before).hexdigest() == row["before_sha256"]
    assert hashlib.sha256(after).hexdigest() == row["after_sha256"]
    before_body = before[before.index(marker):]
    after_body = after[after.index(marker):]
    assert before_body == after_body
    assert hashlib.sha256(after_body).hexdigest() == row["implementation_bytes_sha256"]

specs = read("evidence/spec-final-manifest.json")
for row in specs:
    assert digest(row["target"]) == row["after_sha256"], row["target"]
    assert digest(row["stage"]) == row["after_sha256"], row["stage"]

log = (ROUND / "native-conformance.log").read_text(encoding="utf-8-sig")
counts = [tuple(map(int, match)) for match in re.findall(
    r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored", log)]
assert counts == [(6, 0, 0), (7, 0, 0), (9, 0, 0)], counts
status = read("status-final.json")
for row in status["preserved_assets"]:
    assert digest(ROOT / row["path"]) == row["sha256"], row["path"]
assert all(row["status"] == "open" for row in status["foundation_gates"])
assert status["conformance"]["total"] == {"passed": 22, "failed": 0, "ignored": 0}
print(json.dumps({"status": "passed", "kind": "read_only_evidence_reconciliation",
    "installed_implementation_and_test_files": installed,
    "post_test_documentation_prefixes_verified": len(corrections),
    "implementation_bytes_unchanged_after_test": True,
    "canonical_spec_files_verified": len(specs), "unchanged_assets_verified": 3,
    "native_test_results": counts, "all_six_foundation_gates": "open",
    "builds_or_tests_executed_by_this_script": False}, indent=2))
