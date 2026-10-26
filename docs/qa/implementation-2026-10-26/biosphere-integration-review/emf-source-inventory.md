# EMF and sensory cue source inventory

**Scope:** read-only inventory for the 2026-10-26 biosphere integration review. Sources in `C:/github/QualiaDB` and `C:/github/etalon` were inspected; no builds, tests, runtime execution, or edits to those repositories were made. References below use exact source paths and line numbers. Source presence does not establish execution, qualified native Poet exposure, or validated biology.

## Existing source owners and semantics

1. **Etalon EMF visualizer shader** — `C:/github/etalon/crates/etalon-render/src/emf.rs:9-17,21-60,68-99,171-197,201-225`. It samples a grid indexed `[t][z][y][x]`; each cell is 3 field `f32`s (`amplitude`, `phase`, `frequency`) plus ten epistemic/manifold `f32`s (52 bytes). The shader maps phase to σ then CIE 1931 XYZ/linear sRGB and amplitude to display brightness/opacity. `frequency` is stored/interpolated but is not used by the shown visual projection. This is a spectral-color visualization contract, not arbitrary electromagnetic radiation transport or sensor capture. Etalon has this source; no native Poet route/exposure was established by this inventory.

2. **Qualia shared σ visual/acoustic mapping** — `C:/github/QualiaDB/crates/qualia-core-db/src/render/acoustic.rs:21-36,38-63`. `sigma_to_wavelength_nm` maps fractional σ to 400–700 nm; `sigma_to_center_frequency_hz` maps that range linearly to 1760–110 Hz. `phenomenal_acoustic_params` carries α, μ, position, v, w, q and preview bins; `phenomenal_voice_frequency_hz` blends σ-derived pitch with a preview-bin estimate. This is authored cross-modal sonification, not a measured acoustic response or biological sensory transfer function.

3. **Acoustic audio input view** — `C:/github/QualiaDB/crates/qualia-audio/src/types.rs:19-29,42-52`. `AudioView` has borrowed bytes, frame count, channel count, integer sample rate, frame-stride bytes, and sample format; well-formedness checks nonzero dimensions/rate and sufficient byte length. Units are frames/channels/Hz/bytes; source does not declare an ultrasonic or infrasound range.

4. **Streaming STFT** — `C:/github/QualiaDB/crates/qualia-audio/src/features/stft_stream.rs:5-21,39-47,88-98`. `StreamingStft` exposes integer frame size and hop, accepts mono `f32` samples, and writes frame-major magnitudes with `frame_size/2+1` bins. Sample-rate/frequency calibration is absent from this API; bin count alone does not establish a physical frequency axis.

5. **Log-mel projection** — `C:/github/QualiaDB/crates/qualia-audio/src/features/stft_stream.rs:100-139`. `log_mel_from_mono` accepts sample rate in Hz and outputs `n_mel` natural-log mel-band values per frame; its filterbank spans 0..Nyquist. This is a derived audio feature representation, not a complete spectral capture.

6. **Constant-Q magnitudes** — `C:/github/QualiaDB/crates/qualia-audio/src/features/cqt.rs:5-43`. `forward_cqt_mono` accepts mono samples, sample rate and minimum frequency in Hz, bins per octave, bin count, and caller output array. Bin center `k` is `f_min * 2^(k/bins_per_octave)`; output is magnitude `f32`. No biological hearing bounds are declared here.

7. **Stored audio media metadata** — `C:/github/QualiaDB/crates/qualia-audio/src/media_store.rs:18-27`. `AudioMediaRecord` stores digest, byte length, sample rate (Hz), channels, frames, retention class, and digest integer. It does not contain calibrated sound pressure level, frequency response, source identity semantics, or species-response evidence.

8. **Thermal response kernel** — `C:/github/QualiaDB/crates/qualia-core-db/src/render/physics/response/thermal.rs:4-25,31-53`. `ThermalTargets` contains two `f64` target temperatures and two rates; rates are explicitly inverse seconds. `step_thermal` integrates two temperatures for bounded `dt`. The source does not declare temperature units or environmental sensor coupling; target selection/calibration is caller/domain responsibility.

9. **Generic material mass/compression** — `C:/github/QualiaDB/crates/qualia-core-db/src/render/physics/material.rs:6-22,25-47`. `Material` holds density in caller-defined mass/volume units and an incompressible flag; `Body` computes mass from density times AABB volume. This is not organism-specific hydration/turgor, humidity, or atmospheric-pressure sensing.

10. **Poet-exposed projection example** — `C:/github/QualiaDB/crates/qualia-client-core/src/wellfair/anatomy_view.rs:55-71,98-100,157-175` (source paths/lines). `SystemPercept` serializes `system_id`, σ over 0..1 described as 400–700 nm, RGBA, and a sonic center frequency in Hz. Its comment describes a single burden-derived dual-modality cue. This is an application/client view structure; this inventory did not establish native Poet authoring/execution exposure, nor a biological sensory validity claim.

## Ecopunk contract context and limits

- `docs/planning/tech/tensor-10d.md:89-115` states spectral payload intent (SPD visual, STFT/CQT audio; RGB is projection) and distinguishes temperature/pressure baking prose from additional tensor axes. It is design intent, not proof that the current code stores calibrated optical spectra or environmental sensor series.
- `docs/planning/tech/assets/digital-twin-asset-requirements.md:222-230,332-334` requires applicable physical response/environment coupling and object-linked sound/media; model/source claims need sourced units and actual response evidence.
- `docs/planning/tech/ontology-stack.md:155` names `.emf` spectral & Holosonic wavefield media (`kind = "emf"`); the suffix/name alone does not specify typed physical fields, frequency axis, calibration, or a biological interpretation.
- `docs/planning/tech/engine-requirements.md:345,354` characterizes typed physical/material primitives and acoustic/media integration as source-confirmed substrate with model/host or per-object integration open.
- No hyperspectral cube implementation, calibrated broad EM radiation field, humidity/air-pressure sensory process, or validated species-specific IR/UV/ultrasound/infrasound response was established in this bounded search. The existing Etalon shader and shared σ→sound mapping should be reused only for their demonstrated display/sonification semantics. No species response should be inferred from a symbol, extension, field label, or frequency mapping.

## Coverage boundary

Inspected the listed QualiaDB audio/render/thermal/material/application view sources, selected Ecopunk spectral/physical/media contracts, and Etalon's EMF shader. Other QualiaDB crates and Etalon consumers/backends were not exhaustively searched; no statement here establishes absence beyond the named files and bounded searches. Native Poet exposure and actual runtime wiring remain unqualified.

## Additional canonical QualiaDB spectrum and field owners

11. **Unified public spectral facade** — `C:/github/QualiaDB/crates/qualia-core-db/src/render/spectral_operator.rs:31-78,96-119,142-161`. `SpectralOperator` exposes associated APIs for `[alpha, mu, sigma]` → `Spd` → CIE XYZ → linear/display sRGB, metamer operations, and spectral blending. The facade delegates to specialized modules; its API presence does not by itself establish native Poet exposure or execution.

12. **SPD sample grid and EMF reconstruction** — `C:/github/QualiaDB/crates/qualia-core-db/src/render/spectral_kernel.rs:42-52,92-100,233-252,258-275`. `Spd.samples` is a fixed `[f32; 41]`, indexed from 380 through 780 nm at 10 nm intervals. `emf_to_spd(alpha, mu, sigma)` clamps sigma to [0,1] and maps it to 400–700 nm, clamps mu to [0,1] and uses a Gaussian width of 10–150 nm, and clamps alpha below zero to zero. This is a parameterized Gaussian reconstruction/projection, not evidence of a measured spectrum, calibrated radiometric units, or broad-band EM field samples; wavelengths outside the visual band still exist in the 380–780 nm SPD grid/CMF.

13. **Continuous spectral reflectance basis** — `C:/github/QualiaDB/crates/qualia-core-db/src/render/spectral_basis.rs:7-28,50-65,80-112`. Jakob–Hanika reflectance uses three `f32` coefficients over normalized 380–780 nm wavelengths; evaluation returns bounded reflectance [0,1]. A separate function names Gaussian EMF emission and takes alpha/mu/sigma, but no physical units are declared in the inspected interface. This models/reconstructs reflectance or emission parametrically; it is not a hyperspectral sensor acquisition API.

14. **EMF point interference simulation** — `C:/github/QualiaDB/crates/qualia-core-db/src/specialized_libs/physics_simulation/emf.rs:4-19,22-44,56-70,73-107`. `EmfSource` stores f64 position, amplitude, frequency, and phase; `parse_flat` consumes six values per source `[x,y,z,A,f,φ]`; the field equation uses `A/r * sin(2π f t - k r + φ)` and `k=2πf/c`. `run_emf_interference` superposes sources at a point, uses a caller `c` when positive or defaults to 299,792,458 m/s, and estimates peak over 256 samples spanning a period based on the lowest nonzero absolute frequency. Frequency units are not encoded by the struct; interpretation as Hz follows the equation's time convention and still requires caller-consistent units. This source demonstrates a scalar mathematical model, not measured field values or species response.
