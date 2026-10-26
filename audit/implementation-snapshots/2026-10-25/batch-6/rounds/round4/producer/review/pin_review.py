from pathlib import Path
import hashlib, json
here = Path(__file__).parent
review = here.parents[1] / 'review'
relative = [
    'contract-proposal.md',
    *['after/crates/qualia-cooperative-core/src/authoring/package_review/' + n + '.rs'
      for n in ('types', 'policy', 'records', 'tests')],
    *['after/crates/qualia-core-db/src/poet_host/native_authority/package_review/' + n + '.rs'
      for n in ('mod', 'workflow', 'validation', 'tests', 'admin_tests')],
    'after/crates/qualia-core-db/src/poet_host/native_authority/enrollment/package_review_admin.rs',
    'after/crates/qualia-core-db/src/poet_host/native_authority/enrollment/owner.rs',
    'after/crates/qualia-core-db/src/poet_host/native_authority/admission.rs',
    'after/crates/qualia-core-db/src/poet_host/native_authority/journal.rs',
    'after/crates/qualia-core-db/src/poet_host/native_authority/mod.rs',
    'after/crates/qualia-cooperative-core/src/authoring/records.rs',
]
rows = [dict(path=str(review / p), sha256=hashlib.sha256((review / p).read_bytes()).hexdigest())
    for p in relative]
(here / 'independent-review-source.json').write_text(json.dumps(dict(
    status='source-review-only-one-blocking-finding', execution_performed=False,
    review='independent-review-producer.md', sources=rows), indent=2) + '\n')
print(f'Pinned {len(rows)} source-review inputs.')
