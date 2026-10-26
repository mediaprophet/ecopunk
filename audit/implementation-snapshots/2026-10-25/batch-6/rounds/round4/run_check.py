"""Serial actual-source checks with explicit case selection and scoped scratch."""
from pathlib import Path
import datetime,hashlib,json,os,re,subprocess,sys,tempfile,time

HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[5]
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def snapshot():
    files=set()
    for base in [Path('C:/github/qualiaDB/crates')/name for name in ['qualia-core-db','qualia-cooperative-core','qualia-geospatial']]+[Path('C:/github/poet/native'),Path('C:/github/poet/crates'),ROOT/'tools/format-sdk-wasm-check']:
        for pat in ['*.rs','*.wgsl','*.toml','Cargo.lock']:files.update(base.rglob(pat))
    for base in [Path('C:/github/poet'),Path('C:/github/qualiaDB')]:
        for name in ['Cargo.toml','Cargo.lock']:
            if (base/name).is_file():files.add(base/name)
    return {str(p):sha(p) for p in sorted(files)}

mode=sys.argv[1]
minimum={'public':19,'producer':9,'workflow':1,'source-regression':1,'inspect-regression':6}
base=['--manifest-path','C:/github/poet/Cargo.toml','-p','poet-cli','--features','native-graphical','--locked','--offline','--target-dir',str(ROOT/'target')]
if mode=='check':command=['cargo','check',*base,'--lib','--tests']
elif mode=='public':command=['cargo','test',*base,'--test','foundation_package_policy','--test','foundation_native_package_review','--','--test-threads=1','--nocapture']
elif mode in ['producer','workflow','source-regression','inspect-regression']:
    filters={'producer':'georaster','workflow':'package_workflow_real_producer_separate_reviewer_reopen_and_same_source_successor',
        'source-regression':'cooperative_operator_service_binds_edit_save_request_review_revoke_and_reopen','inspect-regression':'inspection'}
    command=['cargo','test',*base,'--lib',filters[mode],'--','--test-threads=1','--nocapture']
elif mode=='build':command=['cargo','build',*base,'--bin','poet']
elif mode=='wasm':command=['cargo','check','--manifest-path',str(ROOT/'tools/format-sdk-wasm-check/Cargo.toml'),'--locked','--offline','--target','wasm32-unknown-unknown','--target-dir',str(ROOT/'target')]
else:raise ValueError(mode)
before=snapshot();source=HERE/(mode+'-source-before.json');source.write_text(json.dumps(before,indent=2),encoding='utf-8')
env=os.environ.copy();settings={'CARGO_BUILD_JOBS':'2','CARGO_PROFILE_DEV_DEBUG':'0','CARGO_PROFILE_TEST_DEBUG':'0','CARGO_PROFILE_DEV_INCREMENTAL':'false','CARGO_PROFILE_TEST_INCREMENTAL':'false'};env.update(settings)
with tempfile.TemporaryDirectory(prefix='poet-round4-',dir=ROOT/'target') as scratch:
    scratch_path=Path(scratch).resolve();assert scratch_path.is_relative_to((ROOT/'target').resolve());env.update(TEMP=scratch,TMP=scratch)
    logpath=HERE/(mode+'.log');start=datetime.datetime.now(datetime.timezone.utc).isoformat();clock=time.monotonic()
    with logpath.open('w',encoding='utf-8') as log:
        process=subprocess.Popen(command,cwd=ROOT,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,encoding='utf-8',errors='replace')
        for line in process.stdout:log.write(line);log.flush();print(line,end='',flush=True)
        code=process.wait()
    after=snapshot();changed=[p for p in sorted(set(before)|set(after)) if before.get(p)!=after.get(p)]
    logtext=logpath.read_text();counts=[tuple(map(int,m)) for m in re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored',logtext)]
    cases=re.findall(r'^test ([^\s]+) \.\.\. ok$',logtext,re.M)
    selected=mode not in minimum or (sum(n for n,_,_ in counts)>=minimum[mode] and all(f==0 and i==0 for _,f,i in counts))
    receipt=dict(mode=mode,command=command,working_directory=str(ROOT),build_environment=settings,exit_code=code,
        started_utc=start,finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),elapsed_seconds=round(time.monotonic()-clock,3),
        log=str(logpath),log_sha256=sha(logpath),source_snapshot=str(source),source_snapshot_sha256=sha(source),recorded_local_source_files=len(before),
        source_unchanged=not changed,source_changes=changed,expected_minimum_tests=minimum.get(mode),test_selection_ok=selected,test_results=counts,passed_case_names=cases,
        temporary_scratch=str(scratch_path),boundary='Recorded local Core/cooperative/geospatial/Poet source and locks, not every transitive dependency. Native fixtures are software actors; compilation is not GUI/human acceptance or WASM execution.')
    (HERE/(mode+'-result.json')).write_text(json.dumps(receipt,indent=2),encoding='utf-8');print(json.dumps(receipt),flush=True)
    assert scratch_path.is_relative_to((ROOT/'target').resolve())
sys.exit(code if code else int(bool(changed) or not selected))
