use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::hearts::*;
use super::jacks::*;
use super::orbs::*;
use super::trees::*;

use super::{
    WispwoodBoard, WispwoodHeartScoringCard, WispwoodJackScoringCard, WispwoodOrbScoringCard,
    WispwoodTreeScoringCard, WispwoodWitchScoringCard,
};

#[enum_dispatch]
pub trait WispwoodScoringCard {
    fn score(&self, board: &WispwoodBoard) -> u16;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodBoardScore {
    jacks: u16,
    witches: u16,
    orbs: u16,
    hearts: u16,
    trees: u16,
    bonus: u16,
    pub total: u16,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct WispwoodScoringCards {
    jack: WispwoodJackScoringCard,
    witch: WispwoodWitchScoringCard,
    orb: WispwoodOrbScoringCard,
    heart: WispwoodHeartScoringCard,
    tree: WispwoodTreeScoringCard,
}

impl WispwoodScoringCards {
    fn round_bonus(board: &WispwoodBoard) -> u16 {
        let empty = board.count_all_tiles(|tile| tile.is_empty());
        if empty == 0 {
            0
        } else {
            board.round() as u16 * 2 + 2
        }
    }

    pub fn score(&self, board: &WispwoodBoard) -> WispwoodBoardScore {
        let jacks = self.jack.score(board);
        let witches = self.witch.score(board);
        let orbs = self.orb.score(board);
        let hearts = self.heart.score(board);
        let trees = self.tree.score(board);
        let bonus = Self::round_bonus(board);
        let total = jacks + witches + orbs + hearts + trees + bonus;
        WispwoodBoardScore {
            jacks,
            witches,
            orbs,
            hearts,
            trees,
            bonus,
            total,
        }
    }
}
