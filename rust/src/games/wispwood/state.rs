use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::score::WispwoodScoringCards;
use crate::boards::{Board2D, Coordinates};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum WispwoodTile {
    Empty,
    #[default]
    Tree,
    Cat {
        used: bool,
    },
    Jack,
    Witch,
    Orb,
    Heart,
}

impl WispwoodTile {
    pub fn is_jack(&self) -> bool {
        matches!(self, WispwoodTile::Jack)
    }

    pub fn is_witch(&self) -> bool {
        matches!(self, WispwoodTile::Witch)
    }

    pub fn is_orb(&self) -> bool {
        matches!(self, WispwoodTile::Orb)
    }

    pub fn is_heart(&self) -> bool {
        matches!(self, WispwoodTile::Heart)
    }

    pub fn is_wisp(&self) -> bool {
        matches!(
            self,
            WispwoodTile::Jack | WispwoodTile::Witch | WispwoodTile::Orb | WispwoodTile::Heart
        )
    }

    pub fn is_tree(&self) -> bool {
        matches!(self, WispwoodTile::Tree)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, WispwoodTile::Empty)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodBoard {
    tiles: Board2D<WispwoodTile>,
}

impl AsRef<Board2D<WispwoodTile>> for WispwoodBoard {
    fn as_ref(&self) -> &Board2D<WispwoodTile> {
        &self.tiles
    }
}

impl WispwoodBoard {
    fn new(size: usize) -> WispwoodBoard {
        let tiles = Board2D::new(size);
        WispwoodBoard { tiles }
    }

    pub fn wisp_types(
        &self,
        coords: impl IntoIterator<Item = Coordinates>,
    ) -> HashSet<WispwoodTile> {
        let tile_types = self.tiles.tile_types(coords);
        tile_types
            .into_iter()
            .filter(|tile| tile.is_wisp())
            .collect()
    }

    pub fn groups(
        &self,
        coords: impl IntoIterator<Item = Coordinates>,
    ) -> Vec<HashSet<Coordinates>> {
        self.tiles
            .components(coords, |coord| self.tiles.orthogonal_neighbors(coord))
    }

    pub fn wisps(&self) -> Vec<Coordinates> {
        self.tiles.tile_coordinates(|tile| tile.is_wisp())
    }

    pub fn round(&self) -> usize {
        self.tiles.size - 4
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodPlayer {
    pub rounds: [WispwoodBoard; 3],
}

impl Default for WispwoodPlayer {
    fn default() -> Self {
        WispwoodPlayer {
            rounds: [
                WispwoodBoard::new(4),
                WispwoodBoard::new(5),
                WispwoodBoard::new(6),
            ],
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodState {
    pub players: Vec<WispwoodPlayer>,
    pub scoring_cards: WispwoodScoringCards,
}

impl WispwoodState {
    pub fn new(player_count: usize) -> WispwoodState {
        let players = (0..player_count)
            .into_iter()
            .map(|_| WispwoodPlayer::default())
            .collect();
        WispwoodState {
            players,
            scoring_cards: WispwoodScoringCards::default(),
        }
    }
}
