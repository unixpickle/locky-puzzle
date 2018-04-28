//! Projections onto subspaces of the puzzle state.

use std::hash::Hash;

use super::state::{Face, State};

/// A projection of a state onto a subspace of all possible states.
///
/// Projections must satisfy some properties:
/// * The same state always has the same projection.
/// * If you know a projection, you can apply moves and get a new projection.
/// * A projection must know which faces are locked.
pub trait Proj: Clone + Eq + Hash {
    /// Project the state onto the subspace.
    fn project(s: &State) -> Self;
}

/// A projection of a state onto the sticker directions.
///
/// This is the least amount of information a Proj could possibly contain,
/// since any less information could not determine if a move was locked.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct LockProj {
    pub clockwise: [u8; 6],
    pub counter: [u8; 6]
}

impl Proj for LockProj {
    fn project(s: &State) -> Self {
        use super::state::Direction::*;
        let mut res = LockProj{clockwise: [0; 6], counter: [0; 6]};
        let mut clock_idx = 0;
        let mut counter_idx = 0;
        for (i, sticker) in s.0.iter().enumerate() {
            match sticker.direction {
                Clockwise => {
                    res.clockwise[clock_idx] = i as u8;
                    clock_idx += 1;
                },
                Counter => {
                    res.counter[counter_idx] = i as u8;
                    counter_idx += 1;
                },
                Neutral => ()
            }
        }
        res
    }
}

/// A projection of a state onto the corners.
///
/// Corners are encoded by storing two of their three stickers.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct CornerProj {
    pub lock: LockProj,
    pub corners: [Face; 16]
}

impl Proj for CornerProj {
    fn project(s: &State) -> Self {
        use Face::*;
        let mut corners = [U; 16];
        for face_idx in 0..4 {
            for (i, sticker_idx) in [0, 2, 5, 7].iter().enumerate() {
                corners[face_idx * 4 + i] = s.0[face_idx * 8 + sticker_idx].face;
            }
        }
        CornerProj{
            lock: Proj::project(s),
            corners: corners
        }
    }
}
