# Core image vector review handoff

2026-10-26. Read-only source audit and new test-copy edits. No Cargo, canonical source, or existing staging files were changed.

## Owned review copies

- `qualiadb/crates/qualia-core-db/tests/image_composite_completed.rs` SHA-256 `6d4c2050e1e3fcea437cfe372880f1528550492ebe69b7842dcd1e433f295572`: fixed independent partial-alpha and zero-alpha-backdrop byte vectors for all nine non-Normal modes.
- `qualiadb/crates/qualia-core-db/tests/image_filters_completed.rs` SHA-256 `ef34281633239e742d35b50c8fbc4a22e97ec2fe62127b15ef2b3ae8266d516d`: 1x1 HueRotate/Emboss ordered sequence with distinct hand-calculated bytes. Both are copies of `work/integration/qualiadb/.../tests` with only the additional tests appended. Root may review/copy them before its next serial check.

The blend vector derivation uses source and backdrop alpha 128/255; for each channel the straight output is `[127*(Cb_byte+Cs_byte)+128*B_byte]/382`, with B evaluated from the W3C mode equation on unrounded channel fractions. The filter sequence derivation uses the SVG 90-degree hue matrix and a 1x1 clamped-gradient emboss, whose relief is 128. These fixed expected values are not computed from the implementation under test.

## Poet preflight boundary

The selected staged Poet adapter calls `recipe.budget_plan(limits)?` before its first image allocation (`work/poet-domain/native/src/graphical/surfaces/detail/render.rs:65`, `:129`, `:197`; source SHA-256 `d92cc26a8268f334e1b7ae4d436b71067ff2d4b480e161a82dc40731945953e0`). The DTO budget owner restricts side to 8..1024, validates rectangle extent with checked sums, caps layers/work/working bytes, and checks filter plans (`work/poet-domain/native/src/graphical/surfaces/detail/recipe.rs:355-405`, `:485-493`, `:527-588`; SHA-256 `d02c35fc1e4cc31c18cf01ce221d038fd84a20a5b4fb8f004e87a5fcfefa443b`). The adapter uses `ImageDocument::add_layer` and one temporary `ImageLayer::new` only after that preflight; it does not call Core `set_rect_mask`. Its dense rectangle masks are allocated after validated extent and working-byte admission (`render.rs:158-175`). Thus the selected Poet caller does not expose the legacy unchecked constructors to unbounded source dimensions. Other Core callers of `ImageLayer::new` or `set_rect_mask` remain a separate API-hardening concern; this review did not audit all callers.

Source audit pins: canonical Core `image.rs` SHA-256 `fc984b4e4868e6f08ce61dc2512b95ec5cbe82d098c6e16df5ebc5caa6d4973a`, `image_filters.rs` `d5304fe88d8404c5c6792602baac05a53028c3a8a5e16efb33bcdc7676d8d1ff`, `image_composite.rs` `ee2170db389066b9435257a1928e28f17732a4fc2c6df26acede842a1076d5b2`. Source inspection is not executed verification; root owns serial Cargo checks.
