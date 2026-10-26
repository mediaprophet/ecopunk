from pathlib import Path
import hashlib,json
root=Path(__file__).parent
base=Path('C:/github/poet/native/src/graphical')
source=base/'controller.rs';target=base/'editor_contract.rs'
assert not target.exists()
original=source.read_bytes();text=original.decode('utf-8')
start=text.index('#[derive(Clone)]\npub struct EditorSnapshot')
end=text.index('pub struct CanonicalEditorHost')
body=text[start:end].replace('super::','crate::graphical::')
contract='//! Shared native editor contract; canonical execution remains in controller.\nuse crate::graphical::{preview::{self,PreviewFrame,SourceRef}};\nuse qualia_core_db::poet_host::invoke::studio_store;\nuse vibe::Value;\nuse super::controller::SessionEpoch;\n\n'+body
replacement='#[path = "editor_contract.rs"]\nmod contract;\npub use contract::{EditorSnapshot,EditorOutcome,PreviewProvider,MeshPreviewProvider,EditorHost};\n\n'
target.write_text(contract,encoding='utf-8')
source.write_text(text[:start]+replacement+text[end:],encoding='utf-8')
receipt=dict(scope='Split cohesive public contract from canonical execution; preserve existing controller API',original_sha256=hashlib.sha256(original).hexdigest(),controller_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),contract_sha256=hashlib.sha256(target.read_bytes()).hexdigest())
(root/'controller-split.json').write_text(json.dumps(receipt,indent=2)+'\n',encoding='utf-8')
print('Saved editor contract split; controller public API retained.')
