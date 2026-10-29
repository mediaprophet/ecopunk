# Actual-probe-budgeted topology qualification

Status: staged for independent source review; rustfmt parsed; not compiled, installed or executed. The coordinator owns all integration and serial Cargo checks.

The additive topology builder charges each inspected hash slot to a caller-owned remaining-probe balance. The additive qualifier reserves the existing conservative fixed initialization, traversal and fan budget before invoking it; only hash probes are charged dynamically. An exhausted budget fails before the next hash probe. Scratch can be dirty and work consumed after a late error; callers must publish only an `Ok` receipt. Callers of the lower-level builder must separately bound initialization and non-probe work, including the actual supplied slot slice length. The old APIs and old error variants are unchanged. Success certifies indexed topology only, not geometry, self-intersection, UV, lineage or biological quality.

The existing bounds function still calculates the representable worst-case hash bound to derive fixed work; very large overflows fail closed. The new profile provides a bounded run, not a performance guarantee. Tests stage a colliding-key adversary and exhaustion, preflight sentinels, and compare new versus old receipt on the synthetic three-arm field under 2M abstract work. No production asset is made.

| File | Canonical preimage SHA256 | Staged SHA256 |
|---|---|---|
| QualiaDB topology.rs | c0f566cee0f9e89cb3c6b986ea82359199e58f8c6840507a8ed51fbc8f482249 | a6439f95c7fe3916469c8734350e43f0050174670ce37d3632f8d1600de38783 |
| QualiaDB surface_topology_qualification.rs | c5a1810e8bb2940351e7cba5247a061e8a34d7c98aebd8ba94b9199bf8650604 | 3ca4a2338a7ec23ad872d68ec728409878c94a546379fce2ff9e57cdd4de80d6 |
| QualiaDB computational_geometry/mod.rs | 712745423038c0f1cea9f1ec06874a5d69efec5e41c3246b9ec61361579a05c3 | 8c6f1ac527c3ee144c17e7509e3e73ac6b2a2257ce1ee53d96fd998e93f484e2 |
| Poet native/src/graphical/surface_topology_public_tests.rs | ac1f4a16bf4c160d2c202a0ec4352f074e709f94e7ebecbcb7238083e4ada5ee | 4a5a76e32e96ce114d03fa2d248d6a6135faf367f89b364dcd8694a6c409a74d |

Recheck preimages before integration; reconcile shared module exports with concurrent changes. The `2M` test limit is a fixture budget, not an asset quality ceiling. The operation has no authored-source promotion or Vibe/Poet source-profile integration.
