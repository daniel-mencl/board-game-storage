use crate::algorithms::max_weight_set_packing;
use crate::boards::{HexBoard, HexCoordinates};

use std::collections::HashSet;

use super::{
    CascadiaAnimal, CascadiaBoard, CascadiaHabitat, CascadiaScoringCard, map_count_to_points,
};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[enum_dispatch(CascadiaScoringCard)]
#[derive(Clone, Serialize, Deserialize)]
pub enum CascadiaElkScoringCard {
    A(CascadiaElkA),
    B(CascadiaElkB),
    C(CascadiaElkC),
    D(CascadiaElkD),
    E(CascadiaElkE),
    F(CascadiaElkF),
    G(CascadiaElkG),
}

impl Default for CascadiaElkScoringCard {
    fn default() -> Self {
        Self::A(CascadiaElkA)
    }
}

fn elk_positions(animals: &HexBoard<CascadiaAnimal>) -> HashSet<HexCoordinates> {
    animals
        .as_ref()
        .iter()
        .filter(|(_, animal)| **animal == CascadiaAnimal::Elk)
        .map(|(coord, _)| *coord)
        .collect()
}

type ScoredShape = (HashSet<HexCoordinates>, u16);

fn best_elk_shape_weights(shapes: &[ScoredShape]) -> u16 {
    max_weight_set_packing(shapes).max_weight
}

fn basic_elk_group_score(size: usize) -> u16 {
    map_count_to_points(size, 1, &[2, 5, 9, 13], false)
}

fn formations(elk: &HashSet<HexCoordinates>) -> Vec<ScoredShape> {
    // start with single elk, go in main directions (add third elk below - next clockwise offset)
    let mut result = Vec::new();

    for &e in elk {
        result.push((HashSet::from([e]), basic_elk_group_score(1)));

        for direction_index in 0usize..3 {
            let mut current_shape = HashSet::from([e]);

            let second_offset = HexCoordinates::CLOCKWISE_OFFSETS[direction_index];
            let third_offset = HexCoordinates::CLOCKWISE_OFFSETS[direction_index + 1];
            let fourth_offset = second_offset + third_offset;

            for offset in [second_offset, third_offset, fourth_offset] {
                let other_elk = e + offset;
                if !elk.contains(&other_elk) {
                    break;
                }
                current_shape.insert(other_elk);
                result.push((
                    current_shape.clone(),
                    basic_elk_group_score(current_shape.len()),
                ));
            }
        }
    }

    result
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkA;
impl CascadiaElkA {
    fn lines(elk: &HashSet<HexCoordinates>) -> Vec<ScoredShape> {
        // make lines starting with elk but only in three directions - that way, the "start" is unique
        let mut result = Vec::new();
        let main_directions = &HexCoordinates::CLOCKWISE_OFFSETS[0..3];

        for &e in elk {
            result.push((HashSet::from([e]), basic_elk_group_score(1)));

            for &direction in main_directions {
                let mut current_shape = HashSet::from([e]);

                for distance in 1u8..=3 {
                    let current = e + direction * distance;
                    if !elk.contains(&current) {
                        break;
                    }

                    current_shape.insert(current);
                    result.push((
                        current_shape.clone(),
                        basic_elk_group_score(current_shape.len()),
                    ));
                }
            }
        }

        result
    }
}
impl CascadiaScoringCard for CascadiaElkA {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let elk = elk_positions(animals);
        let shapes = Self::lines(&elk);
        best_elk_shape_weights(&shapes)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkB;
impl CascadiaScoringCard for CascadiaElkB {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let elk = elk_positions(animals);
        let shapes = formations(&elk);
        best_elk_shape_weights(&shapes)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkC;
impl CascadiaElkC {
    fn points_under_8(size: usize) -> u16 {
        map_count_to_points(size, 1, &[2, 4, 7, 10, 14, 18, 23], true)
    }
    fn points(size: usize) -> u16 {
        let eights = (size / 8) as u16;
        let left = size % 8;
        eights * 28 + Self::points_under_8(left)
    }
}
impl CascadiaScoringCard for CascadiaElkC {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        animals
            .to_unweighted_graph(
                |_, &animal| animal == CascadiaAnimal::Elk,
                |coord, _| animals.neighbors(coord),
            )
            .components()
            .into_iter()
            .map(|group| Self::points(group.len()))
            .sum()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkD;
impl CascadiaElkD {
    fn points(size: usize) -> u16 {
        map_count_to_points(size, 1, &[2, 5, 8, 12, 16, 21], true)
    }

    fn circles(elk: &HashSet<HexCoordinates>) -> Vec<ScoredShape> {
        // each elk continues the circle clockwise
        let mut result = Vec::new();
        let mut seen = HashSet::new();

        for &e in elk {
            seen.insert(vec![e]);
            result.push((HashSet::from([e]), Self::points(1)));

            for direction_index in 0..6 {
                let center = e + HexCoordinates::CLOCKWISE_OFFSETS[direction_index];
                let elk_offset_index = HexCoordinates::opposite_index(direction_index);

                let mut current_shape = HashSet::from([e]);

                for step in 1..6 {
                    let next_offset_index = (elk_offset_index + step) % 6;
                    let next_coord = center + HexCoordinates::CLOCKWISE_OFFSETS[next_offset_index];

                    if !elk.contains(&next_coord) {
                        break;
                    }

                    current_shape.insert(next_coord);

                    let mut key: Vec<_> = current_shape.iter().copied().collect();
                    key.sort_unstable();

                    if seen.insert(key) {
                        result.push((current_shape.clone(), Self::points(current_shape.len())));
                    }
                }
            }
        }

        result
    }
}
impl CascadiaScoringCard for CascadiaElkD {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let elk = elk_positions(animals);
        let shapes = Self::circles(&elk);
        best_elk_shape_weights(&shapes)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkE;
impl CascadiaElkE {
    fn triple_points(count: usize) -> u16 {
        map_count_to_points(count, 1, &[10, 21, 33], false)
    }

    fn semicircles(elk: &HashSet<HexCoordinates>) -> Vec<ScoredShape> {
        let mut result = Vec::new();

        for &e in elk {
            for direction_index in 0..6 {
                let center = e + HexCoordinates::CLOCKWISE_OFFSETS[direction_index];
                let center_elk_offset_index = HexCoordinates::opposite_index(direction_index);

                let counter_clockwise_elk = center
                    + HexCoordinates::CLOCKWISE_OFFSETS[if center_elk_offset_index != 0 {
                        center_elk_offset_index - 1
                    } else {
                        5
                    }];

                let clockwise_elk =
                    center + HexCoordinates::CLOCKWISE_OFFSETS[(center_elk_offset_index + 1) % 6];

                if elk.contains(&counter_clockwise_elk) && elk.contains(&clockwise_elk) {
                    let current_shape = HashSet::from([e, counter_clockwise_elk, clockwise_elk]);
                    result.push((current_shape, 1));
                }
            }
        }

        result
    }
}
impl CascadiaScoringCard for CascadiaElkE {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let elk = elk_positions(animals);
        let shapes = Self::semicircles(&elk);
        let triples = best_elk_shape_weights(&shapes);
        Self::triple_points(triples as usize)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkF;
impl CascadiaElkF {
    fn points(animals: &HexBoard<CascadiaAnimal>, elk: &[HexCoordinates]) -> u16 {
        13 - animals
            .all_neighbors(elk)
            .into_iter()
            .filter(|coord| animals.as_ref().get(coord).is_some())
            .count() as u16
    }

    fn solo_herds(
        animals: &HexBoard<CascadiaAnimal>,
        elk: &HashSet<HexCoordinates>,
    ) -> Vec<ScoredShape> {
        let mut result = Vec::new();

        for &e in elk {
            for direction_index in 0..3 {
                let second_elk_offset = HexCoordinates::CLOCKWISE_OFFSETS[direction_index];
                let third_elk_offset = HexCoordinates::CLOCKWISE_OFFSETS[direction_index + 1];

                let second_elk = e + second_elk_offset;
                let third_elk = e + third_elk_offset;

                if elk.contains(&second_elk) && elk.contains(&third_elk) {
                    let current_shape = [e, second_elk, third_elk];
                    result.push((
                        HashSet::from(current_shape),
                        Self::points(animals, &current_shape),
                    ));
                }
            }
        }

        result
    }
}
impl CascadiaScoringCard for CascadiaElkF {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let elk = elk_positions(animals);
        let shapes = Self::solo_herds(animals, &elk);
        best_elk_shape_weights(&shapes)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkG;
impl CascadiaElkG {
    fn prairie_formation(board: &CascadiaBoard, formation: ScoredShape) -> Option<ScoredShape> {
        let is_prairie = formation
            .0
            .iter()
            .filter_map(|coord| board.as_ref().as_ref().get(&coord))
            .any(|tile| tile.habitats.contains(&CascadiaHabitat::Prairie));
        if is_prairie {
            Some((formation.0, formation.1 + 1))
        } else {
            None
        }
    }
}
impl CascadiaScoringCard for CascadiaElkG {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let elk = elk_positions(animals);
        let shapes = formations(&elk);
        let shapes: Vec<_> = shapes
            .into_iter()
            .filter_map(|shape| Self::prairie_formation(board, shape))
            .collect();
        best_elk_shape_weights(&shapes)
    }
}
