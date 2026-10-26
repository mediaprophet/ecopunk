from pathlib import Path
p = Path(__file__).parent / 'after/native/src/graphical/cooperation/georaster_edit_tests.rs'
s = p.read_text()
a = s.index('const FILE:')
b = s.index('#[test]')
support = s[a:b].replace('\nfn ', '\npub(super) fn ')
for name in ('FILE', 'KEY', 'SOURCE'):
    support = support.replace(f'const {name}:', f'pub(super) const {name}:')
(p.parent / 'georaster_edit_support.rs').write_text(
    '//! Ephemeral synthetic grids built with public canonical codecs.\nuse super::*;\n\n' + support)
p.write_text(s[:a] + '#[path = "georaster_edit_support.rs"]\nmod support;\nuse support::*;\n\n' + s[b:])
