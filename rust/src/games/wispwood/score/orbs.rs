use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::{Coordinates, WispwoodBoard, WispwoodScoringCard, WispwoodTile};
use std::{cmp::max, collections::HashSet};

#[enum_dispatch(WispwoodScoringCard)]
#[derive(Clone, Serialize, Deserialize)]
pub enum WispwoodOrbScoringCard {
    Groups(WispwoodOrbGroups),
    Around(WispwoodOrbAround),
    Adjacent(WispwoodOrbAdjacent),
    Least(WispwoodOrbLeast),
    RowColumn(WispwoodOrbRowColumn),
    Corner(WispwoodOrbCorner),
}

impl Default for WispwoodOrbScoringCard {
    fn default() -> Self {
        Self::Groups(WispwoodOrbGroups)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodOrbGroups;
impl WispwoodScoringCard for WispwoodOrbGroups {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let wisps = board.tile_locations(|tile| tile.is_wisp());
        let groups = board.groups(wisps);
        2 * groups
            .into_iter()
            .map(|group| board.wisp_types(group))
            .filter(|wisp_types| wisp_types.iter().any(|tile| tile.is_orb()))
            .map(|wisp_types| wisp_types.len())
            .sum::<usize>() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodOrbAround;
impl WispwoodScoringCard for WispwoodOrbAround {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let orbs = board.tile_locations(|tile| tile.is_orb());
        2 * orbs
            .into_iter()
            .map(|coord| board.surrounding(&coord))
            .map(|coords| board.wisp_types(coords))
            .filter(|wisp_types| wisp_types.iter().any(|tile| tile.is_orb()))
            .map(|wisp_types| wisp_types.len())
            .sum::<usize>() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodOrbAdjacent;
impl WispwoodOrbAdjacent {
    fn wisp_score(tile: &WispwoodTile) -> u16 {
        match tile {
            WispwoodTile::Orb => 4,
            WispwoodTile::Jack => 5,
            WispwoodTile::Witch => 6,
            WispwoodTile::Heart => 7,
            _ => u16::MAX,
        }
    }

    fn score_lowest_value_wisp(wisp_types: HashSet<WispwoodTile>) -> u16 {
        wisp_types
            .iter()
            .map(|tile| Self::wisp_score(tile))
            .min()
            .unwrap_or(0)
    }
}
impl WispwoodScoringCard for WispwoodOrbAdjacent {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let orbs = board.tile_locations(|tile| tile.is_orb());
        orbs.into_iter()
            .map(|coord| board.neighbors(&coord))
            .map(|neighbors| board.wisp_types(neighbors))
            .map(|wisp_types| Self::score_lowest_value_wisp(wisp_types))
            .sum()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodOrbLeast;
impl WispwoodScoringCard for WispwoodOrbLeast {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let orbs = board.count_all_tiles(|tile| tile.is_orb());
        let jacks = board.count_all_tiles(|tile| tile.is_jack());
        let witches = board.count_all_tiles(|tile| tile.is_witch());
        let hearts = board.count_all_tiles(|tile| tile.is_heart());

        let least = [orbs, jacks, witches, hearts]
            .iter()
            .min()
            .copied()
            .expect("Shouldn't happen");
        let multiplier = if orbs == least {
            7
        } else if jacks == least {
            5
        } else if witches == least {
            4
        } else {
            3
        };

        (orbs * multiplier) as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodOrbRowColumn;
impl WispwoodOrbRowColumn {
    fn wisp_count_row_or_column(board: &WispwoodBoard, coord: Coordinates) -> usize {
        let row = board.row(coord.row);
        let row_wisps = board.wisp_types(row).len();
        let col = board.col(coord.col);
        let col_wisps = board.wisp_types(col).len();

        max(row_wisps, col_wisps)
    }
}
impl WispwoodScoringCard for WispwoodOrbRowColumn {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let orbs = board.tile_locations(|tile| tile.is_orb());
        2 * orbs
            .into_iter()
            .map(|coord| Self::wisp_count_row_or_column(board, coord))
            .sum::<usize>() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodOrbCorner;
impl WispwoodScoringCard for WispwoodOrbCorner {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}
