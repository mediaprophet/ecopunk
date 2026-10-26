from pathlib import Path
import re,json,hashlib,datetime
base=Path(__file__).parent
def sha(file):
    h=hashlib.sha256()
    with Path(file).open('rb') as stream:
        for block in iter(lambda:stream.read(1024*1024),b''): h.update(block)
    return h.hexdigest()
def read(name): return (base/name).read_text(encoding='utf-8-sig')
def summaries(name):
    target=None;rows=[]
    for line in read(name).splitlines():
        if 'Running unittests' in line: target=line.strip().split(' (')[0]
        elif 'Running tests' in line or 'Running ..' in line: target=line.strip().split(' (')[0]
        elif 'Doc-tests' in line: target=line.strip()
        match=re.search(r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored;.*?(\d+) filtered out;',line)
        if match: rows.append(dict(target=target,status=match[1],passed=int(match[2]),failed=int(match[3]),ignored=int(match[4]),filtered=int(match[5])))
    assert rows,name
    return rows
first=summaries('actual-vibe-full-first.log')
assert len(first)==1 and first[0]['passed']==972 and first[0]['failed']==1
assert 'best took 1100ns' in read('actual-vibe-full-first.log')
passed={}
for name in ['actual-vibe-functional.log','actual-session-tests.log','actual-poet-tests.log']:
    rows=summaries(name);assert all(row['status']=='ok' and row['failed']==0 for row in rows),name
    passed[name]=dict(exit_code=0,passed=sum(row['passed'] for row in rows),ignored=sum(row['ignored'] for row in rows),filtered=sum(row['filtered'] for row in rows),targets=rows)
boundary={}
for file in sorted(base.glob('actual-boundary_*.log')):
    rows=summaries(file.name)
    assert len(rows)==1 and rows[0]['passed']==1 and rows[0]['failed']==0,file
    boundary[file.name]=dict(exit_code=0,sha256=sha(file))
assert len(boundary)==5
for name in ['actual-poet-build.log','actual-vibe-wasm-check.log']:
    assert 'Finished' in read(name) and 'error:' not in read(name),name
for name in ['actual-native-default-session.log','actual-native-granted-session.log']:
    assert 'VibeScript REPL' in read(name),name
assert 'immutable binding `fixed`' in read('actual-native-default-session.log')
assert 'session host has not granted capability `pulse.publish`' in read('actual-native-default-session.log')
assert read('actual-native-default-session.log').count('vibe> 8')==2
assert 'Local REPL grants: pulse.publish' in read('actual-native-granted-session.log')
assert read('actual-native-granted-session.log').count('call exceeds permitted effect class')==2
files=['actual-vibe-full-first.log','actual-vibe-functional.log','actual-session-tests.log',
    'actual-poet-tests.log','actual-poet-build.log','actual-vibe-wasm-check.log',
    'actual-native-default-session.log','actual-native-granted-session.log',
    'actual-source-verification.json','actual-boundary-stack-environment.log',
    'native-default-input.txt','native-granted-input.txt',
    'actual-harness/Cargo.toml','actual-harness/Cargo.lock','actual-harness/lib.rs','actual-harness/poet_adapter.rs']
files+=list(boundary)
actual=json.loads((base/'actual-source-verification.json').read_text())
for entry in actual['targets']: assert sha(entry['target'])==entry['sha256']
binary=base.parents[1]/'poet-repl/harness/target/debug/poet.exe'
assert binary.is_file(),binary
receipt=dict(status='actual canonical and Poet sources verified; timing gate remains failed',
    recorded_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    source_targets_verified=len(actual['targets']),source_maps=actual['maps'],
    first_unfiltered=dict(exit_code=101,unit_passed=972,unit_failed=1,
        failed_test='deontic_interrupt::tests::vc5_prohibition_breach_completes_under_1_microsecond',
        observed_best_ns=1100,required_strictly_less_ns=1000,
        log='actual-vibe-full-first.log'),
    passed_runs=passed,isolated_default_stack_boundary_cases=boundary,
    native_build=dict(exit_code=0,binary=str(binary.resolve()),sha256=sha(binary)),
    native_transcripts=dict(default_exit_code=0,explicit_grant_exit_code=0),
    wasm=dict(target='wasm32-unknown-unknown',command_exit_code=0,scope='actual Vibe library compile/check only; no browser/runtime conformance claim'),
    log_and_input_hashes={name:sha(base/name) for name in files},
    source_changes_during_verification=False,
    counts_are_per_run_not_summed=True,
    cli_compatibility=dict(language='vibe-0.1',host_abi='vibe-host-0.1',
        legacy_api='eval_line(line, host, Env) retained',
        persistent_entry='eval_session_line uses canonical SourceSession; interactive run_repl_with_grants uses it',
        default_grants=[],explicit_local_profile='poet repl --grant pulse.publish',
        authority='trusted local host profile only; source requests do not grant and external project/provider authority is not implied'),
    active_commands_at_handoff=[],
    limits=['single original timing gate failed and is explicitly filtered only in functional rerun',
        'bounded additive 0.1 sessions; no 0.2 release or complete type/language qualification',
        '8user calls,64total expression-evaluation depth,16delimiter groups/64syntax path units',
        'interpolation, replacement/nested-global/top-level-return sessions remain held as documented',
        'no process/allocator isolation claim; host effects require actual host reconciliation',
        'native CLI and library evidence do not satisfy graphical Poet/Craft cooperation/IP-03 C1-C8',
        'durable project checkpoints, jobs and model/high-grade output acceptance remain open'])
(base/'actual-verification.json').write_text(json.dumps(receipt,indent=2),encoding='utf8')
print(json.dumps(dict(source_targets=receipt['source_targets_verified'],runs={name:{k:value[k] for k in ['passed','ignored','filtered']} for name,value in passed.items()},boundaries=len(boundary),timing_gate='failed1100ns'),indent=2))
