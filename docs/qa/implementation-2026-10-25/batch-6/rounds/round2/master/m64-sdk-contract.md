# Bounded M64 SDK foundation contract

The new public `q42::m64_matter::checked_reader` child library will admit existing
little-endian M64 v1/v2 and E64 magic without changing layouts or saved artifacts.
`M64Reader::parse(bytes, limits, interval_scratch)` will copy unaligned headers and
records onto the stack, validate existing CRC validators, and expose borrowed raw
payload ranges. Caller limits bound file bytes, chunks and total records before
record work; caller-owned intervals permit iterative O(n log n) overlap checking
without heap allocation, recursion or a quadratic pair scan. Unknown flags/codecs,
unknown modes, bad lengths/offsets, metadata/payload overlap and corrupt CRCs fail.

Point and holo layouts have no embedded record checksum. Verification reports
their count separately from CRC-checked strata/phase records. It makes no Bao,
cryptographic authenticity, authority, semantic, scientific or physics claim.
Native 64-bit little-endian execution is the intended qualification target;
big-endian targets fail explicitly while POD codecs retain native LE semantics.

Negative cases: truncated header/directory/payload; arithmetic overflow; file-size
mismatch; excessive file/chunk/record limits; insufficient caller scratch;
misaligned logical offsets; directory/header or payload/metadata overlap;
payload/payload overlap including directory order permutations; unknown mode,
codec and container flags; count/byte-length mismatch; header/directory/strata/
phase CRC tamper. Offset-one input must work. Public tests cover every mode,
both versions and magic aliases, borrowed payloads and explicit checksum scope.

The geospatial producer will reject all unimplemented codecs before mutating
output, retain uncompressed output bytes, and describe its existing row-major
chunk emission plus Hilbert keys truthfully. Tests and execution are delegated to
the parent for serial checking; saved code is not an execution or acceptance claim.
