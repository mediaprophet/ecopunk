"""Direct-link actual Poet main with exact completed Cargo library artifacts.

No Cargo call, GUI launch, running-process intervention or sibling mutation.
Run only after the parent reports native library-final completed successfully.
"""
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
from datetime import datetime, timezone

HERE = Path(__file__).resolve().parent
BUILD = HERE.parent
WORKSPACE = Path("C:/github/ecopunk")
DEPS = WORKSPACE / "target/debug/deps"
MAIN = Path("C:/github/poet/native/src/main.rs")
OUTPUT = WORKSPACE / "target/debug/poet-batch3.exe"
CREATE_FLAGS = subprocess.CREATE_NO_WINDOW if os.name == "nt" else 0


def now():
    return datetime.now(timezone.utc).isoformat()


def save(name, value):
    (HERE / name).write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def sha(path):
    digest = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def normalized(path):
    return os.path.normcase(str(Path(path).resolve()))


def cargo_rows(path):
    raw = path.read_bytes()
    encoding = "utf-16" if raw[:2] in (b"\xff\xfe", b"\xfe\xff") else "utf-8-sig"
    rows = []
    non_json_lines = 0
    for line in raw.decode(encoding).splitlines():
        try:
            row = json.loads(line)
        except json.JSONDecodeError:
            non_json_lines += 1
            continue
        if isinstance(row, dict):
            rows.append(row)
    return rows, encoding, non_json_lines


def artifact(rows, package_name, target_name, source_path, required_features=()):
    candidates = {}
    for row in rows:
        target = row.get("target", {})
        package = row.get("package_id", "")
        if row.get("reason") != "compiler-artifact" or target.get("name") != target_name:
            continue
        if "lib" not in target.get("kind", []) or row.get("profile", {}).get("test"):
            continue
        if normalized(target.get("src_path", "")) != normalized(source_path):
            continue
        # Exact source path plus Cargo's package-name component; never select
        # newest modification time or an unrelated same-named dependency.
        component = package.rsplit("#", 1)[-1]
        package_uri = "path+" + Path(source_path).resolve().parent.parent.as_uri()
        if package.rsplit("#", 1)[0].casefold() != package_uri.casefold():
            continue
        # Cargo omits the name when it is implied by the package directory.
        # Exact owner URI and target/source still bind that abbreviated form.
        if component not in (package_name + "@0.0.40", "0.0.40"):
            continue
        if not set(required_features).issubset(row.get("features", [])):
            continue
        files = [path for path in row.get("filenames", []) if path.endswith(".rlib")]
        if len(files) != 1:
            raise RuntimeError(f"Expected exactly one rlib in matching {package_name} artifact")
        candidates[(normalized(files[0]), tuple(sorted(row.get("features", []))))] = row
    if len(candidates) != 1:
        raise RuntimeError(f"Expected one exact Cargo {package_name} lib artifact, found {len(candidates)}; refusing to guess")
    row = next(iter(candidates.values()))
    rlib = next(path for path in row["filenames"] if path.endswith(".rlib"))
    if not Path(rlib).is_file():
        raise RuntimeError(f"Selected {package_name} rlib is absent")
    return row, rlib


def child(argv, label, stdin=None, timeout=30):
    record = {"argv": argv, "started_utc": now(), "timeout_seconds": timeout,
              "creation_flags": CREATE_FLAGS, "gui_requested": False}
    process = subprocess.Popen(argv, cwd=WORKSPACE, stdin=subprocess.PIPE,
                               stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                               creationflags=CREATE_FLAGS, shell=False)
    record["owned_process_id"] = process.pid
    try:
        stdout, stderr = process.communicate(stdin, timeout=timeout)
        record["timed_out"] = False
    except subprocess.TimeoutExpired:
        # Only this invocation's owned child is stopped. No existing Poet PID
        # or user window is queried, signalled, closed or overwritten.
        process.kill()
        stdout, stderr = process.communicate(timeout=10)
        record["timed_out"] = True
    record.update(exit_code=process.returncode, finished_utc=now(),
                  stdout_bytes=len(stdout), stderr_bytes=len(stderr))
    if len(stdout) > 8 * 1024 * 1024 or len(stderr) > 8 * 1024 * 1024:
        raise RuntimeError("Owned child output exceeded evidence budget")
    (HERE / (label + ".stdout.log")).write_bytes(stdout)
    (HERE / (label + ".stderr.log")).write_bytes(stderr)
    record["stdout_file"] = label + ".stdout.log"
    record["stderr_file"] = label + ".stderr.log"
    save(label + ".json", record)
    if record["timed_out"] or process.returncode != 0:
        raise RuntimeError(f"Owned {label} child failed or timed out; inspect its receipt")
    return stdout.decode("utf-8"), stderr.decode("utf-8"), record


def main():
    global HERE, OUTPUT
    label = os.environ.get("POET_CARGO_LIBRARY_LABEL", "library-final")
    if not label.replace("-", "").isalnum():
        raise RuntimeError("Invalid Cargo evidence label")
    receipt_path = BUILD / (label + ".json")
    receipt = json.loads(receipt_path.read_text(encoding="utf-8-sig"))
    if receipt.get("exit_code") != 0:
        raise RuntimeError("Native library build receipt is not successful; no direct link attempted")
    snapshot = receipt.get("input_snapshot", {})
    snapshot_digest = snapshot.get("sha256", "")
    if len(snapshot_digest) != 64 or any(c not in "0123456789abcdef" for c in snapshot_digest):
        raise RuntimeError("Library receipt has no valid source snapshot digest")
    if sha(snapshot.get("path", "")) != snapshot_digest:
        raise RuntimeError("Library source snapshot receipt digest changed")
    # Immutable source-qualified artifacts let later phases coexist with f000.
    OUTPUT = WORKSPACE / ("target/debug/poet-batch3-" + snapshot_digest[:12] + ".exe")
    run_directory = HERE / snapshot_digest[:12]
    if run_directory.exists():
        raise RuntimeError("Source-qualified evidence directory exists; refusing to overwrite it")
    run_directory.mkdir()
    HERE = run_directory
    expected_args = receipt.get("arguments", [])
    if "--lib" not in expected_args or "--message-format=json" not in expected_args:
        raise RuntimeError("Library receipt does not describe the requested Cargo JSON library build")
    if OUTPUT.exists():
        raise RuntimeError("Direct output alias already exists; refusing to overwrite an unverified artifact")
    rows, encoding, non_json_lines = cargo_rows(BUILD / (label + ".log"))
    if not any(row.get("reason") == "build-finished" and row.get("success") is True for row in rows):
        raise RuntimeError("Cargo log has no successful build-finished event")
    poet_row, poet_rlib = artifact(rows, "poet-cli", "poet_cli", MAIN.with_name("lib.rs"), ["native-graphical"])
    vibe_row, vibe_rlib = artifact(rows, "vibe", "vibe", "C:/github/qualiaDB/crates/vibe/src/lib.rs")
    paths = sorted({path for row in rows if row.get("reason") == "build-script-executed"
                    for path in row.get("linked_paths", [])})
    argv = ["rustc", "--edition=2021", "--crate-name", "poet_batch3", str(MAIN),
            "--cfg", 'feature="native-graphical"', "--extern", "poet_cli=" + poet_rlib,
            "--extern", "vibe=" + vibe_rlib, "-L", "dependency=" + str(DEPS),
            "-C", "debuginfo=0", "-o", str(OUTPUT)]
    for path in paths:
        argv += ["-L", path]
    inputs = [MAIN, MAIN.with_name("format.rs"), MAIN.with_name("lint.rs"), MAIN.with_name("repl.rs"),
              Path(poet_rlib), Path(vibe_rlib)]
    pins = [{"path": str(path), "sha256": sha(path)} for path in inputs]
    version, _, _ = child(["rustc", "--version", "--verbose"], "rustc-version", timeout=10)
    save("selection.json", {"cargo_receipt": receipt, "cargo_log_sha256": sha(BUILD / (label + ".log")),
                           "cargo_log_encoding": encoding, "non_json_lines": non_json_lines,
                           "poet_artifact": poet_row, "vibe_artifact": vibe_row,
                           "all_build_script_linked_paths": paths, "input_pins": pins,
                           "rustc_version": version, "argv": argv,
                           "artifact_kind": "direct-linked actual native main, not a Cargo bin build"})
    child(argv, "link", timeout=300)
    for pin in pins:
        if sha(pin["path"]) != pin["sha256"]:
            raise RuntimeError("A direct-link input changed; resulting artifact is not qualified")
    save("artifact.json", {"path": str(OUTPUT), "sha256": sha(OUTPUT), "bytes": OUTPUT.stat().st_size,
                          "main_sha256": sha(MAIN), "poet_rlib_sha256": sha(poet_rlib),
                          "vibe_rlib_sha256": sha(vibe_rlib), "actual_source_entry": str(MAIN),
                          "cargo_bin_build": False, "running_user_executable_untouched": True,
                          "gui_launch_requested": False})
    help_stdout, _, _ = child([str(OUTPUT), "help"], "help", timeout=20)
    if "Poet CLI toolchain" not in help_stdout or "studio" not in help_stdout:
        raise RuntimeError("Actual CLI help output was not recognized")
    check_stdout, _, _ = child([str(OUTPUT), "studio", "--diagnostic-new-source", "--grant",
                               "capability.invoke", "--grant", "ComputationalGeometry.extrude_profile",
                               "--check-source"], "source-check", timeout=30)
    if "source preparation passed" not in check_stdout or "no evaluation/render/window attempted" not in check_stdout:
        raise RuntimeError("Actual diagnostic no-launch source preparation was not confirmed")
    messages = [
        {"jsonrpc":"2.0","id":"init","method":"initialize","params":{
            "protocolVersion":"2025-11-25","capabilities":{},
            "clientInfo":{"name":"batch3-headless-verification","version":"0.0.0"}}},
        {"jsonrpc":"2.0","method":"notifications/initialized"},
        {"jsonrpc":"2.0","id":"tools","method":"tools/list","params":{}},
        {"jsonrpc":"2.0","id":"read","method":"tools/call","params":{"name":"poet_source_read","arguments":{}}},
    ]
    payload = "".join(json.dumps(message, separators=(",", ":")) + "\n" for message in messages).encode("utf-8")
    (HERE / "mcp-input.jsonl").write_bytes(payload)
    mcp_stdout, _, _ = child([str(OUTPUT), "mcp"], "mcp", stdin=payload, timeout=30)
    responses = [json.loads(line) for line in mcp_stdout.splitlines()]
    if len(responses) != 3 or [row.get("id") for row in responses] != ["init", "tools", "read"]:
        raise RuntimeError("MCP emitted unexpected responses or notification output")
    if any(row.get("jsonrpc") != "2.0" for row in responses):
        raise RuntimeError("MCP stdout contains non-protocol output")
    if responses[0].get("result", {}).get("protocolVersion") != "2025-11-25":
        raise RuntimeError("MCP negotiated an unexpected protocol version")
    tools = responses[1].get("result", {}).get("tools", [])
    names = [tool["name"] for tool in tools]
    if "poet_source_read" not in names or "poet_hmc_save" in names:
        raise RuntimeError("Unbound MCP tool catalog was not honest about read/save availability")
    denied = responses[2].get("result", {})
    if denied.get("isError") is not True or denied.get("structuredContent", {}).get("error", {}).get("code") != "authority_unavailable":
        raise RuntimeError("Unbound MCP source read did not deny authenticated authority")
    save("checks.json", {"exit_code":0,"help":True,"diagnostic_check_no_evaluation_or_window":True,
                         "mcp_stdout_strict_jsonrpc":True,"initialized_notification_silent":True,
                         "mcp_unbound_source_read_denied":True,"mcp_advertised_tools":names,
                         "production_authority_supplied":False,"gui_launched":False,
                         "artifact_sha256":sha(OUTPUT),"checked_utc":now()})
    print("Direct actual-source CLI linked; help, no-launch source check, strict MCP/unbound denial passed.")


if __name__ == "__main__":
    try:
        main()
    except Exception as error:
        save("failure.json", {"time_utc":now(),"error":str(error),"cargo_rebuild_attempted":False,
                              "user_gui_process_touched":False})
        print(str(error), file=sys.stderr)
        raise SystemExit(1)
