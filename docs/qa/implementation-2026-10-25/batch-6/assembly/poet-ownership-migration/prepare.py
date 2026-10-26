"""Prepare exact current-source ownership migration; historical candidates stay intact."""
from pathlib import Path

base = Path(__file__).parent / 'candidate'
replacements = {
    'mod.rs': [
        ('shared Craft assembly domain engine', 'internal Poet spatial authoring engine'),
        ('craft_authoring::', 'poet_spatial_authoring::'),
    ],
    'admission.rs': [
        ('Craft owns assembly transforms/topology.', 'Poet spatial authoring owns assembly transforms/topology.'),
        ('craft_authoring::', 'poet_spatial_authoring::'),
        ('CraftPart', 'AssemblyPart'),
        ('native Craft assembly:', 'native Poet assembly:'),
    ],
    'tests.rs': [('actual_craft_hierarchy_', 'actual_native_hierarchy_')],
    'materials.rs': [('shared native Craft assembly projection', 'shared native Poet assembly projection')],
    'master.vibe': [('Craft.profile_sweep', 'ComputationalGeometry.profile_sweep')],
    'README.md': [
        ('source revision 0.1.', 'source revision 0.2.'),
        ('native Craft/Core operations', 'Poet native geometry capabilities over QualiaDB kernels'),
    ],
}
for name, edits in replacements.items():
    path = base / name
    text = path.read_text(encoding='utf-8-sig')
    for old, new in edits:
        if old not in text:
            raise SystemExit(f'Migration input changed: {name}: {old}')
        text = text.replace(old, new)
    if name == 'master.vibe':
        text = '// Source revision 0.2: internal Poet geometry namespace; authored shapes unchanged.\n' + text
    if name == 'README.md':
        text = text.replace('`master.vibe` is the editable native Vibe source.',
            'The 0.2 source revision replaces the unreleased external authoring namespace\n'
            'with `ComputationalGeometry.profile_sweep` hosted by Poet. Part keys, semantic\n'
            'identifiers, dimensions, shapes and preset parameters are unchanged. MIT\n'
            'library provenance remains in the integrated library origin records.\n\n'
            '`master.vibe` is the editable native Vibe source.')
    path.write_text(text, encoding='utf-8', newline='\n')
