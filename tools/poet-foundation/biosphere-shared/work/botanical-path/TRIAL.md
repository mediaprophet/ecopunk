# Candidate native botanical diagnostic — not yet executed

This is a synthetic engineering diagnostic, not a species model or production asset. The source and parameters below reproduce the source fixture in `C:/github/poet/native/src/graphical/native_project_biosphere_round1_tests.rs`; these UI steps have not been executed and the retained executable has no matching source/build receipt. Failure at any step must be recorded without treating this document as evidence of a successful trial.

## Launch

Use a new, empty existing directory under a location permitted for project output. Native project creation writes project configuration, a secure local controller profile, an authority journal and an initial `working.hmc`; do not select or overwrite an existing project. The operator must make their own actual project and consent decisions. Storage use is unmeasured.

In PowerShell, in the launch process set `$env:POET_NATIVE_BOTANICAL_PROFILE = "1"`, then invoke `C:/github/ecopunk/target/debug/poet.exe studio --native-project <absolute-existing-directory>`. The profile is read at creation and again on fresh login; retain it in each launching process. The current executable may not contain the inspected source; if it fails, record its hash, message and step. Never assume compatibility.

The shell prompts for project name, local controller profile label, evidence reference, capacity and explicit local authority consent. The person using it supplies their own truthful values and decides whether to create/adopt the project. Do not assert a completed review or independent human verification from profile creation.

## Candidate operation

1. In the Vibe source draft, paste the fixture below, then select **Submit source**. Record errors; do not replace existing accepted user source in another project.
2. Select **Inspect accepted assembly**, choose `branch`, then **Load accepted recipe / start blank entry**. Paste the JSON below and select **Adopt graph for selected part**.
3. Inspect the accepted assembly again, select `branch` and choose **Load / reload accepted graph**. Change tip radius from `0.02` to `0.04`, choose **Apply pipe radius**, then refresh the viewport and inspect the accepted graph again. A visible change is expected but has not been observed by this work.
4. Choose a fresh checkpoint filename (for example `botanical-diagnostic.hmc`) and **Save accepted source as new HMC**. Close Poet. Relaunch with the same profile flag and project path, adopt the actual project authority if prompted, sign in using the same local profile and enter that exact HMC filename. Inspect the branch/graph and viewport; verify the reopened radius and geometry against what was saved. Actual behavior remains unverified.
5. Record the executable digest, project and output identifiers/digests, actions, refusal/errors and the actual operator. Stop on error; preserve the working source and logs. A successful bounded trial would not accept a model or close the cooperative/production gates.

## Test fixture source

```vibe
let m={id:1,colour_space:"linear",base_color:[0.3,0.4,0.5,1.0],emissive:[0.0,0.0,0.0],metallic:0.1,roughness:0.5,specular:1.0,alpha_cutoff:0.5,opacity_mode:"Opaque",double_sided:true};
let tri={vertices:[[0.0,0.0,0.0],[1.0,0.0,0.0],[0.0,1.0,0.0]],triangles:[[0,1,2]]};
let mut poet_parts=[
  {id:"branch",semantic_id:101,geometry:tri,material:m,translation:[0.0,0.0,0.0],rotation:[0.0,0.0,0.0],scale:[1.0,1.0,1.0]},
  {id:"unrelated",semantic_id:102,geometry:tri,material:m,translation:[2.0,0.0,0.0],rotation:[0.0,0.0,0.0],scale:[1.0,1.0,1.0]}
];
({parts:poet_parts});
```

## Synthetic recipe entry

```json
{
  "version": 1,
  "part_id": "branch",
  "organs": [
    {
      "key": "root",
      "parent": null,
      "position_m": [
        0,
        0,
        0
      ]
    },
    {
      "key": "axis",
      "parent": "root",
      "position_m": [
        0,
        0,
        1
      ]
    },
    {
      "key": "lateral",
      "parent": "axis",
      "position_m": [
        0.5,
        0,
        1.5
      ]
    },
    {
      "key": "tip",
      "parent": "axis",
      "position_m": [
        -0.5,
        0,
        1.5
      ]
    }
  ],
  "attractors_m": [
    [
      0.6,
      0,
      1.9
    ],
    [
      -0.6,
      0,
      1.9
    ]
  ],
  "node_length_m": 0.2,
  "kill_radius_m": 0.05,
  "perception_radius_m": 0.8,
  "cone_angle_rad": 6.283185307179586,
  "max_iterations": 2,
  "tip_radius_m": 0.02,
  "pipe_exponent": 2,
  "ring_slices": 8
}
```

## Source inspection and status

Inspected native entry `create.rs`, `runtime.rs`, `ui.rs`, graphical `ui.rs`, `viewport_controls.rs`, `authoring/controls.rs`, `authoring/botanical/{controls.rs,recipe.rs}` and `native_project_biosphere_round1_tests.rs`. The host profile is an explicit environment opt-in; source-level intent exists but there is no observed native-window human execution. No build, test, launch, source file installation, project or asset creation was performed by this work.
