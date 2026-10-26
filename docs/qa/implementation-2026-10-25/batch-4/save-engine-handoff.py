from pathlib import Path
from datetime import datetime,timezone
import hashlib,json
root=Path(__file__).parent
target=Path('C:/github/qualiaDB/AGENTS.md')
original=target.read_bytes()
expected='83618483abb5d42d92161f4f9e34873d3917c6565a9b1c964e55059121d04b27'
assert hashlib.sha256(original).hexdigest()==expected,'Existing session instructions changed'
for name in ['core-final.json','native-final.json','library-final.json','wasm-final.json','gpu-final.json']:
 record=json.loads((root/name).read_text(encoding='utf-8-sig'))
 assert record['exit_code']==0 and record['source_unchanged'] is True,name
note='''

### 2026-10-25 — RUN09 native authentication and resource-preserving Poet HMC editing

**Saved implementation:** canonical `poet_host::native_authority` verifies a
host-pinned project authority's signed admissions and a fresh one-use native
controller-key challenge before issuing a local QDNF execution grant. Current
qualified identifiers, membership, review scope, expiry and revocation govern
actions. An exclusively locked bounded native journal retains source successors,
contributions, source-only reviews and separately attributed package-write audit
records with expected/result HMC digests. These records never restore a live
session. One active principal/session per owner is supported in this scope.

Poet's NativeProject composes the real authority with one shared canonical
editor/session for native UI and MCP. HMC checkpoints retain all inert resources,
including unknown payloads/metadata and editable masters, across source edits,
reopen and Save As. Both native controls and MCP support explicit 1–64-entry
atomic resource plans, with package/resource SHA guards, preflight before imports,
one aggregate import budget and one canonical save. Known native texture and
10d/Q42 dependency refusals are implemented; explicit companion-graph updates
may retain identical authoritative bytes. Immutable origins are not overwritten.

**Executed on frozen source snapshot 130ad0c42d5e:** full selected Core suite
9,800 passed, zero failed, three ignored in 339.05 seconds. Native Poet 69 library
and 15 CLI tests passed, including signed project/source/HMC/MCP cooperation,
revocation and atomic native 10d/Q42 resource edits. The separately invoked actual
GPU material readback passed. Native library build, WASM library check and actual
source-linked CLI help/no-window source checking/MCP framing and unbound denial
passed. All commands finished and source hashes remained unchanged. Independent
source review covers 51 files; the final qualification snapshot pins 127 inputs.

**Limits and next work:** actual adopted project anchors/admissions and native
contributor/reviewer key enrollment/signing still require real host inputs. Test
actors are software-agent fixtures, not enrolled people or human reviewers.
Native graphical usability, whole-HMC semantic/rights-partition admission,
complete models and accepted high-grade masters/derivatives remain open. Source
review does not approve package resources. Unknown dependencies remain unassessed.
The journal is host-owned local evidence, not an external anti-rollback oracle;
cross-store persistence failures require reconciliation and withhold publication.
Windows power-loss directory durability remains unqualified. No GUI/Computer Use,
commits, pushes, payment, publication or Chora source changes occurred. All 120
production packets remain open. Evidence and remaining requirements are in
C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-4/README.md.
'''
result=original+note.replace('\n','\r\n').encode()
target.write_bytes(result)
assert target.read_bytes()==result
receipt=dict(path=str(target),original_sha256=expected,saved_sha256=hashlib.sha256(result).hexdigest(),saved_utc=datetime.now(timezone.utc).isoformat(),scope='Append session handoff without changing existing instructions')
(root/'engine-handoff-saved.json').write_text(json.dumps(receipt,indent=2)+'\n',encoding='utf-8')
print('Saved and hash-verified the QualiaDB RUN09 session handoff.')
