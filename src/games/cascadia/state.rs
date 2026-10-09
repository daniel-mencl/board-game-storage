use super::scoring::CascadiaScoringCards;
use crate::boards::HexBoard;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Copy, Hash)]
pub enum CascadiaAnimal {
    Bear,
    Elk,
    Salmon,
    Hawk,
    Fox,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Copy, Hash, Default)]
pub enum CascadiaHabitat {
    #[default]
    River,
    Wetland,
    Forest,
    Prairie,
    Mountain,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct CascadiaTile {
    // https://www.redblobgames.com/grids/hexagons/#neighbors-axial
    // pointy option, start at +1, -1 (upper right) and go clockwise
    pub habitats: [CascadiaHabitat; 6],
    possible_animals: Vec<CascadiaAnimal>,
    pub animal: Option<CascadiaAnimal>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct CascadiaBoard {
    tiles: HexBoard<CascadiaTile>,
    nature_tokens: u16,
}

impl AsRef<HexBoard<CascadiaTile>> for CascadiaBoard {
    fn as_ref(&self) -> &HexBoard<CascadiaTile> {
        &self.tiles
    }
}

impl CascadiaBoard {
    pub fn animals(&self) -> HexBoard<CascadiaAnimal> {
        let map = self
            .tiles
            .as_ref()
            .iter()
            .filter_map(|(&coord, tile)| tile.animal.map(|animal| (coord, animal)));
        HexBoard::new(map)
    }

    pub fn habitats(&self) -> HexBoard<[CascadiaHabitat; 6]> {
        let map = self
            .tiles
            .as_ref()
            .iter()
            .map(|(&coord, tile)| (coord, tile.habitats));
        HexBoard::new(map)
    }

    pub fn nature_tokens(&self) -> u16 {
        self.nature_tokens
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaState {
    pub players: Vec<CascadiaBoard>,
    pub scoring_cards: CascadiaScoringCards,
}

impl CascadiaState {
    pub fn new(player_count: usize) -> Self {
        Self { 
            players: vec![CascadiaBoard::default(); player_count],
            scoring_cards: CascadiaScoringCards::default()
        }
    }
}
