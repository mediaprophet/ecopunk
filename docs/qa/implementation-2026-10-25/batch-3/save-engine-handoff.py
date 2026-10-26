from pathlib import Path
import hashlib
import json
from datetime import datetime, timezone

root = Path(__file__).parent
target = Path('C:/github/qualiaDB/AGENTS.md')
expected = '89f13f6dc42d41947827e16b59a73816d0fbd3c2489ebafc4110aad9877cb900'
original = target.read_bytes()
assert hashlib.sha256(original).hexdigest() == expected, 'Preserve concurrent instructions; source changed'
assert not Path('C:/github/qualiaDB/HANDOVER.md').exists(), 'Use the existing handover instead'
for name in ['core-final-saved-tests.json', 'native-build/tests-saved-final.json',
             'native-build/material-pixels-final.json', 'native-build/library-saved-final.json',
             'native-build/wasm-saved-final.json']:
    assert json.loads((root / name).read_text(encoding='utf-8-sig'))['exit_code'] == 0, name
note = '''

### 2026-10-25 — Ecopunk native Poet/Craft authoring and engine failure repair

**Authority and preservation:** Timothy explicitly authorised saving QualiaDB and
Etalon changes after an earlier repository-boundary approval rejection. Reviewed
repairs were saved with original/candidate hashes and receipts. Existing user
changes and the running Poet window were preserved. No commits, pushes,
publication, payments, Computer Use or Chora source changes were made.

**Saved implementation:** canonical Vibe pre-promotion session validation;
host-bound cooperative authoring policy and thin MCP transport; durable exact
source/private-journal HMC checkpoints with fresh authorised activation; native
Poet shared-session authoring and material factors. Engine repairs cover Q42 v4
lexicon resolution, local graph SERVICE handling, honest C2PA verification status,
bounded CML parsing/roundtrip, explicitly authorised host effects, glTF rest/skin
admission, AO depth precision, and chemistry construction/prediction. Forge WGSL
consumers now emit the developed Etalon extension spellings; no Etalon source
defect was established. Invalid tests were corrected against actual contracts;
unsupported shader fallback paths now report unavailability instead of success.

**Executed verification on saved source snapshot e4e2e18c8cba:** full selected
native/GPU/Forge/privacy core suite 9,782 passed, zero failed, three explicitly
ignored dataset/integration probes. Native Poet 62 library plus 15 CLI tests
passed; its separate actual GPU material readback passed for base colour,
roughness, metallic and emissive edits. Native library build and WASM library
check passed. The actual source-linked CLI passed help, no-window source checking
and strict MCP framing/unbound-authority denial. All owned verification commands
finished. The final source audit reconciled 94 saved files and 120 guarded source
transitions. Evidence lives in C:/github/ecopunk/docs/qa/implementation-2026-10-25/batch-3.

**Remaining product work:** verified native authentication/admission-provider
binding and durable contribution/review commit; complete HMC graph/resources and
editable-master retention; usable contributor/reviewer graphical acceptance;
high-grade complete embodiments and reviewed game-demo derivatives. The checkpoint
profile is source-only and rejects overwriting richer resource packages. No GUI
or human acceptance, complete model, Vibe dialect release, universal Etalon
capability or mission-delivery claim is made. All 120 production packets remain
open. HANDOVER.md is absent, so this session note is recorded here under section 7.
'''
result = original + note.replace('\n', '\r\n').encode('utf-8')
target.write_bytes(result)
assert target.read_bytes() == result
receipt = dict(path=str(target), original_sha256=expected,
               saved_sha256=hashlib.sha256(result).hexdigest(),
               saved_utc=datetime.now(timezone.utc).isoformat(),
               scope='Append required session handoff; preserve existing instructions')
(root / 'engine-handoff-saved.json').write_text(json.dumps(receipt, indent=2)+'\n', encoding='utf-8')
print('Saved and hash-verified the QualiaDB session handoff.')
