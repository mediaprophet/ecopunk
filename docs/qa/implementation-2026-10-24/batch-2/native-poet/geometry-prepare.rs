use vibe::session::{SourceSession, SessionLimits};
fn main() {
    let source = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let admitted = SourceSession::new(vec!["capability.invoke".into(), "ComputationalGeometry.extrude_profile".into()], SessionLimits::default());
    admitted.prepare(&source).expect("geometry source must prepare under explicit grant");
    let denied = SourceSession::new(Vec::new(), SessionLimits::default());
    let error = match denied.prepare(&source) { Ok(_) => panic!("self grant admitted"), Err(error) => error };
    assert_eq!(error.code, vibe::DiagCode::E300);
    println!("PASS: actual SourceSession admitted explicit host dispatcher + exact geometry grants; empty grants rejected E300. Prepare only: no engine/host dispatch or geometry output claimed.");
}

