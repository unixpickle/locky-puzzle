use super::state::{Face, State, Sticker};

/// A description of a single move on the cube, in the face-turn metric.
pub struct Move {
    pub face: Face,
    pub turns: Turns
}

impl Move {
    /// Apply the move to a state and get the resulting state.
    ///
    /// Does not check if the move is valid, i.e. if the face is locked.
    pub fn apply(&self, state: &State) -> State {
        let mut result = state.clone();
        self.turns.apply_face(result.face_mut(self.face));
        self.turns.apply_ring(&mut result, self.face);
        result
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
            stickers[7] = stickers[3];
            stickers[3] = c0;

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

/// Get the 9 stickers that are adjacent to a face.
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
        F => [[u + 5, u + 6, u + 7], [r, r + 3, r + 5], [d, d + 1, d + 2], [l + 2, l + 4, l + 7]],
        B => [[u + 2, u + 1, u], [l, l + 3, l + 5], [d + 5, d + 6, d + 7], [r + 2, r + 4, r + 7]],
        R => [[u + 7, u + 4, u + 2], [b, b + 3, b + 5], [d + 7, d + 4, d + 2],
            [f + 7, f + 4, f + 2]],
        L => [[u + 0, u + 3, u + 5], [f + 0, f + 3, f + 5], [d + 0, d + 3, d + 5],
            [b + 7, b + 4, b + 2]]
    }
}
