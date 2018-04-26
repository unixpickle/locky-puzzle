//! Definition of the puzzle state.

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
    pub fn new() -> State {
        use Color::*;
        use Direction::*;

        // Puzzle hyper-parameters.
        let colors = [U, D, F, B, R, L];
        let directions = [Counter, Clockwise, Clockwise, Counter, Clockwise, Counter];
        let arrow_idxs = [[1, 6], [1, 6], [3, 4], [3, 4], [1, 6], [1, 6]];

        let mut stickers = [Sticker{color: U, direction: Neutral}; 48];

        let face_infos = colors.into_iter().zip(&directions).zip(&arrow_idxs).enumerate();
        for (face_idx, ((color, direction), indices)) in face_infos {
            for sub_idx in 0..8 {
                stickers[face_idx * 8 + sub_idx] = Sticker{
                    color: *color,
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

    /// Get the 8 stickers for a given face.
    pub fn face_stickers(&self, face: Color) -> &[Sticker] {
        use Color::*;
        match face {
            U => &self.0[0..8],
            D => &self.0[8..16],
            F => &self.0[16..24],
            B => &self.0[24..32],
            R => &self.0[32..40],
            L => &self.0[40..48]
        }
    }

    /// Check if a face is locked (i.e. cannot be turned).
    pub fn locked(&self, face: Color) -> bool {
        let mut direction = Direction::Neutral;
        for sticker in self.face_stickers(face) {
            if direction == Direction::Neutral {
                direction = sticker.direction;
            } else if sticker.direction != Direction::Neutral {
                if sticker.direction != direction {
                    return false;
                }
            }
        }
        true
    }
}

// TODO: implement Debug and PartialEq for State.

/// A sticker on the puzzle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sticker {
    pub color: Color,
    pub direction: Direction
}

/// A restriction on the direction a sticker can be turned.
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    U,
    D,
    F,
    B,
    R,
    L
}
