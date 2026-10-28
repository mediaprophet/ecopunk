//! Additional negative qualification cases through the current public Vibe API.
//! Synthetic in-process data only; no actual principal, device or external action.
use vibe::{check_program, eval_cell, parse_program, Env, LocalHost, Value};

fn joined_result(value: Value) -> Value {
    match value {
        Value::Record(record) => record.get("result").cloned().unwrap_or(Value::Record(record)),
        other => other,
    }
}

#[test]
fn pure_source_must_reject_task_spawn() {
    let source = "pure fn main() -> Record { return task.spawn(\"= 42;\", {}); }";
    let program = parse_program(source).expect("valid test syntax");
    assert!(check_program(&program).is_err(), "pure checker accepted a task-registry mutation");
}

#[test]
fn numeric_task_id_must_not_disclose_another_environment_result() {
    let mut owner_host = LocalHost::default();
    let mut owner_env = Env::default();
    let task = eval_cell("= task.spawn(\"= 42;\", { owner_scope: \"probe-owner\" })", &mut owner_host, &mut owner_env).unwrap();
    owner_env.vars.insert("job".into(), task.clone());
    assert_eq!(joined_result(eval_cell("= await job", &mut owner_host, &mut owner_env).unwrap()), Value::I64(42));
    let Value::Record(handle) = task else { panic!("task record required") };
    let id = match handle.get("task_id") { Some(Value::U64(id)) => *id, _ => panic!("task id required") };
    let mut other_host = LocalHost::default();
    let mut other_env = Env::default();
    let result = eval_cell(&format!("= task.status({id}u64)"), &mut other_host, &mut other_env);
    if let Ok(Value::Record(record)) = result {
        assert_ne!(record.get("result"), Some(&Value::I64(42)), "integer-only lookup disclosed another environment's stored result");
    }
}

#[test]
fn explicit_await_must_preserve_the_same_environment_as_syntax_await() {
    let mut host = LocalHost::default();
    let mut env = Env::default();
    env.vars.insert("probe_input".into(), Value::I64(41));
    let ast_job = eval_cell("= task.spawn(\"= probe_input + 1;\", {})", &mut host, &mut env).unwrap();
    env.vars.insert("ast_job".into(), ast_job);
    assert_eq!(joined_result(eval_cell("= await ast_job", &mut host, &mut env).unwrap()), Value::I64(42));
    let explicit_job = eval_cell("= task.spawn(\"= probe_input + 1;\", {})", &mut host, &mut env).unwrap();
    env.vars.insert("explicit_job".into(), explicit_job);
    let result = eval_cell("= task.await(explicit_job)", &mut host, &mut env).map(joined_result);
    assert!(matches!(result, Ok(Value::I64(42))), "explicit task.await lost the caller environment: {result:?}");
}

#[test]
fn completed_task_must_not_be_rewritten_by_a_caller_timeout_flag() {
    let mut host = LocalHost::default();
    let mut env = Env::default();
    let task = eval_cell("= task.spawn(\"= 42;\", {})", &mut host, &mut env).unwrap();
    env.vars.insert("job".into(), task.clone());
    assert_eq!(joined_result(eval_cell("= await job", &mut host, &mut env).unwrap()), Value::I64(42));
    let Value::Record(mut handle) = task else { panic!("task record required") };
    handle.insert("timed_out".into(), Value::Bool(true));
    env.vars.insert("flagged_job".into(), Value::Record(handle));
    let result = eval_cell("= await flagged_job", &mut host, &mut env).map(joined_result);
    assert!(matches!(result, Ok(Value::I64(42))), "caller flag replaced the completed outcome: {result:?}");
}

#[test]
fn unchecked_named_predicate_must_not_be_reported_as_a_verified_proof() {
    let mut host = LocalHost::default();
    let mut env = Env::default();
    let result = eval_cell("= crypto.verify_zk_predicate(crypto.prove_zk_predicate(\"1 == 0\", 0, { witness: 1 }), 0)", &mut host, &mut env);
    assert!(!matches!(result, Ok(Value::Bool(true))), "an unevaluated false predicate was accepted as verified");
}

#[test]
fn actual_digest_verification_rejects_changed_source() {
    let mut host = LocalHost::default();
    let mut env = Env::default();
    let hash = eval_cell("= code.hash_ast(\"= 42;\")", &mut host, &mut env).unwrap();
    let Value::Record(record) = hash else { panic!("hash record required") };
    let Some(Value::String(address)) = record.get("hash_hex") else { panic!("address required") };
    let result = eval_cell(&format!("= code.resolve(\"{address}\", \"= 43;\")"), &mut host, &mut env).unwrap();
    let Value::Record(resolved) = result else { panic!("resolve record required") };
    assert_eq!(resolved.get("verified"), Some(&Value::Bool(false)));
}
