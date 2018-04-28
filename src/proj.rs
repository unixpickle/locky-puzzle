//! Projections onto subspaces of the puzzle state.

use std::hash::{Hash, Hasher};

use super::state::{Direction, Face, State};

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
    directions: [Direction; 24]
}

impl Proj for LockProj {
    fn project(s: &State) -> Self {
        use Direction::*;
        let mut res = LockProj{directions: [Neutral; 24]};
        for face_idx in 0..6 {
            for (i, sticker_idx) in [1, 3, 4, 6].iter().enumerate() {
                let dir = s.0[face_idx * 8usize + sticker_idx].direction;
                res.directions[face_idx * 4 + i] = dir;
            }
        }
        res
    }
}

impl Hash for LockProj {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&[
            dir_quad_to_u8(self.directions[0], self.directions[1], self.directions[2],
                self.directions[3]),
            dir_quad_to_u8(self.directions[4], self.directions[5], self.directions[6],
                self.directions[7]),
            dir_quad_to_u8(self.directions[8], self.directions[9], self.directions[10],
                self.directions[11]),
            dir_quad_to_u8(self.directions[12], self.directions[13], self.directions[14],
                self.directions[15]),
            dir_quad_to_u8(self.directions[16], self.directions[17], self.directions[18],
                self.directions[19]),
            dir_quad_to_u8(self.directions[20], self.directions[21], self.directions[22],
                self.directions[23])
        ]);
    }
}

/// A projection of a state onto the corners.
///
/// Corners are encoded by storing two of their three stickers.
#[derive(Clone, Eq, PartialEq)]
pub struct CornerProj {
    lock: LockProj,
    corners: [Face; 16]
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

fn dir_quad_to_u8(d1: Direction, d2: Direction, d3: Direction, d4: Direction) -> u8 {
    dir_to_u8(d1) | (dir_to_u8(d2) << 2) | (dir_to_u8(d3) << 4) | (dir_to_u8(d4) << 6)
}

fn dir_to_u8(dir: Direction) -> u8 {
    use Direction::*;
    match dir {
        Clockwise => 0,
        Counter => 1,
        Neutral => 2
    }
}
