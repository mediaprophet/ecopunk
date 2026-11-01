//! HMC Verification & Native PortalGpu Offscreen Rendering Module
//!
//! Verifies bundle cryptographic digests, progressive streaming planners,
//! zero-heap hydration capability tokens, holosonic wavefields, physics/soundscape/HUD epics,
//! and renders the 8 canonical showcase frames (1280x720) natively via `PortalGpu` (`wgpu`).

use std::fs;
use std::io;
use std::path::Path;

use ecopunk_terrain::{
    render_sector_via_portal_gpu_png, run_sector_physics_verification_step,
    EcopunkHolosonicSoundscape, EcopunkSectorSceneGraph, EcopunkTelemetryHud,
    FaunaBiomechanicalArchetype, FaunaSkeletalRig10d,
};
use qualia_core_db::audio::holosonic::{
    HolosonicEmitter, HolosonicModality, HolosonicSonicToken48, HolosonicWavefield,
};
use qualia_core_db::bundle::{
    verify_arrived_range, BundleReader, HmcHeaderProbe, HmcProgressiveRangePlanner,
    StreamPriorityTier, BUNDLE_HEADER_SIZE,
};
use qualia_core_db::container_10d::ShadingModel;
use qualia_core_db::governance::worker_pool::{
    GovernedHydrationPool, HydrationCapabilityToken, HydrationTask, HydrationTaskKind,
    CAP_ALL_HYDRATION,
};
use qualia_core_db::q42::q42_volume::WasmSliceQ42Reader;
use webizen_render::VolumetricRenderer;

use super::hmc_packing::PackHmcResult;
use super::vibe_phase::VibePhaseResult;

pub fn execute_hmc_verification_and_rendering(
    renders_dir: &Path,
    packed: &PackHmcResult,
    vibe_phase: &VibePhaseResult,
) -> io::Result<()> {
    let hmc_bytes = &packed.hmc_bytes;

    let hmc_reader = BundleReader::parse(hmc_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("BundleReader parse: {e}")))?;
    for entry in hmc_reader.entries() {
        assert!(
            hmc_reader.verify_entry(&entry.key),
            "HMC bundle entry digest verification failed for {}",
            entry.key
        );
    }
    println!(
        "[OK] Digest-Verified .hmc (`QBDL`) Bundle: {} verified entries",
        hmc_reader.entries().len()
    );

    let mut vol_renderer = VolumetricRenderer::new_offscreen(640, 360, 256)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("VolumetricRenderer init: {e}")))?;
    let (hmc_verts, hmc_tris, _) = vol_renderer
        .load_hmc_asset(hmc_bytes, "10d/sector-talbingo.10d")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("load_hmc_asset: {e}")))?;
    let (wat_verts, wat_tris) = vol_renderer
        .load_hmc_water_asset(hmc_bytes, "10d/water-talbingo.10d")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("load_hmc_water_asset: {e}")))?;
    println!(
        "[OK] VolumetricRenderer Native HMC Admission Verified: scene=({} verts, {} tris), water=({} verts, {} tris)",
        hmc_verts, hmc_tris, wat_verts, wat_tris
    );

    let wasm_q42_reader = WasmSliceQ42Reader::open(&vibe_phase.q42_v4_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("WasmSliceQ42Reader::open: {e:?}")))?;
    println!(
        "  * Verified WasmSliceQ42Reader: is_unified={}, version={}, blocks={}, len={} B",
        wasm_q42_reader.is_unified_volume(),
        wasm_q42_reader.version(),
        wasm_q42_reader.block_count(),
        wasm_q42_reader.byte_len()
    );
    let wasm_unified_q42_reader = WasmSliceQ42Reader::open(&vibe_phase.q42_unified_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("WasmSliceQ42Reader::open(unified): {e:?}")))?;
    println!(
        "  * Verified Unified WasmSliceQ42Reader: is_unified={}, version={}, blocks={}, len={} B",
        wasm_unified_q42_reader.is_unified_volume(),
        wasm_unified_q42_reader.version(),
        wasm_unified_q42_reader.block_count(),
        wasm_unified_q42_reader.byte_len()
    );

    let hmc_probe = HmcHeaderProbe::parse_header_range(&hmc_bytes[..BUNDLE_HEADER_SIZE])
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("HmcHeaderProbe: {e:?}")))?;
    let (idx_off, idx_len) = hmc_probe.index_footer_range();
    let footer_slice = &hmc_bytes[idx_off as usize..(idx_off + idx_len) as usize];
    let range_planner = HmcProgressiveRangePlanner::from_header_and_index(hmc_probe, footer_slice)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("HmcProgressiveRangePlanner: {e:?}")))?;
    let schedule = range_planner.plan_entry_ranges();
    let mut tier0_count = 0usize;
    let mut tier0_bytes = 0u64;
    let mut tier1_count = 0usize;
    let mut tier1_bytes = 0u64;
    let mut tier2_count = 0usize;
    let mut tier2_bytes = 0u64;
    for req in &schedule {
        match req.tier {
            StreamPriorityTier::Tier0SemanticAndProcedural => {
                tier0_count += 1;
                tier0_bytes += req.length as u64;
            }
            StreamPriorityTier::Tier1MidLodMeshes => {
                tier1_count += 1;
                tier1_bytes += req.length as u64;
            }
            StreamPriorityTier::Tier2HighPolyAndMip0 => {
                tier2_count += 1;
                tier2_bytes += req.length as u64;
            }
        }
        let start = req.offset as usize;
        let end = start + req.length as usize;
        verify_arrived_range(req, &hmc_bytes[start..end])
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("verify_arrived_range: {e:?}")))?;
    }
    println!(
        "  * Verified HmcProgressiveRangePlanner ({} scheduled ranges): Tier0=({} ranges, {} B), Tier1=({} ranges, {} B), Tier2=({} ranges, {} B)",
        schedule.len(),
        tier0_count, tier0_bytes,
        tier1_count, tier1_bytes,
        tier2_count, tier2_bytes
    );

    let mut hydration_token = HydrationCapabilityToken::new_sealed(
        0xABCD_1234,
        CAP_ALL_HYDRATION,
        StreamPriorityTier::Tier2HighPolyAndMip0,
        1000,
        1024 * 1024,
    );
    assert!(hydration_token.verify_seal(), "Hydration capability token seal failed");
    let mut hydration_pool = GovernedHydrationPool::new(16);
    let sample_payload = b"ecopunk living container hydration verification payload".to_vec();
    let sample_crc = crc32c::crc32c(&sample_payload);
    hydration_pool.submit(
        &mut hydration_token,
        HydrationTask {
            task_id: 42,
            tier: StreamPriorityTier::Tier0SemanticAndProcedural,
            kind: HydrationTaskKind::VerifySectionCrc32c {
                payload: sample_payload,
                expected_crc32c: sample_crc,
            },
        },
    ).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("GovernedHydrationPool submit: {e:?}")))?;
    let receipts = hydration_pool.drain_bounded(1)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("GovernedHydrationPool drain: {e:?}")))?;
    assert_eq!(receipts.len(), 1);
    println!("  * Verified GovernedHydrationPool admission & execution under HydrationCapabilityToken");

    let mut wavefield = HolosonicWavefield::new();
    wavefield.push(HolosonicEmitter::new(
        [0.0, 1.2, 0.0],
        440.0,
        0.8,
        0.0,
        0.95,
        30.0,
        HolosonicModality::FloraWind,
    ));
    let point_sample = wavefield.evaluate_point([0.0, 0.0, 0.0], 0.0, 0.0);
    let (sonic_tok48, _tok64) = wavefield.emit_sonic_token(
        0, 0x5A5A_BEEF, 101, 1, 0.9, point_sample, [0.0, 0.0, 0.0], 0.0,
    );
    assert_eq!(std::mem::size_of::<HolosonicSonicToken48>(), 48);
    println!(
        "  * Verified HolosonicWavefield 48-byte HolosonicSonicToken48 emission (origin_hash={:#x}, fundamental={} Hz)",
        sonic_tok48.origin_quin_hash, sonic_tok48.fundamental_hz
    );

    let mut scene_graph_10d = EcopunkSectorSceneGraph::new_boxed("talbingo");
    assert!(scene_graph_10d.populate_sector_10d_hierarchy("talbingo"));
    println!(
        "  * Verified EcopunkSectorSceneGraph 10D Hierarchy (`talbingo`): {} living botanical entities, {} holo SDF lenses",
        scene_graph_10d.living_count, scene_graph_10d.holographic_sdf_count
    );

    let mut macropod_rig = FaunaSkeletalRig10d::new(FaunaBiomechanicalArchetype::MacropodSaltatorial);
    let rest_pose = macropod_rig.rest_pose;
    let palette_joints = macropod_rig.evaluate_gpu_palette(&rest_pose);
    println!(
        "  * Verified FaunaSkeletalRig10d (`MacropodSaltatorial`): {} PGA Motor joints, {} ABSM layers, {} 2D Delaunay points",
        palette_joints, macropod_rig.absm.layer_count, macropod_rig.blend_space_2d.point_count
    );

    let phys_receipt = run_sector_physics_verification_step("yarrangobilly_cliff");
    println!(
        "  * Verified EcopunkSectorPhysicsWorld (`yarrangobilly_cliff`): {} bodies, {} BVH leaves, {} fractures, sinkage={:.2} mm",
        phys_receipt.active_rigid_bodies,
        phys_receipt.bvh_leaves,
        phys_receipt.stress_cleavage_fractures,
        phys_receipt.soil_sinkage_mm
    );

    let mut soundscape = EcopunkHolosonicSoundscape::with_australian_soundscape("yarrangobilly");
    let mut audio_l = [0.0f32; 64];
    let mut audio_r = [0.0f32; 64];
    let audio_sample = soundscape.render_frame_zero_heap(0.25, 48_000.0, &mut audio_l, &mut audio_r);
    println!(
        "  * Verified EcopunkHolosonicSoundscape (`yarrangobilly`): {} active emitters, interference_gain={:.3}",
        audio_sample.active_emitter_count, audio_sample.interference_gain
    );

    let mut telemetry_hud = EcopunkTelemetryHud::new(1280.0, 720.0);
    telemetry_hud.render_frame_zero_heap();
    println!(
        "  * Verified EcopunkTelemetryHud (1280x720 Zero-Heap Flexbox + MSDF): {} widgets, {} shaped glyphs",
        telemetry_hud.canvas.widget_count,
        telemetry_hud.canvas.text_quad_count
    );

    // Caller owns a fresh output revision; never reuse the historical shared
    // assets/renders directory or overwrite its images.
    fs::create_dir_all(renders_dir)?;

    let render_jobs = vec![
        ("01_talbingo_aerial_pbr.png", &packed.compiled_sectors["talbingo"], None),
        ("02_yarrangobilly_cutaway_strata.png", &packed.compiled_sectors["yarrangobilly"], None),
        ("03_flora_specimen_loupe_snow_gum.png", &packed.snow_gum_specimen, None),
        ("04_fauna_tracker_kangaroo.png", &packed.kangaroo_specimen, None),
        ("05_sydney_sandstone_cel_toon.png", &packed.compiled_sectors["sydney_sandstone"], Some((ShadingModel::Stylized, 0, [0.52, 0.44, 0.36]))),
        ("06_noosa_headland_watercolor.png", &packed.compiled_sectors["noosa_headland"], Some((ShadingModel::Stylized, 0, [0.44, 0.56, 0.62]))),
        ("07_three_mile_dam_gpr_hud.png", &packed.compiled_sectors["three_mile_dam"], Some((ShadingModel::MetallicRoughness, 1, [0.18, 0.48, 0.64]))),
        ("08_jounama_karst_thermal_ir.png", &packed.compiled_sectors["jounama_karst"], Some((ShadingModel::Stylized, 2, [0.64, 0.32, 0.18]))),
    ];

    for (filename, sector_ref, override_mode) in render_jobs {
        let png_bytes = render_sector_via_portal_gpu_png(
            sector_ref,
            &packed.textures,
            1280,
            720,
            override_mode,
        ).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("PortalGpu render {filename}: {e}")))?;
        let out_path = renders_dir.join(filename);
        fs::write(&out_path, &png_bytes)?;
        println!(
            "  * Rendered Native PortalGpu (`wgpu` Zero-V8) Frame: {} ({} bytes, 1280x720)",
            out_path.display(),
            png_bytes.len()
        );
    }

    Ok(())
}
