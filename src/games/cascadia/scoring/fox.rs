use std::collections::{HashMap, HashSet};

use crate::boards::{HexBoard, HexCoordinates};

use super::{
    CascadiaAnimal, CascadiaBoard, CascadiaHabitat, CascadiaScoringCard, CascadiaTile,
    map_count_to_points,
};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[enum_dispatch(CascadiaScoringCard)]
#[derive(Clone, Serialize, Deserialize)]
pub enum CascadiaFoxScoringCard {
    A(CascadiaFoxA),
    B(CascadiaFoxB),
    C(CascadiaFoxC),
    D(CascadiaFoxD),
    E(CascadiaFoxE),
    F(CascadiaFoxF),
    G(CascadiaFoxG),
}

impl Default for CascadiaFoxScoringCard {
    fn default() -> Self {
        Self::A(CascadiaFoxA)
    }
}

fn fox_positions(animals: &HexBoard<CascadiaAnimal>) -> impl IntoIterator<Item = HexCoordinates> {
    animals
        .as_ref()
        .iter()
        .filter(|(_, animal)| **animal == CascadiaAnimal::Fox)
        .map(|(coord, _)| *coord)
}

fn count_animals(
    animals: &HexBoard<CascadiaAnimal>,
    coords: impl IntoIterator<Item = HexCoordinates>,
) -> HashMap<CascadiaAnimal, usize> {
    // non represented animals do not appear
    let mut animal_counts: HashMap<CascadiaAnimal, usize> = HashMap::new();
    for &animal in coords
        .into_iter()
        .filter_map(|other| animals.as_ref().get(&other))
    {
        let entry = animal_counts.entry(animal).or_insert(0);
        *entry += 1
    }
    animal_counts
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxA;
impl CascadiaFoxA {
    fn animals_around(animals: &HexBoard<CascadiaAnimal>, fox: HexCoordinates) -> usize {
        let neighbors = animals.neighbors(fox);
        count_animals(animals, neighbors).len()
    }
}
impl CascadiaScoringCard for CascadiaFoxA {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        fox_positions(animals)
            .into_iter()
            .map(|fox| Self::animals_around(animals, fox))
            .sum::<usize>() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxB;
impl CascadiaFoxB {
    const POINT_MAP: [u16; 3] = [3, 5, 7];

    fn pairs_around(animals: &HexBoard<CascadiaAnimal>, fox: HexCoordinates) -> usize {
        count_animals(animals, animals.neighbors(fox))
            .into_iter()
            .filter(|(animal, count)| *animal != CascadiaAnimal::Fox && *count >= 2)
            .count()
    }

    fn points(animals: &HexBoard<CascadiaAnimal>, fox: HexCoordinates) -> u16 {
        map_count_to_points(Self::pairs_around(animals, fox), 1, &Self::POINT_MAP, false)
    }
}
impl CascadiaScoringCard for CascadiaFoxB {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        fox_positions(animals)
            .into_iter()
            .map(|fox| Self::points(animals, fox))
            .sum()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxC;
impl CascadiaFoxC {
    fn largest_group_around(animals: &HexBoard<CascadiaAnimal>, fox: HexCoordinates) -> usize {
        count_animals(animals, animals.neighbors(fox))
            .into_iter()
            .filter(|(animal, _)| *animal != CascadiaAnimal::Fox)
            .map(|(_, count)| count)
            .max()
            .unwrap_or(0)
    }
}
impl CascadiaScoringCard for CascadiaFoxC {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        fox_positions(animals)
            .into_iter()
            .map(|fox| Self::largest_group_around(animals, fox))
            .sum::<usize>() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxD;
impl CascadiaFoxD {
    const POINT_MAP: [u16; 4] = [5, 7, 9, 11];

    fn points(
        animals: &HexBoard<CascadiaAnimal>,
        fox_a: HexCoordinates,
        fox_b: HexCoordinates,
    ) -> u16 {
        let pairs = count_animals(animals, animals.all_neighbors(&[fox_a, fox_b]))
            .into_iter()
            .filter(|(animal, count)| *animal != CascadiaAnimal::Fox && *count >= 2)
            .count();
        map_count_to_points(pairs, 1, &Self::POINT_MAP, true)
    }
}
impl CascadiaScoringCard for CascadiaFoxD {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let mut points = 0;
        let foxes: Vec<_> = fox_positions(animals).into_iter().collect();

        for i in 0..foxes.len() {
            let fox_a = foxes[i];
            for j in (i + 1)..foxes.len() {
                let fox_b = foxes[j];
                if HexCoordinates::distance(fox_a, fox_b) != 1 {
                    continue;
                }
                points += Self::points(animals, fox_a, fox_b);
            }
        }

        points
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxE;
impl CascadiaFoxE {
    const POINT_MAP: [u16; 3] = [2, 2, 4];

    fn animals_around(animals: &HexBoard<CascadiaAnimal>, fox: HexCoordinates) -> u16 {
        let count = animals.neighbors(fox).len();
        map_count_to_points(count, 3, &Self::POINT_MAP, true)
    }
}
impl CascadiaScoringCard for CascadiaFoxE {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        fox_positions(animals)
            .into_iter()
            .map(|fox| Self::animals_around(animals, fox))
            .sum()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxF;
impl CascadiaFoxF {
    fn animal_kinds_around(animals: &HexBoard<CascadiaAnimal>, fox: HexCoordinates) -> usize {
        count_animals(animals, animals.neighbors(fox)).len()
    }
}
impl CascadiaScoringCard for CascadiaFoxF {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        4 * fox_positions(animals)
            .into_iter()
            .filter(|&fox| Self::animal_kinds_around(animals, fox) == 2)
            .count() as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxG;
impl CascadiaFoxG {
    fn habitats_around(
        board: &HashMap<HexCoordinates, CascadiaTile>,
        fox: HexCoordinates,
    ) -> usize {
        let mut habitats: HashSet<CascadiaHabitat> = HashSet::new();

        for (index, &direction) in HexCoordinates::CLOCKWISE_OFFSETS.iter().enumerate() {
            if let Some(other) = board.get(&(fox + direction)) {
                habitats.insert(other.habitats[HexCoordinates::opposite_index(index)]);
            }
        }

        habitats.len()
    }
}
impl CascadiaScoringCard for CascadiaFoxG {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        fox_positions(animals)
            .into_iter()
            .map(|fox| Self::habitats_around(board.as_ref().as_ref(), fox))
            .sum::<usize>() as u16
    }
}
