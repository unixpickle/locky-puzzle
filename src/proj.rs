//! Projections onto subspaces of the puzzle state.

use std::hash::{Hash, Hasher};

use super::state::{Direction, Face, State, Sticker};

/// A projection of a state onto a subspace of all possible states.
///
/// Projections must satisfy some properties:
/// * The same state always has the same projection.
/// * If you know a projection, you can apply moves and get a new projection.
/// * A projection must know which faces are locked.
pub trait Proj: Clone + Eq + Hash + Send + Sync {
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

impl LockProj {
    fn dirs_u8(d1: Direction, d2: Direction, d3: Direction, d4: Direction) -> u8 {
        LockProj::dir_u8(d1) | (LockProj::dir_u8(d2) << 2) | (LockProj::dir_u8(d3) << 4) |
            (LockProj::dir_u8(d4) << 6)
    }

    fn dir_u8(dir: Direction) -> u8 {
        use Direction::*;
        match dir {
            Clockwise => 0,
            Counter => 1,
            Neutral => 2
        }
    }
}

impl Proj for LockProj {
    fn project(s: &State) -> Self {
        let mut res = LockProj{packed_faces: [0; 6]};
        for face_idx in 0..6 {
            res.packed_faces[face_idx] = LockProj::dirs_u8(
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
#[derive(Clone, Eq, PartialEq)]
pub struct CornerProj {
    lock: LockProj,
    packed_corners: [u8; 8]
}

impl CornerProj {
    fn faces_u8(f1: Face, f2: Face) -> u8 {
        CornerProj::face_u8(f1) | (CornerProj::face_u8(f2) << 4)
    }

    fn face_u8(face: Face) -> u8 {
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
}

impl Proj for CornerProj {
    fn project(s: &State) -> Self {
        let mut corners = [0; 8];
        // Corners are encoded by storing two of their three stickers.
        for face_idx in 0..4 {
            corners[face_idx * 2] = CornerProj::faces_u8(
                s.0[face_idx * 8 + 0].face,
                s.0[face_idx * 8 + 2].face
            );
            corners[face_idx * 2 + 1] = CornerProj::faces_u8(
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

/// A projection that tracks the face axis of all the stickers with arrows.
#[derive(Clone, Eq, PartialEq)]
pub struct ArrowAxisProj {
    lock: LockProj,
    packed_axes: [u8; 6]
}

impl ArrowAxisProj {
    fn face_u8(face: &[Sticker]) -> u8 {
        ArrowAxisProj::sticker_u8(face[1]) |
            (ArrowAxisProj::sticker_u8(face[3]) << 2) |
            (ArrowAxisProj::sticker_u8(face[4]) << 4) |
            (ArrowAxisProj::sticker_u8(face[6]) << 6)
    }

    fn sticker_u8(sticker: Sticker) -> u8 {
        use Face::*;
        if sticker.direction == Direction::Neutral {
            0
        } else {
            match sticker.face {
                U | D => 0,
                F | B => 1,
                R | L => 2
            }
        }
    }
}

impl Proj for ArrowAxisProj {
    fn project(s: &State) -> Self {
        let mut axes = [0; 6];
        // Corners are encoded by storing two of their three stickers.
        for face_idx in 0..6 {
            let face = &s.0[(face_idx * 8)..((face_idx + 1) * 8)];
            axes[face_idx] = ArrowAxisProj::face_u8(face)
        }
        ArrowAxisProj{
            lock: Proj::project(s),
            packed_axes: axes
        }
    }
}

impl Hash for ArrowAxisProj {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lock.hash(state);
        state.write(&self.packed_axes);
    }
}

impl Hash for CornerProj {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lock.hash(state);
        state.write(&self.packed_corners);
    }
}
