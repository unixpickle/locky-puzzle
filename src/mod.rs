//! An API for the "Locky Puzzle".

extern crate rand;

mod state;

mod heuristic;
mod move_gen;
mod moves;
mod proj;
mod scramble;
mod solve;
mod thread;

pub use heuristic::{Heuristic, MaxHeuristic, NopHeuristic, ProjHeuristic};
pub use move_gen::{MoveGen};
pub use moves::{ALL_MOVES, Algo, Move, ParseMoveError, Turns};
pub use proj::{ArrowAxisProj, CornerProj, FBCOProj, Proj, LockProj, RLCOProj, UDCOProj};
pub use scramble::scramble;
pub use solve::{solve, solve_serial};
pub use state::{Face, Direction, State, Sticker};
