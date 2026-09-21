use crate::games::boards::{Coordinates, same_col, same_row};

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
struct WispwoodJackSudoku {}
impl WispwoodJackScoringCard for WispwoodJackSudoku {}

struct WispwoodJackPatchwork {}
impl WispwoodJackScoringCard for WispwoodJackPatchwork {}

fn count_scoring_jacks(
    board: &WispwoodBoard,
    collision_function: impl Fn(&Coordinates, &Coordinates) -> bool,
) -> u16 {
    let jacks = board.tile_locations(|tile| tile.is_jack());
    let mut scoring_jacks = Vec::new();

    for jack in jacks {
        let mut collisions = Vec::new();
        for (index, other) in scoring_jacks.iter().enumerate() {
            if collision_function(&jack, other) {
                collisions.push(index);
            }
        }

        if collisions.len() > 0 {
            for index in collisions.into_iter().rev() {
                scoring_jacks.remove(index);
            }
        } else {
            scoring_jacks.push(jack);
        }
    }

    scoring_jacks.len() as u16
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

impl WispwoodScoringCard for WispwoodJackPatchwork {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}

// WITCH

// ORB

// HEART

// TREE
