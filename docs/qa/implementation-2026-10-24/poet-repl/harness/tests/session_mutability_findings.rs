//! Observations of the pre-existing session checker gap, not acceptance tests.
//! Run unchanged with and without the harness's baseline feature.
use poet_repl_run03_check::repl::eval_line;
use vibe::{load_program, DiagCode, Env, LocalHost, Value};

#[test]
fn upstream_finding_mutable_binding_cannot_be_assigned_in_a_later_submission() {
    let mut host = LocalHost::default();
    let mut env = Env::default();
    eval_line("let radius: f64 = 2.0;", &mut host, &mut env).unwrap();
    assert_eq!(
        eval_line("radius * radius", &mut host, &mut env).unwrap(),
        Value::F64(4.0)
    );
    eval_line("let mut scale: f64 = 1.0;", &mut host, &mut env).unwrap();
    assert!(env.mutables.contains("scale"));

    // This public checker entry point has no prior-session context. The old
    // shell masks its E701 with a later cell parse error; the candidate preserves
    // the selected program check error. Both reject the valid session update.
    assert_eq!(
        load_program("scale = 2.0;").unwrap_err().code,
        DiagCode::E701
    );
    let error = eval_line("scale = 2.0;", &mut host, &mut env).unwrap_err();
    if cfg!(feature = "baseline") {
        assert_eq!(error.code, DiagCode::E001);
        assert_eq!(error.message, "unexpected tokens after cell expression");
    } else {
        assert_eq!(error.code, DiagCode::E701);
        assert_eq!(
            error.message,
            "cannot assign to immutable binding `scale` (declare with `let mut`)"
        );
    }
    // Runtime metadata still records this as mutable. The fresh checker loses
    // that fact and rejects before runtime assignment, leaving the old value.
    assert!(env.mutables.contains("scale"));
    assert_eq!(
        eval_line("scale", &mut host, &mut env).unwrap(),
        Value::F64(1.0)
    );
    assert_eq!(
        eval_line("radius * radius * scale", &mut host, &mut env).unwrap(),
        Value::F64(4.0)
    );
    println!(
        "Observed: persisted mutable scale rejected with {} ({}); scale remains 1.0, derived value remains 4.0. Canonical program check returns E701.",
        error.code.as_str(), error.message
    );
}

#[test]
fn upstream_finding_same_submission_mutable_assignment_succeeds() {
    let mut host = LocalHost::default();
    let mut env = Env::default();
    assert_eq!(
        eval_line(
            "let mut scale: f64 = 1.0; scale = 2.0; scale;",
            &mut host,
            &mut env,
        )
        .unwrap(),
        Value::F64(2.0)
    );
    println!(
        "Observed: declaring and assigning scale in the same submission succeeds with value 2.0."
    );
}
