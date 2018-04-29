//! Time how long it takes to generate various tables.

extern crate locky_puzzle;

use std::time::Instant;

use locky_puzzle::{ArrowAxisProj, CornerProj, FbCoProj, LockProj, Proj, ProjHeuristic,
    RlCoProj, UdCoProj};

fn main() {
    for i in 5..8 {
        time_heuristic::<ArrowAxisProj>("ArrowAxisProj", i);
    }

    for i in 5..8 {
        time_heuristic::<CornerProj>("CornerProj", i);
    }

    for i in 5..8 {
        time_heuristic::<FbCoProj>("FbCoProj", i);
    }

    for i in 5..11 {
        time_heuristic::<LockProj>("LockProj", i);
    }

    for i in 5..8 {
        time_heuristic::<RlCoProj>("RlCoProj", i);
    }

    for i in 5..8 {
        time_heuristic::<UdCoProj>("UdCoProj", i);
    }
}

fn time_heuristic<T: Proj>(label: &str, depth: u8) {
    let start = Instant::now();
    let size = ProjHeuristic::<T>::generate(depth).table.len();
    let elapsed = start.elapsed();
    println!("{}({}) took {} ms (size {})", label, depth, elapsed.as_secs() * 1000 +
        ((elapsed.subsec_nanos() / 1000000) as u64), size)
}
