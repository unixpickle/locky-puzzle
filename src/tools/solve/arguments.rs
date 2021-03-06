//! Command-line arguments.

use clap::{App, Arg};

/// The parsed command-line arguments.
pub struct Args {
    pub heuristic: HeuristicArgs,
    pub scramble: Option<String>,
    pub multi_step: bool
}

/// Arguments that determine the search heuristic.
#[derive(Clone)]
pub struct HeuristicArgs {
    pub corner_depth: u8,
    pub arrow_axis_depth: u8,
    pub co_depth: u8,
    pub corner_axis_depth: u8,
    pub lock_depth: u8
}

/// Parse the command-line arguments.
pub fn parse_args() -> Result<Args, String> {
    let matches = App::new("locky-solve")
        .arg(Arg::with_name("corner-depth")
            .long("corner-depth")
            .value_name("NUM")
            .help("Set the depth of the corner heuristic (default: 0)")
            .takes_value(true))
        .arg(Arg::with_name("arrow-axis-depth")
            .long("arrow-axis-depth")
            .value_name("NUM")
            .help("Set the depth of the arrow axis heuristic (default: 0)")
            .takes_value(true))
        .arg(Arg::with_name("co-depth")
            .long("co-depth")
            .value_name("NUM")
            .help("Set the depth of the corner orientation heuristic (default: 0)")
            .takes_value(true))
        .arg(Arg::with_name("corner-axis-depth")
            .long("corner-axis-depth")
            .value_name("NUM")
            .help("Set the depth of the corner axis heuristic (default: 0)")
            .takes_value(true))
        .arg(Arg::with_name("lock-depth")
            .long("lock-depth")
            .value_name("NUM")
            .help("Set the depth of the lock heuristic (default: 0)")
            .takes_value(true))
        .arg(Arg::with_name("scramble")
            .long("scramble")
            .value_name("ALGO")
            .help("Set a specific algorithm to solve")
            .takes_value(true))
        .arg(Arg::with_name("multi-step")
            .long("multi-step")
            .help("Solve the puzzle in multiple steps"))
        .get_matches();

    macro_rules! parse_arg {
        ( $name:expr, $default:expr ) => {
            matches.value_of($name).unwrap_or($default).parse()
                .map_err(|e| format!("bad {} argument: {}", $name, e))?
        }
    }

    Ok(Args{
        heuristic: HeuristicArgs{
            corner_depth: parse_arg!("corner-depth", "0"),
            arrow_axis_depth: parse_arg!("arrow-axis-depth", "0"),
            co_depth: parse_arg!("co-depth", "0"),
            corner_axis_depth: parse_arg!("corner-axis-depth", "0"),
            lock_depth: parse_arg!("lock-depth", "0")
        },
        scramble: matches.value_of("scramble").map(From::from),
        multi_step: matches.is_present("multi-step")
    })
}
