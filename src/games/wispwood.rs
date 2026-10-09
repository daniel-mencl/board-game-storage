mod score;
mod state;
mod validate;

use crate::core::Game;
use score::WispwoodScoreBreakdown;
use state::WispwoodState;

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
