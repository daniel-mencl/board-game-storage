use std::hash::Hash;
use std::collections::{HashSet, HashMap, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge<T, W = ()> {
    node: T,
    weight: W,
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

impl<T: Hash + Eq, W> Graph<T, W> {
    pub fn new(edges: HashMap<T, Vec<Edge<T, W>>>) -> Graph<T, W> {
        Self { edges }
    }
}

impl<T: Hash + Eq + Clone, W> Graph<T, W> {
    pub fn components(&self) -> Vec<HashSet<T>> {
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

impl<T: Hash + Eq, W> Graph<T, W> {
    pub fn max_weight_matching(&self) -> W {
        todo!()
    }
}