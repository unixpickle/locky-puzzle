//! Create heuristics as specified by the user.

use std::mem::drop;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread::spawn;

use locky_puzzle::{ArrowAxisProj, CornerProj, Heuristic, MaxHeuristic, Proj, ProjHeuristic};
use arguments::HeuristicArgs;

/// Generate the aggregate heuristic from the arguments.
///
/// The computation is done asynchronously.
pub fn make_heuristic(args: &HeuristicArgs) -> Receiver<MaxHeuristic<Box<Heuristic>>> {
    let (send_individual, recv_individual) = channel();
    if args.corner_depth > 0 {
        make_proj_heuristic::<CornerProj>(args.corner_depth, send_individual.clone());
    }
    if args.arrow_axis_depth > 0 {
        make_proj_heuristic::<ArrowAxisProj>(args.arrow_axis_depth, send_individual.clone());
    }
    drop(send_individual);

    let (send_agg, recv_agg) = channel::<MaxHeuristic<Box<Heuristic>>>();
    spawn(move || {
        let heuristics: Vec<Box<Heuristic>> = recv_individual.into_iter().collect();
        send_agg.send(MaxHeuristic(heuristics)).ok();
    });
    recv_agg
}

fn make_proj_heuristic<P: Proj + 'static>(depth: u8, sender: Sender<Box<Heuristic>>) {
    spawn(move || {
        sender.send(Box::new(ProjHeuristic::<P>::generate(depth))).unwrap();
    });
}
