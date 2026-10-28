# Closed indexed surface topology qualification — staged

Status: staged and syntax-parsed, **not compiled or executed**. No canonical edits or Cargo by this worker. Coordinator owns integration and checks. The target is the installed QualiaDB `indexed_tetra_surface` sampled-field extractor; this is a separate optional post-extraction qualification, not a change to that API or a Poet authoring workflow.

## Existing owners and scope

QualiaDB `topology::build_triangle_half_edges` already builds oriented twins, rejects duplicate directed/nonmanifold edges and reports boundaries using caller-provided buffers. `connectivity::label_components` labels face components through those twins. The candidate uses these existing owners, requires a nonempty input, no boundary, exactly one face component, every declared vertex used, and a single closed incident-face fan per vertex. The fan is traversed with the half-edge `next(next(h)).twin` transition and a full visited check catches a second disconnected fan at the same vertex. It does not modify source triangles, and all workspace belongs to the caller; there is no runtime heap allocation.

The caller receives explicit sizes and a conservative work bound from `surface_qualification_bounds` and must supply at least that `max_work`. The bound includes worst-case open-addressing scans in the existing topology owner and an O(F²) bound for the existing face-component labeller. It can be impractically conservative for large production meshes; no measured runtime or scalability result is claimed. Work/size/short-scratch preflight refuses before modifying scratch. Once existing owners execute, errors may mutate scratch and must not be published as a success.

The receipt is topological only: it does **not** prove finite coordinates, nonzero geometric area, no coincident vertices/edges, no self-intersections/foldovers, taper, UV/normal continuity, durable semantic identifiers, biological quality, native Poet usability or a complete asset. The field extractor already has its own finite/edge preflight, but surface geometry and authored lineage remain separate work.

## Candidate pins and integration targets

| Candidate | Intended target | Existing SHA-256 | Candidate SHA-256 |
|---|---|---|---|
| `surface_topology_qualification.rs` | `C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/surface_topology_qualification.rs` | absent | `6708c57283738f3b4922a110fc0e7ada54f9c15ce173b8eb74288a652f2efe83` |
| `core_mod.rs` | `C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/computational_geometry/mod.rs` | `b4ab4b2cd66683d35ebfb9c717664d641e72259aaea09f5382b75cc54cf51776` | `8303428cdf01364c11fa0fa64bd0f19845127437aebe70dc7fe6c23abeaab233` |
| `poet_surface_topology_tests.rs` | `C:/github/poet/native/src/graphical/surface_topology_public_tests.rs` | absent | `81444ad9243c517f938009f6930a89e0443a1de749ff9fe44759030823527928` |
| `poet_mod.rs` | `C:/github/poet/native/src/graphical/mod.rs` | `7aa6d84cc794bee6b3c0021937a04406e065dad2f59b5fad0c90c502701f9ef3` | `1e514037d05d804444eb200d8aedbc9da923b1ede031e6bd364f5bf75fcefa1c` |

Inspected owner pins: `connectivity.rs` `dd0681ae646b1bea1242b9ff45f16d1fba789781b86688e3d9809ad7fee3bf5b`; `topology.rs` `c0f566cee0f9e89cb3c6b986ea82359199e58f8c6840507a8ed51fbc8f482249`. Verify all target preimages after other agent changes; merge shared module registration rather than overwrite.

Public-caller tests stage a valid tetrahedron/icosahedron, open boundary, disconnected closed components, unused vertex, connected pinched icosahedron with valid paired edges but two disjoint vertex fans, explicit work/short-buffer preflight preservation, size overflows, and the installed indexed-tetra extractor on the synthetic three-arm SDF field. The latter is a constant-radius, ordered-blend sampled field; neither its topology qualification nor this candidate is executed or evidence of species/visual acceptance. Rustfmt parsing is not compilation.

Independent reviewer checked fan traversal and conservative owner costs by source inspection and found no definite blocker, while warning of the O(F²) scalability constraint. Independent public-caller review found no obvious API/type or fan-rotation defect; the synthetic branch fixture checks topology, not geometry, intersection absence or visual quality. Run the public-caller tests through the coordinator's serial Poet Cargo workflow; dependency-crate unit tests do not run automatically. No human, GPU, production asset, or HMC acceptance is asserted.
