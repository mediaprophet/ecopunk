"""Hash-pinned installation of the separately reviewed Poet-only inspector."""
import hashlib
import json
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent / "poet-inspection"
OWNER = Path("C:/github/poet").resolve()
if len(sys.argv) == 3:
    assert sys.argv[2] in ("repair", "repair-2")
    HERE = HERE / sys.argv[2]


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


manifest = HERE / "target-manifest.json"
assert len(sys.argv) in (2, 3) and sha(manifest) == sys.argv[1]
rows = json.loads(manifest.read_text())
for row in rows:
    target, stage = Path(row["target"]), HERE / row["after_snapshot"]
    assert target.resolve().is_relative_to(OWNER)
    assert stage.resolve().is_relative_to(HERE)
    assert sha(stage) == row["after_sha256"], str(stage)
    assert (sha(target) if target.exists() else None) == row["before_sha256"], str(target)
receipt = []
for row in rows:
    target, stage = Path(row["target"]), HERE / row["after_snapshot"]
    assert (sha(target) if target.exists() else None) == row["before_sha256"]
    backup = HERE / "installed-before" / target.resolve().relative_to(OWNER)
    if target.exists():
        backup.parent.mkdir(parents=True, exist_ok=True)
        backup.write_bytes(target.read_bytes())
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(stage.read_bytes())
    assert sha(target) == row["after_sha256"]
    receipt.append({**row, "verified_after_sha256": sha(target)})
    (HERE / "applied-source.json").write_text(json.dumps(receipt, indent=2), encoding="utf-8")
print(json.dumps({"applied": len(receipt), "all_after_hashes_verified": True,
                  "manifest_sha256": sha(manifest)}))
