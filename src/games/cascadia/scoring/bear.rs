use std::collections::HashSet;

use crate::boards::{HexBoard, HexCoordinates};

use super::{
    CascadiaAnimal, CascadiaBoard, CascadiaHabitat, CascadiaScoringCard, map_count_to_points,
};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[enum_dispatch(CascadiaScoringCard)]
#[derive(Clone, Serialize, Deserialize)]
pub enum CascadiaBearScoringCard {
    A(CascadiaBearA),
    B(CascadiaBearB),
    C(CascadiaBearC),
    D(CascadiaBearD),
    E(CascadiaBearE),
    F(CascadiaBearF),
    G(CascadiaBearG),
}

impl Default for CascadiaBearScoringCard {
    fn default() -> Self {
        Self::A(CascadiaBearA)
    }
}

fn bear_groups(animals: &HexBoard<CascadiaAnimal>) -> Vec<HashSet<HexCoordinates>> {
    animals
        .to_unweighted_graph(
            |_, &animal| animal == CascadiaAnimal::Bear,
            |coord, _| animals.neighbors(coord),
        )
        .components()
}

fn bear_group_sizes(animals: &HexBoard<CascadiaAnimal>) -> impl IntoIterator<Item = usize> {
    bear_groups(animals)
        .into_iter()
        .map(|component| component.len())
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearA;
impl CascadiaBearA {
    const POINT_MAP: [u16; 4] = [4, 11, 19, 27];

    fn pair_points(pairs: usize) -> u16 {
        map_count_to_points(pairs, 1, &Self::POINT_MAP, false)
    }
}
impl CascadiaScoringCard for CascadiaBearA {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let pairs = bear_group_sizes(animals)
            .into_iter()
            .filter(|size| *size == 2)
            .count();

        Self::pair_points(pairs)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearB;
impl CascadiaScoringCard for CascadiaBearB {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        10 * bear_group_sizes(animals)
            .into_iter()
            .filter(|size| *size == 3)
            .count() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearC;
impl CascadiaBearC {
    const POINT_MAP: [u16; 3] = [2, 5, 8];

    fn group_points(size: usize) -> u16 {
        map_count_to_points(size, 1, &Self::POINT_MAP, true)
    }
}
impl CascadiaScoringCard for CascadiaBearC {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let groups: Vec<_> = bear_group_sizes(animals).into_iter().collect();
        let bonus_points = if (1..=3).into_iter().all(|size| groups.contains(&size)) {
            3
        } else {
            0
        } as u16;
        bonus_points
            + groups
                .into_iter()
                .map(|size| Self::group_points(size))
                .sum::<u16>()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearD;
impl CascadiaBearD {
    const POINT_MAP: [u16; 3] = [5, 8, 13];

    fn group_points(size: usize) -> u16 {
        map_count_to_points(size, 2, &Self::POINT_MAP, true)
    }
}
impl CascadiaScoringCard for CascadiaBearD {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        bear_group_sizes(animals)
            .into_iter()
            .map(|size| Self::group_points(size))
            .sum()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearE;
impl CascadiaScoringCard for CascadiaBearE {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        3 * bear_groups(animals)
            .into_iter()
            .filter(|group| group.len() == 1)
            .map(|group| group.into_iter().next().unwrap())
            .filter(|&bear| board.as_ref().neighbors(bear).len() != 6)
            .count() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearF;
impl CascadiaBearF {
    fn all_animals_neighboring(
        animals: &HexBoard<CascadiaAnimal>,
        bears: &[HexCoordinates],
    ) -> bool {
        let neighboring_animals: HashSet<_> = animals
            .all_neighbors(bears)
            .into_iter()
            .filter_map(|coord| animals.as_ref().get(&coord))
            .collect();

        // there should be no bears (adjacent to bear group), so len == 4 is enough
        neighboring_animals.len() == 4
    }
}

impl CascadiaScoringCard for CascadiaBearF {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        12 * bear_groups(animals)
            .into_iter()
            .filter(|group| group.len() == 3)
            .map(|group| group.into_iter().collect::<Vec<_>>())
            .filter(|group| Self::all_animals_neighboring(animals, group))
            .count() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearG;
impl CascadiaBearG {
    const POINT_MAP: [u16; 4] = [5, 12, 20, 29];

    fn group_points(size: usize) -> u16 {
        map_count_to_points(size, 1, &Self::POINT_MAP, false)
    }
}
impl CascadiaScoringCard for CascadiaBearG {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let forest_pairs = bear_groups(animals)
            .into_iter()
            .filter(|group| group.len() == 2)
            .filter(|group| {
                group.iter().any(|coord| {
                    board
                        .as_ref()
                        .as_ref()
                        .get(coord)
                        .is_some_and(|tile| tile.habitats.contains(&CascadiaHabitat::Forest))
                })
            })
            .count();

        Self::group_points(forest_pairs)
    }
}
