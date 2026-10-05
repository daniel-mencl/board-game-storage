use std::collections::HashSet;
use std::{hash::Hash, ops::Add};

pub struct MwisResult<W> {
    pub max_weight: W,
    pub selected_indices: Vec<usize>,
}

pub fn max_weight_independent_set<W>(weights: &[W], conflicts: &[Vec<bool>]) -> MwisResult<W>
where
    W: Copy + Default + Ord + Add<Output = W>,
{
    let n = weights.len();
    if n == 0 {
        return MwisResult {
            max_weight: W::default(),
            selected_indices: vec![],
        };
    }

    let mut solver = MwisSolver::new(weights, conflicts);
    solver.solve(0, vec![true; n], W::default(), vec![]);
    solver.result()
}

pub fn max_weight_set_packing<T, W>(candidates: &[(HashSet<T>, W)]) -> MwisResult<W>
where
    T: Eq + Hash,
    W: Copy + Default + Ord + Add<Output = W>,
{
    // sort by weight
    let mut sorted_candidates: Vec<_> = candidates.iter().map(|(_, w)| *w).enumerate().collect();
    sorted_candidates.sort_by(|(_, a), (_, b)| b.cmp(a));

    let n = sorted_candidates.len();
    let mut conflicts: Vec<Vec<bool>> = Vec::with_capacity(n);

    // compute set conflicts
    for first_index in 0..n {
        let first_index_old = sorted_candidates[first_index].0;
        let first_set = &candidates[first_index_old].0;
        let mut current_conflicts: Vec<bool> = Vec::with_capacity(n);

        for second_index in 0..n {
            let second_index_old = sorted_candidates[second_index].0;
            let second_set = &candidates[second_index_old].0;
            let conflict = first_set.intersection(second_set).count() > 0;
            current_conflicts.push(conflict);
        }

        conflicts.push(current_conflicts);
    }

    // run algorithm
    let weights: Vec<_> = sorted_candidates.iter().copied().map(|(_, w)| w).collect();
    let result = max_weight_independent_set(&weights, &conflicts);

    // map indices back
    let old_indices: Vec<_> = result
        .selected_indices
        .into_iter()
        .map(|new_index| sorted_candidates[new_index].0)
        .collect();

    MwisResult {
        max_weight: result.max_weight,
        selected_indices: old_indices,
    }
}

struct MwisSolver<'a, W> {
    weights: &'a [W],
    conflicts: &'a [Vec<bool>],
    max_potential: Vec<W>,

    best_weight: W,
    best_selection: Vec<usize>,
}

impl<'a, W: Copy + Default + Add<Output = W>> MwisSolver<'a, W> {
    pub fn new(weights: &'a [W], conflicts: &'a [Vec<bool>]) -> Self {
        let max_potential = calculate_suffix_sums(weights);
        let best_weight = W::default();
        let best_selection = vec![];
        Self {
            weights,
            conflicts,
            max_potential,
            best_weight,
            best_selection,
        }
    }

    pub fn solve(
        &mut self,
        current_index: usize,
        current_available: Vec<bool>,
        current_weight: W,
        current_selection: Vec<usize>,
    ) {
        todo!()
    }

    pub fn result(&self) -> MwisResult<W> {
        MwisResult {
            max_weight: self.best_weight,
            selected_indices: self.best_selection.clone(),
        }
    }
}

fn calculate_suffix_sums<W>(weights: &[W]) -> Vec<W>
where
    W: Copy + Default + Add<Output = W>,
{
    let n = weights.len();
    let mut suffix = vec![W::default(); n + 1];

    for i in (0..n).rev() {
        suffix[i] = weights[i] + suffix[i + 1];
    }

    suffix
}
