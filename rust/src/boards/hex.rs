use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

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
}

pub struct WeightedEdge<T> {
    node: T,
    weight: i64,
}
impl<T> WeightedEdge<T> {
    pub fn new_unweighted(node: T) -> WeightedEdge<T> {
        WeightedEdge { node, weight: 0 }
    }

    pub fn new(node: T, weight: i64) -> WeightedEdge<T> {
        WeightedEdge { node, weight }
    }
}

pub struct WeightedGraph<T: Hash + Eq> {
    // every node should have an entry, even if it is an empty vec
    // undirected graph
    edges: HashMap<T, Vec<WeightedEdge<T>>>,
}
impl<T: Hash + Eq + Clone> WeightedGraph<T> {
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
    pub fn to_graph(
        &self,
        node_predicate: impl Fn(&HexCoordinates, &T) -> bool,
        neighbor_function: impl Fn(&HexCoordinates, &T) -> Vec<(HexCoordinates, T)>,
        weight_function: impl Fn(&HexCoordinates, &T) -> i64,
    ) -> WeightedGraph<HexCoordinates> {
        let nodes: Vec<_> = self
            .tiles
            .iter()
            .filter(|(coord, tile)| node_predicate(*coord, *tile))
            .collect();
        let mut edges = HashMap::new();

        for (coord, tile) in nodes {
            let node_edges = neighbor_function(coord, tile)
                .iter()
                .map(|(coord, tile)| WeightedEdge::new(*coord, weight_function(coord, tile)))
                .collect();
            edges.insert(*coord, node_edges);
        }

        WeightedGraph { edges }
    }
}
