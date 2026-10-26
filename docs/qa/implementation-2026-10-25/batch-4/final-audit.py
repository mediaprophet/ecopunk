from pathlib import Path
import hashlib,json
from datetime import datetime,timezone
root=Path(__file__).parent
def read(p):return json.loads(p.read_text(encoding='utf-8-sig'))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
pointer=read(root/'latest-source-snapshot.json');snapshot_path=Path(pointer['path'])
assert sha(snapshot_path)==pointer['sha256']
snapshot=read(snapshot_path)
for source in snapshot['sources']:assert sha(Path(source['path']))==source['sha256'],source['path']
review=read(root/'review/reviewed-inputs.json')
for source in review['inputs']:assert sha(Path(source['path']))==source['sha256'],source['path']
names=['core-final.json','native-final.json','library-final.json','wasm-final.json','gpu-final.json','native-check-corrected.json']
for name in names:
 receipt=read(root/name)
 assert receipt['exit_code']==0 and receipt['source_unchanged'] is True,name
 assert receipt['input_snapshot']['sha256']==pointer['sha256'],name
cli=root/'direct-cli'/pointer['sha256'][:12]
checks=read(cli/'checks.json');artifact=read(cli/'artifact.json')
assert checks['exit_code']==0 and checks['gui_launched'] is False
assert sha(Path(artifact['path']))==artifact['sha256']==checks['artifact_sha256']
tech=Path('docs/planning/tech')
actual=read(tech/'ecopunk-production-claims.json');template=read(tech/'ecopunk-production-claims.template.json')
assert len(actual['packets'])==120 and all(p['status']=='open' for p in actual['packets'])
assert len(template['packets'])==120 and all(p['status']=='open' and not p['milestone_results'] for p in template['packets'])
summary=actual['execution_summary']
assert summary['live_builds']==0 and summary['batch_4_live_builds']==0
assert summary['human_acceptance_claimed'] is False and summary['model_or_language_release_complete'] is False
assert all(item['status']=='passed' for item in summary['batch_4_receipts'])
catalogue=tech/'assets/model-production-template/deliverables.json'
assert sha(catalogue)=='c0cb76151bf5d0776afaae4d59d7427efb65633bf359d3f4c2bda83c361c91cb'
result=dict(checked_utc=datetime.now(timezone.utc).isoformat(),source_snapshot=pointer,source_inputs_verified=len(snapshot['sources']),reviewed_inputs_verified=len(review['inputs']),receipts=[dict(path=n,sha256=sha(root/n),exit_code=0,source_unchanged=True) for n in names],cli_artifact=artifact,all_owned_commands_exited=True,live_builds=0,production_packets_open=120,starter_milestones=0,human_acceptance_claimed=False)
(root/'final-verification.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print(json.dumps(dict(status='Final saved-source, review, execution and open-ledger audit passed',source_inputs=result['source_inputs_verified'],reviewed_inputs=result['reviewed_inputs_verified'],live_builds=0,packets_open=120)))
