//! Definition of the puzzle state.

use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

/// The sticker configuration of a puzzle.
///
/// The array consists of 8 stickers per face, with the faces appearing in the
/// order U, D, F, B, R, L.
/// The center sticker is not counted.
/// Stickers on a face are enumerated from the top left to the bottom right,
/// reading to the right and then down.
/// To look at each face in sequence, do the following moves:
/// * `U` - x'
/// * `D` - x2
/// * `F` - x'
/// * `B` - y2
/// * `R` - y'
/// * `L` - y2
#[derive(Clone)]
pub struct State(pub [Sticker; 48]);

impl State {
    /// Create a solved state.
    pub fn solved() -> State {
        use Face::*;
        use Direction::*;

        // Puzzle hyper-parameters.
        let faces = [U, D, F, B, R, L];
        let directions = [Counter, Clockwise, Clockwise, Counter, Clockwise, Counter];
        let arrow_idxs = [[1, 6], [1, 6], [3, 4], [3, 4], [1, 6], [1, 6]];

        let mut stickers = [Sticker::default(); 48];

        let face_infos = faces.iter().zip(&directions).zip(&arrow_idxs).enumerate();
        for (face_idx, ((face, direction), indices)) in face_infos {
            for sub_idx in 0..8 {
                stickers[face_idx * 8 + sub_idx] = Sticker{
                    face: *face,
                    direction: if sub_idx == indices[0] || sub_idx == indices[1] {
                        *direction
                    } else {
                        Neutral
                    }
                }
            }
        }

        State(stickers)
    }

    pub fn is_solved(&self) -> bool {
        use Face::*;
        for face in &[U, D, F, B, R, L] {
            for sticker in self.face(*face) {
                if &sticker.face != face {
                    return false;
                }
            }
        }
        true
    }

    /// Get the 8 stickers for a given face.
    pub fn face(&self, face: Face) -> &[Sticker] {
        use Face::*;
        match face {
            U => &self.0[0..8],
            D => &self.0[8..16],
            F => &self.0[16..24],
            B => &self.0[24..32],
            R => &self.0[32..40],
            L => &self.0[40..48]
        }
    }

    /// Get the 8 stickers for a given face.
    pub fn face_mut(&mut self, face: Face) -> &mut [Sticker] {
        use Face::*;
        match face {
            U => &mut self.0[0..8],
            D => &mut self.0[8..16],
            F => &mut self.0[16..24],
            B => &mut self.0[24..32],
            R => &mut self.0[32..40],
            L => &mut self.0[40..48]
        }
    }

    /// Check if a face is locked (i.e. cannot be turned).
    pub fn is_locked(&self, face: Face) -> bool {
        let mut direction = Direction::Neutral;
        for sticker in self.face(face) {
            if direction == Direction::Neutral {
                direction = sticker.direction;
            } else if sticker.direction != Direction::Neutral {
                if sticker.direction != direction {
                    return true;
                }
            }
        }
        false
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for sticker in self.0.iter() {
            sticker.hash(state);
        }
    }
}

impl Default for State {
    /// Create the solved state.
    fn default() -> State {
        State::solved()
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        let iter = self.0.iter().zip(other.0.iter());
        for (s1, s2) in iter {
            if s1 != s2 {
                return false;
            }
        }
        true
    }

    fn ne(&self, other: &State) -> bool {
        !(self == other)
    }
}

impl Eq for State {
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use Face::*;
        use Direction::*;
        write!(f, "[")?;
        for (i, color) in [U, D, F, B, R, L].iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            for (j, sticker) in self.face(*color).iter().enumerate() {
                if j > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}{}", sticker.face, match sticker.direction {
                    Clockwise => "c",
                    Counter => "c'",
                    Neutral => ""
                })?;
            }
        }
        write!(f, "]")
    }
}

// TODO: implement Debug for State.

/// A sticker on the puzzle.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Sticker {
    pub face: Face,
    pub direction: Direction
}

impl Default for Sticker {
    fn default() -> Sticker {
        Sticker{face: Face::U, direction: Direction::Neutral}
    }
}

/// A restriction on the direction a sticker can be turned.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Clockwise,
    Counter,
    Neutral
}

/// A sticker's face color, which indicates the face it is from.
///
/// Each face is assigned the following color:
/// * `U` - yellow
/// * `D` - red
/// * `F` - blue
/// * `B` - green
/// * `R` - black
/// * `L` - white
///
/// This color scheme is based on a physical version of the puzzle.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Face {
    U,
    D,
    F,
    B,
    R,
    L
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use Face::*;
        write!(f, "{}", match self {
            &U => "U",
            &D => "D",
            &F => "F",
            &B => "B",
            &R => "R",
            &L => "L"
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the Display output for a solved puzzle.
    #[test]
    fn solved_display() {
        let actual = format!("{}", State::default());
        let expected = "[U Uc' U U U U Uc' U, D Dc D D D D Dc D, F F F Fc Fc F F F, ".to_owned() +
            "B B B Bc' Bc' B B B, R Rc R R R R Rc R, L Lc' L L L L Lc' L]";
        assert_eq!(actual, expected);
    }
}
