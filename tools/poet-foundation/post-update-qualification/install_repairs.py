"""Guarded coordinator promotion of reviewed source candidates; no build or commit."""
import hashlib
import json
from pathlib import Path
import sys
import tempfile
import os

ROOT = Path("C:/github/ecopunk")
BASE = ROOT / "tools/poet-foundation/post-update-qualification/repairs"
EVIDENCE = ROOT / "docs/qa/implementation-2026-10-27/post-update-qualification"
ALLOWED = (Path("C:/github/qualiaDB").resolve(), Path("C:/github/poet").resolve())


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None


def candidates():
    specifications = [
        (BASE / "runtime/manifest.json", "files", "original_path", "original_sha256", "candidate_path"),
        (BASE / "store/source-hashes.json", "files", "canonical_source", "source_original_sha256", "candidate"),
        (BASE / "quality/source-candidate-hashes.json", "candidates", "canonical_source", "source_original_sha256", "candidate"),
    ]
    result, seen = [], set()
    for manifest, key, destination_key, original_key, candidate_key in specifications:
        data = json.loads(manifest.read_text(encoding="utf-8"))
        for entry in data[key]:
            if entry.get(destination_key) is None:
                if Path(entry[candidate_key]).suffix != ".md":
                    raise ValueError("non-document candidate has no canonical target")
                continue
            target = Path(entry[destination_key]).resolve()
            # Track README files are already workspace artifacts, not sibling source.
            if target.suffix != ".rs":
                continue
            if not any(target.is_relative_to(root) for root in ALLOWED):
                raise ValueError(f"source outside authorised roots: {target}")
            candidate = Path(entry[candidate_key])
            if not candidate.is_absolute():
                candidate = ROOT / candidate
            candidate = candidate.resolve()
            if not candidate.is_relative_to(BASE.resolve()):
                raise ValueError(f"candidate outside staging: {candidate}")
            if str(target).casefold() in seen:
                raise ValueError(f"overlapping workers: {target}")
            seen.add(str(target).casefold())
            if digest(candidate) != entry["candidate_sha256"]:
                raise ValueError(f"candidate changed after manifest: {candidate}")
            expected = entry[original_key]
            if digest(target) != expected:
                raise ValueError(f"canonical source changed; preserve/review before promotion: {target}")
            result.append({"target": str(target), "candidate": str(candidate),
                           "before_sha256": expected, "after_sha256": digest(candidate),
                           "manifest": str(manifest)})
    return result


def main():
    rows = candidates()
    if sys.argv[1:] != ["--apply"]:
        print(json.dumps({"state": "preflight", "files": len(rows), "writes": False}))
        return
    receipt = {"state": "in_progress", "files": rows, "builds_run": False, "committed": False}
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    path = EVIDENCE / "repair-install.json"
    if path.exists():
        raise FileExistsError("installation receipt exists; retain it and review successor scope")
    path.write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
    installed = []
    try:
        for row in rows:
            target, candidate = Path(row["target"]), Path(row["candidate"])
            if digest(target) != row["before_sha256"]:
                raise ValueError(f"concurrent source change: {target}")
            target.parent.mkdir(parents=True, exist_ok=True)
            with tempfile.NamedTemporaryFile(prefix=".ecopunk-repair-", dir=target.parent, delete=False) as stream:
                temporary = Path(stream.name)
                stream.write(candidate.read_bytes())
                stream.flush()
                os.fsync(stream.fileno())
            try:
                if digest(target) != row["before_sha256"]:
                    raise ValueError(f"concurrent source change before replacement: {target}")
                os.replace(temporary, target)
            finally:
                if temporary.exists():
                    temporary.unlink()
            if digest(target) != row["after_sha256"]:
                raise ValueError(f"installed bytes differ: {target}")
            installed.append(str(target))
        receipt["state"] = "saved_not_yet_verified"
    finally:
        receipt["installed"] = installed
        path.write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"state": receipt["state"], "files": len(installed)}))


if __name__ == "__main__":
    main()
