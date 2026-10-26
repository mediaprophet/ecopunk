from pathlib import Path
import hashlib,json
raise SystemExit('Historical bootstrap only. Use the frozen Poet candidate and poet-patch-map.json; this bootstrap predates reviewed grant plumbing.')
base=Path(__file__).parent
target=Path('C:/github/poet/native/src/repl.rs')
data=target.read_bytes()
original=base/'original/poet/native/src/repl.rs'
original.parent.mkdir(parents=True,exist_ok=True)
if original.exists(): assert original.read_bytes()==data
else: original.write_bytes(data)
source=data.decode('utf8').replace('\r\n','\n')
source=source.replace('use std::io::{self, BufRead, Write};','use std::io::{self, BufRead, Write};\nuse vibe::session::{SourceSession, SessionLimits, SubmissionError, SubmissionResult};')
at=source.index('/// Run an interactive REPL')
source=source[:at]+'''/// Evaluate once through the canonical persistent checked source gateway.
/// The caller supplies a session whose actual grants were admitted by its host.
pub fn eval_session_line(
    line: &str, host: &mut impl Host, session: &mut SourceSession,
) -> Result<SubmissionResult, SubmissionError> {
    session.submit(line, host, Budget::default())
}

'''+source[at:]
source=source.replace('pub fn run_repl() -> io::Result<()> {','pub fn run_repl() -> io::Result<()> {\n    run_repl_with_grants(Vec::new())\n}\n\n/// Host-admitted local capability profile. Source requests never grant authority.\n/// This local host profile is not external project/provider authorization.\npub fn run_repl_with_grants(grants: Vec<String>) -> io::Result<()> {',1)
source=source.replace('    let mut env = Env::default();\n    let stdin', '    let mut session = SourceSession::new(grants, SessionLimits::default());\n    let stdin',1)
source=source.replace('        match eval_line(trimmed, &mut host, &mut env) {\n            Ok(val) => println!("{val}"),\n            Err(diag) => println!("Error: {}", diag.message),\n        }','''        match eval_session_line(trimmed, &mut host, &mut session) {
            Ok(result) => println!("{}", result.value),
            Err(error) => {
                println!("Error: {}", error.diagnostic.message);
                if error.host_reconciliation_required {
                    println!("Host outcome requires reconciliation; no retry was attempted.");
                }
            }
        }''',1)
out=base/'candidate/poet/native/src/repl.rs';out.parent.mkdir(parents=True,exist_ok=True)
out.write_text(source,encoding='utf8',newline='\n')
(base/'poet-patch-map.json').write_text(json.dumps([dict(target=str(target),original=str(original.resolve()),
    original_sha256=hashlib.sha256(data).hexdigest(),candidate=str(out.resolve()),
    candidate_sha256=hashlib.sha256(out.read_bytes()).hexdigest())],indent=2),encoding='utf8')
print(out)
