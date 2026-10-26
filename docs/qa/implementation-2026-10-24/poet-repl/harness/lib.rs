// Select either exact candidate source or the recorded original with the same
// regression tests. This harness never changes the sibling repositories.
#[cfg(not(feature = "baseline"))]
#[path = "../candidate/native/src/repl.rs"]
pub mod repl;

#[cfg(feature = "baseline")]
#[path = "baseline.rs"]
pub mod repl;
