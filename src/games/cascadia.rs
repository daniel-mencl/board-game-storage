mod scoring;
mod state;
mod ui;

use scoring::CascadiaScoreBreakdown;
use state::{CascadiaAnimal, CascadiaBoard, CascadiaHabitat, CascadiaState, CascadiaTile};

use crate::core::GameEngine;

#[derive(Default, Clone, Copy)]
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
}
