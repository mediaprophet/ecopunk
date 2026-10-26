from pathlib import Path
import hashlib, json

raise SystemExit('Historical bootstrap only. Reviewed candidate files and patch-map.json are authoritative; do not regenerate with this script.')

BASE = Path(__file__).parent
SOURCE = Path('C:/github/qualiaDB/crates/vibe/src')
CAND = BASE / 'candidate/vibe/src'
guards = []

def original(rel):
    data = (SOURCE / rel).read_bytes()
    out = BASE / 'original/vibe/src' / rel
    out.parent.mkdir(parents=True, exist_ok=True)
    if out.exists():
        assert out.read_bytes() == data, f'upstream changed: {rel}'
    else:
        out.write_bytes(data)
    guards.append(dict(target=str(SOURCE / rel), original=str(out), sha256=hashlib.sha256(data).hexdigest()))
    return data.decode('utf-8').replace('\r\n', '\n')

def write(rel, text):
    out = CAND / rel
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text, encoding='utf-8', newline='\n')

check = original('check.rs')
# Ownership review: decompose 985-line implementation into routing, statements,
# expressions and focused session/call context; preserve crate exports.
check = check.replace('env: &mut HashMap<String, Type>', 'env: &mut CheckEnv')
check = check.replace('env: &HashMap<String, Type>', 'env: &CheckEnv')
check = check.replace('let mut env = HashMap::new();', 'let mut env = CheckEnv::default();')
check = check.replace('Item::Function(f) => check_function(f, aliases, granted),', 'Item::Function(f) => check_function_in(f, env, aliases, granted),')
check = check.replace('Item::Hook(h) => check_hook(h, aliases, granted),', 'Item::Hook(h) => check_hook(h, env, aliases, granted),')
start = check.index('/// Checks function.')
end = check.index('fn check_hook(', start)
check = check[:start] + check[start:end].replace('pub(crate) fn check_function(', 'fn check_function_in(').replace('    f: &FunctionDecl,', '    f: &FunctionDecl,\n    outer: &CheckEnv,').replace('let mut env = CheckEnv::default();', 'let mut env = outer.function_scope();') + check[end:]
check = check.replace('    h: &HookDecl,', '    h: &HookDecl,\n    outer: &CheckEnv,')
pos=check.index('fn check_hook(')
check=check[:pos]+check[pos:].replace('let mut env = CheckEnv::default();', 'let mut env = outer.function_scope();',1)
check=check.replace('    let granted: Vec<&str> = program.requires', '    let granted: Vec<&str> = program.requires',1)
check=check.replace('    let mut env = CheckEnv::default();\n    let mut mutables = HashSet::new();\n    for item', '    let mut env = CheckEnv::default();\n    env.functions = infer_functions(program, &aliases, &granted);\n    let mut mutables = HashSet::new();\n    for item',1)
check=check.replace('            walk_expr(&c.value, env, aliases, granted, Effect::Pure, false, false)?;\n            Ok(Effect::Pure)', '            let effect = walk_expr(&c.value, env, aliases, granted, Effect::Pure, false, false)?;\n            env.insert(c.name.clone(), c.ty.as_ref().map(Type::from_ast).unwrap_or(Type::Unknown));\n            env.values.insert(c.name.clone(), c.value.clone());\n            Ok(effect)',1)
# Retain callable syntax for higher-order lambda inspection; no new runtime calls.
check=check.replace('            env.insert(name.clone(), t);', '            env.insert(name.clone(), t);\n            if let Some(v) = value { env.values.insert(name.clone(), v.clone()); } else { env.values.remove(name); }',1)
check=check.replace('            let a = walk_expr(target, env, aliases, granted, ambient, tick, budgeted)?;', '            if let Some(n) = target.ident_name() { env.values.insert(n.to_string(), value.clone()); }\n            let a = walk_expr(target, env, aliases, granted, ambient, tick, budgeted)?;',1)
check=check.replace('            let path = call_path(callee, aliases);', '            let path = call_path(callee, aliases);\n            let call_effect = resolved_call_effect(callee, args, env, aliases, granted, budgeted)?;\n            if call_effect > ambient {\n                return Err(Diagnostic::new(DiagCode::E200, expr.span, "call exceeds permitted effect class"));\n            }',1)
check=check.replace('let mut e = path.as_deref().map(binding_effect).unwrap_or(Effect::Pure);', 'let mut e = call_effect;',1)
check=check.replace('            let e2 = walk_expr(right, env, aliases, granted, ambient, tick, budgeted)?;\n            Ok(e1.join(e2))', '            let e2 = match &right.kind {\n                ExprKind::Call { .. } => walk_expr(right, env, aliases, granted, ambient, tick, budgeted)?,\n                _ => {\n                    let call = Expr { span: right.span, kind: ExprKind::Call { callee: right.clone(), args: Vec::new() } };\n                    walk_expr(&call, env, aliases, granted, ambient, tick, budgeted)?\n                }\n            };\n            Ok(e1.join(e2))',1)
# Keep direct host policy checks, but local fns take evaluator precedence.
check=check.replace('            if ambient == Effect::Pure {', '            if ambient == Effect::Pure && !path.as_ref().is_some_and(|p| env.functions.contains_key(p)) {',1)
check=check.replace('            if let Some(p) = &path {\n                if p == "capability.invoke"', '            if let Some(p) = path.as_ref().filter(|p| !env.functions.contains_key(*p)) {\n                if p == "capability.invoke"',1)
# Extract existing cohesive parts; child modules inherit checker internals.
stmt_start = check.index('fn walk_stmt(')
expr_start = check.index('fn walk_expr(')
helpers_start = check.index('fn quantity_unit_of(')
test_start=check.index('#[cfg(test)]')
header='use super::*;\n\n'
write('check/statements.rs', header + check[stmt_start:expr_start].replace('fn walk_stmt(', 'pub(super) fn walk_stmt(',1))
write('check/expressions.rs', header + check[expr_start:helpers_start].replace('fn walk_expr(', 'pub(super) fn walk_expr(',1))
helpers=check[helpers_start:test_start]
for name in ['quantity_unit_of','bind_pattern_in_env','call_path','is_take_arg','is_literal_true','expr_is_bounded_source','iter_is_bounded']:
    helpers=helpers.replace('fn '+name+'(', 'pub(super) fn '+name+'(')
write('check/helpers.rs',header+helpers)
write('check/tests.rs', check[test_start:].replace('#[cfg(test)]\nmod tests {\n    use super::*;\n', 'use super::*;\n',1).rstrip()[:-1])
route=check[:stmt_start]
route=route.replace('use std::collections::{HashMap, HashSet};','use std::collections::{HashMap, HashSet};\nmod context;\nmod expressions;\nmod statements;\nmod helpers;\n#[cfg(test)]\nmod tests;\nuse context::*;\nuse expressions::walk_expr;\nuse statements::walk_stmt;\nuse helpers::*;')
write('check/mod.rs',route)
# check.rs stays as compatibility entry point without module conflict.
write('check.rs', '//! Type and effect checker implementation and compatibility exports.\n#[path = "check/mod.rs"]\nmod implementation;\npub use implementation::*;\npub(crate) use implementation::{CheckEnv, check_submission};\n')

lib=original('lib.rs').replace('mod span;', 'mod span;\npub mod session;')
write('lib.rs',lib)
engine=original('eval/mod.rs')
engine=engine.replace('pub struct Env {', '#[derive(Clone)]\npub struct Env {',1)
engine=engine.replace('program: Option<*const Program>', 'program: Option<std::sync::Arc<Program>>')
engine=engine.replace('program: Some(program as *const Program)', 'program: Some(std::sync::Arc::new(program.clone()))')
engine=engine.replace('    /// Stored as a raw pointer because the program\'s lifetime is not tied to\n    /// `\'a` â€” it is passed by reference to `call_function` / `call_hook` and\n    /// must outlive the engine evaluation (the caller guarantees this).', '    /// Owned once per engine; recursive calls share the same immutable AST.')
write('eval/mod.rs',engine)
prog=original('eval/program.rs').replace('let prev = self.program;', 'let prev = self.program.take();').replace('self.program = Some(program as *const Program);', 'self.program = Some(std::sync::Arc::new(program.clone()));')
write('eval/program.rs',prog)
call=original('eval/call.rs').replace('if let Some(program_ptr) = self.program {\n            let program = unsafe { &*program_ptr };', 'if let Some(program) = self.program.clone() {')
write('eval/call.rs',call)
(BASE/'source-guards.json').write_text(json.dumps(guards,indent=2),encoding='utf-8')
