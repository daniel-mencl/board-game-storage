use std::collections::HashSet;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::games::boards::Coordinates;

use super::{WispwoodBoard, WispwoodScoringCard};

#[enum_dispatch(WispwoodScoringCard)]
#[derive(Clone, Serialize, Deserialize)]
pub enum WispwoodTreeScoringCard {
    Largest(WispwoodTreeLargest),
    SecondLargest(WispwoodTreeSecondLargest),
    Groups(WispwoodTreeGroups),
    RowColumn(WispwoodTreeRowColumn),
    Central(WispwoodTreeCentral),
    Diagonal(WispwoodTreeDiagonal),
}

impl Default for WispwoodTreeScoringCard {
    fn default() -> Self {
        Self::Largest(WispwoodTreeLargest)
    }
}

fn get_trees(board: &WispwoodBoard) -> Vec<Coordinates> {
    board.as_ref().tile_coordinates(|tile| tile.is_tree())
}

fn tree_group_sizes(board: &WispwoodBoard) -> impl IntoIterator<Item = usize> {
    let trees = get_trees(board);
    let groups = board.groups(trees);
    groups.into_iter().map(|group| group.len())
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodTreeLargest;
impl WispwoodScoringCard for WispwoodTreeLargest {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let trees: Vec<_> = tree_group_sizes(board).into_iter().collect();
        trees.into_iter().max().unwrap_or(0) as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodTreeSecondLargest;
impl WispwoodScoringCard for WispwoodTreeSecondLargest {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let mut trees: Vec<_> = tree_group_sizes(board).into_iter().collect();
        if trees.len() < 2 {
            return 0;
        }
        trees.sort_by(|a, b| b.cmp(a));
        2 * trees[1] as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodTreeGroups;
impl WispwoodScoringCard for WispwoodTreeGroups {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        4 * tree_group_sizes(board)
            .into_iter()
            .filter(|size| *size >= 3)
            .count() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodTreeRowColumn;
impl WispwoodTreeRowColumn {
    fn at_least_three_trees(board: &WispwoodBoard, line: &Vec<Coordinates>) -> bool {
        line.iter()
            .filter(|coord| {
                board
                    .as_ref()
                    .get(**coord)
                    .expect("Input should be valid coords")
                    .is_tree()
            })
            .count()
            >= 3
    }
}
impl WispwoodScoringCard for WispwoodTreeRowColumn {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let mut rows_and_columns = board.as_ref().rows();
        rows_and_columns.extend(board.as_ref().cols());
        2 * rows_and_columns
            .into_iter()
            .filter(|line| Self::at_least_three_trees(board, line))
            .count() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodTreeCentral;
impl WispwoodTreeCentral {
    fn at_least_three_trees_beside(
        board: &WispwoodBoard,
        tree: &Coordinates,
        all_trees: &HashSet<Coordinates>,
    ) -> bool {
        let neighbors = board.as_ref().orthogonal_neighbors(tree);
        neighbors
            .into_iter()
            .filter(|other| all_trees.contains(other))
            .count()
            >= 3
    }
}
impl WispwoodScoringCard for WispwoodTreeCentral {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let trees = get_trees(board);
        let tree_set: HashSet<_> = trees.iter().copied().collect();

        3 * trees
            .into_iter()
            .filter(|tree| Self::at_least_three_trees_beside(board, tree, &tree_set))
            .count() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodTreeDiagonal;

impl WispwoodTreeDiagonal {
    fn tree_count_no_wisps(board: &WispwoodBoard, coords: Vec<Coordinates>) -> Option<usize> {
        let mut trees = 0;
        for coord in coords {
            let tile = board.as_ref().get(coord)?;

            if tile.is_wisp() {
                return None;
            }

            if tile.is_tree() {
                trees += 1;
            }
        }

        Some(trees)
    }
}

impl WispwoodScoringCard for WispwoodTreeDiagonal {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let mut diagonals = board.as_ref().diagonals();
        diagonals.extend(board.as_ref().antidiagonals());

        3 * diagonals
            .into_iter()
            .filter_map(|diagonal| Self::tree_count_no_wisps(board, diagonal))
            .max()
            .unwrap_or(0) as u16
    }
}
