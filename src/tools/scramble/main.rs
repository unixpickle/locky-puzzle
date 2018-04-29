//! A tool for scrambling the locky puzzle.

extern crate clap;
extern crate locky_puzzle;

use clap::{App, Arg};
use locky_puzzle::scramble;

fn main() {
    let matches = App::new("locky-scramble")
        .arg(Arg::with_name("moves")
            .long("moves")
            .takes_value(true)
            .help("Solve a specific a sequence of moves"))
        .get_matches();
    let moves = matches.value_of("moves").unwrap_or("25").parse().unwrap();
    println!("{}", scramble(moves))
}
