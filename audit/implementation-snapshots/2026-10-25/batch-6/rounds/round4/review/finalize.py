"""Read-only source guards and staged patch/hash inventory. Does not run Rust."""
from pathlib import Path
import difflib, hashlib, json
ROOT=Path(__file__).resolve().parent
SOURCE=Path('C:/github/QualiaDB')
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
rows=[];patch=[]
for candidate in sorted((ROOT/'after').rglob('*')):
    if not candidate.is_file():continue
    rel=candidate.relative_to(ROOT/'after').as_posix()
    before=ROOT/'before'/rel;actual=SOURCE/rel
    if before.exists():
        if not actual.is_file() or before.read_bytes()!=actual.read_bytes():raise RuntimeError(f'baseline drift: {actual}')
        old=before.read_text(encoding='utf-8').splitlines(keepends=True)
    else:
        if actual.exists():raise RuntimeError(f'new target unexpectedly exists: {actual}')
        old=[]
    new=candidate.read_text(encoding='utf-8').splitlines(keepends=True)
    if not before.exists() and len(new)>=500:raise RuntimeError(f'new source reaches limit: {rel}: {len(new)}')
    patch.extend(difflib.unified_diff(old,new,fromfile='a/'+rel if before.exists() else '/dev/null',tofile='b/'+rel))
    rows.append({'target':str(actual),'relative':rel,'before':str(before) if before.exists() else None,
        'after':str(candidate),'before_sha256':digest(before) if before.exists() else None,
        'after_sha256':digest(candidate),'lines':len(new),'baseline_unchanged':True})
public=[]
for path in [ROOT/'after/crates/qualia-cooperative-core/tests/package_review.rs',
             ROOT/'after/crates/qualia-core-db/tests/native_package_review.rs',
             ROOT/'after/crates/qualia-core-db/tests/native_package_review/admin.rs',
             ROOT/'after/crates/qualia-core-db/tests/native_package_review/history.rs',
             ROOT/'after/crates/qualia-core-db/tests/native_package_review/source_history.rs',
             ROOT/'after/crates/qualia-core-db/tests/native_package_review/target_isolation.rs',
             ROOT/'after/crates/qualia-core-db/tests/native_package_review/publication.rs']:
    import re
    for case in re.findall(r'#\[test\]\s*fn\s+(\w+)',path.read_text()):
        public.append({'case_id':'urn:ecopunk:qa:round4:package-review:'+case,'rust_test':case,'source':str(path),'status':'not_run'})
(ROOT/'target-manifest.json').write_text(json.dumps({'status':'staged_not_applied_or_executed','targets':rows},indent=2)+'\n',encoding='utf-8')
(ROOT/'review.patch').write_text(''.join(patch),encoding='utf-8',newline='\n')
(ROOT/'case-inventory.json').write_text(json.dumps({'status':'unexecuted','public_cases':public,
    'private_only_cases':['package_review_evidence_counts_refuse_before_any_append','package_review_entry_byte_budget_refuses_before_creating_a_file',
        'package_review_generation_overflow_never_admits_signed_successor']},indent=2)+'\n',encoding='utf-8')
print(json.dumps({'targets':len(rows),'new_targets':sum(r['before'] is None for r in rows),'public_cases':len(public),
    'status':'staged_only_no_build_or_tests','patch_sha256':digest(ROOT/'review.patch')}))
