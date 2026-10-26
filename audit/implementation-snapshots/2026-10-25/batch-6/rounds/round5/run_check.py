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

def passed_names(logtext):
    names=[];pending=None
    for line in logtext.splitlines():
        match=re.match(r'^test ([^\s]+) \.\.\. (.*)$',line)
        if match:
            pending=match[1] if not match[2].strip() else None
            if match[2].strip()=='ok':names.append(match[1])
        elif pending and line.strip()=='ok':
            names.append(pending);pending=None
    return names

mode=sys.argv[1]
minimum={'authoring':14,'typed':8,'workflow':1,'fiel':3,'intents':5,'jobs':13,'inspection':6}
base=['--manifest-path','C:/github/poet/Cargo.toml','-p','poet-cli','--features','native-graphical','--locked','--offline','--target-dir',str(ROOT/'target')]
if mode=='check':command=['cargo','check',*base,'--lib','--tests']
elif mode=='fiel':command=['cargo','test',*base,'--test','foundation_fiel','--','--test-threads=1','--nocapture']
elif mode in minimum:
    filters={'authoring':'graphical::authoring::','typed':'graphical::checkpoints::typed_10d::','workflow':'spatial_authoring_native_intents_compile_typed_inspect_and_separate_package_review_reopen','intents':'graphical::poet_workspace::tests::','jobs':'graphical::jobs::tests::','inspection':'inspection'}
    command=['cargo','test',*base,'--lib',filters[mode],'--','--test-threads=1','--nocapture']
elif mode=='build':command=['cargo','build',*base,'--bin','poet']
elif mode=='wasm':command=['cargo','check','--manifest-path',str(ROOT/'tools/format-sdk-wasm-check/Cargo.toml'),'--locked','--offline','--target','wasm32-unknown-unknown','--target-dir',str(ROOT/'target')]
else:raise ValueError(mode)
before=snapshot();source=HERE/(mode+'-source-before.json');source.write_text(json.dumps(before,indent=2),encoding='utf-8')
env=os.environ.copy();settings={'CARGO_BUILD_JOBS':'2','CARGO_PROFILE_DEV_DEBUG':'0','CARGO_PROFILE_TEST_DEBUG':'0','CARGO_PROFILE_DEV_INCREMENTAL':'false','CARGO_PROFILE_TEST_INCREMENTAL':'false'};env.update(settings)
with tempfile.TemporaryDirectory(prefix='poet-round5-',dir=ROOT/'target') as scratch:
    scratch_path=Path(scratch).resolve();assert scratch_path.is_relative_to((ROOT/'target').resolve());env.update(TEMP=scratch,TMP=scratch)
    logpath=HERE/(mode+'.log');start=datetime.datetime.now(datetime.timezone.utc).isoformat();clock=time.monotonic()
    with logpath.open('w',encoding='utf-8') as log:
        process=subprocess.Popen(command,cwd=ROOT,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,encoding='utf-8',errors='replace')
        for line in process.stdout:log.write(line);log.flush();print(line,end='',flush=True)
        code=process.wait()
    after=snapshot();changed=[p for p in sorted(set(before)|set(after)) if before.get(p)!=after.get(p)]
    logtext=logpath.read_text();counts=[tuple(map(int,m)) for m in re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored',logtext)]
    cases=passed_names(logtext)
    selected=mode not in minimum or (sum(n for n,_,_ in counts)>=minimum[mode] and all(f==0 and i==0 for _,f,i in counts))
    receipt=dict(mode=mode,command=command,working_directory=str(ROOT),build_environment=settings,exit_code=code,
        started_utc=start,finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),elapsed_seconds=round(time.monotonic()-clock,3),
        log=str(logpath),log_sha256=sha(logpath),source_snapshot=str(source),source_snapshot_sha256=sha(source),recorded_local_source_files=len(before),
        source_unchanged=not changed,source_changes=changed,expected_minimum_tests=minimum.get(mode),test_selection_ok=selected,test_results=counts,passed_case_names=cases,
        temporary_scratch=str(scratch_path),boundary='Recorded local Core/cooperative/geospatial/Poet source and locks, not every transitive dependency. Native fixtures are software actors; compilation is not GUI/human acceptance or WASM execution.')
    (HERE/(mode+'-result.json')).write_text(json.dumps(receipt,indent=2),encoding='utf-8');print(json.dumps(receipt),flush=True)
    assert scratch_path.is_relative_to((ROOT/'target').resolve())
sys.exit(code if code else int(bool(changed) or not selected))
