# Manual verification route for Wave 2

This describes the implemented controls. It is not evidence of a performed human review or current-window rendering test. Use a valid admitted project and the existing native contributor login; fixture actors in the tests do not enrol a real person.

```powershell
& 'C:\github\ecopunk\target\debug\poet.exe' studio --native-project '<your admitted project directory>'
```

1. Open an accepted assembly source. Submit any source draft first, or explicitly reload accepted source; part controls preserve unsubmitted text and refuse to overwrite it.
2. Use **Inspect accepted assembly**, select a part, then **Inspect current selected UV and texture pixels**. Existing imported texture inputs must already be in an authorized, bound HMC resource snapshot; a path or digest alone grants no access.
3. Under **Editable texture and detail source**, load the accepted recipe. If it has no imported Paint recipe, enter the imported base SHA and exact square side, then explicitly **Start imported paint recipe**. This profile requires opaque sRGB base-colour pixels at their exact square size (8–1024).
4. Existing paint layers/strokes remain editable. In the post-composite effect controls choose Grayscale, Invert or Sepia, then **Add effect (paint version 2)**. Change operation, enable/disable, move up/down or remove a step. Sepia has finite intensity 0..1; Grayscale/Invert are full-strength. There are at most eight ordered effects.
5. Use **Apply shared texture/detail recipe**, then inspect accepted texture pixels. The selected inspection summary includes the actual encoded base-colour SHA-256. Source carries the original digest, retained layers/strokes and effect steps. Invalid numeric text remains visible and prevents applying stale numeric values.
6. Check Undo/Redo and explicit execution rebuild. Use the existing source/materials/parts HMC compile workflow to retain generated outputs and original dependencies. Reopen the saved HMC, explicitly bind its authorized resource snapshot and rebuild, then compare accepted recipe, resource digests and pixels. Plain source save by itself does not assert that a new generated resource was compiled into the package.

Reload is explicit: source/selection/context changes retain a local draft and prevent applying it to a new target. Reload replaces that local draft. Shared material edits affect all parts sharing the material.

Agents admitted through the same project can call `poet_texture_effects_inspect(context, part_id)` and `poet_texture_effects_edit(context, material_id, effects)`. The latter changes only effects/version on an existing Paint recipe. Read a fresh source context after a mutation attempt; malformed parameter preflight does not consume it. An unbound MCP process has no project authority.

The retained `diagnostic/*.png` files are actual encoded results from an 8×8 synthetic test input, not production assets or biological references. Full compositor, RAW/ICC/linear-light processing, current-window/GPU quality and actual contributor/reviewer acceptance remain separate open work. Asset production remains HOLD.
