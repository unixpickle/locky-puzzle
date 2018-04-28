//! Solving the puzzle.

use std::sync::mpsc::channel;

use super::heuristic::Heuristic;
use super::move_gen::MoveGen;
use super::moves::Move;
use super::state::State;
use super::thread::ThreadScope;

/// Find a solution of the given depth.
///
/// This may find sub-optimal solutions if the given depth is too large.
/// Thus, it is recommended that callers iteratively try deeper and deeper
/// searches until a solution is found.
pub fn solve<H: Heuristic + Send + Sync + 'static>(
    state: &State,
    heuristic: &H,
    depth: u8,
    use_threads: bool
) -> Option<Vec<Move>> {
    if !use_threads {
        let mut solution = Vec::new();
        if solve_search(state, heuristic, depth, &mut solution, MoveGen::new()) {
            return Some(solution)
        } else {
            return None;
        }
    }

    let (send, recv) = channel();

    let mut threads = Vec::new();
    for (gen, m) in MoveGen::new() {
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

    let mut best_solution: Option<Vec<Move>> = None;
    for solution in recv {
        if best_solution.is_none() || solution.len() < best_solution.as_ref().unwrap().len() {
            best_solution = Some(solution);
        }
    }
    best_solution
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
    } else if depth == 0 {
        return false;
    }
    for (new_gen, m) in gen {
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
