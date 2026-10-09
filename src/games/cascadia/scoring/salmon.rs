use std::collections::HashSet;

use crate::boards::{HexBoard, HexCoordinates};

use super::{
    CascadiaAnimal, CascadiaBoard, CascadiaHabitat, CascadiaScoringCard, map_count_to_points,
};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[enum_dispatch(CascadiaScoringCard)]
#[derive(Clone, Serialize, Deserialize)]
pub enum CascadiaSalmonScoringCard {
    A(CascadiaSalmonA),
    B(CascadiaSalmonB),
    C(CascadiaSalmonC),
    D(CascadiaSalmonD),
    E(CascadiaSalmonE),
    F(CascadiaSalmonF),
    G(CascadiaSalmonG),
}

impl Default for CascadiaSalmonScoringCard {
    fn default() -> Self {
        Self::A(CascadiaSalmonA)
    }
}

fn correct_salmon_group(
    animals: &HexBoard<CascadiaAnimal>,
    salmon: &HashSet<HexCoordinates>,
) -> bool {
    for &coord in salmon {
        let neighbor_salmon = animals
            .neighbors(coord)
            .into_iter()
            .filter(|other| salmon.contains(other))
            .count();

        if neighbor_salmon > 2 {
            return false;
        }
    }

    true
}

fn salmon_groups(animals: &HexBoard<CascadiaAnimal>) -> Vec<HashSet<HexCoordinates>> {
    animals
        .to_unweighted_graph(
            |_, &animal| animal == CascadiaAnimal::Salmon,
            |coord, _| animals.neighbors(coord),
        )
        .components()
        .into_iter()
        .filter(|group| correct_salmon_group(animals, group))
        .collect()
}

fn salmon_group_sizes(animals: &HexBoard<CascadiaAnimal>) -> impl IntoIterator<Item = usize> {
    salmon_groups(animals)
        .into_iter()
        .map(|component| component.len())
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonA;
impl CascadiaSalmonA {
    const POINT_MAP: [u16; 7] = [2, 5, 8, 12, 16, 20, 25];

    fn group_points(size: usize) -> u16 {
        map_count_to_points(size, 1, &Self::POINT_MAP, false)
    }
}
impl CascadiaScoringCard for CascadiaSalmonA {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        salmon_group_sizes(animals)
            .into_iter()
            .map(|size| Self::group_points(size))
            .sum()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonB;
impl CascadiaSalmonB {
    const POINT_MAP: [u16; 5] = [2, 4, 9, 11, 17];

    fn group_points(size: usize) -> u16 {
        map_count_to_points(size, 1, &Self::POINT_MAP, false)
    }
}
impl CascadiaScoringCard for CascadiaSalmonB {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        salmon_group_sizes(animals)
            .into_iter()
            .map(|size| Self::group_points(size))
            .sum()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonC;
impl CascadiaSalmonC {
    const POINT_MAP: [u16; 3] = [10, 12, 15];

    fn group_points(size: usize) -> u16 {
        map_count_to_points(size, 3, &Self::POINT_MAP, false)
    }
}
impl CascadiaScoringCard for CascadiaSalmonC {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        salmon_group_sizes(animals)
            .into_iter()
            .map(|size| Self::group_points(size))
            .sum()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonD;
impl CascadiaSalmonD {
    fn count_animals_around(
        animals: &HexBoard<CascadiaAnimal>,
        salmon: Vec<HexCoordinates>,
    ) -> usize {
        animals.all_neighbors(&salmon).into_iter().count() + salmon.len()
    }
}
impl CascadiaScoringCard for CascadiaSalmonD {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        salmon_groups(animals)
            .into_iter()
            .map(|group| Self::count_animals_around(animals, group.into_iter().collect()))
            .sum::<usize>() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonE;
impl CascadiaSalmonE {
    const POINT_MAP: [u16; 6] = [2, 5, 9, 13, 17, 21];

    fn group_points(size: usize) -> u16 {
        map_count_to_points(size, 1, &Self::POINT_MAP, false)
    }

    fn find_endpoint(salmon: &HashSet<HexCoordinates>) -> Option<HexCoordinates> {
        let mut endpoints = Vec::with_capacity(2);
        for &coord in salmon {
            let neighbor_count = HexCoordinates::CLOCKWISE_OFFSETS
                    .iter()
                    .filter(|&&offset| salmon.contains(&(coord + offset)))
                    .count();

            match neighbor_count {
                1 => endpoints.push(coord),
                2 => {}
                _ => return None // shouldn't happen in correct salmon group
            }
        }

        if endpoints.len() == 2 {
            Some(endpoints[0])
        } else {
            None
        }
    }

    fn correct_shape(salmon: &HashSet<HexCoordinates>) -> bool {
        let n = salmon.len();
        if n <= 2 {
            return true;
        }

        let Some(first) = Self::find_endpoint(salmon) else { return false; };

        let second = HexCoordinates::CLOCKWISE_OFFSETS
            .iter()
            .map(|&offset| first + offset)
            .find(|neighbor| salmon.contains(neighbor))
            .expect("Is a path with len > 2");

        let third = HexCoordinates::CLOCKWISE_OFFSETS
            .iter()
            .map(|&offset| second + offset)
            .find(|neighbor| salmon.contains(neighbor) && *neighbor != first)
            .expect("Is a path with len > 2");

        if (second - first) == (third - second) {
            return false;
        }

        let delta = third - first;
        // even salmon will be separated by delta, odd ones as well
        for i in 3..n {
            let start = if i % 2 == 0 { first } else { second };
            let current = start + delta * (i / 2) as u8;
            if !salmon.contains(&current) {
                return false;
            }
        }

        true
    }
}
impl CascadiaScoringCard for CascadiaSalmonE {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        salmon_groups(animals)
            .into_iter()
            .filter(|group| Self::correct_shape(group))
            .map(|group| Self::group_points(group.len()))
            .sum()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonF;
impl CascadiaSalmonF {
    const POINT_MAP: [u16; 7] = [1, 3, 7, 10, 13, 17, 22];

    fn salmon_points(size: usize) -> u16 {
        map_count_to_points(size, 1, &Self::POINT_MAP, false)
    }

    fn group_points(animals: &HexBoard<CascadiaAnimal>, salmon: HashSet<HexCoordinates>) -> u16 {
        let salmon: Vec<_> = salmon.into_iter().collect();

        let bears_around = animals
            .all_neighbors(&salmon)
            .into_iter()
            .filter(|other| animals.as_ref().get(&other) == Some(&CascadiaAnimal::Bear))
            .count();

        Self::salmon_points(salmon.len()) + bears_around as u16
    }
}
impl CascadiaScoringCard for CascadiaSalmonF {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        salmon_groups(animals)
            .into_iter()
            .map(|group| Self::group_points(animals, group))
            .sum()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonG;
impl CascadiaSalmonG {
    const POINT_MAP: [u16; 5] = [2, 4, 7, 10, 14];

    fn salmon_points(size: usize) -> u16 {
        map_count_to_points(size, 1, &Self::POINT_MAP, false)
    }

    fn is_river_salmon(board: &CascadiaBoard, salmon: HexCoordinates) -> bool {
        board
            .as_ref()
            .as_ref()
            .get(&salmon)
            .is_some_and(|tile| tile.habitats.contains(&CascadiaHabitat::River))
    }

    fn group_points(board: &CascadiaBoard, salmon: HashSet<HexCoordinates>) -> u16 {
        let river_salmon = salmon
            .iter()
            .filter(|&&coord| Self::is_river_salmon(board, coord))
            .count();

        Self::salmon_points(salmon.len()) + river_salmon as u16
    }
}
impl CascadiaScoringCard for CascadiaSalmonG {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        salmon_groups(animals)
            .into_iter()
            .map(|group| Self::group_points(board, group))
            .sum()
    }
}
