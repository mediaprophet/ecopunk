"""Apply exact preimage-pinned Poet files authorized by the current task."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, os, tempfile
ROOT=Path("C:/github/ecopunk").resolve()
TARGET=Path("C:/github/poet").resolve()
STAGE=ROOT/"tools/poet-foundation/motion-integration/work/integration"
RECEIPT=ROOT/"docs/qa/implementation-2026-10-26/poet-motion-integration/applied-source.json"
manifest=json.loads((STAGE/"install-manifest.json").read_text(encoding="utf-8"))
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
prepared=[]
for row in manifest["files"]:
    dst=(TARGET/row["path"]).resolve()
    if not dst.is_relative_to(TARGET) or ".git" in dst.parts: raise RuntimeError("Invalid destination")
    src=Path(row["staged"]).resolve()
    if not src.is_relative_to(STAGE) or sha(src)!=row["sha256_after"]: raise RuntimeError("Staged source changed")
    actual=sha(dst) if dst.is_file() else None
    if actual!=row["sha256_before"] and actual!=row["sha256_after"]: raise RuntimeError("Canonical preimage changed: "+str(dst))
    prepared.append((row,src,dst,actual))
result={"date_utc":datetime.now(timezone.utc).isoformat(),"scope":manifest["scope"],"files":[],"completed":False}
RECEIPT.parent.mkdir(parents=True,exist_ok=True)
for row,src,dst,actual in prepared:
    if actual!=row["sha256_after"]:
        if actual is not None:
            backup=STAGE/"preimages"/actual/row["path"]
            backup.parent.mkdir(parents=True,exist_ok=True)
            if backup.exists() and sha(backup)!=actual: raise RuntimeError("Backup collision")
            backup.write_bytes(dst.read_bytes())
        dst.parent.mkdir(parents=True,exist_ok=True)
        with tempfile.NamedTemporaryFile(dir=dst.parent,prefix=".poet-motion-",delete=False) as f:
            f.write(src.read_bytes());f.flush();os.fsync(f.fileno());pending=Path(f.name)
        if (sha(dst) if dst.is_file() else None)!=actual: raise RuntimeError("Concurrent source change; staged file retained")
        os.replace(pending,dst)
    result["files"].append({"path":str(dst),"sha256_before":row["sha256_before"],"sha256_after":sha(dst)})
    RECEIPT.write_text(json.dumps(result,indent=2)+"\n",encoding="utf-8",newline="\n")
result["completed"]=True
RECEIPT.write_text(json.dumps(result,indent=2)+"\n",encoding="utf-8",newline="\n")
print("Saved "+str(len(result["files"]))+" reviewed Poet source/notice files; "+str(RECEIPT))
