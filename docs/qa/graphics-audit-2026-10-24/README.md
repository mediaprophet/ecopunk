# Graphics audit evidence — 24 October 2026

Audit: [findings](../../planning/tech/ecopunk-graphics-audit-2026-10-24.md) · [tasks](../../planning/tech/ecopunk-graphics-tasks.md).

These are unedited screenshots from the published Ecopunk page, not generated concepts or new asset renders. The main browser is Chrome; the badge reports WebGPU. Exact deployed source identity, GPU adapter, driver and timing were not established.

| Capture | View |
|---|---|
| [01](01-published-continuum.png) | Default Talbingo continuum |
| [02](02-published-whole-organism.png) | Lab scale 1, whole organism |
| [03](03-published-stem-wedge.png) | Lab scale 2, stem |
| [04](04-published-flower-gumnut.png) | Lab scale 3, flower/gumnut |
| [05](05-published-leaf-histology.png) | Lab scale 4, leaf/tissue |
| [06](06-published-vertebrate.png) | Lab scale 5, default kangaroo; partially peeled |
| [07](07-narrow-viewport.png) | Narrow in-app browser showing layout failure |
| [08](08-published-noosa.png) | Noosa selection after returning from the vertebrate lab |

[browser-observations.json](browser-observations.json) contains viewport/canvas sizes, visible backend and console observations. Connection errors are unattributed. Later click timeouts are not performance measurements.

[evidence.json](evidence.json) records local HTML/JS/WASM hashes and all component reference checks. Published byte comparison failed at network fetch; no matching deployment identity is claimed. Component hash and byte-count checks do not verify the binary container format or visual quality.

Reproduce the read-only inventory with `node docs/qa/graphics-audit-2026-10-24/collect-evidence.mjs` from the repo root. It overwrites only this folder's `evidence.json`; preserve the dated evidence before rerunning after content changes.
