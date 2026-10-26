use super::repl;
use vibe::{Budget, DiagCode, LocalHost, Value};
use vibe::session::SourceSession;

#[test]
fn native_poet_entry_retains_mutable_and_function_context() {
    let mut host=LocalHost::default(); let mut session=SourceSession::default();
    repl::eval_session_line("let mut scale: i64 = 2; pure fn twice(x: i64) { return x * 2; }", &mut host, &mut session).unwrap();
    repl::eval_session_line("scale = 4;", &mut host, &mut session).unwrap();
    let value=repl::eval_session_line("twice(scale)", &mut host, &mut session).unwrap().value;
    assert_eq!(value,Value::I64(8));
}
#[test]
fn native_poet_entry_cannot_self_grant_or_replace_immutable_value() {
    let mut host=LocalHost::default(); let mut session=SourceSession::default();
    repl::eval_session_line("let fixed: i64 = 1;",&mut host,&mut session).unwrap();
    let e=repl::eval_session_line("fixed = 3;",&mut host,&mut session).unwrap_err();
    assert_eq!(e.diagnostic.code,DiagCode::E701); assert!(!e.execution_attempted);
    let e=repl::eval_session_line("requires [ capability(\"pulse.publish\") ]; pulse.publish(\"denied\", 1);",&mut host,&mut session).unwrap_err();
    assert_eq!(e.diagnostic.code,DiagCode::E300); assert!(!e.execution_attempted);
    assert!(host.published.is_empty());
    assert_eq!(session.submit("fixed",&mut host,Budget::default()).unwrap().value,Value::I64(1));
}

#[test]
fn cli_grants_are_explicit_repeatable_and_never_inferred() {
    let input=vec!["--grant".into(),"pulse.publish".into(),"--grant".into(),"graph.read".into()];
    assert_eq!(repl::parse_repl_grants(&input).unwrap(),["pulse.publish","graph.read"]);
    assert!(repl::parse_repl_grants(&[]).unwrap().is_empty());
    assert!(repl::parse_repl_grants(&["--grant".into()]).is_err());
    assert!(repl::parse_repl_grants(&["pulse.publish".into()]).is_err());
}
