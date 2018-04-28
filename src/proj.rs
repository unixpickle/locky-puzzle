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
    packed_faces: [u8; 6]
}

impl Proj for LockProj {
    fn project(s: &State) -> Self {
        let mut res = LockProj{packed_faces: [0; 6]};
        for face_idx in 0..6 {
            res.packed_faces[face_idx] = dir_quad_to_u8(
                s.0[face_idx * 8usize + 1].direction,
                s.0[face_idx * 8usize + 3].direction,
                s.0[face_idx * 8usize + 4].direction,
                s.0[face_idx * 8usize + 6].direction
            );
        }
        res
    }
}

impl Hash for LockProj {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.packed_faces);
    }
}

/// A projection of a state onto the corners.
///
/// Corners are encoded by storing two of their three stickers.
#[derive(Clone, Eq, PartialEq)]
pub struct CornerProj {
    lock: LockProj,
    packed_corners: [u8; 8]
}

impl Proj for CornerProj {
    fn project(s: &State) -> Self {
        let mut corners = [0; 8];
        for face_idx in 0..4 {
            corners[face_idx * 2] = face_pair_to_u8(
                s.0[face_idx * 8 + 0].face,
                s.0[face_idx * 8 + 2].face
            );
            corners[face_idx * 2 + 1] = face_pair_to_u8(
                s.0[face_idx * 8 + 5].face,
                s.0[face_idx * 8 + 7].face
            );
        }
        CornerProj{
            lock: Proj::project(s),
            packed_corners: corners
        }
    }
}

impl Hash for CornerProj {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lock.hash(state);
        state.write(&self.packed_corners);
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
