//! Observations of unresolved upstream defects, not acceptance tests.
use vibe::{check_cell, load_program, parse_cell, Budget, Engine, Env, LocalHost, Value};

#[test]
fn upstream_finding_cell_parser_ignores_trailing_tokens_after_semicolon() {
    let expr = parse_cell("= 1; not valid @").unwrap();
    let mut host = LocalHost::default();
    let mut env = Env::default();
    let mut engine = Engine::new(&mut host, Budget::default());
    assert_eq!(engine.eval_expr(&expr, &mut env).unwrap(), Value::I64(1));
}

#[test]
fn upstream_finding_pure_caller_can_dispatch_an_effectful_function() {
    let program = load_program(
        "requires [ capability(\"pulse.publish\") ];
         effect fn publish_it() { pulse.publish(\"observed-effect\", 1); }
         pure fn labelled_pure() { return publish_it(); }",
    )
    .unwrap();
    let expr = parse_cell("= labelled_pure()").unwrap();
    check_cell(&expr).unwrap();
    let mut host = LocalHost::default();
    let mut env = Env::default();
    let mut engine = Engine::with_program(&mut host, Budget::default(), &program);
    engine.eval_expr(&expr, &mut env).unwrap();
    assert_eq!(host.published, ["observed-effect"]);
}
