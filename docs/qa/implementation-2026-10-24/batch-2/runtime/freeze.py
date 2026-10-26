from pathlib import Path
import hashlib,json,datetime
base=Path(__file__).parent
def digest(file):
    h=hashlib.sha256()
    with Path(file).open('rb') as stream:
        for chunk in iter(lambda:stream.read(1024*1024),b''): h.update(chunk)
    return h.hexdigest()
maps=['patch-map.json','poet-patch-map.json']
targets=[]
for name in maps:
    for entry in json.loads((base/name).read_text()):
        assert digest(entry['candidate'])==entry['candidate_sha256'],entry['candidate']
        target=Path(entry['target'])
        if entry['original_sha256'] is None:
            assert not target.exists(),f'new target already exists: {target}'
        else:
            assert digest(target)==entry['original_sha256'],f'changed original target: {target}'
            assert digest(entry['original'])==entry['original_sha256']
        targets.append(entry['target'])
files=maps+['verification-receipt.json','verification-overlay.json','verification-inputs.json',
    'verification-includes.json','poet-verification-inputs.json','native-poet-build.log',
    'native-default-session.log','native-granted-session.log']
files += [p.name for p in sorted(base.glob('guarded-boundary_*.log'))]
receipt=dict(candidate='RUN-05B-2026-10-24-final',
    frozen_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    status='review-ready, sibling targets verified unchanged, NOT APPLIED',
    source_targets=len(targets), artifacts={name:digest(base/name) for name in files},
    native_binary=dict(path='verification/qualiaDB/crates/vibe/target/debug/poet.exe',
        sha256=digest(base/'verification/qualiaDB/crates/vibe/target/debug/poet.exe')),
    checks=dict(full_private_suite_exit_code=0,full_private_passes=1278,
        isolated_boundary_passes=5,native_default_exit_code=0,native_granted_exit_code=0),
    remaining=['supervisor guarded canonical apply and target verification',
        'durable shared sessions and checked replacement/live jobs',
        'actual native graphical Poet/Craft cooperative authoring/IP-03 C1-C8',
        'full proposed 0.2 conformance and exceptional Ecopunk output/model acceptance'])
(base/'frozen-package.json').write_text(json.dumps(receipt,indent=2),encoding='utf8')
print(json.dumps(dict(targets=len(targets),patch_map=receipt['artifacts']['patch-map.json'],
    poet_map=receipt['artifacts']['poet-patch-map.json'],status=receipt['status']),indent=2))
