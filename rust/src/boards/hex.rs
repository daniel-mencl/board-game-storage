use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::Add;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub struct HexCoordinates {
    // https://www.redblobgames.com/grids/hexagons/#neighbors-axial
    // axial, pointy variant
    q: i8,
    r: i8,
}

impl HexCoordinates {
    fn q(&self) -> i8 {
        self.q
    }

    fn r(&self) -> i8 {
        self.r
    }

    fn s(&self) -> i8 {
        -self.q - self.r
    }

    const CLOCKWISE_OFFSETS: [HexCoordinates; 6] = [
        HexCoordinates { q: 1, r: -1 },
        HexCoordinates { q: 1, r: 0 },
        HexCoordinates { q: 0, r: 1 },
        HexCoordinates { q: -1, r: 1 },
        HexCoordinates { q: -1, r: 0 },
        HexCoordinates { q: 0, r: -1 },
    ];
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge<T, W = ()> {
    node: T,
    weight: W,
}
impl<T> Edge<T> {
    pub fn unweighted(node: T) -> Self {
        Self { node, weight: () }
    }
}
impl<T, W> Edge<T, W> {
    pub fn weighted(node: T, weight: W) -> Self {
        Self { node, weight }
    }
}

pub struct Graph<T: Hash + Eq, W = ()> {
    // every node should have an entry, even if it is an empty vec
    edges: HashMap<T, Vec<Edge<T, W>>>,
}
impl<T: Hash + Eq + Clone, W> Graph<T, W> {
    fn components(&self) -> Vec<HashSet<T>> {
        let mut result = Vec::new();
        let mut unseen: HashSet<T> = self.edges.keys().cloned().collect();

        while let Some(start) = unseen.iter().next().cloned() {
            unseen.remove(&start);
            let mut component = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(start);

            while let Some(current) = queue.pop_front() {
                let neighbors = self.edges.get(&current).expect("Should be in edges");
                component.insert(current);
                for node in neighbors.iter().map(|edge| &edge.node) {
                    if unseen.contains(node) {
                        unseen.remove(node);
                        component.insert(node.clone());
                        queue.push_back(node.clone());
                    }
                }
            }

            result.push(component);
        }

        result
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct HexBoard<T> {
    tiles: HashMap<HexCoordinates, T>,
}

impl<T> HexBoard<T> {
    pub fn to_graph<W>(
        &self,
        node_predicate: impl Fn(&HexCoordinates, &T) -> bool,
        neighbor_function: impl Fn(&HexBoard<T>, &HexCoordinates, &T) -> Vec<(HexCoordinates, W)>,
    ) -> Graph<HexCoordinates, W> {
        let active_nodes: HashMap<_, _> = self
            .tiles
            .iter()
            .filter(|(coord, val)| node_predicate(coord, val))
            .collect();

        let mut edges = HashMap::new();

        for (&coord, &val) in active_nodes.iter() {
            let current_edges = neighbor_function(&self, coord, val)
                .into_iter()
                .filter(|(coord, _)| active_nodes.contains_key(coord))
                .map(|(coord, weight)| Edge::weighted(coord, weight))
                .collect();

            edges.insert(*coord, current_edges);
        }

        Graph { edges }
    }

    pub fn to_unweighted_graph(
        &self,
        node_predicate: impl Fn(&HexCoordinates, &T) -> bool,
        neighbor_function: impl Fn(&HexBoard<T>, &HexCoordinates, &T) -> Vec<HexCoordinates>,
    ) -> Graph<HexCoordinates> {
        fn add_unit_weight(coords: Vec<HexCoordinates>) -> Vec<(HexCoordinates, ())> {
            coords.into_iter().map(|coord| (coord, ())).collect()
        }

        self.to_graph(node_predicate, |board, coord, t| {
            add_unit_weight(neighbor_function(board, coord, t))
        })
    }

    pub fn neighbors(&self, center: HexCoordinates, _tile: &T) -> Vec<HexCoordinates> {
        HexCoordinates::CLOCKWISE_OFFSETS
            .iter()
            .copied()
            .map(|coord| center + coord)
            .filter(|coord| self.tiles.contains_key(coord))
            .collect()
    }
}
