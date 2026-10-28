//! Typed Vibe source through the native host into existing QualiaDB Sheet owners.
use super::*;
use vibe::session::{SessionLimits, SourceSession};
fn grants() -> Vec<String> { ["capability.invoke", "Sheet.stats", "Sheet.sum_range"].into_iter().map(str::to_owned).collect() }
#[test]
fn source_calls_stats_and_huge_range_over_actual_supplied_cells() {
 let g=grants(); let mut host=GeometryHost::new(g.clone(),200_000); let mut session=SourceSession::new(g,SessionLimits::default());
 let stats=r#"requires [capability("capability.invoke"),capability("Sheet.stats")]; capability.invoke("Sheet.stats",[[1,2],[3],[],[4,5]]);"#;
 let Value::Record(stats)=session.submit(stats,&mut host,vibe::Budget::default()).unwrap().value else { panic!("stats") };
 assert_eq!(stats.get("sum"),Some(&Value::F64(15.0))); assert_eq!(stats.get("count"),Some(&Value::U64(5)));
 host.begin_submission();
 let source=format!(r#"requires [capability("capability.invoke"),capability("Sheet.sum_range")]; capability.invoke("Sheet.sum_range",{{grid:[[1,2],[3],[],[4,5]],range:"A1:ZZZZZZ{}"}});"#,usize::MAX);
 let Value::Record(result)=session.submit(&source,&mut host,vibe::Budget::default()).unwrap().value else { panic!("range") };
 assert_eq!(result.get("sum"),Some(&Value::F64(15.0))); assert_eq!(result.get("count"),Some(&Value::U64(5)));
}
#[test]
fn source_denies_missing_target_and_obeys_work_dispatch_limits() {
 let g=vec!["capability.invoke".into()]; let mut host=GeometryHost::new(g.clone(),200_000); let mut session=SourceSession::new(g,SessionLimits::default());
 let dynamic=r#"requires [capability("capability.invoke")]; let op="Sheet.stats"; capability.invoke(op,[1,2]);"#;
 let error=session.submit(dynamic,&mut host,vibe::Budget::default()).unwrap_err(); assert_eq!(error.diagnostic.code,DiagCode::E300); assert!(error.execution_attempted);
 let g=grants(); let mut host=GeometryHost::with_limits(g.clone(),GeometryHostLimits{max_vertices:10,submission_work:4,dispatches:1,scratch_bytes:4096,profile_points:3}).unwrap(); let mut session=SourceSession::new(g,SessionLimits::default());
 let source=r#"requires [capability("capability.invoke"),capability("Sheet.stats")]; capability.invoke("Sheet.stats",[1,2]);"#;
 assert!(session.submit(source,&mut host,vibe::Budget::default()).is_ok());
 assert!(session.submit(source,&mut host,vibe::Budget::default()).is_err());
 host.begin_submission(); assert!(session.submit(source,&mut host,vibe::Budget::default()).is_ok());
 let g=grants(); let mut host=GeometryHost::with_limits(g.clone(),GeometryHostLimits{max_vertices:10,submission_work:3,dispatches:1,scratch_bytes:4096,profile_points:3}).unwrap(); let mut session=SourceSession::new(g,SessionLimits::default()); assert!(session.submit(source,&mut host,vibe::Budget::default()).is_err());
}

#[test]
fn native_dotted_positional_stats_matches_checked_capability_forms() {
 let g=grants(); let mut host=GeometryHost::new(g.clone(),200_000); let mut session=SourceSession::new(g,SessionLimits::default());
 for (src,sum,count) in [
  (r#"requires [capability("Sheet.stats")]; Sheet.stats([1,2]);"#,3.0,2),
  (r#"requires [capability("Sheet.stats")]; Sheet.stats([[1,2],[3],[],[4,5]]);"#,15.0,5),
  (r#"requires [capability("Sheet.stats")]; Sheet.stats({grid:[[1,2],[3,4]]});"#,10.0,4),
 ] {
  host.begin_submission();
  let Value::Record(result)=session.submit(src,&mut host,vibe::Budget::default()).unwrap().value else { panic!("stats") };
  assert_eq!(result.get("sum"),Some(&Value::F64(sum)));
  assert_eq!(result.get("count"),Some(&Value::U64(count)));
 }
}
#[test]
fn native_positional_wrapper_is_bounded_and_bit_exact() {
 use std::collections::BTreeMap;
 let g=grants(); let mut host=GeometryHost::new(g,200_000);
 let wrapper=|a:Value,b:Value| Value::Record(BTreeMap::from([("t".into(),a),("value".into(),b)]));
 let span=Span::point(0);
 for n in [f64::NAN,f64::INFINITY,-0.0] {
  let grid=Value::List(vec![Value::F64(n)]);
  host.begin_submission(); let raw=host.capability_invoke("Sheet.stats",&grid,span).unwrap();
  host.begin_submission(); let wrapped=host.capability_invoke("Sheet.stats",&wrapper(grid.clone(),grid),span).unwrap();
  let (Value::Record(raw),Value::Record(wrapped))=(raw,wrapped) else { panic!("records") };
  let (Some(Value::F64(a)),Some(Value::F64(b)))=(raw.get("sum"),wrapped.get("sum")) else { panic!("sum") };
  assert!((a.is_nan() && b.is_nan()) || a.to_bits()==b.to_bits());
 }
 let huge=Value::List(vec![Value::I64(0);4097]);
 assert!(host.capability_invoke("Sheet.stats",&wrapper(huge.clone(),huge),span).is_err());
 let deep=Value::List(vec![Value::List(vec![Value::List(vec![Value::I64(1)])])]);
 assert!(host.capability_invoke("Sheet.stats",&wrapper(deep.clone(),deep),span).is_err());
 let diff=wrapper(Value::List(vec![Value::F64(0.0)]), Value::List(vec![Value::F64(-0.0)]));
 assert!(host.capability_invoke("Sheet.stats",&diff,span).is_err());
}

#[test]
fn native_positional_wrapper_scans_and_comparison_consume_work_before_dispatch() {
 use std::collections::BTreeMap;
 let wrapper=|| { let values=Value::List(vec![Value::I64(1), Value::I64(2)]); Value::Record(BTreeMap::from([("t".into(),values.clone()),("value".into(),values)])) };
 let limits=|work| GeometryHostLimits {max_vertices:10,submission_work:work,dispatches:1,scratch_bytes:4096,profile_points:3};
 let mut exact=GeometryHost::with_limits(grants(),limits(19)).unwrap();
 let Value::Record(result)=exact.capability_invoke("Sheet.stats",&wrapper(),Span::point(0)).unwrap() else { panic!("stats") };
 assert_eq!(result.get("sum"),Some(&Value::F64(3.0)));
 assert_eq!(exact.remaining_work,0); assert_eq!(exact.remaining_dispatches,0);
 assert!(exact.capability_invoke("Sheet.stats",&wrapper(),Span::point(0)).is_err());
 let mut insufficient=GeometryHost::with_limits(grants(),limits(18)).unwrap();
 assert!(insufficient.capability_invoke("Sheet.stats",&wrapper(),Span::point(0)).is_err());
 assert_eq!(insufficient.remaining_dispatches,0);
 // Ordinary non-wrapper flat lists retain the original 4-unit charge.
 let mut raw=GeometryHost::with_limits(grants(),limits(4)).unwrap();
 assert!(raw.capability_invoke("Sheet.stats",&Value::List(vec![Value::I64(1),Value::I64(2)]),Span::point(0)).is_ok());
}
