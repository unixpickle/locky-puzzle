//! Moves that can be applied to states.

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use super::state::{Face, State, Sticker};

pub const ALL_MOVES: [Move; 18] = [
    Move{face: Face::U, turns: Turns::Clockwise},
    Move{face: Face::U, turns: Turns::Double},
    Move{face: Face::U, turns: Turns::Counter},
    Move{face: Face::D, turns: Turns::Clockwise},
    Move{face: Face::D, turns: Turns::Double},
    Move{face: Face::D, turns: Turns::Counter},
    Move{face: Face::F, turns: Turns::Clockwise},
    Move{face: Face::F, turns: Turns::Double},
    Move{face: Face::F, turns: Turns::Counter},
    Move{face: Face::B, turns: Turns::Clockwise},
    Move{face: Face::B, turns: Turns::Double},
    Move{face: Face::B, turns: Turns::Counter},
    Move{face: Face::R, turns: Turns::Clockwise},
    Move{face: Face::R, turns: Turns::Double},
    Move{face: Face::R, turns: Turns::Counter},
    Move{face: Face::L, turns: Turns::Clockwise},
    Move{face: Face::L, turns: Turns::Double},
    Move{face: Face::L, turns: Turns::Counter}
];

/// A description of a single move on the cube, in the face-turn metric.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Move {
    pub face: Face,
    pub turns: Turns
}

impl Move {
    /// Apply the move to a state.
    ///
    /// Does not check if the move is valid, i.e. if the face is locked.
    pub fn apply(&self, state: &mut State) {
        self.turns.apply_face(state.face_mut(self.face));
        self.turns.apply_ring(state, self.face);
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use Turns::*;
        self.face.fmt(f)?;
        write!(f, "{}", match self.turns {
            Clockwise => "",
            Double => "2",
            Counter => "'"
        })
    }
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Move, ParseMoveError> {
        ALL_MOVES.iter().find(|m| format!("{}", m) == s)
            .map(|m| *m)
            .ok_or(ParseMoveError::new(s.to_owned()))
    }
}

/// A sequence of moves.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Algo(pub Vec<Move>);

impl Algo {
    pub fn apply(&self, s: &mut State) {
        for m in &self.0 {
            m.apply(s);
        }
    }
}

impl Display for Algo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for (i, m) in (&self.0).iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", m)?;
        }
        Ok(())
    }
}

impl FromStr for Algo {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Algo, ParseMoveError> {
        let mut res = Vec::new();
        for token in s.split_whitespace() {
            res.push(token.parse()?);
        }
        Ok(Algo(res))
    }
}

/// An error from parsing a move.
#[derive(Clone, Debug)]
pub struct ParseMoveError {
    message: String,
    move_str: String
}

impl ParseMoveError {
    fn new(move_str: String) -> ParseMoveError {
        ParseMoveError{
            message: format!("invalid move: {}", move_str),
            move_str: move_str
        }
    }

    /// Get the move that failed to parse.
    pub fn move_str(&self) -> &str {
        &self.move_str
    }
}

impl Display for ParseMoveError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseMoveError {
    fn description(&self) -> &str {
        &self.message
    }
}

/// The number of times a face is turned (once, twice, or thrice).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Turns {
    Clockwise,
    Double,
    Counter
}

impl Turns {
    /// Apply the turn to the stickers of a face.
    fn apply_face(&self, stickers: &mut [Sticker]) {
        // Corner permutation.
        self.permute_indirect(stickers, &[0, 2, 7, 5]);

        // Edge permutation.
        self.permute_indirect(stickers, &[1, 4, 6, 3]);
    }

    /// Apply the turn to the ring around a face.
    fn apply_ring(&self, state: &mut State, face: Face) {
        let indices = face_ring(face);
        let mut cur_stickers = [[Sticker::default(); 3]; 4];
        for i in 0..4 {
            for j in 0..3 {
                cur_stickers[i][j] = state.0[indices[i][j]];
            }
        }
        self.permute(&mut cur_stickers);
        for i in 0..4 {
            for j in 0..3 {
                state.0[indices[i][j]] = cur_stickers[i][j]
            }
        }
    }

    /// Permute the list of items, assuming they are listed clockwise.
    fn permute<T>(&self, list: &mut [T]) {
        use Turns::*;
        match self {
            &Clockwise => {
                list.swap(0, 3);
                list.swap(1, 3);
                list.swap(2, 3);
            },
            &Double => {
                list.swap(0, 2);
                list.swap(1, 3);
            },
            &Counter => {
                list.swap(0, 3);
                list.swap(0, 2);
                list.swap(0, 1);
            }
        }
    }

    // Like permute(), but uses indices into a larger list.
    fn permute_indirect<T>(&self, list: &mut [T], indices: &[usize]) {
        use Turns::*;
        match self {
            &Clockwise => {
                list.swap(indices[0], indices[3]);
                list.swap(indices[1], indices[3]);
                list.swap(indices[2], indices[3]);
            },
            &Double => {
                list.swap(indices[0], indices[2]);
                list.swap(indices[1], indices[3]);
            },
            &Counter => {
                list.swap(indices[0], indices[3]);
                list.swap(indices[0], indices[2]);
                list.swap(indices[0], indices[1]);
            }
        }
    }
}

/// Get the stickers that are adjacent to a face.
///
/// The stickers are grouped by 3's from the same adjacent face.
/// The indices go in clockwise order from the top left back to
/// the top left left.
fn face_ring(face: Face) -> [[usize; 3]; 4] {
    use Face::*;
    let u = 0;
    let d = 8;
    let f = 16;
    let b = 24;
    let r = 32;
    let l = 40;
    match face {
        U => [[b + 2, b + 1, b], [r + 2, r + 1, r], [f + 2, f + 1, f], [l + 2, l + 1, l]],
        D => [[f + 5, f + 6, f + 7], [r + 5, r + 6, r + 7], [b + 5, b + 6, b + 7],
            [l + 5, l + 6, l + 7]],
        F => [[u + 5, u + 6, u + 7], [r, r + 3, r + 5], [d + 2, d + 1, d], [l + 7, l + 4, l + 2]],
        B => [[u + 2, u + 1, u], [l, l + 3, l + 5], [d + 5, d + 6, d + 7], [r + 7, r + 4, r + 2]],
        R => [[u + 7, u + 4, u + 2], [b, b + 3, b + 5], [d + 7, d + 4, d + 2],
            [f + 7, f + 4, f + 2]],
        L => [[u, u + 3, u + 5], [f, f + 3, f + 5], [d, d + 3, d + 5], [b + 7, b + 4, b + 2]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use state::Direction;

    use Face::*;
    use Turns::*;

    /// Test algorithm parsing.
    #[test]
    fn parse_algo() {
        let actual: Algo = "R' U D'   F2 \t L' B2".parse().unwrap();
        let expected = vec![
            Move{face: Face::R, turns: Turns::Counter},
            Move{face: Face::U, turns: Turns::Clockwise},
            Move{face: Face::D, turns: Turns::Counter},
            Move{face: Face::F, turns: Turns::Double},
            Move{face: Face::L, turns: Turns::Counter},
            Move{face: Face::B, turns: Turns::Double}
        ];
        assert_eq!(actual.0, expected);

        assert!(Algo::from_str("R3 U").is_err());
        assert!(Algo::from_str("RU").is_err());
    }

    /// Test algorithm stringification.
    #[test]
    fn stringify_algo() {
        let algo = Algo(vec![
            Move{face: Face::R, turns: Turns::Counter},
            Move{face: Face::U, turns: Turns::Clockwise},
            Move{face: Face::D, turns: Turns::Counter},
            Move{face: Face::F, turns: Turns::Double},
            Move{face: Face::L, turns: Turns::Counter},
            Move{face: Face::B, turns: Turns::Double}
        ]);
        assert_eq!(format!("{}", algo), "R' U D' F2 L' B2");
    }

    /// Test U moves.
    #[test]
    fn u_move() {
        let scramble = [Move{face: U, turns: Counter}];
        let stickers = [
            U, U, U, U, U, U, U, U,
            D, D, D, D, D, D, D, D,
            L, L, L, F, F, F, F, F,
            R, R, R, B, B, B, B, B,
            F, F, F, R, R, R, R, R,
            B, B, B, L, L, L, L, L
        ];
        test_scramble(&scramble, &stickers);
    }

    /// Test R moves.
    #[test]
    fn r_move() {
        let scramble = [Move{face: R, turns: Double}];
        let stickers = [
            U, U, D, U, D, U, U, D,
            D, D, U, D, U, D, D, U,
            F, F, B, F, B, F, F, B,
            F, B, B, F, B, F, B, B,
            R, R, R, R, R, R, R, R,
            L, L, L, L, L, L, L, L
        ];
        test_scramble(&scramble, &stickers);
    }

    /// Test B moves.
    #[test]
    fn b_move() {
        let scramble = [Move{face: B, turns: Counter}];
        let stickers = [
            L, L, L, U, U, U, U, U,
            D, D, D, D, D, R, R, R,
            F, F, F, F, F, F, F, F,
            B, B, B, B, B, B, B, B,
            R, R, U, R, U, R, R, U,
            D, L, L, D, L, D, L, L
        ];
        test_scramble(&scramble, &stickers);
    }

    /// Test the scramble "U' R2".
    #[test]
    fn short_scramble_u3_r2() {
        let scramble = [Move{face: U, turns: Counter}, Move{face: R, turns: Double}];
        let stickers = [
            U, U, D, U, D, U, U, D,
            D, D, U, D, U, D, D, U,
            L, L, B, F, B, F, F, R,
            F, R, R, F, B, L, B, B,
            R, R, R, R, R, F, F, F,
            B, B, B, L, L, L, L, L
        ];
        test_scramble(&scramble, &stickers);
    }

    /// Test the scramble "D B2".
    #[test]
    fn short_scramble_d_b2() {
        let scramble = [Move{face: D, turns: Clockwise}, Move{face: B, turns: Double}];
        let stickers = [
            D, D, D, U, U, U, U, U,
            D, D, D, D, D, U, U, U,
            F, F, F, F, F, L, L, L,
            R, R, R, B, B, B, B, B,
            R, R, B, R, L, F, F, L,
            F, L, L, R, L, R, B, B
        ];
        test_scramble(&scramble, &stickers);
    }

    /// Test the scramble "F2 L'".
    #[test]
    fn short_scramble_f2_l3() {
        let scramble = [Move{face: F, turns: Double}, Move{face: L, turns: Counter}];
        let stickers = [
            F, U, U, F, U, F, D, D,
            B, U, U, B, D, B, D, D,
            U, F, F, D, F, D, F, F,
            B, B, D, B, U, B, B, U,
            L, R, R, L, R, L, R, R,
            R, R, R, L, L, L, L, L
        ];
        test_scramble(&scramble, &stickers);
    }

    /// Test a long scramble for a traditional cube.
    /// This may make invalid moves, but still tests edge directions.
    #[test]
    fn long_scramble() {
        // U F' D' B2 F R2 F2 L' D2 B U R2 D R2 B2 D L2 F2 L2 U B2
        let moves = [
            Move{face: U, turns: Clockwise}, Move{face: F, turns: Counter},
            Move{face: D, turns: Counter}, Move{face: B, turns: Double},
            Move{face: F, turns: Clockwise}, Move{face: R, turns: Double},
            Move{face: F, turns: Double}, Move{face: L, turns: Counter},
            Move{face: D, turns: Double}, Move{face: B, turns: Clockwise},
            Move{face: U, turns: Clockwise}, Move{face: R, turns: Double},
            Move{face: D, turns: Clockwise}, Move{face: R, turns: Double},
            Move{face: B, turns: Double}, Move{face: D, turns: Clockwise},
            Move{face: L, turns: Double}, Move{face: F, turns: Double},
            Move{face: L, turns: Double}, Move{face: U, turns: Clockwise},
            Move{face: B, turns: Double}
        ];
        let stickers = [
            L, B, R, F, R, L, U, B,
            U, D, L, B, U, R, D, B,
            F, L, L, L, R, R, B, U,
            F, R, B, R, L, R, F, D,
            U, F, U, U, D, F, B, D,
            D, U, D, D, F, F, L, B
        ];
        test_scramble(&moves, &stickers);
        let counter_indices = [8 * 5 + 1, 8 + 4, 8 * 3 + 4, 8 * 2 + 1, 1, 8 + 3];
        let clockwise_indices = [8 + 1, 8 + 6, 8 * 2 + 4, 8 * 3 + 3, 8 * 5 + 4, 8 * 4 + 1];
        let state = state_from_moves(&moves);
        for index in &counter_indices {
            assert_eq!(state.0[*index].direction, Direction::Counter);
        }
        for index in &clockwise_indices {
            assert_eq!(state.0[*index].direction, Direction::Clockwise);
        }
        for (i, sticker) in state.0.iter().enumerate() {
            if !counter_indices.contains(&i) && !clockwise_indices.contains(&i) {
                assert_eq!(sticker.direction, Direction::Neutral);
            }
        }
    }

    /// Test that the moves give rise to the stickers.
    fn test_scramble(moves: &[Move], stickers: &[Face]) {
        let state = state_from_moves(moves);
        for (i, (actual, expected)) in state.0.iter().zip(stickers).enumerate() {
            assert!(&actual.face == expected, "bad sticker at {} (expected {}, got {})",
                i, expected, actual.face);
        }
    }

    fn state_from_moves(moves: &[Move]) -> State {
        let mut state = State::default();
        for m in moves {
            m.apply(&mut state);
        }
        state
    }
}
