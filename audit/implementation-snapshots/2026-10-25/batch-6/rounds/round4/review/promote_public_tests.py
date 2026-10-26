"""Expose focused cases as public integration targets, without running them."""
from pathlib import Path
ROOT=Path(__file__).resolve().parent
AFTER=ROOT/'after'
COOP=AFTER/'crates/qualia-cooperative-core'
CORE=AFTER/'crates/qualia-core-db'

canonical=COOP/'src/authoring/package_review/tests.rs'
public=COOP/'tests/package_review.rs'
public.parent.mkdir(parents=True,exist_ok=True)
public.write_text(canonical.read_text().replace('use super::{super::*, *};',
    'use qualia_cooperative_core::authoring::{package_review::*, *};').replace(
    'use crate::projects::{Project, ProjectMembership, ProjectRole};',
    'use qualia_cooperative_core::projects::{Project, ProjectMembership, ProjectRole};'),encoding='utf-8',newline='\n')

native=CORE/'src/poet_host/native_authority/package_review/tests.rs'
text=native.read_text()
start=text.index('fn package_target()')
cut=text.index('#[test]\nfn package_review_evidence_counts_refuse_before_any_append()')
body=text[start:cut]
body=body.replace('    assert_eq!(legacy.verify(&anchor(), 1024 * 1024).unwrap(), digest);',
    '    let root=tempfile::tempdir().unwrap();\n    let owner=NativeAuthority::open(root.path(),anchor(),legacy,document(),1024*1024,256).unwrap();\n    assert_eq!(admission_digest(&owner.enrollment_snapshot().unwrap().snapshot,1024*1024).unwrap(),digest);')
source=text.index('#[test]\nfn package_review_source_successor_and_revoked_membership_withhold_old_targets()')
end=text.index('#[test]\nfn package_review_generation_overflow_never_admits_signed_successor()')
body+=text[source:end]
public_native=CORE/'tests/native_package_review.rs'
public_native.parent.mkdir(parents=True,exist_ok=True)
header='''//! Public native package APIs; signed software fixtures are not human acceptance.
#[path="native_package_review/fixture.rs"]
mod fixture;
use fixture::*;
#[cfg(feature="sanctuary-crypto")]
#[path="native_package_review/admin.rs"]
mod admin;
#[cfg(feature="sanctuary-crypto")]
#[path="native_package_review/history.rs"]
mod history;
'''
public_native.write_text(header+body,encoding='utf-8',newline='\n')
admin=CORE/'src/poet_host/native_authority/package_review/admin_tests.rs'
admin_text=admin.read_text().replace('use super::super::super::enrollment::NativeSigner;','use qualia_core_db::poet_host::native_authority::enrollment::NativeSigner;')
private_audit=admin_text.index('    let state = owner.state.lock().unwrap();')
admin_text=admin_text[:private_audit]+'''    let retained=owner.enrollment_snapshot().unwrap();
    assert!(retained.snapshot.package_reviews[0].revoked);
}
'''
public_admin=CORE/'tests/native_package_review/admin.rs'
public_admin.parent.mkdir(parents=True,exist_ok=True)
public_admin.write_text(admin_text,encoding='utf-8',newline='\n')
