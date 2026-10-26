from pathlib import Path
import json,hashlib
base=Path(__file__).parent
source=Path('C:/github/qualiaDB/crates/vibe')
guards=json.loads((base/'source-guards.json').read_text())
def preserve(rel):
    target=source/rel; data=target.read_bytes(); original=base/'original/vibe'/rel
    original.parent.mkdir(parents=True,exist_ok=True)
    if original.exists(): assert original.read_bytes()==data
    else: original.write_bytes(data)
    if not any(Path(e['target'])==target for e in guards):
        guards.append(dict(target=str(target),original=str(original),sha256=hashlib.sha256(data).hexdigest()))
    return data.decode('utf8').replace('\r\n','\n')
text=preserve('tests/conformance.rs')
cases=[
('g1_tick_hook_loads_and_dispatches','g1_tick_hook_rejects_transitive_external','g1_tick_hook.vibe'),
('h1_tick_counter_publishes','h1_tick_counter_rejects_transitive_external','h1_tick_counter.vibe'),
('h3_tick_time_publish_uses_time','h3_tick_time_rejects_transitive_external','h3_tick_time_publish.vibe'),
('n4_unknown_math_fn_runtime_error','n4_unknown_math_fn_rejected_before_execution','n4_unknown_math_fn.vibe')]
for old,new,fixture in cases:
    start=text.index('fn '+old+'()'); end=text.index('\n}',start)+2
    block=f'''fn {new}() {{
    // Unsafe legacy input is retained as a negative compatibility fixture.
    let src = include_str!("../fixtures/{fixture}");
    let error = load_program(src).expect_err("must reject before dispatch");
    assert_eq!(error.code, DiagCode::E200);
}}'''
    text=text[:start]+block+text[end:]
out=base/'candidate/vibe/tests/conformance.rs';out.parent.mkdir(parents=True,exist_ok=True)
out.write_text(text,encoding='utf8',newline='\n')
fixture=preserve('fixtures/crypto1_hash_sign.vibe')
fixture=fixture.replace('fn sign_data(', 'effect fn sign_data(').replace('fn new_ed25519_key(', 'effect fn new_ed25519_key(')
out=base/'candidate/vibe/fixtures/crypto1_hash_sign.vibe';out.parent.mkdir(parents=True,exist_ok=True)
out.write_text(fixture,encoding='utf8',newline='\n')
(base/'source-guards.json').write_text(json.dumps(guards,indent=2),encoding='utf8')
print('4 negative conformance migrations; 2 explicit External declarations in crypto fixture')
