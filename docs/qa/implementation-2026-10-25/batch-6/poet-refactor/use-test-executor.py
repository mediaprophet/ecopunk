from pathlib import Path
p=Path('C:/github/poet/native/src/graphical/poet_workspace/tests.rs')
text=p.read_text(encoding='utf-8-sig')
assert 'fn block_on<' in text
assert 'pollster::block_on' in text
p.write_text(text.replace('pollster::block_on','block_on'))
print('Native intent tests now use their safe std-only executor; no new runtime dependency.')
