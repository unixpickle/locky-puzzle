//! Solving the puzzle.

use std::mem::drop;
use std::sync::mpsc::channel;

use super::heuristic::Heuristic;
use super::move_gen::MoveGen;
use super::moves::{Algo, Move};
use super::proj::Proj;
use super::state::State;
use super::thread::ThreadScope;

/// Find a solution of the given depth.
///
/// Uses multiple threads for the search.
///
/// This may find sub-optimal solutions if the given depth is too large.
/// Thus, it is recommended that callers iteratively try deeper and deeper
/// searches until a solution is found.
pub fn solve<H: Heuristic>(
    state: &State,
    heuristic: &H,
    depth: u8
) -> Option<Algo> {
    if depth == 0 {
        return solve_serial(state, heuristic, depth);
    }

    let (send, recv) = channel();

    let mut threads = Vec::new();
    for (gen, m) in MoveGen::new() {
        if state.is_locked(m.face) {
            continue;
        }
        let local_send = send.clone();
        threads.push(ThreadScope::spawn(move || {
            let mut local_state = state.clone();
            m.apply(&mut local_state);
            let mut hist = vec![m];
            if solve_search(&local_state, heuristic, depth - 1, &mut hist, gen) {
                local_send.send(hist).unwrap();
            }
        }));
    }

    drop(send);

    let mut best_solution: Option<Vec<Move>> = None;
    for solution in recv {
        if best_solution.is_none() || solution.len() < best_solution.as_ref().unwrap().len() {
            best_solution = Some(solution);
        }
    }
    best_solution.map(Algo)
}

/// Find a solution of the given depth.
///
/// Uses a single thread.
///
/// See solve() for details.
pub fn solve_serial<H: Heuristic>(
    state: &State,
    heuristic: &H,
    depth: u8
) -> Option<Algo> {
    let mut solution = Vec::new();
    if solve_search(state, heuristic, depth, &mut solution, MoveGen::new()) {
        Some(Algo(solution))
    } else {
        None
    }
}

fn solve_search<H: Heuristic>(
    state: &State,
    heuristic: &H,
    depth: u8,
    history: &mut Vec<Move>,
    gen: MoveGen
) -> bool {
    if state.is_solved() {
        return true;
    } else if depth == 0 || depth < heuristic.lower_bound(state, Proj::project(state)) {
        return false;
    }
    for (new_gen, m) in gen {
        if state.is_locked(m.face) {
            continue;
        }
        let mut new_state = state.clone();
        m.apply(&mut new_state);
        history.push(m);
        if solve_search(&new_state, heuristic, depth - 1, history, new_gen) {
            return true;
        }
        history.pop();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use heuristic::NopHeuristic;
    use moves::Algo;

    /// Test solving zero-move scrambles.
    #[test]
    fn zero_move_scramble() {
        let actual = solve(&State::default(), &NopHeuristic(), 0).unwrap();
        assert_eq!(actual, Algo(Vec::new()));
    }

    /// Test solving a one-move scramble.
    #[test]
    fn one_move_scramble() {
        let algo: Algo = "L'".parse().unwrap();
        let actual = solve(&algo.state(), &NopHeuristic(), 1).unwrap();
        assert_eq!(actual, "L".parse().unwrap());
    }

    /// Test the case when the depth isn't high enough.
    #[test]
    fn not_enough_depth() {
        let algo: Algo = "B D2 B' U2 L2".parse().unwrap();
        let actual = solve(&algo.state(), &NopHeuristic(), 4);
        assert!(actual.is_none());
    }

    /// Test solving a five-move scramble.
    #[test]
    fn five_move_scramble() {
        let algo: Algo = "B D2 B' U2 L2".parse().unwrap();
        let actual = solve(&algo.state(), &NopHeuristic(), 5).unwrap();
        assert_eq!(actual, "L2 U2 B D2 B'".parse().unwrap());
    }
}
