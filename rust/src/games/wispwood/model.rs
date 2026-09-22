use std::collections::HashSet;

use crate::games::boards::Coordinates;

use super::super::boards::Board2D;
use super::score::WispwoodScoringCards;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
        matches!(self, WispwoodTile::Jack | WispwoodTile::Witch | WispwoodTile::Orb | WispwoodTile::Heart)
    }

    pub fn is_tree(&self) -> bool {
        matches!(self, WispwoodTile::Tree)
    }
}

pub struct WispwoodBoard {
    tiles: Board2D<WispwoodTile>,
}

impl WispwoodBoard {
    fn new(size: usize) -> WispwoodBoard {
        let tiles = Board2D::new(size, WispwoodTile::Tree);
        WispwoodBoard { tiles }
    }

    pub fn tile_locations(&self, predicate: impl Fn(&WispwoodTile) -> bool) -> Vec<Coordinates> {
        self.tiles.tile_coordinates(predicate)
    }

    pub fn neighbors(&self, coord: &Coordinates) -> Vec<Coordinates> {
        self.tiles.orthogonal_neighbors(coord)
    }

    pub fn groups(&self, coords: Vec<Coordinates>) -> Vec<HashSet<Coordinates>> {
        self.tiles.components(coords, |coord| self.neighbors(coord))
    }

    pub fn count_tiles(&self, coords: impl IntoIterator<Item = Coordinates>, predicate: impl Fn(&WispwoodTile) -> bool) -> usize {
        self.tiles.count_tiles(coords, predicate)
    }

    pub fn count_all_tiles(&self, predicate: impl Fn(&WispwoodTile) -> bool) -> usize {
        self.tiles.count_all_tiles(predicate)
    }

    pub fn wisp_types(&self, coords: impl IntoIterator<Item = Coordinates>) -> HashSet<WispwoodTile> {
        coords
            .into_iter()
            .filter_map(|coord| self.tiles.get(coord))
            .filter(|tile| tile.is_wisp())
            .copied()
            .collect()
    }

    pub fn surrounding(&self, coord: &Coordinates) -> Vec<Coordinates> {
        let mut result = self.tiles.orthogonal_neighbors(coord);
        result.extend(self.tiles.diagonal_neighbors(coord));
        result
    }

    pub fn row(&self, row: usize) -> Vec<Coordinates> {
        self.tiles.row(row)
    }

    pub fn col(&self, col: usize) -> Vec<Coordinates> {
        self.tiles.col(col)
    }

    pub fn row_and_column(&self, coord: Coordinates) -> Vec<Coordinates> {
        let mut result = self.row(coord.row);
        result.extend(self.col(coord.col));
        result
    }

    pub fn diagonal(&self, diagonal: isize) -> Vec<Coordinates> {
        self.tiles.diagonal(diagonal)
    }

    pub fn antidiagonal(&self, antidiagonal: usize) -> Vec<Coordinates> {
        self.tiles.antidiagonal(antidiagonal)
    }

    pub fn orthogonal_lines(&self, coord: Coordinates) -> Vec<Vec<Coordinates>> {
        self.tiles.orthogonal_lines(coord)
    }

    pub fn get(&self, coord: Coordinates) -> Option<&WispwoodTile> {
        self.tiles.get(coord)
    }

    pub fn below(&self, coord: Coordinates) -> Vec<Coordinates> {
        self.tiles.line(coord, (-1, 0))
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
