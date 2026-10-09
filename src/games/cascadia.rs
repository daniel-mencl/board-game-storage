mod scoring;
mod state;
mod validation;
mod ui;

use scoring::CascadiaScoreBreakdown;
use state::{CascadiaAnimal, CascadiaBoard, CascadiaHabitat, CascadiaState, CascadiaTile};

use crate::core::GameEngine;

#[derive(Default)]
pub struct Cascadia;
impl GameEngine for Cascadia {
    type ScoreBreakdown = CascadiaScoreBreakdown;
    type State = CascadiaState;

    fn id(&self) -> &'static str {
        "cascadia"
    }

    fn name(&self) -> &'static str {
        "Cascadia"
    }

    fn default_state(&self, player_count: usize) -> Self::State {
        CascadiaState::new(player_count)
    }

    fn default_score(&self, player_count: usize) -> Self::ScoreBreakdown {
        CascadiaScoreBreakdown::new(player_count)
    }
}