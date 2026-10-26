"""Serial bounded native checks. All output/evidence goes to biological-reference QA."""
from pathlib import Path
import datetime,hashlib,json,os,re,subprocess,sys,tempfile,time
ROOT=Path('C:/github/ecopunk');OUT=ROOT/'docs/qa/implementation-2026-10-26/biological-reference'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def snapshot():
    files=set()
    bases=[Path('C:/github/qualiaDB/crates')/n for n in ['qualia-core-db','qualia-cooperative-core','qualia-geospatial','vibe']]+[Path('C:/github/poet/native'),Path('C:/github/poet/crates'),ROOT/'tools/format-sdk-wasm-check']
    for base in bases:
        for pattern in ['*.rs','*.wgsl','*.toml','Cargo.lock']:
            files.update(base.rglob(pattern))
    files.update((ROOT/'tools/poet-foundation/biological-reference/fixtures').glob('*.vibe'))
    for owner in ['poet','qualiaDB']:
        for name in ['Cargo.toml','Cargo.lock']:
            files.add(Path('C:/github')/owner/name)
    return {str(p):sha(p) for p in sorted(files)}
mode=sys.argv[1];attempt=sys.argv[2] if len(sys.argv)>2 else 'final'
assert re.fullmatch(r'[a-z0-9-]+',attempt)
base=['--manifest-path','C:/github/poet/Cargo.toml','-p','poet-cli','--features','native-graphical','--locked','--offline','--target-dir',str(ROOT/'target')]
filters={'biology-host':'graphical::geometry_host::', 'biology-render':'graphical::biological_reference_tests::native_branching_diagnostics', 'authoring':'graphical::authoring::','draft':'graphical::draft_buffer::tests::','jobs':'graphical::jobs::tests::','checkpoints':'graphical::checkpoints::','surfaces':'graphical::surfaces::integration_tests::','surface-inputs':'graphical::surfaces::tests::','workflow':'uv_atlas_paint_native_intents_resource_closure_review_and_reopen'}
if mode=='biology-render': command=['cargo','test',*base,'--lib',filters[mode],'--','--ignored','--test-threads=1']
elif mode in filters: command=['cargo','test',*base,'--lib',filters[mode],'--','--test-threads=1']
elif mode=='viewport':command=['cargo','test',*base,'--lib','graphical::viewport','--','--test-threads=1','--skip','r9_native_gpu']
elif mode=='gpu':command=['cargo','test',*base,'--lib','r9_native_gpu','--','--ignored','--test-threads=1']
elif mode=='workbench':command=['cargo','test',*base,'--lib','graphical::cooperative_workbench','--','--test-threads=1']
elif mode=='input':command=['cargo','test',*base,'--lib','graphical::viewport_input','--','--test-threads=1']
elif mode=='profile':command=['cargo','test',*base,'--lib','graphical::native_authoring_profile','--','--test-threads=1']
elif mode=='intents':command=['cargo','test',*base,'--lib','graphical::poet_workspace::tests::','--','--test-threads=1']
elif mode=='lookdev':command=['cargo','test',*base,'--lib','graphical::lookdev_controls','--','--test-threads=1']
elif mode=='reference':command=['cargo','test',*base,'--lib','graphical::reference_authoring_tests::r9_reference_master_source_layers_viewport_and_fresh_hmc','--','--ignored','--exact','--test-threads=1']
elif mode=='check':command=['cargo','check',*base,'--lib','--tests']
elif mode=='build':command=['cargo','build',*base,'--bin','poet']
elif mode=='carrier':command=['cargo','test',*base,'--test','foundation_source_carrier','--','--test-threads=1']
elif mode=='wasm':command=['cargo','check','--manifest-path',str(ROOT/'tools/format-sdk-wasm-check/Cargo.toml'),'--locked','--offline','--target','wasm32-unknown-unknown','--target-dir',str(ROOT/'target')]
else:raise ValueError(mode)
folder=OUT/attempt;folder.mkdir(parents=True,exist_ok=True)
source=folder/(mode+'-source-before.json');logpath=folder/(mode+'.log');result=folder/(mode+'-result.json')
assert not any(p.exists() for p in [source,logpath,result]), 'Retain prior attempts; choose a new label'
before=snapshot();source.write_text(json.dumps(before,indent=2)+'\n')
env=os.environ.copy();settings=dict(CARGO_BUILD_JOBS='2',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',CARGO_PROFILE_DEV_INCREMENTAL='false',CARGO_PROFILE_TEST_INCREMENTAL='false');env.update(settings)
if mode in ['reference','biology-render']:
    artifacts=folder/'reference-output';artifacts.mkdir()
    env['POET_REFERENCE_QUALIFICATION_ARTIFACTS']=str(artifacts.resolve())
    env['POET_BIOLOGICAL_DIAGNOSTICS']=str(artifacts.resolve())
    settings['POET_REFERENCE_QUALIFICATION_ARTIFACTS']=str(artifacts.resolve())
    settings['POET_BIOLOGICAL_DIAGNOSTICS']=str(artifacts.resolve())
start=datetime.datetime.now(datetime.timezone.utc).isoformat();clock=time.monotonic()
with tempfile.TemporaryDirectory(prefix='poet-biological-',dir=ROOT/'target') as scratch:
    env.update(TEMP=scratch,TMP=scratch)
    with logpath.open('w',encoding='utf-8') as log:
        p=subprocess.Popen(command,cwd=ROOT,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,encoding='utf-8',errors='replace')
        for line in p.stdout:log.write(line);log.flush()
        code=p.wait()
    after=snapshot();changed=[p for p in sorted(set(before)|set(after)) if before.get(p)!=after.get(p)]
    log=logpath.read_text();cases=re.findall(r'^test (\S+) \.\.\. ok$',log,re.M)
    counts=[list(map(int,m)) for m in re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored',log)]
    selected=mode in ['check','build','wasm'] or (len(cases)>0 and counts and all(f==0 and i==0 for _,f,i in counts))
    receipt=dict(command=command,mode=mode,attempt=attempt,exit_code=code,started_utc=start,finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),elapsed_seconds=round(time.monotonic()-clock,3),log=str(logpath),log_sha256=sha(logpath),source_snapshot=str(source),source_snapshot_sha256=sha(source),recorded_local_source_files=len(before),source_unchanged=not changed,source_changes=changed,test_selection_ok=bool(selected),test_results=counts,passed_case_names=cases,build_environment=settings,boundary='Software fixtures; not native GUI/human acceptance or WASM runtime execution. Local recorded sources and locks, not every transitive dependency.')
    result.write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(dict(mode=mode,exit_code=code,tests=counts,source_unchanged=not changed,receipt=str(result))),flush=True)
    if code:print(log[-14000:])
sys.exit(code if code else int(bool(changed) or not selected))
