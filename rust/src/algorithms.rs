mod mwis;
mod graph;

pub use mwis::{MwisResult, max_weight_independent_set, max_weight_set_packing};
pub use graph::{Edge, Graph};