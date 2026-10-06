use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::{Add, Mul, Sub};
use crate::algorithms::{Edge, Graph};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
pub struct HexCoordinates {
    // https://www.redblobgames.com/grids/hexagons/#neighbors-axial
    // axial, pointy variant
    pub q: i8,
    pub r: i8,
}

impl HexCoordinates {
    pub const CLOCKWISE_OFFSETS: [HexCoordinates; 6] = [
        HexCoordinates { q: 1, r: -1 },
        HexCoordinates { q: 1, r: 0 },
        HexCoordinates { q: 0, r: 1 },
        HexCoordinates { q: -1, r: 1 },
        HexCoordinates { q: -1, r: 0 },
        HexCoordinates { q: 0, r: -1 },
    ];

    pub fn diff_size(diff: HexCoordinates) -> u8 {
        (diff.q.abs() as u8 + (diff.q + diff.r).abs() as u8 + diff.r.abs() as u8) / 2
    }

    pub fn distance(a: HexCoordinates, b: HexCoordinates) -> u8 {
        Self::diff_size(b - a)
    }

    pub fn opposite_index(index: usize) -> usize {
        if index >= 3 { index - 3 } else { index + 3 }
    }
}

impl Add for HexCoordinates {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl Sub for HexCoordinates {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

impl Mul<i8> for HexCoordinates {
    type Output = Self;
    fn mul(self, rhs: i8) -> Self::Output {
        Self {
            r: self.r * rhs,
            q: self.q * rhs,
        }
    }
}

impl Mul<u8> for HexCoordinates {
    type Output = Self;
    fn mul(self, rhs: u8) -> Self::Output {
        self * (rhs as i8)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct HexBoard<T> {
    tiles: HashMap<HexCoordinates, T>,
}

impl<T> HexBoard<T> {
    pub fn new(tiles: impl IntoIterator<Item = (HexCoordinates, T)>) -> Self {
        let tiles = tiles.into_iter().collect();
        Self { tiles }
    }

    pub fn to_graph<W>(
        &self,
        node_predicate: impl Fn(HexCoordinates, &T) -> bool,
        neighbor_function: impl Fn(HexCoordinates, &T) -> Vec<(HexCoordinates, W)>,
    ) -> Graph<HexCoordinates, W> {
        let active_nodes: HashMap<_, _> = self
            .tiles
            .iter()
            .filter(|(coord, val)| node_predicate(**coord, val))
            .collect();

        let mut edges = HashMap::new();

        for (&coord, &val) in active_nodes.iter() {
            let current_edges = neighbor_function(*coord, val)
                .into_iter()
                .filter(|(coord, _)| active_nodes.contains_key(coord))
                .map(|(coord, weight)| Edge::weighted(coord, weight))
                .collect();

            edges.insert(*coord, current_edges);
        }

        Graph::new(edges)
    }

    pub fn to_unweighted_graph(
        &self,
        node_predicate: impl Fn(HexCoordinates, &T) -> bool,
        neighbor_function: impl Fn(HexCoordinates, &T) -> Vec<HexCoordinates>,
    ) -> Graph<HexCoordinates> {
        fn add_unit_weight(coords: Vec<HexCoordinates>) -> Vec<(HexCoordinates, ())> {
            coords.into_iter().map(|coord| (coord, ())).collect()
        }

        self.to_graph(node_predicate, |coord, t| {
            add_unit_weight(neighbor_function(coord, t))
        })
    }

    pub fn neighbors(&self, center: HexCoordinates) -> Vec<HexCoordinates> {
        HexCoordinates::CLOCKWISE_OFFSETS
            .iter()
            .copied()
            .map(|coord| center + coord)
            .filter(|coord| self.tiles.contains_key(coord))
            .collect()
    }

    pub fn all_neighbors(
        &self,
        coords: &[HexCoordinates],
    ) -> impl IntoIterator<Item = HexCoordinates> {
        let mut result = HashSet::new();

        for coord in coords {
            for other in self.neighbors(*coord) {
                result.insert(other);
            }
        }

        for coord in coords {
            result.remove(coord);
        }

        result
    }
}

impl<T> AsRef<HashMap<HexCoordinates, T>> for HexBoard<T> {
    fn as_ref(&self) -> &HashMap<HexCoordinates, T> {
        &self.tiles
    }
}
