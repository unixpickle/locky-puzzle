//! A tool for solving the locky puzzle.

extern crate clap;
extern crate locky_puzzle;

mod arguments;
mod heuristic;
mod input;

use std::process::exit;

use locky_puzzle::solve;

use arguments::{Args, parse_args};
use heuristic::make_heuristic;
use input::read_state;

fn main() {
    match parse_args() {
        Ok(args) => {
            if let Err(e) = main_with_args(args) {
                eprintln!("{}", e);
                exit(1);
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

fn main_with_args(args: Args) -> Result<(), String> {
    let heuristic_future = make_heuristic(&args.heuristic);
    let state = read_state(&args)?;
    println!("Waiting for heuristic...");
    let heuristic = heuristic_future.recv().unwrap();
    for depth in 0..255 {
        println!("Trying depth {}...", depth);
        if let Some(solution) = solve(&state, &heuristic, depth) {
            println!("Found solution: {}", solution);
            return Ok(());
        }
    }
    Ok(())
}
