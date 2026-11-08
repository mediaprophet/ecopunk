//! Actual public SDK use compiled for wasm32, not a replacement codec or runtime proof.
use qualia_core_db::{
    container_10d::{
        copy_opaque_container_into, read_container_mesh_frames_into, MeshFrameDecodeLimits,
        MeshFrameLayout, OpaquePreservationBudget, OpaquePreservationError, TypedPayloadError,
    },
    q42::m64_matter::{
        M64PayloadInterval, M64ReadError, M64ReadLimits, M64Reader, M64Verification,
    },
};
use qualia_geospatial::{
    raster_m64::{compile_raster_to_m64_v3, RasterM64Error, RasterM64Limits, RasterM64Policy},
    ElevationRaster, GeodeticFrame,
};
use std::sync::atomic::AtomicBool;

pub fn inspect_matter(
    bytes: &[u8], limits: M64ReadLimits, scratch: &mut [M64PayloadInterval],
) -> Result<M64Verification, M64ReadError> {
    Ok(M64Reader::parse(bytes, limits, scratch)?.verification())
}

pub fn decode_frames(
    bytes: &[u8], count: usize, limits: MeshFrameDecodeLimits,
    normals: &mut [[f32; 3]], tangents: &mut [[f32; 4]],
) -> Result<Option<MeshFrameLayout>, TypedPayloadError> {
    read_container_mesh_frames_into(bytes, count, limits, normals, tangents)
}

pub fn retain_representation(
    bytes: &[u8], budget: OpaquePreservationBudget, output: &mut [u8],
) -> Result<usize, OpaquePreservationError> {
    copy_opaque_container_into(bytes, budget, output)
}

pub fn encode_georaster(
    raster: &ElevationRaster<'_>, frame: &GeodeticFrame, epoch: u16,
    policy: RasterM64Policy, limits: RasterM64Limits,
    cancelled: &AtomicBool, output: &mut [u8],
) -> Result<usize, RasterM64Error> {
    compile_raster_to_m64_v3(raster, frame, epoch, policy, limits, cancelled, output)
}
