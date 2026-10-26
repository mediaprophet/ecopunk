use vibe::session::{SessionLimits, SourceSession};
use vibe::{check_program, parse_program, Budget, DiagCode, LocalHost, Value};

fn accepts(source: &str) {
    check_program(&parse_program(source).expect("valid source fixture")).expect("check");
}
fn effect_denied(source: &str) {
    let source = if source.contains("pulse.") {
        format!("import \"pulse\" as pulse; {source}")
    } else {
        source.to_string()
    };
    let error = check_program(&parse_program(&source).expect("valid source fixture")).unwrap_err();
    assert_eq!(error.code, DiagCode::E200, "{error:?}");
}
fn submit(session: &mut SourceSession, source: &str, host: &mut LocalHost) -> Value {
    session
        .submit(source, host, Budget::default())
        .expect(source)
        .value
}

#[test]
fn direct_chain_forward_and_recursive_external_calls_are_rejected() {
    for body in [
        "pure fn bad() { return send(); } effect fn send() { return pulse.publish(\"x\", 1); }",
        "pure fn bad() { return middle(); } effect fn middle() { return send(); } effect fn send() { return pulse.publish(\"x\", 1); }",
        "pure fn bad() { return middle(); } effect fn middle() { return bad(); }",
    ] {
        effect_denied(&format!("requires [ capability(\"pulse.publish\") ]; {body}"));
    }
}
#[test]
fn inferred_recursive_cycle_effect_reaches_pure_caller() {
    effect_denied("requires [ capability(\"pulse.publish\") ]; pure fn bad() { return a(); } effect fn a() { return b(); } effect fn b() { pulse.publish(\"x\", 1); return a(); }");
}
#[test]
fn imported_alias_and_pipeline_do_not_hide_effects() {
    effect_denied("import \"pulse\" as p; requires [ capability(\"pulse.publish\") ]; pure fn bad() { return p.publish(\"x\", 1); }");
    effect_denied("requires [ capability(\"pulse.publish\") ]; effect fn send(x: i64) { return pulse.publish(\"x\", x); } pure fn bad() { return 1 |> send; }");
}
#[test]
fn higher_order_indirect_callbacks_fail_closed() {
    for callback in ["f", "box.f", "fs[0]"] {
        effect_denied(&format!("requires [ capability(\"pulse.publish\") ]; let f = |x| pulse.publish(\"x\", x); let box = {{ f: f }}; let fs = [f]; pure cell result := [1].map({callback});"));
    }
}
#[test]
fn pure_named_forward_calls_recursion_and_lambda_are_allowed() {
    accepts("pure fn first(x: i64) { return second(x); } pure fn second(x: i64) { return x * 2; } pure fn recursive(x: i64) { return if x > 0 { recursive(x - 1) } else { 0 }; } pure fn mapped() { let f = |x| x * 2; return [1, 2].map(f); }");
}
#[test]
fn unknown_dynamic_targets_are_not_pure() {
    effect_denied("pure fn bad() { return [1][0](); }");
    effect_denied("pure fn bad() { return missing(); }");
}
#[test]
fn mutable_typed_binding_survives_and_immutable_assignment_is_denied() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    submit(&mut s, "let mut scale: i64 = 2;", &mut h);
    assert_eq!(s.binding_type("scale").as_deref(), Some("I64"));
    submit(&mut s, "scale = 4;", &mut h);
    assert_eq!(submit(&mut s, "scale * 2", &mut h), Value::I64(8));
    submit(&mut s, "let fixed: i64 = 9;", &mut h);
    let rev = s.revision();
    let e = s
        .submit("fixed = 3;", &mut h, Budget::default())
        .unwrap_err();
    assert_eq!(e.diagnostic.code, DiagCode::E701);
    assert!(!e.execution_attempted);
    assert_eq!(s.revision(), rev);
    assert_eq!(s.value("fixed"), Some(&Value::I64(9)));
}
#[test]
fn functions_imports_and_effect_context_survive_without_replay() {
    let mut s = SourceSession::new(vec!["pulse.publish".into()], SessionLimits::default());
    let mut h = LocalHost::default();
    submit(&mut s, "import \"math\" as m; requires [ capability(\"pulse.publish\") ]; pure fn double(x: i64) { return x * 2; } effect fn send() { return pulse.publish(\"later\", 1); } pulse.publish(\"once\", 1);", &mut h);
    assert_eq!(submit(&mut s, "double(4)", &mut h), Value::I64(8));
    assert_eq!(submit(&mut s, "m.abs(-3)", &mut h), Value::I64(3));
    let e = s.submit("= send()", &mut h, Budget::default()).unwrap_err();
    assert_eq!(e.diagnostic.code, DiagCode::E200);
    assert!(!e.execution_attempted);
    submit(&mut s, "send();", &mut h);
    assert_eq!(h.published, ["once", "later"]);
}
#[test]
fn requests_do_not_grant_authority_and_failed_candidates_do_not_promote() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    submit(&mut s, "let mut scale: i64 = 2;", &mut h);
    let rev = s.revision();
    for source in [
        "requires [ capability(\"pulse.publish\") ]; pulse.publish(\"denied\", 1);",
        "let broken = ;",
        "let mut scale = 8;",
        "scale = \"bad\";",
    ] {
        let e = s.submit(source, &mut h, Budget::default()).unwrap_err();
        assert!(!e.execution_attempted, "{source}: {e:?}");
        assert_eq!(s.revision(), rev);
        assert_eq!(s.value("scale"), Some(&Value::I64(2)));
    }
    assert!(h.published.is_empty());
}
#[test]
fn checked_candidates_are_bound_to_session_and_revision() {
    let mut s = SourceSession::default();
    let mut other = SourceSession::default();
    let mut h = LocalHost::default();
    let candidate = s.prepare("let x = 1;").unwrap();
    assert!(
        !other
            .execute(candidate, &mut h, Budget::default())
            .unwrap_err()
            .execution_attempted
    );
    let stale = s.prepare("let x = 1;").unwrap();
    submit(&mut s, "let y = 2;", &mut h);
    assert!(
        !s.execute(stale, &mut h, Budget::default())
            .unwrap_err()
            .execution_attempted
    );
}
#[test]
fn runtime_failure_keeps_external_outcome_and_last_accepted_local_state() {
    let mut s = SourceSession::new(vec!["pulse.publish".into()], SessionLimits::default());
    let mut h = LocalHost::default();
    submit(&mut s, "let mut x: i64 = 1;", &mut h);
    let rev = s.revision();
    let e = s.submit("requires [ capability(\"pulse.publish\") ]; x = 9; pulse.publish(\"committed\", 1); missing;", &mut h, Budget::default()).unwrap_err();
    assert_eq!(e.diagnostic.code, DiagCode::E600);
    assert_eq!(e.diagnostic.message, "undefined missing");
    assert!(e.execution_attempted && e.host_reconciliation_required);
    assert_eq!(h.published, ["committed"]);
    assert_eq!(s.revision(), rev);
    assert_eq!(s.value("x"), Some(&Value::I64(1)));
    submit(&mut s, "x + 1", &mut h);
    assert_eq!(h.published, ["committed"]);
}
#[test]
fn pure_runtime_failure_does_not_install_function_or_mutated_values() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    submit(&mut s, "let mut x = 1;", &mut h);
    assert!(s
        .submit(
            "pure fn tentative() { return 7; } x = 9; 1 / 0;",
            &mut h,
            Budget::default()
        )
        .is_err());
    assert_eq!(s.value("x"), Some(&Value::I64(1)));
    assert!(s.prepare("tentative()").is_err());
}
#[test]
fn functions_keep_non_capturing_0_1_semantics() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    submit(
        &mut s,
        "let global = 12; pure fn value() { return global; }",
        &mut h,
    );
    let e = s.submit("value()", &mut h, Budget::default()).unwrap_err();
    assert_eq!(e.diagnostic.message, "undefined global");
}
#[test]
fn session_limits_fail_without_dispatch() {
    let mut s = SourceSession::new(
        Vec::new(),
        SessionLimits {
            submission_bytes: 4,
            ..SessionLimits::default()
        },
    );
    let mut h = LocalHost::default();
    let e = s
        .submit("let long = 1;", &mut h, Budget::default())
        .unwrap_err();
    assert_eq!(e.diagnostic.code, DiagCode::E400);
    assert!(!e.execution_attempted);
}

#[test]
fn copied_alias_keeps_original_external_lambda_effect() {
    effect_denied("requires [ capability(\"pulse.publish\") ]; let mut f = |x| pulse.publish(\"escaped\", x); let g = f; f = |x| x; pure cell c := [1].map(g);");
}
#[test]
fn control_flow_and_bind_cannot_leave_stale_pure_callbacks() {
    for statement in [
        "if true { f = |x| pulse.publish(\"escaped\", x); }",
        "for f in fs {}",
        "match fs { [f] => { } }",
        "bind f <-> g resolve right;",
    ] {
        effect_denied(&format!("requires [ capability(\"pulse.publish\") ]; let mut f = |x| x; let g = |x| pulse.publish(\"escaped\", x); let fs = [g]; {statement} pure cell c := [1].map(f);"));
    }
}
#[test]
fn namespace_shadowing_does_not_hide_collection_callback_effects() {
    effect_denied("requires [ capability(\"pulse.publish\") ]; let math = [1]; let f = |x| pulse.publish(\"escaped\", x); pure cell c := math.map(f);");
    effect_denied("import \"math\" as m; requires [ capability(\"pulse.publish\") ]; let m = [1]; let f = |x| pulse.publish(\"escaped\", x); pure cell c := m.map(f);");
}
#[test]
fn material_transaction_and_clock_cannot_gain_ambient_session_authority() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    for source in [
        "material m: Material p: pulse.publish(\"escaped\", 1);",
        "transaction {}",
        "time.now();",
    ] {
        let e = s.submit(source, &mut h, Budget::default()).unwrap_err();
        assert!(!e.execution_attempted, "{source}: {e:?}");
    }
    assert!(h.published.is_empty());
}
#[test]
fn immutable_values_cannot_be_redeclared_via_patterns_blocks_or_bind() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    submit(&mut s, "let fixed: i64 = 1; let other: i64 = 9;", &mut h);
    for source in [
        "let [fixed] = [9];",
        "{ let fixed: i64 = 9; }",
        "bind fixed <-> other resolve right;",
    ] {
        let e = s.submit(source, &mut h, Budget::default()).unwrap_err();
        assert!(!e.execution_attempted, "{source}: {e:?}");
        assert_eq!(s.value("fixed"), Some(&Value::I64(1)));
    }
}
#[test]
fn explicit_type_is_not_discarded_for_bounded_lists() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    let e = s
        .submit("let xs: i64 = [1];", &mut h, Budget::default())
        .unwrap_err();
    assert_eq!(e.diagnostic.code, DiagCode::E100);
    assert!(!e.execution_attempted);
}
#[test]
fn pure_builtin_dispatch_precedes_same_named_user_function() {
    accepts("effect fn len() { return time.now(); } pure fn f() { return len([1]); }");
}
#[test]
fn unknown_namespaced_bindings_and_effectful_budget_are_rejected() {
    effect_denied("pure fn f() { return math.not_a_real_operation(); }");
    effect_denied("pure fn f() budget(steps: time.now()) { return 1; }");
}

#[test]
fn invalidated_unknown_alias_does_not_follow_later_reassignment() {
    effect_denied("requires [ capability(\"pulse.publish\") ]; let mut f = |x| x; if true { f = |x| pulse.publish(\"escaped\", x); } let g = f; f = |x| x; pure cell c := [1].map(g);");
}
#[test]
fn admitted_external_callback_on_named_collection_runs_once() {
    let mut s = SourceSession::new(vec!["pulse.publish".into()], SessionLimits::default());
    let mut h = LocalHost::default();
    submit(&mut s, "import \"pulse\" as pulse; requires [ capability(\"pulse.publish\") ]; let xs = [1]; let f = |x| pulse.publish(\"ok\", x); xs.map(f);", &mut h);
    assert_eq!(h.published, ["ok"]);
}

#[test]
fn early_return_cannot_promote_unexecuted_pure_callback_metadata() {
    let mut s = SourceSession::new(vec!["pulse.publish".into()], SessionLimits::default());
    let mut h = LocalHost::default();
    submit(&mut s, "import \"pulse\" as p; requires [ capability(\"pulse.publish\") ]; let mut f = |x| p.publish(\"escaped\", x);", &mut h);
    for source in ["return 1; f = |x| x;", "if true { return 1; } f = |x| x;"] {
        let e = s.submit(source, &mut h, Budget::default()).unwrap_err();
        assert_eq!(e.diagnostic.code, DiagCode::E100);
        assert!(!e.execution_attempted);
    }
    let e = s
        .submit("= [1].map(f)", &mut h, Budget::default())
        .unwrap_err();
    assert_eq!(e.diagnostic.code, DiagCode::E200);
    assert!(!e.execution_attempted);
    assert!(h.published.is_empty());
}
#[test]
fn recursive_and_failed_budget_diagnostics_are_bounded_and_preserved() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    let source = "pure fn recursive() budget(steps: recursive()) { return 1; }";
    let e = s.submit(source, &mut h, Budget::default()).unwrap_err();
    assert_eq!(e.diagnostic.code, DiagCode::E400);
    assert!(!e.execution_attempted);
    let program = parse_program(source).unwrap();
    let mut env = vibe::Env::default();
    let mut engine = vibe::Engine::with_program(&mut h, Budget::default(), &program);
    let e = engine
        .call_function(&program, "recursive", vec![], &mut env)
        .unwrap_err();
    assert_eq!(e.code, DiagCode::E400);
    assert_eq!(e.message, "budget expressions must not call user functions");
    submit(
        &mut s,
        "pure fn broken() budget(steps: 1 / 0) { return 1; }",
        &mut h,
    );
    let e = s.submit("broken()", &mut h, Budget::default()).unwrap_err();
    assert_eq!(e.diagnostic.code, DiagCode::E600);
    assert_eq!(e.diagnostic.message, "division by zero");
}

#[test]
fn effectful_native_host_seams_require_actual_grants() {
    for call in [
        "cue.post(\"x\", 1)",
        "dag.execute({}, {})",
        "hid.poll()",
        "hid.wait(1)",
        "crypto.sign(\"key\", \"x\")",
        "crypto.generate_key(\"ed25519\")",
    ] {
        effect_denied(&format!("pure fn bad() {{ return {call}; }}"));
        let mut s = SourceSession::default();
        let mut h = LocalHost::default();
        let e = s
            .submit(&format!("{call};"), &mut h, Budget::default())
            .unwrap_err();
        assert_eq!(e.diagnostic.code, DiagCode::E300);
        assert!(!e.execution_attempted);
    }
}

#[derive(Default)]
struct CueCounter {
    local: LocalHost,
    cues: usize,
    fail_after_dispatch: bool,
}
impl vibe::Host for CueCounter {
    fn graph_query(
        &mut self,
        args: &[Value],
        take: u64,
        span: vibe::Span,
    ) -> Result<Value, vibe::Diagnostic> {
        self.local.graph_query(args, take, span)
    }
    fn graph_stage(&mut self, value: &Value, span: vibe::Span) -> Result<Value, vibe::Diagnostic> {
        self.local.graph_stage(value, span)
    }
    fn graph_commit(&mut self, span: vibe::Span) -> Result<Value, vibe::Diagnostic> {
        self.local.graph_commit(span)
    }
    fn aura_validate(
        &mut self,
        n: &Value,
        s: &Value,
        span: vibe::Span,
    ) -> Result<Value, vibe::Diagnostic> {
        self.local.aura_validate(n, s, span)
    }
    fn pulse_publish(
        &mut self,
        topic: &str,
        value: &Value,
        span: vibe::Span,
    ) -> Result<Value, vibe::Diagnostic> {
        self.local.pulse_publish(topic, value, span)
    }
    fn cue_post(
        &mut self,
        _cue: &str,
        _value: &Value,
        span: vibe::Span,
    ) -> Result<Value, vibe::Diagnostic> {
        self.cues += 1;
        if self.fail_after_dispatch {
            return Err(
                vibe::Diagnostic::new(DiagCode::E600, span, "cue failed after dispatch")
                    .with_fix("reconcile the original request before retrying")
                    .with_evidential(0.25, 0.75),
            );
        }
        Ok(Value::Receipt)
    }
}
#[test]
fn denied_native_cue_has_no_host_dispatch_and_explicit_grant_allows_one() {
    let mut h = CueCounter::default();
    let mut denied = SourceSession::default();
    let source = "requires [ capability(\"cue.post\") ]; cue.post(\"once\", 1);";
    let e = denied
        .submit(source, &mut h, Budget::default())
        .unwrap_err();
    assert_eq!(e.diagnostic.code, DiagCode::E300);
    assert_eq!(h.cues, 0);
    let mut admitted = SourceSession::new(vec!["cue.post".into()], SessionLimits::default());
    admitted.submit(source, &mut h, Budget::default()).unwrap();
    assert_eq!(h.cues, 1);
}

#[test]
fn session_function_recursion_has_a_qualified_stack_bound() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    submit(&mut s,"pure fn finite(n: i64) { return if n > 0 { finite(n - 1) } else { 5 }; } pure fn endless() { return endless(); }",&mut h);
    assert_eq!(submit(&mut s, "finite(3)", &mut h), Value::I64(5));
    let e = s
        .submit("endless()", &mut h, Budget::default())
        .unwrap_err();
    assert_eq!(e.diagnostic.code, DiagCode::E400);
    assert_eq!(e.diagnostic.message, "call depth exceeded");
}

#[test]
fn source_session_guards_actual_parser_ingress_before_execution() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    submit(&mut s, "let mut accepted: i64 = 2;", &mut h);
    let revision = s.revision();
    for source in [
        format!("= {}0{}", "[".repeat(1000), "]".repeat(1000)),
        format!("= {}true", "!".repeat(1000)),
        format!("= 1{}", "+1".repeat(1000)),
        format!("= x{}", ".field".repeat(1000)),
        format!("let x: {}i64{};", "List<".repeat(1000), ">".repeat(1000)),
    ] {
        let e = s.submit(&source, &mut h, Budget::default()).unwrap_err();
        assert_eq!(e.diagnostic.code, DiagCode::E400);
        assert!(!e.execution_attempted);
        assert_eq!(s.revision(), revision);
        assert_eq!(s.value("accepted"), Some(&Value::I64(2)));
    }
}

#[test]
fn canonical_session_preserves_complete_host_error_without_retry_or_rollback_claim() {
    let mut h = CueCounter {
        fail_after_dispatch: true,
        ..CueCounter::default()
    };
    let mut s = SourceSession::new(vec!["cue.post".into()], SessionLimits::default());
    let e = s
        .submit(
            "requires [ capability(\"cue.post\") ]; cue.post(\"partial\", 1);",
            &mut h,
            Budget::default(),
        )
        .unwrap_err();
    let expected = vibe::Diagnostic::new(
        DiagCode::E600,
        e.diagnostic.span,
        "cue failed after dispatch",
    )
    .with_fix("reconcile the original request before retrying")
    .with_evidential(0.25, 0.75);
    assert_eq!(e.diagnostic, expected);
    assert!(e.execution_attempted && e.host_reconciliation_required);
    assert_eq!(s.revision(), 0);
    assert_eq!(h.cues, 1);
    s.submit("1 + 1", &mut h, Budget::default()).unwrap();
    assert_eq!(h.cues, 1);
}

#[test]
fn source_mode_detection_handles_leading_bom_and_retains_original_bytes() {
    let mut s = SourceSession::default();
    let mut h = LocalHost::default();
    let source = "\u{FEFF}= 1 + 2";
    assert_eq!(submit(&mut s, source, &mut h), Value::I64(3));
    assert_eq!(s.sources()[0], source);
    submit(&mut s, "\u{FEFF}let mut x: i64 = 1;", &mut h);
    assert_eq!(submit(&mut s, "x + 1", &mut h), Value::I64(2));
}
