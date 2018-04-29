//! Solving the puzzle.

use std::mem::drop;
use std::sync::mpsc::channel;

use super::heuristic::Heuristic;
use super::move_gen::MoveGen;
use super::moves::{Algo, Move};
use super::proj::{LockProj, Proj};
use super::state::State;
use super::thread::ThreadScope;

macro_rules! parallel_search {
    ( $state:expr, $heuristic:expr, $depth:expr, $serial_fn:expr, $search_fn:expr ) => {
        if $depth == 0 {
            $serial_fn($state, $heuristic, $depth)
        } else {
            let (send, recv) = channel();

            let mut threads = Vec::new();
            for (gen, m) in MoveGen::new() {
                if $state.is_locked(m.face) {
                    continue;
                }
                let local_send = send.clone();
                threads.push(ThreadScope::spawn(move || {
                    let mut local_state = $state.clone();
                    m.apply(&mut local_state);
                    let mut hist = vec![m];
                    if $search_fn(&local_state, $heuristic, $depth - 1, &mut hist, gen) {
                        local_send.send(hist).unwrap();
                    }
                }));
            }

            drop(send);

            let mut best_solution: Option<Vec<Move>> = None;
            for solution in recv {
                if best_solution.is_none() ||
                    solution.len() < best_solution.as_ref().unwrap().len() {
                    best_solution = Some(solution);
                }
            }
            best_solution.map(Algo)
        }
    }
}

/// Find a solution of the given depth.
///
/// Uses multiple threads for the search.
///
/// This may find sub-optimal solutions if the given depth is too large.
/// Thus, it is recommended that callers iteratively try deeper and deeper
/// searches until a solution is found.
pub fn solve<H: Heuristic + ?Sized>(
    state: &State,
    heuristic: &H,
    depth: u8
) -> Option<Algo> {
    parallel_search!(state, heuristic, depth, solve_serial, solve_search)
}

/// Find a solution of the given depth.
///
/// Uses a single thread.
///
/// See solve() for details.
pub fn solve_serial<H: Heuristic + ?Sized>(
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

/// Find a solution under a projection of the given depth.
///
/// Uses multiple threads for the search.
///
/// This may find sub-optimal solutions if the given depth is too large.
/// Thus, it is recommended that callers iteratively try deeper and deeper
/// searches until a solution is found.
pub fn proj_solve<P: Proj, H: Heuristic + ?Sized>(
    state: &State,
    heuristic: &H,
    depth: u8
) -> Option<Algo> {
    // TODO: why does this blow up without argument types?
    let search = |a: &State, b: &H, c: u8, d: &mut Vec<Move>, e: MoveGen| {
        let solved_state = P::project(&State::default());
        proj_solve_search::<P, H>(&solved_state, a, b, c, d, e)
    };
    let search_ref = &search;
    parallel_search!(state, heuristic, depth, proj_solve_serial::<P, H>, search_ref)
}

/// Find a solution under a projection of the given depth.
///
/// Uses a single thread.
///
/// See proj_solve() for details.
pub fn proj_solve_serial<P: Proj, H: Heuristic + ?Sized>(
    state: &State,
    heuristic: &H,
    depth: u8
) -> Option<Algo> {
    let mut solution = Vec::new();
    let success = proj_solve_search::<P, H>(
        &P::project(&State::default()),
        state,
        heuristic,
        depth,
        &mut solution,
        MoveGen::new()
    );
    if success {
        Some(Algo(solution))
    } else {
        None
    }
}

macro_rules! search_step {
    ( $state:expr, $history:expr, $m:expr ) => {
        {
            if $state.is_locked($m.face) {
                continue;
            }
            let mut new_state = $state.clone();
            $m.apply(&mut new_state);
            $history.push($m);
            new_state
        }
    }
}

fn solve_search<H: Heuristic + ?Sized>(
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
        let new_state = search_step!(state, history, m);
        if solve_search(&new_state, heuristic, depth - 1, history, new_gen) {
            return true;
        }
        history.pop();
    }
    false
}

fn proj_solve_search<P: Proj, H: Heuristic + ?Sized>(
    solution: &P,
    state: &State,
    heuristic: &H,
    depth: u8,
    history: &mut Vec<Move>,
    gen: MoveGen
) -> bool {
    let lock_proj = LockProj::project(state);
    let projection = Proj::project_with_lock(state, lock_proj.clone());
    if solution == &projection {
        return true;
    } else if depth == 0 || depth < heuristic.lower_bound(state, lock_proj) {
        return false;
    }
    for (new_gen, m) in gen {
        let new_state = search_step!(state, history, m);
        if proj_solve_search(solution, &new_state, heuristic, depth - 1, history, new_gen) {
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
    use proj::LockProj;

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

    /// Test solving the arrows on a five-move scramble.
    #[test]
    fn proj_five_move_scramble() {
        let algo: Algo = "B D2 B' U2 L2".parse().unwrap();
        let actual = proj_solve::<LockProj, _>(&algo.state(), &NopHeuristic(), 5).unwrap();
        assert_eq!(actual, "L2 U2 B D2 B'".parse().unwrap());
    }

    /// Test a case when it's easier to solve a projection than the whole cube.
    #[test]
    fn proj_simpler_solution() {
        // Apply a T-perm, which just swaps the top right and top left edges.
        let algo: Algo = "R U R' U' R' F R2 U' R' U' R U R' F'".parse().unwrap();
        let actual = proj_solve_serial::<LockProj, _>(&algo.state(), &NopHeuristic(), 1).unwrap();
        assert_eq!(actual, "U2".parse().unwrap());
    }
}
