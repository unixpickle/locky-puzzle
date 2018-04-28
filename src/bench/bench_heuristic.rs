//! Time how long it takes to generate various tables.

extern crate locky_puzzle;

use std::time::Instant;

use locky_puzzle::{CornerProj, Proj, ProjHeuristic};

fn main() {
    time_heuristic::<CornerProj>("CornerProj(5)", 5);
    time_heuristic::<CornerProj>("CornerProj(6)", 6);
    time_heuristic::<CornerProj>("CornerProj(7)", 7);
}

fn time_heuristic<T: Proj>(label: &str, depth: u8) {
    let start = Instant::now();
    ProjHeuristic::<CornerProj>::generate(depth);
    let elapsed = start.elapsed();
    println!("{} took {} ms", label, elapsed.as_secs() * 1000 +
        ((elapsed.subsec_nanos() / 1000000) as u64))
}
