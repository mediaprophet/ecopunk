from stage import *
assert not (HERE/"PREPARATION-FROZEN.json").exists(), "Historical preparation only; install pinned source images, do not rerun generators"
s=get('native/Cargo.toml');s+='''
[[test]]
name="foundation_geometry_catalog"
path="../../qualiaDB/crates/vibe/tests/poet_geometry_catalog.rs"
required-features=["native-graphical"]
''';put('native/Cargo.toml',s)
print('Canonical geometry catalogue test consumer staged')
