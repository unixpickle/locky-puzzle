//! Generating move sequences for searches.

use super::moves::{ALL_MOVES, Move};
use super::state::Face;

/// An object representing a certain point in a search tree, and in particular
/// representing the moves that make sense to search in the next step.
#[derive(Clone)]
pub struct MoveGen {
    axis: Axis,
    axis_state: AxisState
}

impl MoveGen {
    pub fn new() -> MoveGen {
        MoveGen{
            axis: Axis::UD,
            axis_state: AxisState::Enabled
        }
    }
}

impl IntoIterator for MoveGen {
    type Item = (MoveGen, Move);
    type IntoIter = MoveGenIter;

    fn into_iter(self) -> MoveGenIter {
        MoveGenIter{state: self, idx: 0}
    }
}

/// An iterator over possible next moves.
pub struct MoveGenIter {
    state: MoveGen,
    idx: usize
}

impl Iterator for MoveGenIter {
    type Item = (MoveGen, Move);

    fn next(&mut self) -> Option<(MoveGen, Move)> {
        loop {
            if self.idx == ALL_MOVES.len() {
                return None;
            }
            let m = ALL_MOVES[self.idx];
            let (axis, primary) = decompose_face(m.face);
            self.idx += 1;
            if axis != self.state.axis || self.state.axis_state == AxisState::Enabled {
                return Some((MoveGen{
                    axis: axis,
                    axis_state: if primary {
                        AxisState::HalfDisabled
                    } else {
                        AxisState::Disabled
                    }
                }, m));
            } else if !primary && self.state.axis_state == AxisState::HalfDisabled {
                return Some((MoveGen{
                    axis: axis,
                    axis_state: AxisState::Disabled
                }, m));
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Axis {
    UD,
    FB,
    RL
}

#[derive(Clone, Eq, PartialEq)]
enum AxisState {
    Enabled,
    HalfDisabled,
    Disabled
}

/// Get the axis for a face, and check if this is the axis's primary face.
fn decompose_face(f: Face) -> (Axis, bool) {
    use Face::*;
    match f {
        U => (Axis::UD, true),
        D => (Axis::UD, false),
        F => (Axis::FB, true),
        B => (Axis::FB, false),
        R => (Axis::RL, true),
        L => (Axis::RL, false)
    }
}
