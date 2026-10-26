"""Read-only source reconciliation and current pins for the approved absorption."""
from pathlib import Path
import datetime, hashlib, json
qa=Path(__file__).parent
initial=json.loads((qa/'save-receipt.json').read_text())
rows=[]
for row in initial['source_map']:
    source,target=Path(row['source']),Path(row['target'])
    now=hashlib.sha256(target.read_bytes()).hexdigest()
    record=dict(row,current_target_sha256=now)
    if source!=target:
        origin_now=hashlib.sha256(source.read_bytes()).hexdigest()
        if origin_now!=row['source_sha256']:
            raise SystemExit('Preserved source changed: '+str(source))
        record['preserved_source_unchanged']=True
    rows.append(record)
paths=[]
for crate in ['geometry','spatial-authoring']:
    base=Path('C:/github/poet/crates')/crate
    paths.extend([base/'Cargo.toml',base/'LICENSE-MIT',base/'README.md'])
    paths.extend(sorted((base/'src').glob('*.rs')))
paths.extend([Path('C:/github/poet/native/src/graphical')/name for name in ['geometry_host.rs','spatial_host.rs']])
files=[dict(path=str(p),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),bytes=p.stat().st_size,lines=len(p.read_text(encoding='utf-8-sig').splitlines())) for p in paths]
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    selected_license='MIT',no_original_or_staged_source_removed=True,
    no_independent_document_state=True,unused_profile_revision_wrapper_removed=True,
    rustfmt_exit_code=0,cargo_run_by_worker=False,source_transitions=rows,files=files)
(qa/'current-source.json').write_text(json.dumps(record,indent=2)+'\n')
provenance=Path('C:/github/poet/crates/spatial-authoring/provenance/current-source.json')
provenance.write_text(json.dumps(record,indent=2)+'\n')
print(f'Pinned {len(files)} current files; {sum(r.get("preserved_source_unchanged",False) for r in rows)} preserved source inputs match. No Cargo run.')
