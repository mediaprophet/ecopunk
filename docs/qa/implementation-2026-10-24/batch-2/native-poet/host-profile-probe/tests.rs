//! Isolated actual native Host profile + verbatim production parser functions.
//! Uses mutually matching cached core/Vibe artifacts; no current session/UI claim.
mod marshal;
use poet_geometry_profile::GeometryHost;
use std::collections::BTreeMap;
use vibe::{DiagCode, Host, Span, Value};

fn grant(id: &str) -> Vec<String> { vec!["capability.invoke".into(), id.into()] }
fn rows(values: &[&[f64]]) -> Value {
    Value::List(values.iter().map(|row| Value::List(row.iter().map(|v| Value::F64(*v)).collect())).collect())
}
fn extrusion() -> Value {
    Value::Record(BTreeMap::from([
        ("axis".into(), rows(&[&[0.0,0.0,-1.0], &[0.12,0.06,-0.4], &[0.18,0.02,0.3], &[0.1,0.0,1.0]])),
        ("section".into(), rows(&[&[-0.2,-0.1], &[0.18,-0.12], &[0.24,0.05], &[0.02,0.2], &[-0.18,0.1]])),
        ("closed".into(), Value::Bool(true)), ("caps".into(), Value::Bool(true)),
    ]))
}
fn swung() -> Value {
    Value::Record(BTreeMap::from([
        ("profiles".into(), rows(&[&[1.0,0.0,1.0,1.0], &[1.1,0.0,1.1,1.0]])),
        ("angles".into(), Value::List(vec![Value::F64(0.0),Value::F64(3.14)])),
        ("slices".into(), Value::U64(8)),
    ]))
}

#[test]
fn independent_actual_grants_deny_dynamic_ungranted_operation() {
    let mut host = GeometryHost::new(grant("ComputationalGeometry.extrude_profile"), 200_000);
    let dynamic = String::from("ComputationalGeometry.swung_surface");
    let error = host.capability_invoke(&dynamic, &swung(), Span::point(0)).unwrap_err();
    assert_eq!(error.code, DiagCode::E300);
    assert!(error.message.contains("outside the admitted native geometry preview profile"));
    let mut default = GeometryHost::default();
    assert!(default.capability_invoke("ComputationalGeometry.extrude_profile", &extrusion(), Span::point(0)).is_err());
}

#[test]
fn real_extrusion_nested_result_marshals_to_canonical_mesh() {
    let mut host = GeometryHost::new(grant("ComputationalGeometry.extrude_profile"), 200_000);
    let value = host.capability_invoke("ComputationalGeometry.extrude_profile", &extrusion(), Span::point(0)).unwrap();
    let mesh = marshal::mesh_from_invoke(&value, marshal::PreviewLimits::default()).unwrap();
    assert!(!mesh.positions.is_empty()); assert!(!mesh.triangles.is_empty());
    assert!(mesh.positions.iter().flatten().all(|v| v.is_finite()));
}

#[test]
fn real_swung_flat_result_marshals_and_caps_budget_is_preflighted() {
    let mut host = GeometryHost::new(grant("ComputationalGeometry.swung_surface"), 200_000);
    let value = host.capability_invoke("ComputationalGeometry.swung_surface", &swung(), Span::point(0)).unwrap();
    assert!(!marshal::mesh_from_invoke(&value, marshal::PreviewLimits::default()).unwrap().positions.is_empty());
    let mut bounded = GeometryHost::new(grant("ComputationalGeometry.extrude_profile"), 21);
    assert_eq!(bounded.capability_invoke("ComputationalGeometry.extrude_profile", &extrusion(), Span::point(0)).unwrap_err().code, DiagCode::E400);
}
