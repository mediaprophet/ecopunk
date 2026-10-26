from pathlib import Path
import hashlib, json
here = Path(__file__).parent
round4 = here.parents[1]
manifest = round4 / 'review/target-manifest.json'
data = json.loads(manifest.read_text())
rows = []
for item in data['targets']:
    path = Path(item['after'])
    sha = hashlib.sha256(path.read_bytes()).hexdigest()
    if sha != item['after_sha256']:
        raise SystemExit(f'Frozen candidate drift: {path}')
    rows.append(dict(path=str(path), sha256=sha, manifest_match=True))
glue = [
    'package_workflow/mod.rs', 'package_workflow/service.rs', 'shared_host.rs',
    'package_review_controls.rs', 'native_entry/package_admin.rs',
    'native_entry/package_grant_controls.rs', 'native_project.rs', 'jobs.rs',
]
glue_rows = []
for relative in glue:
    path = round4 / 'integration/after/native/src/graphical' / relative
    glue_rows.append(dict(path=str(path), sha256=hashlib.sha256(path.read_bytes()).hexdigest()))
(here / 'recheck-source.json').write_text(json.dumps(dict(
    status='source-review-no-open-blocker-in-inspected-current-source',
    execution_performed=False, original_P1='resolved-at-source', UI_P2='resolved-at-source',
    callback_guard_lifetime='satisfied-at-source; execution-pending',
    canonical_manifest_sha256=hashlib.sha256(manifest.read_bytes()).hexdigest(),
    canonical_targets=rows, integration_review=glue_rows), indent=2) + '\n')
print(f'Pinned {len(rows)} manifest-matching canonical targets and {len(glue_rows)} inspected integration files.')
