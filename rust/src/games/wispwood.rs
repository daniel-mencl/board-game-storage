mod model;
mod score;
mod validate;

use crate::core::Game;
use model::{WispwoodScoreBreakdown, WispwoodState};

pub struct Wispwood;
impl Game for Wispwood {
    type State = WispwoodState;
    type ScoreBreakdown = WispwoodScoreBreakdown;

    fn id(&self) -> &'static str {
        "wispwood"
    }

    fn name(&self) -> &'static str {
        "Wispwood"
    }
}
