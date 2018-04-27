//! An API for the "Locky Puzzle".

mod state;
mod moves;

pub use moves::{Move, Turns};
pub use state::{Face, Direction, State};
