use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::{Coordinates, WispwoodBoard, WispwoodScoringCard};
use std::cmp::max;

#[enum_dispatch(WispwoodScoringCard)]
#[derive(Clone, Serialize, Deserialize)]
pub enum WispwoodHeartScoringCard {
    RowColumn(WispwoodHeartRowColumn),
    Diagonal(WispwoodHeartDiagonal),
    Adjacent(WispwoodHeartAdjacent),
    Line(WispwoodHeartLine),
    Below(WispwoodHeartBelow),
}

impl Default for WispwoodHeartScoringCard {
    fn default() -> Self {
        Self::RowColumn(WispwoodHeartRowColumn)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodHeartRowColumn;
impl WispwoodScoringCard for WispwoodHeartRowColumn {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let hearts = board.tile_locations(|tile| tile.is_heart());
        hearts
            .into_iter()
            .map(|coord| board.row_and_column(coord))
            .map(|coords| board.count_tiles(coords, |tile| tile.is_tree()))
            .sum::<usize>() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodHeartDiagonal;
impl WispwoodHeartDiagonal {
    fn tree_count_either_diagonal(board: &WispwoodBoard, coord: Coordinates) -> usize {
        let diagonal = board.diagonal(coord.diagonal_index());
        let antidiagonal = board.antidiagonal(coord.antidiagonal_index());

        let diagonal_trees = board.count_tiles(diagonal, |tile| tile.is_tree());
        let antidiagonal_trees = board.count_tiles(antidiagonal, |tile| tile.is_tree());

        max(diagonal_trees, antidiagonal_trees)
    }
}
impl WispwoodScoringCard for WispwoodHeartDiagonal {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let hearts = board.tile_locations(|tile| tile.is_heart());
        2 * hearts
            .into_iter()
            .map(|coord| Self::tree_count_either_diagonal(board, coord))
            .sum::<usize>() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodHeartAdjacent;
impl WispwoodScoringCard for WispwoodHeartAdjacent {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let hearts = board.tile_locations(|tile| tile.is_heart());
        2 * hearts
            .into_iter()
            .map(|coord| board.neighbors(&coord))
            .map(|neighbors| board.count_tiles(neighbors, |tile| tile.is_tree()))
            .sum::<usize>() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodHeartLine;
impl WispwoodHeartLine {
    fn uninterupted_trees(board: &WispwoodBoard, line: &[Coordinates]) -> usize {
        let mut trees = 0;

        for coord in line.iter().copied() {
            if board.get(coord).map_or(false, |tile| tile.is_tree()) {
                trees += 1;
            } else {
                break;
            }
        }

        trees
    }
}
impl WispwoodScoringCard for WispwoodHeartLine {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let hearts = board.tile_locations(|tile| tile.is_heart());
        2 * hearts
            .into_iter()
            .map(|coord| board.orthogonal_lines(coord))
            .map(|lines| {
                lines
                    .into_iter()
                    .map(|line| Self::uninterupted_trees(board, &line))
            })
            .map(|tree_counts| tree_counts.max().unwrap_or(0))
            .sum::<usize>() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodHeartBelow;
impl WispwoodScoringCard for WispwoodHeartBelow {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let hearts = board.tile_locations(|tile| tile.is_heart());
        2 * hearts
            .into_iter()
            .map(|coord| board.below(coord))
            .map(|coords| board.count_tiles(coords, |tile| tile.is_tree()))
            .sum::<usize>() as u16
    }
}
