//! Optional launcher imports the actual applied Poet modules.
//! The primary product launch is poet studio, not this harness.
#[path = "C:/github/poet/native/src/graphical/mod.rs"]
mod graphical;
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(error) = graphical::entry::run(&args) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
