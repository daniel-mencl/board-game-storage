use std::collections::{HashMap, HashSet};

use crate::boards::{HexBoard, HexCoordinates};
use crate::algorithms::max_weight_matching;

use super::{
    CascadiaAnimal, CascadiaBoard, CascadiaScoringCard, CascadiaTile, map_count_to_points,
};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[enum_dispatch(CascadiaScoringCard)]
#[derive(Clone, Serialize, Deserialize)]
pub enum CascadiaHawkScoringCard {
    A(CascadiaHawkA),
    B(CascadiaHawkB),
    C(CascadiaHawkC),
    D(CascadiaHawkD),
    E(CascadiaHawkE),
    F(CascadiaHawkF),
    G(CascadiaHawkG),
}

impl Default for CascadiaHawkScoringCard {
    fn default() -> Self {
        Self::A(CascadiaHawkA)
    }
}

fn hawk_positions(animals: &HexBoard<CascadiaAnimal>) -> impl IntoIterator<Item = HexCoordinates> {
    animals
        .as_ref()
        .iter()
        .filter(|(_, animal)| **animal == CascadiaAnimal::Hawk)
        .map(|(coord, _)| *coord)
}

fn hawk_is_alone(hawks: &HashSet<HexCoordinates>, hawk: HexCoordinates) -> bool {
    hawks
        .iter()
        .all(|other| HexCoordinates::distance(hawk, *other) != 1)
}

fn line_of_sight(
    hawks: &HashSet<HexCoordinates>,
    hawk: HexCoordinates,
    other: HexCoordinates,
) -> Option<(HexCoordinates, u8)> {
    // returns (direction from a to b, distance) if tiles are in a line
    let diff = other - hawk;
    let distance = HexCoordinates::diff_size(diff) as i8;
    if distance == 0 {
        return None;
    }
    if diff.r % distance == 0 || diff.q % distance == 0 {
        let direction = HexCoordinates {
            r: diff.r / distance,
            q: diff.q / distance,
        };
        for d in 1..distance {
            let coord = hawk + diff * d;
            if hawks.contains(&coord) {
                return None;
            }
        }
        Some((direction, distance as u8))
    } else {
        None
    }
}

fn can_see(hawks: &HashSet<HexCoordinates>, hawk: HexCoordinates, other: HexCoordinates) -> bool {
    line_of_sight(hawks, hawk, other).is_some()
}

fn under_line_of_sight(
    source: HexCoordinates,
    direction: HexCoordinates,
    distance: u8,
) -> impl IntoIterator<Item = HexCoordinates> {
    (1..distance)
        .into_iter()
        .map(move |mult| source + direction * mult)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkA;
impl CascadiaHawkA {
    const POINT_MAP: [u16; 8] = [2, 5, 8, 11, 14, 18, 22, 26];

    fn points(size: usize) -> u16 {
        map_count_to_points(size, 1, &Self::POINT_MAP, false)
    }
}
impl CascadiaScoringCard for CascadiaHawkA {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let hawks: HashSet<_> = hawk_positions(animals).into_iter().collect();
        let scoring_hawks = hawks
            .iter()
            .filter(|&&hawk| hawk_is_alone(&hawks, hawk))
            .count();
        Self::points(scoring_hawks)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkB;
impl CascadiaHawkB {
    const POINT_MAP: [u16; 7] = [5, 9, 12, 16, 20, 24, 28];

    fn can_see_another_hawk(hawks: &HashSet<HexCoordinates>, hawk: HexCoordinates) -> bool {
        hawks.iter().any(|&other| can_see(hawks, hawk, other))
    }

    fn points(size: usize) -> u16 {
        map_count_to_points(size, 2, &Self::POINT_MAP, false)
    }
}
impl CascadiaScoringCard for CascadiaHawkB {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let hawks: HashSet<_> = hawk_positions(animals).into_iter().collect();
        let scoring_hawks = hawks
            .iter()
            .filter(|&&hawk| hawk_is_alone(&hawks, hawk))
            .filter(|&&hawk| Self::can_see_another_hawk(&hawks, hawk))
            .count();
        Self::points(scoring_hawks)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkC;
impl CascadiaScoringCard for CascadiaHawkC {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let hawks: HashSet<_> = hawk_positions(animals).into_iter().collect();
        let mut count = 0;

        for (index_a, &hawk_a) in hawks.iter().enumerate() {
            for (index_b, &hawk_b) in hawks.iter().enumerate() {
                if index_b <= index_a {
                    continue;
                }
                if let Some((_, dist)) = line_of_sight(&hawks, hawk_a, hawk_b) {
                    if dist > 1 {
                        count += 1;
                    }
                }
            }
        }

        3 * count as u16
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkD;
impl CascadiaHawkD {
    const POINT_MAP: [u16; 3] = [4, 7, 9];

    fn points(size: usize) -> u16 {
        map_count_to_points(size, 1, &Self::POINT_MAP, false)
    }

    fn evaluate_hawk_pair(
        animals: &HexBoard<CascadiaAnimal>,
        hawks: &HashSet<HexCoordinates>,
        hawk_a: HexCoordinates,
        hawk_b: HexCoordinates,
    ) -> Option<u16> {
        let (direction, distance) = line_of_sight(hawks, hawk_a, hawk_b)?;
        let animal_types_under: HashSet<_> = under_line_of_sight(hawk_a, direction, distance)
            .into_iter()
            .filter_map(|coord| animals.as_ref().get(&coord))
            .collect();
        Some(Self::points(animal_types_under.len()))
    }
}
impl CascadiaScoringCard for CascadiaHawkD {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let hawk_set: HashSet<_> = hawk_positions(animals).into_iter().collect();
        let hawk_vec: Vec<_> = hawk_positions(animals).into_iter().collect();
        let n = hawk_set.len();

        let mut pairs: Vec<(usize, usize, u16)> = Vec::new();

        for i in 0..n {
            let hawk_a = hawk_vec[i];
            for j in (i+1)..n {
                let hawk_b = hawk_vec[j];
                if let Some(points) = Self::evaluate_hawk_pair(animals, &hawk_set, hawk_a, hawk_b) {
                    pairs.push((i, j, points));
                }
            }
        }

        max_weight_matching(&pairs).max_weight
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkE;
impl CascadiaScoringCard for CascadiaHawkE {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let hawks: HashSet<_> = hawk_positions(animals).into_iter().collect();
        let mut love_map: HashMap<HexCoordinates, HexCoordinates> = HashMap::new();

        for &hawk in &hawks {
            let mut single: Option<HexCoordinates> = None;
            for &other in &hawks {
                if hawk == other {
                    continue;
                }

                if let Some((other, dist)) = line_of_sight(&hawks, hawk, other) {
                    if dist == 1 {
                        continue;
                    }

                    if single.is_none() {
                        single = Some(other)
                    } else {
                        single = None;
                        break;
                    }
                }
            }

            if let Some(other) = single {
                love_map.insert(hawk, other);
            }
        }

        let mut count = 0;
        for (_, &other) in &love_map {
            if love_map.contains_key(&other) {
                count += 1;
            }
        }

        7 * (count / 2)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkF;
impl CascadiaHawkF {
    const POINT_MAP: [u16; 7] = [2, 5, 8, 10, 16, 18, 25];

    fn points(size: usize) -> u16 {
        map_count_to_points(size, 1, &Self::POINT_MAP, false)
    }

    fn can_see_fox(
        hawks: &HashSet<HexCoordinates>,
        hawk: HexCoordinates,
        fox: HexCoordinates,
    ) -> bool {
        line_of_sight(hawks, hawk, fox).is_some_and(|(_, dist)| dist > 1)
    }

    fn can_see_any_fox(
        hawks: &HashSet<HexCoordinates>,
        foxes: &[HexCoordinates],
        hawk: HexCoordinates,
    ) -> bool {
        foxes.iter().any(|&fox| Self::can_see_fox(hawks, hawk, fox))
    }
}
impl CascadiaScoringCard for CascadiaHawkF {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let hawks: HashSet<_> = hawk_positions(animals).into_iter().collect();
        let foxes: Vec<_> = animals
            .as_ref()
            .iter()
            .filter(|(_, animal)| **animal == CascadiaAnimal::Fox)
            .map(|(coord, _)| *coord)
            .collect();

        let scoring_hawks = hawks
            .iter()
            .filter(|&&hawk| Self::can_see_any_fox(&hawks, &foxes, hawk))
            .count();

        Self::points(scoring_hawks)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkG;
impl CascadiaHawkG {
    const POINT_MAP: [u16; 4] = [7, 14, 22, 30];

    fn points(size: usize) -> u16 {
        map_count_to_points(size, 1, &Self::POINT_MAP, false)
    }

    fn habitats_between(
        board: &HashMap<HexCoordinates, CascadiaTile>,
        hawks: &HashSet<HexCoordinates>,
        hawk_a: HexCoordinates,
        hawk_b: HexCoordinates,
    ) -> usize {
        if let Some((direction, distance)) = line_of_sight(hawks, hawk_a, hawk_b) {
            let habitats: HashSet<_> = under_line_of_sight(hawk_a, direction, distance)
                .into_iter()
                .filter_map(|coord| board.get(&coord))
                .flat_map(|tile| tile.habitats)
                .collect();
            habitats.len()
        } else {
            0
        }
    }
}
impl CascadiaScoringCard for CascadiaHawkG {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let hawk_set: HashSet<_> = hawk_positions(animals).into_iter().collect();
        let hawk_vec: Vec<_> = hawk_positions(animals).into_iter().collect();
        let n = hawk_set.len();

        let mut separated_pairs: Vec<(usize, usize, usize)> = Vec::new();
        let board_map = board.as_ref().as_ref();

        for i in 0..n {
            let hawk_a = hawk_vec[i];
            for j in (i+1)..n {
                let hawk_b = hawk_vec[j];
                if Self::habitats_between(board_map, &hawk_set, hawk_a, hawk_b) >= 3 {
                    separated_pairs.push((i, j, 1));
                }
            }
        }

        let pairs = max_weight_matching(&separated_pairs).max_weight;
        Self::points(pairs)
    }
}
