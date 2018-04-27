use super::state::{Face, State, Sticker};

/// A description of a single move on the cube, in the face-turn metric.
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

/// The number of times a face is turned (once, twice, or thrice).
pub enum Turns {
    Clockwise,
    Double,
    Counter
}

impl Turns {
    /// Apply the turn to the stickers of a face.
    fn apply_face(&self, stickers: &mut [Sticker]) {
        // TODO: optimize so that double/counter are not slower.
        for _ in 0..self.move_count() {
            // Corner permutation.
            let c0 = stickers[0];
            stickers[0] = stickers[5];
            stickers[5] = stickers[7];
            stickers[7] = stickers[2];
            stickers[2] = c0;

            // Edge permutation.
            let e0 = stickers[1];
            stickers[1] = stickers[3];
            stickers[3] = stickers[6];
            stickers[6] = stickers[4];
            stickers[4] = e0;
        }
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
        // TODO: optimize so that double/counter are not slower.
        for _ in 0..self.move_count() {
            let first = cur_stickers[0];
            cur_stickers[0] = cur_stickers[3];
            cur_stickers[3] = cur_stickers[2];
            cur_stickers[2] = cur_stickers[1];
            cur_stickers[1] = first;
        }
        for i in 0..4 {
            for j in 0..3 {
                state.0[indices[i][j]] = cur_stickers[i][j]
            }
        }
    }

    fn move_count(&self) -> i32 {
        use Turns::*;
        match self {
            &Clockwise => 1,
            &Double => 2,
            &Counter => 3
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

    use Face::*;
    use Turns::*;

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
        // TODO: check orientations.
    }

    /// Test that the moves give rise to the stickers.
    fn test_scramble(moves: &[Move], stickers: &[Face]) {
        let mut state = State::default();
        for m in moves {
            m.apply(&mut state);
        }
        for (i, (actual, expected)) in state.0.iter().zip(stickers).enumerate() {
            assert!(&actual.face == expected, "bad sticker at {} (expected {}, got {})",
                i, expected, actual.face);
        }
    }
}
