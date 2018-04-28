//! Time how long it takes to solve certain scrambles on the puzzle.

extern crate locky_puzzle;

use std::time::Instant;
use locky_puzzle::{Algo, Heuristic, NopHeuristic, solve};

fn main() {
    time_solve("NopHeuristic", "B D2 B' U2 L2", &NopHeuristic());
    time_solve("NopHeuristic", "U2 D' B R B R L2", &NopHeuristic());
}

fn time_solve<H: Heuristic + Sync>(label: &str, scramble: &str, heuristic: &H) {
    let scramble: Algo = scramble.parse().unwrap();
    let state = scramble.state();

    let start = Instant::now();
    solve(&state, heuristic, scramble.0.len() as u8).unwrap();
    let elapsed = start.elapsed();

    println!("{}({}) took {} ms", label, scramble, elapsed.as_secs() * 1000 +
        ((elapsed.subsec_nanos() / 1000000) as u64))
}
