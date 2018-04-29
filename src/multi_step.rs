//! Solving the puzzle in a multi-stage process.

use std::fmt;
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::sync::mpsc::channel;
use std::thread::spawn;

use super::heuristic::{Heuristic, MaxHeuristic, ProjHeuristic};
use super::moves::Algo;
use super::proj::{ArrowAxisProj, CoFbProj, CoRlProj, CoUdProj, CornerProj, LockProj, PairProj,
    Proj};
use super::solve::{proj_solve, solve};
use super::state::State;

/// A multi-step solver.
pub struct MultiStep {
    pub arrow: ProjHeuristic<ArrowAxisProj>,
    pub co_fb: ProjHeuristic<CoFbProj>,
    pub co_rl: ProjHeuristic<CoRlProj>,
    pub co_ud: ProjHeuristic<CoUdProj>,
    pub corner: ProjHeuristic<CornerProj>,
    pub lock: ProjHeuristic<LockProj>,
}

impl MultiStep {
    /// Generate a MultiStep solver with reasonable default settings.
    pub fn generate_default() -> MultiStep {
        macro_rules! generate_table {
            ( $proj:ident, $depth:expr ) => {
                {
                    let (tx, rx) = channel();
                    spawn(move || {
                        tx.send(ProjHeuristic::<$proj>::generate($depth)).unwrap();
                    });
                    rx
                }
            }
        }

        let arrow_rx = generate_table!(ArrowAxisProj, 7);
        let co_fb_rx = generate_table!(CoFbProj, 7);
        let co_rl_rx = generate_table!(CoRlProj, 7);
        let co_ud_rx = generate_table!(CoUdProj, 7);
        let corner_rx = generate_table!(CornerProj, 7);
        let lock_rx = generate_table!(LockProj, 8);

        MultiStep{
            arrow: arrow_rx.recv().unwrap(),
            co_fb: co_fb_rx.recv().unwrap(),
            co_rl: co_rl_rx.recv().unwrap(),
            co_ud: co_ud_rx.recv().unwrap(),
            corner: corner_rx.recv().unwrap(),
            lock: lock_rx.recv().unwrap()
        }
    }

    /// Find a solution for the state.
    ///
    /// Returns both the complete solution, and a decomposition of the
    /// solution into its component steps.
    pub fn solve(&self, s: &State) -> Result<(Algo, [Algo; 5]), MultiStepError> {
        use MultiStepError::*;
        let (a1, s1) = MultiStep::step::<LockProj>(s, &self.lock, 11).ok_or(InvalidEdges)?;
        let (a2, s2) = MultiStep::step::<ArrowAxisProj>(&s1, &self.arrow, 255)
            .ok_or(InvalidEdges)?;

        type Combo1Proj = PairProj<PairProj<ArrowAxisProj, CoFbProj>,
                                   PairProj<CoRlProj, CoUdProj>>;
        let combo1 = MaxHeuristic::<&Heuristic>(vec![&self.arrow, &self.co_fb, &self.co_rl,
            &self.co_ud]);
        let (a3, s3) = MultiStep::step::<Combo1Proj>(&s2, &combo1, 255).ok_or(InvalidCorners)?;

        type Combo2Proj = PairProj<ArrowAxisProj, CornerProj>;
        let combo2 = MaxHeuristic::<&Heuristic>(vec![&self.arrow, &self.corner]);
        let (a4, s4) = MultiStep::step::<Combo2Proj>(&s3, &combo2, 255).ok_or(InvalidState)?;

        for i in 0..255 {
            if let Some(a5) = solve(&s4, &combo2, i) {
                let pieces = [a1, a2, a3, a4, a5];
                let combined = pieces.iter().flat_map(|alg| alg.0.clone()).collect();
                return Ok((Algo(combined), pieces));
            }
        }
        Err(InvalidState)
    }

    fn step<P: Proj>(
        s: &State,
        h: &Heuristic,
        upper_bound: u8
    ) -> Option<(Algo, State)> {
        for i in 0..upper_bound {
            if let Some(solution) = proj_solve::<P, _>(s, h, i) {
                let mut new_state = s.clone();
                solution.apply(&mut new_state);
                return Some((solution, new_state));
            }
        }
        None
    }
}

/// An error describing why a MultiStep solve failed.
#[derive(Debug, Eq, PartialEq)]
pub enum MultiStepError {
    InvalidCorners,
    InvalidEdges,
    InvalidState
}

impl Display for MultiStepError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}

impl Error for MultiStepError {
    fn description(&self) -> &str {
        use MultiStepError::*;
        match self {
            &InvalidCorners => "invalid corners",
            &InvalidEdges => "invalid edges",
            &InvalidState => "invalid cube state"
        }
    }
}
