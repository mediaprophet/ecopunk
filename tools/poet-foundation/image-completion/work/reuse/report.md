# Image operation reuse and notice trace

Inspected for the 2026-10-26 deferred image-case implementation. This is a
bounded source trace, not a repository-wide search or legal opinion. No source
was copied or changed, and no tests or builds were run.

## Scope and source pins

The operative deferred inventory is
`tools/poet-foundation/texture-effects/work/deferred-functionality.json`
(15 entries; source pin for the original `image.rs` finding is
`6D492B8DB1E1A78E2F43BD95BC0A9FD2B7DA9DCC054FCA7EAEAE8E789F0AAB49`). The
scope contract is `tools/poet-foundation/image-completion/CONTRACT.md`.

Inspected sibling checkout `C:/github/qualiaDB`, limited to:

| Source | SHA-256 | Relevant fact |
|---|---|---|
| `crates/qualia-core-db/src/specialized_libs/computer_vision/cv/filter/gaussian_blur_u8.rs` | `7A128E9D8B96F40DC06671D33734929F5A2C19B6442628ADE0A35ADF59A88FF9` | `gaussian_blur_u8(src: GrayView<'_>, out: &mut [u8]) -> Result<(), CvError>`; fixed 3×3 `[1,2,1;2,4,2;1,2,1]`, clamped edges, integer floor division by 16. Gray8 only. |
| `crates/qualia-core-db/src/specialized_libs/computer_vision/cv/filter/mod.rs` | `D9A9CFCEE1D782E56A1B949B60A1DB0CA452BDDF185B033942A21E17F2788520` | Public module and re-export for the Gaussian function alongside box/median blur. |
| `crates/qualia-core-db/src/specialized_libs/computer_vision/cv/mod.rs` | `03CC753A54E9204E12285757AF277D9BE069ACA743735E8773CA2983E9049416` | `pub use filter::{box_blur_u8, gaussian_blur_u8, median_blur_u8}`; module also exposes `GrayView` / `RgbView`. |
| `crates/qualia-core-db/src/specialized_libs/computer_vision/cv/buffer/gray_view.rs` | not pinned in this pass | `GrayView::new(width,height,stride,bytes) -> Option<Self>` checks nonzero dimensions, stride and byte length; `pixel(x,y) -> u8`. |
| `crates/qualia-core-db/src/poet_host/invoke/vision/cv.rs` | `12FAA8579E46E392B8F37AB9668BF3C658A6AC81FC9BEE09E35290535BA7DEBC` | `ComputerVision.gaussian_blur` host call parses grayscale bytes/dimensions/stride, allocates output, invokes the typed grayscale kernel, and returns a value record. |
| `crates/qualia-core-db/Cargo.toml` | not pinned in this pass | Package metadata names `license-file = "LICENSE"`. |
| `crates/qualia-core-db/LICENSE` | `0AB3C88349C20A896BEB3BB5FC9A2F1087B41A5CA32B961C09CCE5457C8E12E6` | Package notice text identifies CC BY-NC-ND 4.0 International. Preserve the actual notice and obtain appropriate rights review before redistributing literal source; this report is not legal advice. |

The first four implementation files establish a real CoreCV operation and a
Poet host caller, but neither changes its representation from grayscale bytes
to the image-editor's `[u8;4]` RGBA pixels. The host wrapper is not a suitable
allocation/validation seam for the requested caller-owned-buffer filter API.
Its fixed radius/kernel also does not implement the requested sigma-parametric
Gaussian semantics. It may be useful as a separately named grayscale
operation; it should not be substituted as the requested Gaussian blur.

## Exact typed-operator check

Targeted symbol searches covered `qualia-core-db`'s
`specialized_libs/computer_vision/cv` and `hypermedia_authoring` trees for
RGBA8 blur/filter functions and `sharpen`, hue rotation, and emboss names.
Within that inspected scope, I found no exact existing parametric RGBA8 typed
operator for Gaussian blur, sharpen, hue rotate, or emboss. This is a bounded
finding, not proof of absence elsewhere in QualiaDB. The existing public typed
Gaussian is the GrayView/u8 operation above; there are no exact typed
parametric RGBA candidates in this inspected scope to route these four image
cases to. Do not claim a global absence from this trace.

For equations and any new implementation, the image-completion contract
specifies fresh code over the authored parameters. Prefer referencing the
existing CV API only where the image-domain semantics actually match. If any
source body is copied in later work, carry its source notice and package
license attribution with that copied material; no copied-body exception or
license conclusion is made here.
