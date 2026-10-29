"""Finish packaging an already compiled WASM using the installed matching CLI."""
from pathlib import Path
import hashlib
import json
import subprocess

root = Path(__file__).resolve().parents[2]
evidence = root / "docs/qa/implementation-2026-10-29/species-production-round1"
record_path = sorted(evidence.glob("wasm-build-*.json"))[-1]
record = json.loads(record_path.read_text())
log = Path(record["log"]).read_text()
if "Finished `release` profile" not in log or "Installing wasm-bindgen with cargo" not in log:
    raise SystemExit("Not a completed compilation with the known binding installer failure")
cli = Path("C:/Users/tim_h/AppData/Local/.wasm-pack/wasm-bindgen-ba0b07921e305a69/wasm-bindgen.exe")
version = subprocess.check_output([str(cli), "--version"], text=True).strip()
if version != "wasm-bindgen 0.2.129":
    raise SystemExit("Installed binding CLI version does not match this lockfile")
binary = Path(record["target"]) / "wasm32-unknown-unknown/release/ecopunk_wasm.wasm"
command = [str(cli), "--target", "web", "--typescript", "--out-dir", record["package"],
           "--out-name", "ecopunk_wasm", str(binary)]
result = subprocess.run(command, capture_output=True, text=True)
recovery = dict(build_record=record_path.name, command=command, version=version,
                input_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                exit_code=result.returncode, stdout=result.stdout, stderr=result.stderr,
                scope="Local binding packaging recovery; original wasm-pack failure retained")
if result.returncode == 0:
    recovery["outputs"] = {name:hashlib.sha256((Path(record["package"])/name).read_bytes()).hexdigest()
        for name in ("ecopunk_wasm.js", "ecopunk_wasm_bg.wasm", "ecopunk_wasm.d.ts")}
output = evidence / record_path.name.replace("wasm-build-", "wasm-packaging-")
output.write_text(json.dumps(recovery, indent=2)+"\n")
print(json.dumps(recovery, indent=2))
raise SystemExit(result.returncode)
