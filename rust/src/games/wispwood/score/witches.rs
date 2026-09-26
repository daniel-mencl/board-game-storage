use super::WispwoodBoard;
use super::WispwoodScoringCard;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum WispwoodWitchScoringCard {
    Adjacent(WispwoodWitchAdjacent),
    Diagonal(WispwoodWitchDiagonal),
    Checkers(WispwoodWitchCheckers),
    RowColumn(WispwoodWitchRowColumn),
    Far(WispwoodWitchFar),
}

impl Default for WispwoodWitchScoringCard {
    fn default() -> Self {
        Self::Adjacent(WispwoodWitchAdjacent)
    }
}

impl WispwoodScoringCard for WispwoodWitchScoringCard {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        score_witches(board)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodWitchAdjacent;
#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodWitchDiagonal;
#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodWitchCheckers;
#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodWitchRowColumn;
#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodWitchFar;

fn score_witches(board: &WispwoodBoard) -> u16 {
    let witches = board.count_all_tiles(|tile| tile.is_witch());
    let triples = witches / 3;
    let leftover = match witches % 3 {
        0 => 0,
        1 => 4,
        _ => 10,
    };
    (triples * 18 + leftover) as u16
}
