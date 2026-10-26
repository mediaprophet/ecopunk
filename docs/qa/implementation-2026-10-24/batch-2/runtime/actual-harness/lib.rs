//! Integration against actual applied sibling source paths, without source edits.
#[path = "C:/github/poet/native/src/repl.rs"]
pub mod repl;
#[cfg(test)]
#[path = "../harness/tests.rs"]
mod tests;
#[cfg(test)]
mod poet_adapter;
