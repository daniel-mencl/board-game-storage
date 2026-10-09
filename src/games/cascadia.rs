mod scoring;
mod state;
mod validation;

use scoring::CascadiaScoreBreakdown;
use state::{CascadiaAnimal, CascadiaBoard, CascadiaHabitat, CascadiaState, CascadiaTile};

use crate::core::Game;

pub struct Cascadia;
impl Game for Cascadia {
    type ScoreBreakdown = CascadiaScoreBreakdown;
    type State = CascadiaState;

    fn id(&self) -> &'static str {
        "cascadia"
    }

    fn name(&self) -> &'static str {
        "Cascadia"
    }
}
