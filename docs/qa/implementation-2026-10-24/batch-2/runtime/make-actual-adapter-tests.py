from pathlib import Path
base=Path(__file__).parent
text=(base/'harness/poet_tests.rs').read_text(encoding='utf8')
old='#[path = "C:/github/ecopunk/docs/qa/implementation-2026-10-24/batch-2/runtime/candidate/poet/native/src/repl.rs"]\nmod repl;'
assert text.count(old)==1
text=text.replace(old,'use super::repl;',1)
(base/'actual-harness/poet_adapter.rs').write_text(text,encoding='utf8',newline='\n')
print('same adapter cases now target actual applied Poet module')
