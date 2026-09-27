use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::core::{PlayerScore, Score, ScoreInto, Validate};
use crate::games::boards::Coordinates;

use super::super::boards::Board2D;
use super::score::{WispwoodBoardScore, WispwoodScoringCards};

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
struct WispwoodPlayerScore {
    rounds: [WispwoodBoardScore; 3],
}

impl WispwoodPlayerScore {
    pub fn total(&self) -> u16 {
        self.rounds.iter().map(|round| round.total).sum()
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct WispwoodPlayer {
    rounds: [WispwoodBoard; 3],
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

impl WispwoodPlayer {
    pub fn score(&self, scoring_cards: &WispwoodScoringCards) -> WispwoodPlayerScore {
        let [b0, b1, b2] = &self.rounds;
        WispwoodPlayerScore {
            rounds: [
                scoring_cards.score(b0),
                scoring_cards.score(b1),
                scoring_cards.score(b2),
            ],
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodScoreBreakdown {
    players: Vec<WispwoodPlayerScore>,
}

impl ScoreInto<Score> for WispwoodScoreBreakdown {
    fn score(&self) -> Score {
        let scores = self.players.iter().map(|player| player.total()).into_iter();
        let max = scores.clone().max().expect("Should have 1+ players");
        let scores = scores
            .clone()
            .map(|score| PlayerScore::new(score as i16, score == max))
            .collect();
        Score { scores }
    }
}

impl Validate for WispwoodScoreBreakdown {
    fn validate(&self) -> Result<(), String> {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodState {
    players: Vec<WispwoodPlayer>,
    scoring_cards: WispwoodScoringCards,
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

impl ScoreInto<WispwoodScoreBreakdown> for WispwoodState {
    fn score(&self) -> WispwoodScoreBreakdown {
        let players = self
            .players
            .iter()
            .map(|player| player.score(&self.scoring_cards))
            .collect();
        WispwoodScoreBreakdown { players }
    }
}

impl Validate for WispwoodState {
    fn validate(&self) -> Result<(), String> {
        todo!()
    }
}
