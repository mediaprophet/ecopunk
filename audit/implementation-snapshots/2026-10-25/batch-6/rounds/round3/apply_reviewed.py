"""Exact-target install; run only after supervisor review and write approval."""
import hashlib
import json
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
OWNER = Path("C:/github/qualiaDB").resolve()
manifest = HERE / "install-manifest.json"


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


assert len(sys.argv) == 2 and sha(manifest) == sys.argv[1], "Manifest pin required"
rows = json.loads(manifest.read_text(encoding="utf-8"))
for row in rows:
    target, stage = Path(row["target"]), Path(row["stage"])
    assert target.resolve().is_relative_to(OWNER)
    assert stage.resolve().is_relative_to(HERE)
    assert sha(stage) == row["after_sha256"], str(stage)
    assert (sha(target) if target.exists() else None) == row["before_sha256"], str(target)

# Whole set is checked before any write; preserve source bytes alongside receipt.
receipt = []
for row in rows:
    target, stage = Path(row["target"]), Path(row["stage"])
    assert (sha(target) if target.exists() else None) == row["before_sha256"], str(target)
    backup = HERE / "integration/before" / target.resolve().relative_to(OWNER)
    if target.exists():
        backup.parent.mkdir(parents=True, exist_ok=True)
        backup.write_bytes(target.read_bytes())
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(stage.read_bytes())
    assert sha(target) == row["after_sha256"], str(target)
    receipt.append({**row, "verified_after_sha256": sha(target)})
    (HERE / "applied-source.json").write_text(json.dumps(receipt, indent=2), encoding="utf-8")
print(json.dumps({"applied": len(receipt), "all_after_hashes_verified": True,
                  "manifest_sha256": sha(manifest)}))
