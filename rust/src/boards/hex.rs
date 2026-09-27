use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
