"""Save a reviewed follow-up manifest, preserving its source and install evidence."""
import hashlib
import json
import os
from pathlib import Path
import sys
import tempfile

ROOT = Path("C:/github/ecopunk")
ALLOWED = [Path("C:/github/qualiaDB").resolve(), Path("C:/github/poet").resolve()]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None


def main():
    manifest = Path(sys.argv[1]).resolve()
    if not manifest.is_relative_to(ROOT):
        raise ValueError("manifest must be in the reviewed workspace")
    data = json.loads(manifest.read_text(encoding="utf-8"))
    rows = data["files"] if "files" in data else data["patches"]
    receipt = manifest.with_suffix(".installed.json")
    resuming = sys.argv[2:] == ["--resume"]
    if receipt.exists() and not resuming:
        raise FileExistsError(receipt)
    prior = json.loads(receipt.read_text(encoding="utf-8")) if receipt.exists() else None
    if resuming and (prior is None or prior["files"] != rows):
        raise ValueError("resume requires this exact recorded installation")
    installed = list(prior["installed"]) if prior else []
    targets = set()
    for row in rows:
        target, candidate = Path(row["target"]).resolve(), Path(row["candidate"]).resolve()
        if target in targets or (target.suffix != ".rs" and target not in {Path("C:/github/qualiaDB/Cargo.lock").resolve(),Path("C:/github/poet/Cargo.lock").resolve(),Path("C:/github/qualiaDB/crates/qualia-cooperative-core/Cargo.toml").resolve()}) or not any(target.is_relative_to(p) for p in ALLOWED):
            raise ValueError("overlapping or unauthorised source target")
        targets.add(target)
        if not candidate.is_relative_to(ROOT) or sha(candidate) != row["after_sha256"]:
            raise ValueError(f"candidate changed: {candidate}")
        expected = row["after_sha256"] if str(target) in installed else row["before_sha256"]
        if sha(target) != expected:
            raise ValueError(f"source changed: {target}")
    try:
        for row in rows:
            target, candidate = Path(row["target"]), Path(row["candidate"])
            if str(target) in installed:
                continue
            target.parent.mkdir(parents=True, exist_ok=True)
            with tempfile.NamedTemporaryFile(prefix=".ecopunk-followup-", dir=target.parent, delete=False) as stream:
                temp = Path(stream.name)
                stream.write(candidate.read_bytes())
                stream.flush()
                os.fsync(stream.fileno())
            try:
                if sha(target) != row["before_sha256"]:
                    raise ValueError(f"concurrent source change: {target}")
                os.replace(temp, target)
            finally:
                temp.unlink(missing_ok=True)
            if sha(target) != row["after_sha256"]:
                raise ValueError(f"saved source differs: {target}")
            installed.append(str(target))
    finally:
        receipt.write_text(json.dumps({"manifest": str(manifest), "files": rows,
            "installed": installed, "verified": False}, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"saved": len(installed), "receipt": str(receipt)}))


if __name__ == "__main__":
    main()
