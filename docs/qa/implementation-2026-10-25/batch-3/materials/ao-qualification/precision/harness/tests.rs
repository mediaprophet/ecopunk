//! Candidate AO math against actual native camera constants; no core rebuild.
pub use qualia_core_db::render;
#[path = "../candidate/src/render/gpu/ao_depth.rs"]
mod depth;

#[test]
fn actual_ao_mask_fixture_ray_has_correct_linear_diagnosis_and_log_precision() {
    use render::camera::{CameraState, CAMERA_FAR_PLANE, CAMERA_TAN_HALF_FOV};
    let camera = CameraState::default();
    let eye = render::camera::orbit_eye_position_target(camera.yaw, camera.pitch, camera.zoom, camera.target);
    // Default yaw is zero. Ray through the AO half-target pixel centre (13,16).
    let ndc_y = 1.0 - 2.0 * 16.5 / 32.0;
    let ray_z = -camera.pitch.cos() - ndc_y * CAMERA_TAN_HALF_FOV * camera.pitch.sin();
    let rear = (-0.35 - eye[2]) / ray_z;
    let front = (0.35 - eye[2]) / ray_z;
    let linear = |d: f32| 1 + (d / CAMERA_FAR_PLANE * 65534.0).round() as u16;
    assert_eq!((linear(rear), linear(front)), (18, 15));
    let hole = depth::encode_depth(rear).unwrap();
    let solid = depth::encode_depth(front).unwrap();
    assert!(hole > solid + 100);
    assert!((depth::decode_depth(hole).unwrap() - rear).abs() < 0.001);
    assert!((depth::decode_depth(solid).unwrap() - front).abs() < 0.001);
    eprintln!("actual_camera_oracle rear_depth={rear} front_depth={front} original_codes=18,15 proposed_codes={hole},{solid} old_depth_quantum={}", CAMERA_FAR_PLANE / 65534.0);
}
