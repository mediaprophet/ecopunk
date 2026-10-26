"""Append the required repository handoff after final evidence checks pass."""
from pathlib import Path
import argparse
import hashlib
import json
import re

root = Path(__file__).resolve().parent
parser = argparse.ArgumentParser()
parser.add_argument("--expected-sha", required=True)
args = parser.parse_args()


def read(path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


def sha(data):
    return hashlib.sha256(data).hexdigest()


audit = read(root / "final-verification.json")
pointer = read(root / "latest-source-snapshot.json")
assert audit["source_snapshot"]["sha256"] == pointer["sha256"]
assert audit["live_builds"] == 0 and audit["human_acceptance_claimed"] is False
core = re.findall(r"test result: ok\. (\d+) passed; 0 failed; (\d+) ignored;[^\r\n]+", (root / "core-final.log").read_text(encoding="utf-8-sig"))[-1]
native = re.findall(r"test result: ok\. (\d+) passed; 0 failed; (\d+) ignored;[^\r\n]+", (root / "native-final.log").read_text(encoding="utf-8-sig"))
target = Path("C:/github/qualiaDB/AGENTS.md")
before = target.read_bytes()
assert sha(before).lower() == args.expected_sha.lower(), "Handoff file changed; inspect before append"
assert b"RUN10 native contributor enrollment and Poet review" not in before
addition = f"""

### RUN10 native contributor enrollment and Poet review — 2026-10-25

Saved canonical signed enrollment requests, explicit steward decisions and
separate source-review grants, native key-owner signing, retained admission and
fresh login. Strict Windows native vault selection has no plaintext fallback.
Poet now exposes `studio --native-project EXISTING_DIRECTORY` in its native
operator shell, with explicit fingerprint adoption/profile creation/login and
contributor/reviewer controls sharing one authority/session/editor owner.
Contributions and decisions target exact source revisions; actor handoff
preserves private drafts and rejects stale replies. Accepted-source replacement
requires confirmation. This remains source-only review, not whole-HMC acceptance.

Windows checkpoint installation now retries only bounded OS 5/32/33 refusals,
retaining the staged file and writer lock and rechecking expected destination
bytes before every attempt. Real deny-delete tests cover transient release,
persistent refusal and external destination changes. An earlier full-suite
OS5 failure and the initial child-selector failure remain in the QA history;
the original OS5 cause is unproven, and no OS protection was disabled.
The network-restricted full run is also preserved. The identical hashed
loopback test failed inside the sandbox and passed with native socket access;
the final full-suite run uses that native boundary without skips or source edits.

Executed on frozen snapshot {pointer['sha256']}:
selected Core {core[0]} passed, zero failed, {core[1]} ignored;
native Poet {native[0][0]} library tests and {native[1][0]} CLI tests passed.
Native library build, WASM library check, actual source-linked CLI help,
public-config-only entry and MCP checks, plus actual GPU material readback
passed. Independent saved-source review and final receipt hashes are recorded
under C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-5/README.md.

Tests use software-agent fixtures. Real OS credential-provider execution and
persistence, external anchor adoption, human verification/enrollment and native
GUI acceptance remain unqualified. Fresh-process encrypted-key possession/login
is exercised; full operator HMC/source recovery remains separate. IP-03 C1–C8,
complete HMC semantics/model dossiers, high-grade masters and reviewed film,
open-world game and cartoon derivatives remain open. All 120 production packets
remain open. No GUI/Computer Use, commits, pushes, payment, publication or Chora
source changes were performed. Existing user work and running Poet were preserved.
"""
(root / "AGENTS-before-RUN10.md").write_bytes(before)
assert target.read_bytes() == before
with target.open("ab") as stream:
    stream.write(addition.replace("\n", "\r\n").encode("utf-8"))
after = target.read_bytes()
assert after == before + addition.replace("\n", "\r\n").encode("utf-8")
receipt = dict(path=str(target), before_sha256=sha(before), after_sha256=sha(after),
               input_snapshot=pointer, append_only=True, prior_content_preserved=True)
(root / "handoff-saved.json").write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
print(json.dumps(receipt))
