# Independent review — image completion

The selected fifteen operations are implemented in the owning QualiaDB image module, with Poet controls, source recipes, resource resolution and MCP adapters. Runtime qualification is recorded separately; source review is not a test pass.

Review corrections retained in the final candidates:

- Overlay and HardLight initially selected the opposite branch variable. Corrected the operator calls against unchanged independent opaque vectors.
- A Gaussian conservation fixture was too narrow for its kernel; replaced it with a centred nine-pixel impulse and documented the independently calculated, byte-rounded distribution.
- Precomputed Gaussian weights once per call in bounded stack storage instead of recalculating exponentials for every pixel.
- Native layer selection previously retained another layer's loaded stroke. Each layer now owns its pending stroke form and selected stroke. Layer navigation preserves invalid raw text; explicit stroke reorder/removal loads the selected stored stroke and reports discarded pending text. Mounted event regressions qualify these paths.
- Extended independent reference vectors to partial alpha and transparent backdrops for every blend mode, plus a noncommuting hue/emboss sequence.
- The first GPU refusal fixture tried to inspect after a missing-resource evaluation held execution. It now separately proves native refusal, exact accepted source/history, authorised MCP rebuild, identical recovered GPU output, MCP refusal from current execution, and another rebuild before parameter-context reuse. The original context-reuse oracle is retained. The passed positive workflow is byte-pinned unchanged and reused.
- The requested broader Core library run exposed two existing test compilation blockers: FIEL's checked test module path pointed to the wrong directory, and a source-carrier test held a mutable resource borrow across immutable bundle encoding. Corrected the path to the existing tests and cloned the changed test fixture to end its borrow. No runtime parser, archive validation or test corruption oracle was altered.

## Tracked adjacent API hardening

`qualia-core-db/src/hypermedia_authoring/image.rs`: existing `ImageLayer::new` and `set_rect_mask` allocate from dimensions without accepting a caller budget. They are **not** general untrusted-input entry points. The selected Poet adapter calls `recipe.budget_plan` first, checks side 8–1024, layers, points, filter work and memory, and never calls `set_rect_mask`. The new filter/compositor kernels accept checked budgets and caller buffers; cold filter/composite wrappers validate before allocation.

Follow-up `PA-03-API-ALLOCATION-HARDENING`: add fallible dimension/storage/budget-checked layer construction and rectangle-mask mutation APIs, migrate other untrusted callers after their inventory, and test hostile dimensions without allocating. This is adjacent existing API debt, not an undisclosed implementation of any of the fifteen cases. Do not claim a repository-wide allocation audit.

## Qualification limits

Encoded RGBA8 artistic filters/compositing; no colour-management, linear-light, physical or biomedical accuracy claim. Poet's existing admitted imported-paint profile remains opaque square sRGB base colour. Masks consume explicit raw red or alpha coverage from retained HMC resources. Core supports partial-alpha image operations independently.

No native window was launched. Mounted component events and native GPU readback are engineering checks; human usability/domain acceptance remains unassessed. Asset production remains **HOLD**, with no model/package completion claim.
