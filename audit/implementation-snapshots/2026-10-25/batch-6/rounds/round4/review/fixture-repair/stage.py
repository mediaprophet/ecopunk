"""Capture current public fixture and stage an isolated alias-scope repair only."""
from pathlib import Path
import difflib,hashlib,json
ROOT=Path(__file__).resolve().parent
SOURCE=Path('C:/github/QualiaDB/crates/qualia-core-db/tests/native_package_review/history.rs')
REL=Path('crates/qualia-core-db/tests/native_package_review/history.rs')
original=SOURCE.read_bytes()
before=ROOT/'before'/REL;after=ROOT/'after'/REL
before.parent.mkdir(parents=True,exist_ok=True);after.parent.mkdir(parents=True,exist_ok=True)
if before.exists() and before.read_bytes()!=original:raise RuntimeError('live fixture changed before staging')
before.write_bytes(original)
text=original.decode('utf-8').replace('\r\n','\n')
old='''    new_grant.id = "urn:fixture:new-aliased-grant".into();
'''
new='''    new_grant.id = "urn:fixture:new-aliased-grant".into();
    // A second declared release alias keeps the saved bytes/source identical,
    // while avoiding duplicate exact-target/reviewer scope in the signed body.
    // That structural duplicate would refuse before testing ancestor exclusion.
    new_grant.target.package_id = "urn:fixture:second-alias-same-successor".into();
    assert_ne!(new_grant.target.package_id, successor.package_id);
    assert_eq!(new_grant.target.package_digest, successor.package_digest);
    assert_eq!(new_grant.target.source, successor.source);
    assert_eq!(new_grant.target.producer_subject, successor.producer_subject);
'''
if text.count(old)!=1:raise RuntimeError('expected one new-grant assignment')
text=text.replace(old,new)
old='''    let mut replacement = owner.enrollment_snapshot().unwrap().snapshot;
    replacement.sequence += 1;
    replacement.package_reviews.push(new_grant);
    assert!(owner
        .replace_admission(signer.sign_admission(replacement).unwrap())
        .unwrap_err()
        .contains("predecessor"));
'''
new='''    let retained = owner.enrollment_snapshot().unwrap().snapshot;
    let mut replacement = retained.clone();
    replacement.sequence += 1;
    replacement.package_reviews.push(new_grant);
    assert_eq!(
        &replacement.package_reviews[..retained.package_reviews.len()],
        retained.package_reviews.as_slice(),
        "every old signed scope must remain unchanged"
    );
    let signed = signer.sign_admission(replacement).unwrap();
    // Prove the signed body passes ordinary signature/project/subject/grant
    // shape admission without the original owner's contributor history.
    let structural_root = tempfile::tempdir().unwrap();
    let structural_owner = NativeAuthority::open(
        structural_root.path(),
        TrustedProject {
            project_id: signed.snapshot.project.id.clone(),
            issuer_identifier: signed.snapshot.issuer_identifier.clone(),
            public_key: signer.public_key(),
        },
        signed.clone(),
        successor.source.clone(),
        1024 * 1024,
        256,
    ).expect("replacement must pass signature and ordinary scope-shape admission");
    drop(structural_owner);
    let error = owner.replace_admission(signed).unwrap_err();
    assert!(error.contains("predecessor"), "expected ancestor denial after valid structural admission: {error}");
'''
if text.count(old)!=1:raise RuntimeError('expected one signed replacement assertion')
after.write_text(text.replace(old,new),encoding='utf-8',newline='\n')
import subprocess
subprocess.run(['C:/Users/tim_h/.cargo/bin/rustfmt.exe','--edition','2021','--config','skip_children=true',str(after)],check=True)
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
patch=''.join(difflib.unified_diff(before.read_text().splitlines(keepends=True),after.read_text().splitlines(keepends=True),fromfile='a/'+REL.as_posix(),tofile='b/'+REL.as_posix()))
(ROOT/'fixture.patch').write_text(patch,encoding='utf-8',newline='\n')
(ROOT/'target-manifest.json').write_text(json.dumps({'status':'test_only_staged_not_executed','targets':[{
    'target':str(SOURCE),'relative':REL.as_posix(),'before':str(before),'after':str(after),
    'before_sha256':digest(before),'after_sha256':digest(after)}]},indent=2)+'\n',encoding='utf-8')
print(json.dumps({'before_sha256':digest(before),'after_sha256':digest(after),'test_only':True}))
