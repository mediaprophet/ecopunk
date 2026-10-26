# Independent frame SDK source review

Root reviewed the frozen decoder, typed adapter, compatibility-wrapper and three
consumer diffs on 2026-10-25. All tangent validation, count/byte limits and both
output capacities precede writes; malformed cold input is refused before
allocation. Active stream sizes are checked under the existing hard vertex cap,
which also bounds arithmetic on wasm32. Unaligned LE input avoids typed casts.
Existing signatures delegate and typed selection retains the canonical MID2
resolver. No blocking source finding in this scope.

Limits cover frame output/payload, not the whole mesh/scene or container CRC
traversal. Compatibility wrappers deliberately retain the old maximum-count
policy; the actual scene/portal consumers select explicit limits. New error enum
variants may require exhaustive downstream match updates. Whole-scene budgeting
and decomposition of the old compiler/portal files remain open. Authored tests
are not execution evidence; root's actual-source run is required.
