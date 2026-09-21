use crate::games::boards::Coordinates;

use super::super::boards::Board2D;
use super::score::WispwoodScoringCards;

#[derive(Clone)]
pub enum WispwoodTile {
    Empty,
    Tree,
    Cat { used: bool },
    Jack,
    Witch,
    Orb,
    Heart,
}

impl WispwoodTile {
    pub fn is_jack(&self) -> bool {
        matches!(self, WispwoodTile::Jack)
    }
}

pub struct WispwoodBoard {
    size: usize,
    tiles: Board2D<WispwoodTile>,
}

impl WispwoodBoard {
    fn new(size: usize) -> WispwoodBoard {
        let tiles = Board2D::new(size, WispwoodTile::Tree);
        WispwoodBoard { size, tiles }
    }

    pub fn tile_locations(&self, predicate: impl Fn(&WispwoodTile) -> bool) -> Vec<Coordinates> {
        self.tiles.tile_coordinates(predicate)
    }
}

struct WispwoodPlayer {
    rounds: [WispwoodBoard; 3],
}

impl WispwoodPlayer {
    fn new() -> WispwoodPlayer {
        WispwoodPlayer {
            rounds: [
                WispwoodBoard::new(4),
                WispwoodBoard::new(5),
                WispwoodBoard::new(6),
            ],
        }
    }
}

pub struct WispwoodState {
    players: Vec<WispwoodPlayer>,
    scoring_cards: WispwoodScoringCards,
}
