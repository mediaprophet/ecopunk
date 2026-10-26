"""Stage reviewed candidates; never writes to sibling repositories."""
from pathlib import Path
import hashlib, json

ROOT = Path(__file__).resolve().parent
SOURCE = Path('C:/github/QualiaDB')

def stage(relative, transform):
    source = SOURCE / relative
    data = source.read_bytes()
    for folder in ('before', 'after'):
        target = ROOT / folder / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        if folder == 'before':
            if target.exists() and target.read_bytes() != data:
                raise RuntimeError(f'baseline changed: {relative}')
            target.write_bytes(data)
    text = data.decode('utf-8').replace('\r\n', '\n')
    result = transform(text)
    (ROOT / 'after' / relative).write_text(result, encoding='utf-8', newline='\n')

def replace_once(text, old, new):
    if text.count(old) != 1:
        raise RuntimeError(f'expected one replacement: {old[:90]!r}')
    return text.replace(old, new)

COOP = 'crates/qualia-cooperative-core/src/authoring/'
NATIVE = 'crates/qualia-core-db/src/poet_host/native_authority/'
stage(COOP+'mod.rs', lambda t: replace_once(t, 'mod policy;', 'pub mod package_review;\nmod policy;'))
stage(COOP+'records.rs', lambda t: replace_once(replace_once(t,
    '    fn evidence(&self)', '    pub(super) fn evidence(&self)'),
    "fn reserve_record<'a>(", "pub(super) fn reserve_record<'a>("))
stage(NATIVE+'admission.rs', lambda t: replace_once(replace_once(t,
    '    pub subjects: Vec<SubjectAdmission>,',
    '    pub subjects: Vec<SubjectAdmission>,\n    /// Explicit exact-package grants. Empty preserves pre-extension signed bytes.\n    #[serde(default, skip_serializing_if = "Vec::is_empty")]\n    pub package_reviews: Vec<package_review::PackageReviewGrant>,'),
    '        Ok(digest)\n', '        super::package_review::validate_admission_packages(snapshot)?;\n        Ok(digest)\n').replace(
    'pub const ADMISSION_CONTEXT:',
    'use qualia_cooperative_core::authoring::package_review;\n\npub const ADMISSION_CONTEXT:'))
stage(NATIVE+'mod.rs', lambda t: replace_once(replace_once(replace_once(replace_once(t,
    'mod package;', 'mod package;\npub mod package_review;'),
    '                enrollment: Vec::new(),',
    '                enrollment: Vec::new(),\n                package_contributions: Vec::new(),\n                package_requests: Vec::new(),\n                package_reviews: Vec::new(),\n                package_grant_decisions: Vec::new(),'),
    '        let mut next = state.durable.clone();\n        next.admission = replacement;',
    '        package_review::validate_grant_successor(&state.durable.admission.snapshot, &replacement.snapshot)?;\n        let generation = state.generation.checked_add(1).ok_or("authority generation exhausted")?;\n        let mut next = state.durable.clone();\n        next.admission = replacement;'),
    '        state.generation = state\n            .generation\n            .checked_add(1)\n            .ok_or("authority generation exhausted")?;',
    '        state.generation = generation;'))
stage(NATIVE+'journal.rs', lambda t: replace_once(replace_once(replace_once(replace_once(t,
    '    pub enrollment: Vec<super::enrollment::EnrollmentRecord>,',
    '    pub enrollment: Vec<super::enrollment::EnrollmentRecord>,\n    #[serde(default, skip_serializing_if = "Vec::is_empty")]\n    pub package_contributions: Vec<super::package_review::PackageContributionRecord>,\n    #[serde(default, skip_serializing_if = "Vec::is_empty")]\n    pub package_requests: Vec<super::package_review::PackageReviewRequest>,\n    #[serde(default, skip_serializing_if = "Vec::is_empty")]\n    pub package_reviews: Vec<super::package_review::PackageReviewRecord>,\n    #[serde(default, skip_serializing_if = "Vec::is_empty")]\n    pub package_grant_decisions: Vec<super::package_review::PackageGrantDecision>,'),
    '            last_admission_sequence = entry.state.admission.snapshot.sequence;',
    '            super::package_review::validate_ledger(&entry.state)?;\n            if let Some(previous) = latest.as_ref() {\n                super::package_review::validate_grant_successor(&previous.admission.snapshot, &entry.state.admission.snapshot)?;\n            }\n            last_admission_sequence = entry.state.admission.snapshot.sequence;'),
    '    pub fn append(&mut self, state: &DurableState) -> Result<(), String> {',
    '    pub fn append(&mut self, state: &DurableState) -> Result<(), String> {\n        super::package_review::validate_ledger(state)?;'),
    '        let mut latest = None;', '        let mut latest: Option<DurableState> = None;'))
stage(NATIVE+'enrollment/mod.rs', lambda t: replace_once(t, 'mod owner;', 'mod owner;\nmod package_review_admin;'))
stage(NATIVE+'enrollment/owner.rs', lambda t: replace_once(replace_once(t.replace(
    '    fn require_enrollment_permission(', '    pub(super) fn require_enrollment_permission(').replace(
    '    fn sign_enrollment_admission(', '    pub(super) fn sign_enrollment_admission(').replace(
    '    fn commit_enrollment(', '    pub(super) fn commit_enrollment('),
    '        // Preflight generation before writing a durable successor.',
    '        // Validate all fallible admission/generation work before durable write.\n        let admission_digest = if changed {\n            super::super::package_review::validate_grant_successor(&state.durable.admission.snapshot, &next.admission.snapshot)?;\n            next.admission.verify(&self.anchor, 4 * 1024 * 1024)?\n        } else { state.admission_digest };\n        // Preflight generation before writing a durable successor.'),
    '            state.admission_digest = state\n                .durable\n                .admission\n                .verify(&self.anchor, 4 * 1024 * 1024)?;',
    '            state.admission_digest = admission_digest;'))
stage(NATIVE+'tests.rs', lambda t: replace_once(replace_once(t,
    'mod package_tests;', 'mod package_tests;\n#[path = "package_review/tests.rs"]\nmod package_review_tests;'),
    '        subjects: vec![subject("author", &AUTHOR), subject("reviewer", &REVIEWER)],',
    '        subjects: vec![subject("author", &AUTHOR), subject("reviewer", &REVIEWER)],\n        package_reviews: Vec::new(),'))
stage(NATIVE+'enrollment/fixture.rs', lambda t: replace_once(t,
    '        subjects: vec![subject(', '        package_reviews: Vec::new(),\n        subjects: vec![subject('))

def manifest():
    records=[]
    for candidate in sorted((ROOT/'after').rglob('*')):
        if not candidate.is_file():
            continue
        relative=candidate.relative_to(ROOT/'after').as_posix()
        original=ROOT/'before'/relative
        records.append({'target': str(SOURCE/relative), 'relative':relative,
            'before_sha256': hashlib.sha256(original.read_bytes()).hexdigest() if original.exists() else None,
            'after_sha256': hashlib.sha256(candidate.read_bytes()).hexdigest(),
            'before':str(original) if original.exists() else None, 'after':str(candidate),
            'lines':len(candidate.read_text(encoding='utf-8').splitlines())})
    (ROOT/'target-manifest.json').write_text(json.dumps({'status':'staged_not_applied_or_executed',
        'targets':records},indent=2)+'\n',encoding='utf-8')

if __name__=='__main__':
    manifest()
