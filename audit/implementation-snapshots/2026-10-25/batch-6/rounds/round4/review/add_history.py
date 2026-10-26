"""One-shot staged source refinement; no sibling writes or checks."""
from pathlib import Path
R=Path(__file__).resolve().parent/'after'
C=R/'crates/qualia-cooperative-core/src/authoring/package_review'
N=R/'crates/qualia-core-db/src/poet_host/native_authority'
def edit(path,old,new):
    text=path.read_text()
    if text.count(old)!=1:raise RuntimeError((path,old[:80],text.count(old)))
    path.write_text(text.replace(old,new),encoding='utf-8',newline='\n')
edit(C/'types.rs','    pub predecessor_digest: Option<[u8; 32]>,','    /// Prior destination bytes from the native overwrite guard, not full derivation.\n    pub predecessor_digest: Option<[u8; 32]>,')
edit(C/'types.rs',"    pub contributor_subjects: &'a [&'a str],","    pub contributor_subjects: &'a [&'a str],\n    pub known_destination_lineage_complete: bool,")
edit(C/'types.rs','    pub contribution_rights_scope: String,','    pub contribution_rights_scope: String,\n    pub known_destination_lineage_complete: bool,')
edit(C/'types.rs','    pub decision_evidence_ref: String,','    pub decision_evidence_ref: String,\n    pub known_destination_lineage_complete: bool,\n    pub contribution_rights_scope: String,')
edit(C/'policy.rs','    pub(super) action: PackageAction,','    pub(super) action: PackageAction,\n    pub(super) known_destination_lineage_complete: bool,')
edit(C/'policy.rs','view.contributor_subjects.len() > 512','view.contributor_subjects.len() > 1024')
edit(C/'policy.rs','        action,\n    })','        action,\n        known_destination_lineage_complete:view.known_destination_lineage_complete,\n    })')
edit(C/'records.rs','            decision_ids: Vec::new(),','            decision_ids: Vec::new(),\n            known_destination_lineage_complete:self.known_destination_lineage_complete,')
edit(C/'records.rs','            decision_evidence_ref: evidence.into(),','            decision_evidence_ref: evidence.into(),\n            known_destination_lineage_complete:self.known_destination_lineage_complete,\n            contribution_rights_scope:"Known destination/source contributors only; other authorship, rights and completeness unassessed".into(),')
test=C/'tests.rs'
text=test.read_text().replace('                source,\n                current_package:', '                source,\n                known_destination_lineage_complete:true,\n                current_package:').replace('            source,\n            current_package:', '            source,\n            known_destination_lineage_complete:true,\n            current_package:')
test.write_text(text,encoding='utf-8',newline='\n')
edit(N/'package_review/history.rs',"pub(super) struct ContributorHistory<'a>","pub(crate) struct ContributorHistory<'a>")
edit(N/'package_review/history.rs',"pub(super) fn contributor_history<'a>","pub(crate) fn contributor_history<'a>")
edit(N/'package_review/mod.rs','mod validation;','mod history;\npub(super) use history::{contributor_history,validate_new_grant_reviewers};\nmod validation;')
edit(N/'package_review/mod.rs','    pub contribution_rights_scope: String,','    pub contribution_rights_scope: String,\n    pub known_destination_lineage_complete: bool,')
path=N/'package_review/mod.rs'
text=path.read_text();start=text.index('            let subjects: Vec<_> = state');end=text.index('            let view = PackageAuthorityView',start)
text=text[:start]+'            let history=contributor_history(&state.durable,target)?;\n'+text[end:]
text=text.replace('                contributor_subjects: &subjects,','                contributor_subjects: &history.subjects,\n                known_destination_lineage_complete:history.complete,')
text=text.replace('                target: target.clone(),\n                principal_subject:', '                target: target.clone(),\n                known_destination_lineage_complete:contributor_history(&state.durable,target)?.complete,\n                principal_subject:')
path.write_text(text,encoding='utf-8',newline='\n')
edit(N/'enrollment/package_review_admin.rs','        let mut next = state.durable.clone();\n        if next.admission.snapshot.package_reviews.len()',
    '        let history=super::super::package_review::contributor_history(&state.durable,&grant.target)?;\n        if history.subjects.contains(&grant.reviewer_subject.as_str()) {return Err("known predecessor contributor cannot receive package review grant".into());}\n        let mut next = state.durable.clone();\n        if next.admission.snapshot.package_reviews.len()')
edit(N/'mod.rs','        let generation = state\n            .generation\n            .checked_add(1)',
    '        package_review::validate_new_grant_reviewers(&state.durable,&replacement.snapshot)?;\n        let generation = state\n            .generation\n            .checked_add(1)')
