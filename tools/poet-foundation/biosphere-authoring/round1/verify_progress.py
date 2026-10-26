"""Read-only source/document/result checks; never launches Cargo or grants rights."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib
import json
import re

ROOT = Path('C:/github/ecopunk')
QA = ROOT / 'docs/qa/implementation-2026-10-26/biosphere-authoring-round1'

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def result(name, required=False):
    path = QA / name
    if not path.exists():
        return {'state':'not_run'}
    text = path.read_text(encoding='utf-8', errors='replace')
    matches = re.findall(r'test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', text)
    if not matches:
        return {'state':'no_complete_result', 'log':str(path), 'log_sha256':sha(path)}
    status, passed, failed, ignored = matches[-1]
    passed, failed, ignored = map(int, (passed, failed, ignored))
    if required:
        assert status == 'ok' and passed > 0 and failed == 0, name
    return {'state':'passed' if status == 'ok' and passed > 0 and failed == 0 else 'not_passed',
            'passed':passed, 'failed':failed, 'ignored':ignored, 'log':str(path), 'log_sha256':sha(path)}

receipts = ['auth-installed.json','auth-privacy-installed.json','poet-installed.json']
receipts += [p.name for p in sorted(QA.glob('poet-corrections-installed*.json'))]
files = {}
for name in receipts:
    data = json.loads((QA/name).read_text(encoding='utf-8'))
    assert data['completed'], name
    for row in data['files']:
        files[row['path']] = row['after']
integrity = [{'path':p, 'expected':expected, 'actual':sha(Path(p))} for p,expected in files.items()]
assert all(row['expected'] == row['actual'] for row in integrity), 'Canonical source changed after save'
(QA/'source-integrity.json').write_text(json.dumps({'passed':True,
    'scope':'Saved canonical files only, not execution or acceptance evidence',
    'files':[dict(row,matches=True) for row in integrity]},indent=2)+'\n',encoding='utf-8')

documents = [ROOT/'docs/planning/biosphere-authoring-round1.md',
             ROOT/'docs/planning/tech/human-principal-agent-authority.md', QA/'README.md']
links = []
for doc in documents:
    text = doc.read_text(encoding='utf-8')
    assert '\ufffd' not in text, doc
    for target in re.findall(r'\]\(([^)]+)\)', text):
        if target.startswith(('http:','https:','#')):
            continue
        target = target.strip('<>').split('#')[0]
        target = re.sub(r':\d+$', '', target)
        path = Path(target)
        if not path.is_absolute():
            path = doc.parent/path
        links.append({'document':str(doc),'target':target,'exists':path.exists()})
assert all(row['exists'] for row in links), [r for r in links if not r['exists']]
for name in ['biosphere-work-packets.json','implementation-queue.json']:
    json.loads((ROOT/'docs/planning'/name).read_text(encoding='utf-8'))

report = {'date_utc':datetime.now(timezone.utc).isoformat(),
          'scope':'Engineering result and saved-source consistency, not human/domain or model acceptance',
          'asset_production':'HOLD', 'hierarchy_auth_h1_h6':'open',
          'core_authority':result('auth-focused-final.log',True),
          'core_regression':result('core-regression.log',True),
          'core_regression_input_qualification':'Historical compiled-harness pass; Vibe source changed outside this round after that harness build. Do not infer a current entire-stack pass.',
          'poet_library':result('poet-library-final.log'),
          'poet_attempts':{name:result(name) for name in [
              'poet-library-attempt-1.log','poet-library-attempt-2.log',
              'poet-library-attempt-3-native.log','poet-library-attempt-4-native.log',
              'poet-library-attempt-5-native.log']},
          'excluded_zero_test_attempt':result('auth-attempt-2.log'),
          'saved_files':integrity,'checked_document_links':links}
(QA/'verification.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'saved_files':len(integrity),'document_links':len(links),
                  'core_authority':report['core_authority']['state'],
                  'core_regression':report['core_regression']['state'],
                  'poet_library':report['poet_library']['state'],'asset_production':'HOLD'}))
