//! Isolated default-stack process probes: run one ignored case per process.
use vibe::session::SourceSession;
use vibe::{Budget, LocalHost, Value};
fn submit(s: &mut SourceSession, h: &mut LocalHost, source: &str) -> Value {
    s.submit(source, h, Budget::default()).expect(source).value
}
#[test]
#[ignore = "run alone in isolated process with default stack"]
fn boundary_unary_64_path_units() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    assert_eq!(
        submit(&mut s, &mut h, &format!("= {}true", "!".repeat(62))),
        Value::Bool(true)
    );
}
#[test]
#[ignore = "run alone in isolated process with default stack"]
fn boundary_groups_16() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    assert_eq!(
        submit(
            &mut s,
            &mut h,
            &format!("= {}1{}", "(".repeat(16), ")".repeat(16))
        ),
        Value::I64(1)
    );
}
#[test]
#[ignore = "run alone in isolated process with default stack"]
fn boundary_postfix_64_path_units() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    submit(&mut s, &mut h, "let r0 = 1;");
    for i in 1..=31 {
        submit(
            &mut s,
            &mut h,
            &format!("let r{i} = {{ field: r{} }};", i - 1),
        );
    }
    assert_eq!(
        submit(&mut s, &mut h, &format!("= r31{}", ".field".repeat(31))),
        Value::I64(1)
    );
}
#[test]
#[ignore = "run alone in isolated process with default stack"]
fn boundary_combined_calls_and_expression_path() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    for i in (0..8).rev() {
        let tail = if i == 7 {
            "true".to_string()
        } else {
            format!("f{}()", i + 1)
        };
        submit(
            &mut s,
            &mut h,
            &format!("pure fn f{i}() {{ return {}{tail}; }}", "!".repeat(60)),
        );
    }
    let error = s.submit("f0()", &mut h, Budget::default()).unwrap_err();
    assert_eq!(error.diagnostic.code, vibe::DiagCode::E400);
    assert_eq!(
        error.diagnostic.message,
        "source-session expression evaluation depth exceeded"
    );
}
#[test]
#[ignore = "run alone in isolated process with default stack"]
fn boundary_combined_qualified_success() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    for i in (0..8).rev() {
        let tail = if i == 7 {
            "true".to_string()
        } else {
            format!("f{}()", i + 1)
        };
        submit(
            &mut s,
            &mut h,
            &format!(
                "pure fn f{i}() {{ return {}{}{tail}{}; }}",
                "!".repeat(4),
                "(".repeat(14),
                ")".repeat(14)
            ),
        );
    }
    assert_eq!(submit(&mut s, &mut h, "f0()"), Value::Bool(true));
}
