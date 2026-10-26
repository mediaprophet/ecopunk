from pathlib import Path
import hashlib,json
root=Path(__file__).parent/'shared-routing'
root.mkdir(exist_ok=True)
changes=[]
def edit(relative,transform):
    target=Path('C:/github/poet/native')/relative
    old=target.read_bytes()
    candidate=root/'candidate'/relative
    candidate.parent.mkdir(parents=True,exist_ok=True)
    candidate.write_text(transform(old.decode()),encoding='utf-8',newline='\n')
    changes.append(dict(target=str(target),candidate='candidate/'+relative,original_sha256=hashlib.sha256(old).hexdigest(),candidate_sha256=hashlib.sha256(candidate.read_bytes()).hexdigest()))
def route(s):
    assert s.count('pub mod mcp;')==1
    return s.replace('pub mod mcp;','pub mod mcp;\npub mod shared_host;')
edit('src/graphical/mod.rs',route)
def mcp(s):
    s=s.replace('use std::sync::atomic::{AtomicU64, Ordering};','use std::sync::atomic::{AtomicU64, Ordering};\nuse std::hash::{BuildHasher, Hasher};')
    assert s.count('    connection: u64,')==1
    s=s.replace('    connection: u64,','    connection: String,')
    assert s.count('connection: NEXT_CONNECTION.fetch_add(1, Ordering::Relaxed)')==1
    s=s.replace('connection: NEXT_CONNECTION.fetch_add(1, Ordering::Relaxed)', '''connection: format!("{}-{:016x}{:016x}-{}", std::process::id(),
            std::collections::hash_map::RandomState::new().build_hasher().finish(),
            std::collections::hash_map::RandomState::new().build_hasher().finish(),
            NEXT_CONNECTION.fetch_add(1, Ordering::Relaxed))''')
    s=s.replace('    fn overwrite_digest_is_exact_and_bounded() {','''    fn overwrite_digest_is_exact_and_bounded() {
        // A process-local counter alone repeats across restarts. Randomised
        // connection labels prevent old context reuse; they grant no authority.
        assert_ne!(NativeAuthoringBackend::unbound().connection, NativeAuthoringBackend::unbound().connection);''')
    return s
edit('src/graphical/mcp/mod.rs',mcp)
(root/'apply-manifest.json').write_text(json.dumps(dict(scope='Shared native UI/MCP session route and restart-distinct non-authority context labels',mutations=changes),indent=2))
