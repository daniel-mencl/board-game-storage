use crate::boards::HexBoard;

use super::{
    CascadiaAnimal, CascadiaBearScoringCard, CascadiaBoard, CascadiaElkScoringCard,
    CascadiaFoxScoringCard, CascadiaHawkScoringCard, CascadiaSalmonScoringCard,
};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::bear::*;
use super::elk::*;
use super::fox::*;
use super::hawk::*;
use super::salmon::*;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct CascadiaScoringCards {
    bear: CascadiaBearScoringCard,
    elk: CascadiaElkScoringCard,
    salmon: CascadiaSalmonScoringCard,
    hawk: CascadiaHawkScoringCard,
    fox: CascadiaFoxScoringCard,
}

#[derive(Clone, Serialize, Deserialize, Copy)]
pub struct CascadiaAnimalScore {
    bears: u16,
    elks: u16,
    salmon: u16,
    hawks: u16,
    foxes: u16,
    total: u16,
}
impl CascadiaAnimalScore {
    fn new(bears: u16, elks: u16, salmon: u16, hawks: u16, foxes: u16) -> CascadiaAnimalScore {
        let total = bears + elks + salmon + hawks + foxes;
        CascadiaAnimalScore {
            bears,
            elks,
            salmon,
            hawks,
            foxes,
            total,
        }
    }
}

impl CascadiaScoringCards {
    pub fn score(&self, board: &CascadiaBoard) -> CascadiaAnimalScore {
        let animals = board.animals();
        let bears = self.bear.score(board, &animals);
        let elks = self.elk.score(board, &animals);
        let salmon = self.salmon.score(board, &animals);
        let hawks = self.hawk.score(board, &animals);
        let foxes = self.fox.score(board, &animals);
        CascadiaAnimalScore::new(bears, elks, salmon, hawks, foxes)
    }
}

#[enum_dispatch]
pub trait CascadiaScoringCard {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16;
}
