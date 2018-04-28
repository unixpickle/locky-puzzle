//! Abstractions for search heuristics.

use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry;

use super::move_gen::MoveGen;
use super::proj::Proj;
use super::state::State;

/// A lower-bound on the number of moves to achieve a certain goal.
pub trait Heuristic: Send + Sync {
    fn lower_bound(&self, s: &State) -> u8;
}

impl Heuristic for Box<Heuristic> {
    fn lower_bound(&self, s: &State) -> u8 {
        self.as_ref().lower_bound(s)
    }
}

/// A heuristic that takes a max over other heuristics.
pub struct MaxHeuristic<T: Heuristic>(pub Vec<T>);

impl<T: Heuristic> Heuristic for MaxHeuristic<T> {
    fn lower_bound(&self, s: &State) -> u8 {
        let mut res = 0;
        for heuristic in &self.0 {
            res = heuristic.lower_bound(s).max(res);
        }
        res
    }
}

/// A heuristic that always returns 0.
pub struct NopHeuristic();

impl Heuristic for NopHeuristic {
    fn lower_bound(&self, _: &State) -> u8 {
        0
    }
}

/// A heuristic that uses a lookup table of projections.
pub struct ProjHeuristic<T: Proj> {
    pub table: HashMap<T, u8>,
    pub default: u8
}

impl<T: Proj> ProjHeuristic<T> {
    /// Uses a simple search algorithm to build a heuristic table.
    pub fn generate(depth: u8) -> Self {
        let mut table = HashMap::new();
        let mut states = VecDeque::new();
        states.push_back((MoveGen::new(), State::default()));
        table.insert(Proj::project(&State::default()), 0);
        for i in 0..depth {
            let pop_size = states.len();
            if pop_size == 0 {
                break;
            }
            for _ in 0..pop_size {
                let (moves, state) = states.pop_front().unwrap();
                for (new_moves, m) in moves {
                    if state.is_locked(m.face) {
                        continue;
                    }
                    let mut new_state = state.clone();
                    m.apply(&mut new_state);
                    let proj = Proj::project(&new_state);
                    if let Entry::Vacant(v) = table.entry(proj) {
                        v.insert(i + 1);
                        states.push_back((new_moves.clone(), new_state));
                    }
                }
            }
        }
        ProjHeuristic{
            table: table,
            default: depth + 1
        }
    }
}

impl<T: Proj> Heuristic for ProjHeuristic<T> {
    fn lower_bound(&self, s: &State) -> u8 {
        *self.table.get(&Proj::project(s)).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proj::CornerProj;

    #[test]
    fn generate_heuristic() {
        let heuristic_1: ProjHeuristic<CornerProj> = ProjHeuristic::generate(1);
        assert_eq!(heuristic_1.table.len(), 19);

        let heuristic_2: ProjHeuristic<CornerProj> = ProjHeuristic::generate(2);
        assert_eq!(heuristic_2.table.len(), 190);

        let heuristic_5: ProjHeuristic<CornerProj> = ProjHeuristic::generate(5);
        assert_eq!(heuristic_5.table.len(), 77362);
    }
}
