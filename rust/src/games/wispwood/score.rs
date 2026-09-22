use std::{cmp::max, collections::HashSet};

use crate::games::{boards::{Coordinates, diagonally_adjacent, same_antidiagonal, same_col, same_diagonal, same_row}, wispwood::model::WispwoodTile};

use super::model::WispwoodBoard;

trait WispwoodScoringCard {
    fn score(&self, board: &WispwoodBoard) -> u16;
}

trait WispwoodJackScoringCard: WispwoodScoringCard {}
trait WispwoodWitchScoringCard: WispwoodScoringCard {}
trait WispwoodOrbScoringCard: WispwoodScoringCard {}
trait WispwoodHeartScoringCard: WispwoodScoringCard {}
trait WispwoodTreeScoringCard: WispwoodScoringCard {}

pub struct WispwoodScoringCards {
    jack: Box<dyn WispwoodJackScoringCard>,
    witch: Box<dyn WispwoodWitchScoringCard>,
    orb: Box<dyn WispwoodOrbScoringCard>,
    heart: Box<dyn WispwoodHeartScoringCard>,
    tree: Box<dyn WispwoodTreeScoringCard>,
}

// JACK
struct WispwoodJackAdjacent {}
impl WispwoodJackScoringCard for WispwoodJackAdjacent {}

struct WispwoodJackSudoku {}
impl WispwoodJackScoringCard for WispwoodJackSudoku {}

struct WispwoodJackDiagonal {}
impl WispwoodJackScoringCard for WispwoodJackDiagonal {}

struct WispwoodJackPairs {}
impl WispwoodJackScoringCard for WispwoodJackPairs {}

struct WispwoodJackGroups {}
impl WispwoodJackScoringCard for WispwoodJackGroups {}

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

impl WispwoodScoringCard for WispwoodJackAdjacent {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let scoring_jacks = count_scoring_jacks(board, |first, second|
            diagonally_adjacent(first, second)
        );

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

impl WispwoodScoringCard for WispwoodJackSudoku {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let scoring_jacks = count_scoring_jacks(board, |first, second| {
            same_row(first, second) || same_col(first, second)
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

impl WispwoodScoringCard for WispwoodJackDiagonal {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let scoring_jacks = count_scoring_jacks(board, |first,second|
            same_diagonal(first, second) || same_antidiagonal(first, second)
        );

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

impl WispwoodScoringCard for WispwoodJackPairs {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let jacks = board.tile_locations(|tile| tile.is_jack());
        let jack_groups = board.groups(jacks);
        let scoring_groups = jack_groups.into_iter().filter(|group|
            board.count_tiles(group.iter().copied(), |tile| tile.is_jack()) == 2
        ).count() as u16;

        match scoring_groups {
            0 => 0,
            1 => 11,
            _ => 23 + (scoring_groups - 2) * 14
        }
    }
}

impl WispwoodScoringCard for WispwoodJackGroups {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let wisps = board.tile_locations(|tile| tile.is_wisp());
        let groups = board.groups(wisps);
        let scoring_groups = groups.into_iter().filter(|group|
            board.count_tiles(group.iter().copied(), |tile| tile.is_jack()) >= 1
        ).count() as u16;

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

// WITCH

impl<T: WispwoodWitchScoringCard> WispwoodScoringCard for T {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        score_witches(board)
    }
}

struct WispwoodWitchAdjacent {}
impl WispwoodWitchScoringCard for WispwoodWitchAdjacent {}

struct WispwoodWitchDiagonal {}
impl WispwoodWitchScoringCard for WispwoodWitchDiagonal {}

struct WispwoodWitchCheckers {}
impl WispwoodWitchScoringCard for WispwoodWitchCheckers {}

struct WispwoodWitchRowColumn {}
impl WispwoodWitchScoringCard for WispwoodWitchRowColumn {}

struct WispwoodWitchFar {}
impl WispwoodWitchScoringCard for WispwoodWitchFar {}

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

// ORB
struct WispwoodOrbGroups {}
impl WispwoodOrbScoringCard for WispwoodOrbGroups {}

struct WispwoodOrbAround {}
impl WispwoodOrbScoringCard for WispwoodOrbAround {}

struct WispwoodOrbAdjacent {}
impl WispwoodOrbScoringCard for WispwoodOrbAdjacent {}

struct WispwoodOrbLeast {}
impl WispwoodOrbScoringCard for WispwoodOrbLeast {}

struct WispwoodOrbRowColumn {}
impl WispwoodOrbScoringCard for WispwoodOrbRowColumn {}

struct WispwoodOrbCorner {}
impl WispwoodOrbScoringCard for WispwoodOrbCorner {}

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

fn score_lowest_value_wisp(wisp_types: HashSet<WispwoodTile>) -> u16 {
    fn wisp_score(tile: &WispwoodTile) -> u16 {
        match tile {
            WispwoodTile::Orb => 4,
            WispwoodTile::Jack => 5,
            WispwoodTile::Witch => 6,
            WispwoodTile::Heart => 7,
            _ => u16::MAX,
        }
    }

    wisp_types
        .iter()
        .map(|tile| wisp_score(tile))
        .min()
        .unwrap_or(0)
}

impl WispwoodScoringCard for WispwoodOrbAdjacent {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let orbs = board.tile_locations(|tile| tile.is_orb());
        orbs
            .into_iter()
            .map(|coord| board.neighbors(&coord))
            .map(|neighbors| board.wisp_types(neighbors))
            .map(|wisp_types| score_lowest_value_wisp(wisp_types))
            .sum()
    }
}

impl WispwoodScoringCard for WispwoodOrbLeast {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let orbs = board.count_all_tiles(|tile| tile.is_orb());
        let jacks = board.count_all_tiles(|tile| tile.is_jack());
        let witches = board.count_all_tiles(|tile| tile.is_witch());
        let hearts = board.count_all_tiles(|tile| tile.is_heart());

        let least = [orbs, jacks, witches, hearts].iter().min().copied().expect("Shouldn't happen");
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

fn wisp_count_row_or_column(board: &WispwoodBoard, coord: Coordinates) -> usize {
    let row = board.row(coord.row);
    let row_wisps = board.wisp_types(row).len();
    let col = board.col(coord.col);
    let col_wisps = board.wisp_types(col).len();

    max(row_wisps, col_wisps)
}

impl WispwoodScoringCard for WispwoodOrbRowColumn {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let orbs = board.tile_locations(|tile| tile.is_orb());
        2 * 
        orbs
            .into_iter()
            .map(|coord| wisp_count_row_or_column(board, coord))
            .sum::<usize>() as u16
    }
}

impl WispwoodScoringCard for WispwoodOrbCorner {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}

// HEART
struct WispwoodHeartRowColumn {}
impl WispwoodHeartScoringCard for WispwoodHeartRowColumn {}

struct WispwoodHeartDiagonal {}
impl WispwoodHeartScoringCard for WispwoodHeartDiagonal {}

struct WispwoodHeartAdjacent {}
impl WispwoodHeartScoringCard for WispwoodHeartAdjacent {}

struct WispwoodHeartLine {}
impl WispwoodHeartScoringCard for WispwoodHeartLine {}

struct WispwoodHeartBelow {}
impl WispwoodHeartScoringCard for WispwoodHeartBelow {}

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

fn tree_count_either_diagonal(board: &WispwoodBoard, coord: Coordinates) -> usize {
    let diagonal = board.diagonal(coord.diagonal_index());
    let antidiagonal = board.antidiagonal(coord.antidiagonal_index());

    let diagonal_trees = board.count_tiles(diagonal, |tile| tile.is_tree());
    let antidiagonal_trees = board.count_tiles(antidiagonal, |tile| tile.is_tree());

    max(diagonal_trees, antidiagonal_trees)
}

impl WispwoodScoringCard for WispwoodHeartDiagonal {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let hearts = board.tile_locations(|tile| tile.is_heart());
        2 * hearts
            .into_iter()
            .map(|coord| tree_count_either_diagonal(board, coord))
            .sum::<usize>() as u16
    }
}

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

impl WispwoodScoringCard for WispwoodHeartLine {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        let hearts = board.tile_locations(|tile| tile.is_heart());
        2 * hearts
            .into_iter()
            .map(|coord| board.orthogonal_lines(coord))
            .map(|lines| lines.into_iter().map(|line| uninterupted_trees(board, &line)))
            .map(|tree_counts| tree_counts.max().unwrap_or(0))
            .sum::<usize>() as u16
    }
}

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

// TREE

struct WispwoodTreeLargest {}
impl WispwoodTreeScoringCard for WispwoodTreeLargest {}

struct WispwoodTreeSecondLargest {}
impl WispwoodTreeScoringCard for WispwoodTreeSecondLargest {}

struct WispwoodTreeGroups {}
impl WispwoodTreeScoringCard for WispwoodTreeGroups {}

struct WispwoodTreeRowColumn {}
impl WispwoodTreeScoringCard for WispwoodTreeRowColumn {}

struct WispwoodTreeCentral {}
impl WispwoodTreeScoringCard for WispwoodTreeCentral {}

struct WispwoodTreeDiagonal {}
impl WispwoodTreeScoringCard for WispwoodTreeDiagonal {}

impl WispwoodScoringCard for WispwoodTreeLargest {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}

impl WispwoodScoringCard for WispwoodTreeSecondLargest {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}

impl WispwoodScoringCard for WispwoodTreeGroups {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}

impl WispwoodScoringCard for WispwoodTreeRowColumn {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}

impl WispwoodScoringCard for WispwoodTreeCentral {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}

impl WispwoodScoringCard for WispwoodTreeDiagonal {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}