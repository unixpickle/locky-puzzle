//! Generating valid scrambles.

use super::rand::random;

use super::move_gen::MoveGen;
use super::moves::{Algo, Move};
use super::state::State;

/// Produce a scramble that is the given number of moves.
pub fn scramble(moves: usize) -> Algo {
    let mut state = State::default();
    let mut scramble = Vec::new();
    let status = scramble_search(&mut state, moves, &mut scramble, MoveGen::new());

    // It is always possible to generate a scramble of a given length.
    // There are plenty of sequences S with a solution S' that is not the simple
    // reverse of S.
    // Thus, we can lengthen a scramble by |S|*2 moves by simply inserting S S'.
    assert!(status);

    Algo(scramble)
}

fn scramble_search(
    state: &mut State,
    moves: usize,
    history: &mut Vec<Move>,
    gen: MoveGen
) -> bool {
    if moves == 0 {
        return true;
    }
    let mut next_options: Vec<(MoveGen, Move)> = gen.into_iter().collect();
    while next_options.len() > 0 {
        let idx = random::<usize>() % next_options.len();
        let (next_gen, m) = next_options.remove(idx);
        m.apply(state);
        history.push(m);
        if scramble_search(state, moves - 1, history, next_gen) {
            return true;
        }
        history.pop();
        m.inverse().apply(state);
    }
    false
}
