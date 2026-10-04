use std::collections::HashSet;
use std::{hash::Hash, ops::Add};

pub struct MwisResult<W> {
    pub max_weight: W,
    pub selected_indices: Vec<usize>,
}

pub fn max_weight_independent_set<T, W>(
    candidates: &[(T, W)],
    conflict: impl Fn(&T, &T) -> bool,
) -> MwisResult<W>
where
    W: Copy + Default + Ord + Add<Output = W>,
{
    todo!()
}

pub fn max_weight_set_packing<T, W>(candidates: &[(HashSet<T>, W)]) -> MwisResult<W>
where
    T: Eq + Hash,
    W: Copy + Default + Ord + Add<Output = W>,
{
    max_weight_independent_set(candidates, |set_a, set_b| {
        set_a.intersection(set_b).count() > 0
    })
}
