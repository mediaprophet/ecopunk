# Explicit retirement for non-retaining providers

Status: staged only; no canonical changes, compilation or execution. `manifest.json` pins exact candidate and target bytes. These files do not overlap the source-submit worker’s declared `cooperation/mod.rs`, `cooperation/editor.rs`, or `cooperation/tests.rs`.

`PreviewProvider::retire_retained_viewport` defaults to an error so a shared host can quarantine unretired state. `MeshPreviewProvider` stores only material, camera and limits; its render methods return freshly computed frames without retaining a viewport. Its staged override explicitly reports successful retirement. This does not alter or bypass the fail-closed default for providers whose retained state has not been established.

The eight `native_project_*` unit fixture providers inspected are unit structs whose preview methods return an error and retain no frame; the rig fixture’s staging work uses local temporary values. `SelectionProbe` stores an observation flag and selection handle for the test, delegates preview to `NoPixels`, and retains no viewport or frame. Their overrides allow safe no-op retirement in shared NativeProject tests. No production GPU provider, unknown provider or mutable retained preview gets a no-op hook through these candidates. The existing interactive provider has a real invalidating override.

The coordinator should verify each preimage and candidate hash and preserve old bytes before installation, compile and run the transaction and native-project regression tests, and confirm GPU behavior separately. No claim is made about source-submit atomicity, human usability, or GPU execution based on these source-only changes.
