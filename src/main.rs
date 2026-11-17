//! SigmaOS--Dante — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos__dante_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS--Dante: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS--Dante: ok");
}
