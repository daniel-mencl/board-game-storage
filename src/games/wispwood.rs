mod score;
mod state;
mod validate;
mod ui;

use crate::core::GameEngine;
use score::WispwoodScoreBreakdown;
use state::WispwoodState;

#[derive(Default)]
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

    fn default_state(&self, player_count: usize) -> Self::State {
        Self::State::new(player_count)
    }

    fn default_score(&self, player_count: usize) -> Self::ScoreBreakdown {
        Self::ScoreBreakdown::new(player_count)
    }
}
