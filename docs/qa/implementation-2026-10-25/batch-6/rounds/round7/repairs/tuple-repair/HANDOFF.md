# Compiler fixture tuple access

The initial actual native check failed after Core and Poet library checking.
Three new test expressions treated `compiled_assets` elements as `CompiledAsset`;
the public return type is `(String, CompiledAsset)`. The two-file repair selects
the tuple's `.1` element before reading the mesh or texture dependencies. Test
assertions and implementation behavior are unchanged. Rustfmt passed; compilation
and runtime results remain unclaimed until the rerun.

The failed command/log/source snapshot is retained under
`check-before-tuple-repair-*` at the round root. Exact preimages and candidates are
pinned by this repair's install manifest. No production asset was changed.
