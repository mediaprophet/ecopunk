from pathlib import Path
import json,re,hashlib,subprocess,time,datetime,os
root=Path(__file__).parent
triage=json.loads((root/'core-failure-triage.json').read_text())
log=(root/'core-library-final-retry-tests.log').read_text(errors='replace')
match=re.search(r'Running unittests[^\r\n]*\(([^\r\n()]*qualia_core_db-[0-9a-f]+\.exe)\)',log)
assert match,'Current canonical test executable has not been built'
binary=(Path('C:/github/ecopunk')/match.group(1)).resolve()
assert binary.is_file() and binary.parent==Path('C:/github/ecopunk/target/debug/deps').resolve()
targets=[f['name'] for f in triage['failures'] if f['name'].startswith(('net::','p2p::'))]
assert len(targets)==21 and len(set(targets))==21
folder=root/'network-qualification'
folder.mkdir(exist_ok=True)
results=[]
env=os.environ.copy()
env['TMP']=env['TEMP']='C:/github/ecopunk/target/qa-tmp'
for index,name in enumerate(targets):
    assert re.fullmatch(r'[a-zA-Z0-9_:]+',name)
    start=time.monotonic()
    try:
        process=subprocess.run([str(binary),'--exact',name,'--nocapture','--test-threads=1'],
            capture_output=True,timeout=45,env=env,creationflags=subprocess.CREATE_NO_WINDOW)
        output=process.stdout+process.stderr
        code=process.returncode
    except subprocess.TimeoutExpired as exc:
        output=(exc.stdout or b'')+(exc.stderr or b'')+b'\nOWNED TEST TIMED OUT\n'
        code='timeout'
    path=folder/f'{index+1:02d}.log'
    path.write_bytes(output)
    row=dict(name=name,exit_code=code,duration_seconds=round(time.monotonic()-start,3),log=path.name,
        log_sha256=hashlib.sha256(output).hexdigest())
    results.append(row)
    print(json.dumps(row),flush=True)
payload=dict(scope='Inspected localhost-only native transport cases, same compiled core executable, outside the socket-restricted command sandbox',
    captured_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),binary=str(binary),
    binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),tests=results,
    passed=sum(r['exit_code']==0 for r in results),failed=sum(r['exit_code']!=0 for r in results))
(folder/'results.json').write_text(json.dumps(payload,indent=2))
raise SystemExit(0 if payload['failed']==0 else 1)
