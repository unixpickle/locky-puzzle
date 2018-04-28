//! An API for the "Locky Puzzle".

mod state;

mod heuristic;
mod move_gen;
mod moves;
mod proj;

pub use heuristic::{Heuristic, MaxHeuristic, ProjHeuristic};
pub use move_gen::{MoveGen};
pub use moves::{ALL_MOVES, Move, Turns};
pub use proj::{CornerProj, Proj, LockProj};
pub use state::{Face, Direction, State};
