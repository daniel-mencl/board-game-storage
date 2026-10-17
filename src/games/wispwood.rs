mod score;
mod state;
mod ui;

use crate::core::GameEngine;
use score::WispwoodScoreBreakdown;
use state::WispwoodState;

#[derive(Default, Clone, Copy)]
pub struct Wispwood;
impl GameEngine for Wispwood {
    type State = WispwoodState;
    type ScoreBreakdown = WispwoodScoreBreakdown;

    fn id(&self) -> &'static str {
        "wispwood"
    }

    fn name(&self) -> &'static str {
        "Wispwood"
    }
}
