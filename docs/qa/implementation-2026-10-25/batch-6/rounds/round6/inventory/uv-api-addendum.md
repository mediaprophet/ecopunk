# Round 6 UV API addendum — profile mesh coordinates

**Read-only source inspection:** 2026-10-25. Live-source SHA256 values are at the end. No build, test, implementation, or sibling write was performed.

## Canonical public APIs found

1. `qualia_core_db::render::surface_authoring::planar_uvs_into(positions: &[[f32; 3]], axes: [usize; 2], metres_per_repeat: f32, out: &mut [[f32; 2]]) -> Result<usize, SurfaceAuthoringError>` writes plane-projected UV coordinates from selected Cartesian axes, preserving input vertex cardinality. Public module: `render::surface_authoring` (`render/mod.rs`). It does not take triangles, so it cannot split hard boundaries or seams; on a sweep it is a planar projection, not a profile-aware surface parameterization.

2. `qualia_core_db::render::mesh_surface_frames::box_project_mesh_into(positions, triangles, source_normals, metres_per_repeat, smooth, out_positions, out_triangles, out_normals, out_uv, chart_vertices) -> Result<SurfaceFrameReceipt, SurfaceFrameError>` creates triangle charts, duplicates vertices at UV seams, preserves supplied source normals, and retains triangle order. The exact parameter types are in `render/mesh_surface_frames.rs`; `box_project_mesh_with_sources_into` adds `mut source_vertices: Option<&mut [u32]>` to return original-vertex lineage, which is needed to remap UV-split vertex-aligned fields. Public module: `render::mesh_surface_frames` (`render/mod.rs`). This is a box/dominant-face projection, not a general unwrap; inspect output sizing/budget requirements in source before use.

## What was not found

No public cylindrical UV generator/projection or general mesh UV unwrap API was found in the inspected Core `render`, `container_10d`, computational-geometry sources, or Poet geometry/spatial authoring paths. No such operation is wired in Poet. `spatial-authoring::profile_sweep` calls Core `extrude_profile_along_path` and returns Core `Mesh` positions+triangles; it does not return UVs. Poet's current mesh adapter also returns only `Mesh`, and the renderer-side GPU upload path's zero/default UV behavior is not authored UV output.

Thus the immediate reliable options from existing APIs are (a) `planar_uvs_into` for simple basis projection with unchanged topology, or (b) `box_project_mesh[_with_sources]_into` for explicit chart seams with remapped topology/source lineage. For actual profile wall/end-cap UVs preserving semantic design intent, add a domain-appropriate generated UV output (likely where authored sweep stations/profile correspondences are available) in the owning geometry layer and route it through Poet + the existing Core UV `.10d` section codec. Do not call transformed/projected UV a general unwrap.

## Source hashes

- `148D3CA2F1586EC2C3F4C9B7815316B09A0AF6EE0D761484C903F41ED7BA88D0` — `C:/github/qualiaDB/crates/qualia-core-db/src/render/surface_authoring.rs`
- `2ABFB7DE6E29950F2119D7E8D2C28AA618CC5F52FC1C7766DE612A90BD3F933E` — `C:/github/qualiaDB/crates/qualia-core-db/src/render/mesh_surface_frames.rs`
- `FD9C9EA10F81869488AEAD648F2084715961CF338ECF49AB040639FBFB7E287A` — `C:/github/poet/crates/spatial-authoring/src/geometry.rs`
- `E2A5375C6B448A82662E1FE3062497E21C88EAF6ED14C69B8F1671D9ECB02794` — `C:/github/poet/native/src/graphical/spatial_host.rs`
