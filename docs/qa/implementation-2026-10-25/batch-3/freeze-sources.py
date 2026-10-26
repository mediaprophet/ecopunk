from pathlib import Path
import json,hashlib,datetime
root=Path(__file__).parent
known={}
receipts=[]
for receipt in sorted(root.glob('applied-*.json')):
    data=json.loads(receipt.read_text(encoding='utf-8-sig'))
    receipts.append(dict(path=receipt.name,sha256=hashlib.sha256(receipt.read_bytes()).hexdigest()))
    for item in data['mutations']:
        key=str(Path(item['target'])).lower()
        if key in known and item['original_sha256'] is not None:
            assert known[key]['sha256'].lower()==item['original_sha256'].lower(),f'Broken reviewed source chain: {key}'
        known[key]=dict(path=item['target'],sha256=item['sha256'].lower(),receipt=receipt.name)
for item in known.values():
    actual=hashlib.sha256(Path(item['path']).read_bytes()).hexdigest()
    assert actual==item['sha256'],f"Source changed: {item['path']}"
extra=[]
for name in ['C:/github/poet/Cargo.lock','C:/github/poet/Cargo.toml','C:/github/qualiaDB/Cargo.lock']:
    extra.append(dict(path=name,sha256=hashlib.sha256(Path(name).read_bytes()).hexdigest()))
payload=dict(captured_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    status='Exact batch-3 applied source chain verified; execution evidence remains separate',
    source_count=len(known),transition_count=sum(len(json.loads((root/r['path']).read_text(encoding='utf-8-sig'))['mutations']) for r in receipts),
    sources=sorted(known.values(),key=lambda x:x['path'].lower()),dependency_pins=extra,receipts=receipts)
encoded=(json.dumps(payload,indent=2)+'\n').encode()
digest=hashlib.sha256(encoded).hexdigest()
folder=root/'source-snapshots'
folder.mkdir(exist_ok=True)
destination=folder/(digest+'.json')
if destination.exists(): assert destination.read_bytes()==encoded
else: destination.write_bytes(encoded)
(root/'latest-source-snapshot.json').write_text(json.dumps(dict(path=str(destination),sha256=digest),indent=2))
print(json.dumps(dict(sources=len(known),transitions=payload['transition_count'],snapshot=str(destination),sha256=digest)))
