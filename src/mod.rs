//! An API for the "Locky Puzzle".

mod state;

mod heuristic;
mod move_gen;
mod moves;
mod proj;
mod solve;
mod thread;

pub use heuristic::{Heuristic, MaxHeuristic, NopHeuristic, ProjHeuristic};
pub use move_gen::{MoveGen};
pub use moves::{ALL_MOVES, Algo, Move, ParseMoveError, Turns};
pub use proj::{CornerProj, Proj, LockProj};
pub use solve::{solve, solve_serial};
pub use state::{Face, Direction, State, Sticker};
