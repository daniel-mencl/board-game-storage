mod bear;
mod elk;
mod fox;
mod hawk;
mod salmon;
mod score;
mod scoring_cards;

use super::{CascadiaAnimal, CascadiaBoard, CascadiaHabitat, CascadiaState, CascadiaTile};
use bear::CascadiaBearScoringCard;
use elk::CascadiaElkScoringCard;
use fox::CascadiaFoxScoringCard;
use hawk::CascadiaHawkScoringCard;
use salmon::CascadiaSalmonScoringCard;
use scoring_cards::CascadiaScoringCard;

pub use score::{CascadiaScoreBreakdown, map_count_to_points};
pub use scoring_cards::{CascadiaAnimalScore, CascadiaScoringCards};
