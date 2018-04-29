//! Projections onto subspaces of the puzzle state.

use std::hash::{Hash, Hasher};

use super::state::{Direction, Face, State, Sticker};

/// The UD/FB/RL stickers for each corner on the cube.
const CORNERS: [(usize, usize, usize); 8] = [
    (0, 26, 40), (2, 24, 34), (5, 16, 42), (7, 18, 32),
    (13, 31, 45), (15, 29, 39), (8, 21, 47), (10, 23, 37)
];

/// A projection of a state onto a subspace of all possible states.
///
/// Projections must satisfy some properties:
/// * The same state always has the same projection.
/// * If you know a projection, you can apply moves and get a new projection.
/// * A projection must know which faces are locked.
pub trait Proj: Clone + Eq + Hash + Send + Sync {
    /// Project the state onto the subspace.
    fn project(s: &State) -> Self {
        Self::project_with_lock(s, LockProj::project(s))
    }

    /// Project the state onto the subspace, given a pre-computed LockProj.
    fn project_with_lock(s: &State, l: LockProj) -> Self;
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

    fn project_with_lock(_: &State, l: LockProj) -> Self {
        l
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
    fn project_with_lock(s: &State, l: LockProj) -> Self {
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
            lock: l,
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
    fn project_with_lock(s: &State, l: LockProj) -> Self {
        let mut axes = [0; 6];
        // Corners are encoded by storing two of their three stickers.
        for face_idx in 0..6 {
            let face = &s.0[(face_idx * 8)..((face_idx + 1) * 8)];
            axes[face_idx] = ArrowAxisProj::face_u8(face)
        }
        ArrowAxisProj{
            lock: l,
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

macro_rules! make_co {
    ( $name:ident, $face1:tt, $face2:tt ) => {
        /// A projection that tracks the corner orientation with respect to the
        /// $face1 and $face2 faces.
        #[derive(Clone, Eq, Hash, PartialEq)]
        pub struct $name {
            lock: LockProj,
            packed_co: u16
        }

        impl Proj for $name {
            fn project_with_lock(s: &State, l: LockProj) -> Self {
                use Face::*;
                let mut orientations = 0u16;
                for &(ud, fb, _) in &CORNERS {
                    orientations <<= 2;
                    let ud_face = s.0[ud].face;
                    let fb_face = s.0[fb].face;
                    orientations |= if ud_face == $face1 || ud_face == $face2 {
                        0
                    } else if fb_face == $face1 || fb_face == $face2 {
                        1
                    } else {
                        2
                    };
                }
                $name{
                    lock: l,
                    packed_co: orientations
                }
            }
        }
    }
}

make_co!(CoUdProj, U, D);
make_co!(CoFbProj, F, B);
make_co!(CoRlProj, R, L);

macro_rules! make_corner_axis {
    ( $name:tt, $face1:tt, $face2:tt ) => {
        /// A projection that tracks whether each corner originates from the
        /// $face1 or $face2 face.
        #[derive(Clone, Eq, Hash, PartialEq)]
        pub struct $name {
            lock: LockProj,
            packed_faces: u8
        }

        impl Proj for $name {
            fn project_with_lock(s: &State, l: LockProj) -> Self {
                use Face::*;
                let mut faces = 0u8;
                for &(ud, fb, rl) in &CORNERS {
                    faces <<= 1;
                    let ud_face = s.0[ud].face;
                    let fb_face = s.0[fb].face;
                    let rl_face = s.0[rl].face;
                    faces |= if $face1 == ud_face || $face1 == fb_face || $face1 == rl_face {
                        0
                    } else {
                        1
                    };
                }
                $name{
                    lock: l,
                    packed_faces: faces
                }
            }
        }
    }
}

make_corner_axis!(CornerUdProj, U, D);
make_corner_axis!(CornerFbProj, F, B);
make_corner_axis!(CornerRlProj, R, L);
