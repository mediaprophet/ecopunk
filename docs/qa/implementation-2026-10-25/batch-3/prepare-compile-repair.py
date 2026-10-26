from pathlib import Path
import hashlib,json
root=Path(__file__).parent/'native-build/compile-repair'
root.mkdir(exist_ok=True)
changes=[]
def edit(relative,transform):
    target=Path('C:/github/poet/native')/relative
    old=target.read_bytes()
    candidate=root/'candidate'/relative
    candidate.parent.mkdir(parents=True,exist_ok=True)
    candidate.write_text(transform(old.decode()),encoding='utf-8',newline='\n')
    changes.append(dict(target=str(target),candidate='candidate/'+relative,original_sha256=hashlib.sha256(old).hexdigest(),candidate_sha256=hashlib.sha256(candidate.read_bytes()).hexdigest()))
def cargo(s):
    assert '[dev-dependencies]' not in s
    return s+'\n[dev-dependencies]\ntempfile = "=3.27.0"\n'
edit('Cargo.toml',cargo)
def test(s):
    old='        &host.rebuild_limits,\n    )'
    assert s.count(old)==1
    return s.replace(old,'        &host.rebuild_limits,\n        host.preview.as_ref(),\n    )')
edit('src/graphical/controller_tests.rs',test)
edit('src/graphical/cooperation/mod.rs',lambda s:s.replace('identity/session/membership/review','identifier/session/membership/review').replace('principal, CLI identity,','principal, caller-supplied identifiers,'))
edit('src/graphical/shared_host_tests.rs',lambda s:s.replace('production identity.','production authentication.'))
(root/'apply-manifest.json').write_text(json.dumps(dict(scope='Native test-only linkage corrections and required identifier terminology',mutations=changes),indent=2))
