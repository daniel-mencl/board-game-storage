use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::Coordinates;
use super::WispwoodBoard;
use super::WispwoodScoringCard;

#[enum_dispatch(WispwoodScoringCard)]
#[derive(Clone, Serialize, Deserialize)]
pub enum WispwoodJackScoringCard {
    Adjacent(WispwoodJackAdjacent),
    Sudoku(WispwoodJackSudoku),
    Diagonal(WispwoodJackDiagonal),
    Pairs(WispwoodJackPairs),
    Groups(WispwoodJackGroups),
}

impl Default for WispwoodJackScoringCard {
    fn default() -> Self {
        Self::Adjacent(WispwoodJackAdjacent)
    }
}

fn count_scoring_jacks(
    board: &WispwoodBoard,
    collision_function: impl Fn(&Coordinates, &Coordinates) -> bool,
) -> u16 {
    let jacks = board.tile_locations(|tile| tile.is_jack());
    let mut scoring_jacks = 0;

    for (index, jack) in jacks.iter().enumerate() {
        let mut scoring = true;
        for (other_index, other_jack) in jacks.iter().enumerate() {
            if index == other_index {
                continue;
            }

            if collision_function(jack, other_jack) {
                scoring = false;
                break;
            }
        }

        if scoring {
            scoring_jacks += 1;
        }
    }

    scoring_jacks
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodJackAdjacent;
impl WispwoodScoringCard for WispwoodJackAdjacent {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let scoring_jacks = count_scoring_jacks(board, |first, second| {
            Coordinates::diagonally_adjacent(first, second)
        });

        match scoring_jacks {
            0 => 0,
            1 => 4,
            2 => 9,
            3 => 15,
            4 => 22,
            _ => 30 + (scoring_jacks - 5) * 8,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodJackSudoku;
impl WispwoodScoringCard for WispwoodJackSudoku {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let scoring_jacks = count_scoring_jacks(board, |first, second| {
            Coordinates::same_row(first, second) || Coordinates::same_col(first, second)
        });

        match scoring_jacks {
            0 => 0,
            1 => 4,
            2 => 9,
            3 => 15,
            4 => 22,
            5 => 30,
            6 => 40,
            _ => u16::MAX,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodJackDiagonal;
impl WispwoodScoringCard for WispwoodJackDiagonal {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let scoring_jacks = count_scoring_jacks(board, |first, second| {
            Coordinates::same_diagonal(first, second)
                || Coordinates::same_antidiagonal(first, second)
        });

        match scoring_jacks {
            0 => 0,
            1 => 4,
            2 => 9,
            3 => 15,
            4 => 22,
            _ => 30 + (scoring_jacks - 5) * 9,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodJackPairs;
impl WispwoodScoringCard for WispwoodJackPairs {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let jacks = board.tile_locations(|tile| tile.is_jack());
        let jack_groups = board.groups(jacks);
        let scoring_groups = jack_groups
            .into_iter()
            .filter(|group| board.count_tiles(group.iter().copied(), |tile| tile.is_jack()) == 2)
            .count() as u16;

        match scoring_groups {
            0 => 0,
            1 => 11,
            _ => 23 + (scoring_groups - 2) * 14,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodJackGroups;
impl WispwoodScoringCard for WispwoodJackGroups {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let wisps = board.tile_locations(|tile| tile.is_wisp());
        let groups = board.groups(wisps);
        let scoring_groups = groups
            .into_iter()
            .filter(|group| board.count_tiles(group.iter().copied(), |tile| tile.is_jack()) >= 1)
            .count() as u16;

        match scoring_groups {
            0 => 0,
            1 => 4,
            2 => 9,
            3 => 15,
            4 => 22,
            _ => 30 + (scoring_groups - 5) * 8,
        }
    }
}
