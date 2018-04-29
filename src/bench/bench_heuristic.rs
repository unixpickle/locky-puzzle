//! Time how long it takes to generate various tables.

extern crate locky_puzzle;

use std::time::Instant;

use locky_puzzle::{ArrowAxisProj, CornerProj, LockProj, Proj, ProjHeuristic};

fn main() {
    time_heuristic::<ArrowAxisProj>("ArrowAxisProj(5)", 5);
    time_heuristic::<ArrowAxisProj>("ArrowAxisProj(6)", 6);
    time_heuristic::<ArrowAxisProj>("ArrowAxisProj(7)", 7);

    time_heuristic::<CornerProj>("CornerProj(5)", 5);
    time_heuristic::<CornerProj>("CornerProj(6)", 6);
    time_heuristic::<CornerProj>("CornerProj(7)", 7);

    time_heuristic::<LockProj>("LockProj(5)", 5);
    time_heuristic::<LockProj>("LockProj(6)", 6);
    time_heuristic::<LockProj>("LockProj(7)", 7);
    time_heuristic::<LockProj>("LockProj(8)", 8);
    time_heuristic::<LockProj>("LockProj(9)", 9);
    time_heuristic::<LockProj>("LockProj(10)", 10);
}

fn time_heuristic<T: Proj>(label: &str, depth: u8) {
    let start = Instant::now();
    let size = ProjHeuristic::<T>::generate(depth).table.len();
    let elapsed = start.elapsed();
    println!("{} took {} ms (size {})", label, elapsed.as_secs() * 1000 +
        ((elapsed.subsec_nanos() / 1000000) as u64), size)
}
