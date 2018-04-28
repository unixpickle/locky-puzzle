//! Getting the state of the cube from the user.

use std::io::{Write, stdin, stdout};

use locky_puzzle::{Algo, Direction, State, Sticker};
use arguments::Args;

pub fn read_state(args: &Args) -> Result<State, String> {
    if let &Some(ref scramble) = &args.scramble {
        let algo: Algo = scramble.parse().map_err(|e| format!("parse scramble: {}", e))?;
        Ok(algo.state())
    } else {
        read_stdin()
    }
}

fn read_stdin() -> Result<State, String> {
    println!("Enter each face of the scramble. Read each face from the top left");
    println!("to the bottom right, where you view the B face using y2, and when");
    println!("you look at every other face with the simplest rotation from the");
    println!("situation where U is on top and F is in front.");
    println!("");
    println!("When a sticker has an arrow, put the '^' character after it.");
    println!("For example, the top face of a solved cube is UU^UUUUU^U.");
    println!("");
    println!("Color scheme:");
    println!("  U - yellow");
    println!("  D - red");
    println!("  F - blue");
    println!("  B - green");
    println!("  R - black");
    println!("  L - white");
    let mut state = State::default();
    use locky_puzzle::Face::*;
    for face in &[U, D, F, B, R, L] {
        print!("Enter {} face: ", face);
        stdout().flush().unwrap();
        let row = read_sticker_row()?;
        if &row[4].face != face || row[4].direction != Direction::Neutral {
            return Err("invalid center sticker".to_owned());
        }
        let face_dst = state.face_mut(*face);
        for i in 0..4 {
            face_dst[i] = row[i];
        }
        for i in 4..8 {
            face_dst[i] = row[i + 1];
        }
    }
    validate_state(&state)?;
    Ok(state)
}

fn read_sticker_row() -> Result<[Sticker; 9], String> {
    // TODO: why does path need to be qualified?
    use locky_puzzle::Face::*;
    use locky_puzzle::Direction::*;

    let mut buf = String::new();
    stdin().read_line(&mut buf).map_err(|e| format!("failed to read face: {}", e))?;
    let line = buf.trim();
    if line.len() < 9 {
        return Err("line not long enough".to_owned());
    }

    let mut result = [Sticker{face: U, direction: Neutral}; 9];
    let mut idx = 0;
    for ch in line.chars() {
        if ch == '^' {
            if idx != 2 && idx != 4 && idx != 6 && idx != 8 {
                return Err("invalid placement of '^' (must be after an edge)".to_owned());
            } else {
                result[idx - 1].direction = result[idx - 1].face.standard_direction()
            }
        } else if idx == 9 {
            return Err(format!("excess character: {}", ch));
        } else {
            result[idx].face = match ch {
                'U' => U,
                'D' => D,
                'F' => F,
                'B' => B,
                'R' => R,
                'L' => L,
                _ => return Err(format!("invalid character: {}", ch))
            };
            idx += 1;
        }
    }
    if idx < 9 {
        Err("not enough characters".to_owned())
    } else {
        Ok(result)
    }
}

fn validate_state(_state: &State) -> Result<(), String> {
    // TODO: check that there are exactly 6 clockwise and 6 counter-clockwise
    // directions.
    Ok(())
}
