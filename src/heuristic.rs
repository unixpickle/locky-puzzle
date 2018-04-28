//! Abstractions for search heuristics.

use std::collections::HashMap;

use super::state::State;
use super::proj::Proj;

/// A lower-bound on the number of moves to achieve a certain goal.
pub trait Heuristic {
    fn lower_bound(&self, s: &State) -> u8;
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

/// A heuristic that uses a lookup table of projections.
pub struct ProjHeuristic<T: Proj> {
    pub table: HashMap<T, u8>,
    pub default: u8
}

impl<T: Proj> Heuristic for ProjHeuristic<T> {
    fn lower_bound(&self, s: &State) -> u8 {
        *self.table.get(&Proj::project(s)).unwrap_or(&0)
    }
}
