//! Projections onto subspaces of the puzzle state.

use std::hash::{Hash, Hasher};

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
#[derive(Clone, Eq, PartialEq)]
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

impl Hash for LockProj {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&[self.clockwise[0], self.clockwise[1], self.clockwise[2],
                      self.clockwise[3], self.clockwise[4], self.clockwise[5],
                      self.counter[0], self.counter[1], self.counter[2],
                      self.counter[3], self.counter[4], self.counter[5]]);
    }
}

/// A projection of a state onto the corners.
///
/// Corners are encoded by storing two of their three stickers.
#[derive(Clone, Eq, PartialEq)]
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

impl Hash for CornerProj {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lock.hash(state);
        state.write(&[
            face_pair_to_u8(self.corners[0], self.corners[1]),
            face_pair_to_u8(self.corners[2], self.corners[3]),
            face_pair_to_u8(self.corners[4], self.corners[5]),
            face_pair_to_u8(self.corners[6], self.corners[7]),
            face_pair_to_u8(self.corners[8], self.corners[9]),
            face_pair_to_u8(self.corners[10], self.corners[11]),
            face_pair_to_u8(self.corners[12], self.corners[13]),
            face_pair_to_u8(self.corners[14], self.corners[15])
        ]);
    }
}

fn face_pair_to_u8(f1: Face, f2: Face) -> u8 {
    face_to_u8(f1) | (face_to_u8(f2) << 4)
}

fn face_to_u8(face: Face) -> u8 {
    use Face::*;
    match face {
        U => 0,
        D => 1,
        F => 2,
        B => 3,
        R => 4,
        L => 5
    }
}
