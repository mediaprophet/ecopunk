# Inspect the bounded native material-key workflow

This is a manual verification route, not a record that a person has performed or accepted it. Use an existing project with its valid native admission and contributor credentials. The automated checks use isolated signed fixtures; they do not enrol a real contributor or supply a production project.

After the native build succeeds, launch the saved executable with your actual project directory:

```powershell
& 'C:\github\ecopunk\target\debug\poet.exe' studio --native-project '<your admitted project directory>'
```

1. Sign in through the existing native project workflow and open an accepted assembly source. Keep any unsubmitted source edits: submit them or explicitly load accepted source before using part controls.
2. Under **Assembly · parts, profiles and materials**, use **Inspect accepted assembly** and choose a part with an explicit linear material. Shared material edits affect all parts using that material.
3. Under **Material motion · explicit editorial samples**, use **Load / reload accepted track (replace local draft)**. Enter a key time and RGBA/roughness/metallic values, then **Add key from fields**. Add another distinct time and values. Interpolation belongs to the outgoing segment.
4. Set the playhead and use **Apply / save track and sample at playhead**. Inspect the accepted source revision and material values. Reload accepted track explicitly before making another accepted-track sample; successful writes retain the local form rather than silently replacing it.
5. Check Undo/Redo and the existing **Compile source, materials and parts into HMC** / project save-and-reopen workflow. Inspect retained `poet_material_motion` source and factors after reopen. Record the exact project, source/package identifiers, revisions and results when reviewing.

The corresponding admitted MCP backend has `poet_material_motion_inspect` and `poet_material_motion_edit`, using its source-read context and the same authoring operation. An unbound MCP process supplies no contributor authority. Inspect the published tool schema rather than inventing context or credentials.

Current support is explicit sample/apply for finite linear factors from 0 to 1, up to 8 material tracks and 64 ordered keys per track. Hold, Linear and CubicInOut are supported. Continuous playback, a compositing timeline and arbitrary EffectCraft project import remain open. No window rendering or human acceptance has been exercised in this implementation report.
