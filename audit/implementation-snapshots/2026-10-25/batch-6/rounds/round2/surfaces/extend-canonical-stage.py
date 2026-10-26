"""Extend staged candidates only; never write a Core destination."""
from pathlib import Path
import hashlib,json
HERE=Path(__file__).resolve().parent
CORE=Path('C:/github/qualiaDB/crates/qualia-core-db')
manifest=HERE/'sdk-save-manifest.json'
rows=json.loads(manifest.read_text())
relative='src/container_10d/section.rs'
before=(CORE/relative).read_bytes()
text=before.decode();newline='\r\n' if b'\r\n' in before else '\n'
text=text.replace('\r\n','\n')
def replace(old,new):
    global text
    if text.count(old)!=1:raise RuntimeError('Unexpected canonical source anchor: '+old[:50])
    text=text.replace(old,new)
replace('use bytemuck::{Pod, Zeroable};','#[path = "section/table_validation.rs"]\nmod table_validation;\n\nuse bytemuck::{Pod, Zeroable};')
replace('    /// A descriptor\'s `reserved16` is non-zero.','    /// Borrowed descriptor bytes are not aligned for the zero-copy table view.\n    UnalignedSectionTable,\n    /// A nonempty payload intersects the container header or section table.\n    SectionOverlapsMetadata { index: usize },\n    /// A descriptor\'s `reserved16` is non-zero.')
replace('            Self::NonZeroDescriptorReserved { index } => {','            Self::UnalignedSectionTable => write!(f, "10d section table is not aligned for borrowing"),\n            Self::SectionOverlapsMetadata { index } => write!(f, "10d section {index} overlaps header or section table"),\n            Self::NonZeroDescriptorReserved { index } => {')
replace('    // SAFETY: SectionDescriptor is repr(C) + Pod + size 24 with no padding;\n    // the table slice is byte-aligned and fully within `data`.\n    let descs: &[SectionDescriptor] = bytemuck::cast_slice(&data[table_start..table_end]);','    // Return a clear error for unaligned borrowed bytes instead of panicking.\n    let descs = table_validation::borrow_table(&data[table_start..table_end])?;')
replace('    for (i, d) in descs.iter().enumerate() {\n        if d.reserved16 != 0 {','    for (i, d) in descs.iter().enumerate() {\n        if descs[..i].iter().any(|old| old.section_type == d.section_type) {\n            return Err(SectionTableError::DuplicateSectionType { section_type: d.section_type });\n        }\n        if d.reserved16 != 0 {')
replace('        // Stride consistency.','        table_validation::metadata_range(d, i, table_start, table_end)?;\n        // Stride consistency.')
start=text.index('    // Padding-between-sections is zero: scan the gaps.')
end=text.index('\n    Ok(descs)',start)
text=text[:start]+'    // Focused canonical validation also supports payloads before a later table.\n    table_validation::padding(data, descs, table_start, table_end)?;\n'+text[end:]
for dirname,bytes_ in [('before',before),('after',text.replace('\n',newline).encode())]:
    target=HERE/dirname/relative;target.parent.mkdir(parents=True,exist_ok=True)
    if target.exists():raise RuntimeError('Snapshot exists: '+str(target))
    target.write_bytes(bytes_)
rows.append(dict(path=str(CORE/relative),relative=relative,before_sha256=hashlib.sha256(before).hexdigest()))
child='src/container_10d/section/table_validation.rs'
assert not (CORE/child).exists()
rows.append(dict(path=str(CORE/child),relative=child,before_sha256=None))
resolver=HERE/'after/src/container_10d/typed_payloads/resolver.rs'
text=resolver.read_text()
text=text.replace('    UnalignedSectionTable,\n','').replace('    SectionOverlapsTable { index: usize },\n','')
start=text.index('        // The shared zero-copy table parser requires native descriptor alignment.')
end=text.index('        let has_manifest =',start)
text=text[:start]+'''        let sections = parse_section_table(data, &header).map_err(TypedPayloadError::Sections)?;
'''+text[end:]
resolver.write_text(text)
manifest.write_text(json.dumps(rows,indent=2)+'\n')
print('Canonical table validation staged; resolver delegates without private parser guards')
