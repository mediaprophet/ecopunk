# Human-directed internal Poet library correction

The human clarified that existing MIT source libraries support Poet/Webizen/
QualiaDB implementation. They do not establish an integrated Craft suite,
product, UI, capability namespace or replacement authoring model. This direction
supersedes the earlier staging and naming plan; historical evidence is retained.

The saved internal libraries are `poet-geometry` and
`poet-spatial-authoring` under Poet's existing workspace. Both use explicit
version 0.4.0, edition 2024 and MIT declarations, preserving the original MIT
copyright/permission notice. No global workspace package/lint/resolver migration
was introduced. Source origin records distinguish original MIT code, the earlier
staging copy, and refactored support functions. Historical dual-option source
notices remain provenance only, not an adopted dual-licence assertion.

The active native module is `spatial_host.rs`, routed by `geometry_host.rs`.
The operator is explicitly `ComputationalGeometry.profile_sweep`. There is no
old-name capability alias. The old unmounted host file and exact QA preimages
are retained pending separately reviewed cleanup; originals and the staged
`C:/github/craft` directory were not moved or deleted.

The unused new ProfileDoc/revision wrapper was removed from the active helper.
Profiles are operator parameters. Existing Poet SourceSession, undo/redo,
cooperative authority and checkpoint owners retain all state. Prior Tool-Chest,
Manifold, ContextRef and intent behavior was not removed or redefined. The
existing `studio_manifold_seed` defines Dual Studio, scene and audio containers;
`ActionType::Invoke` already represents registered capability calls. The new
helpers support those responsibilities rather than substitute for them.

`save-receipt.json` and the library provenance map retain before/after hashes.
Cargo/member/dependency/lock changes are owned by the supervisor; assembly adapter
and reference source renames are owned by their existing worker. No Cargo, GUI,
credential-store access, payment or publication was performed by this refactor
worker. Source presence does not establish completed native graphical recovery
or human/model acceptance.
