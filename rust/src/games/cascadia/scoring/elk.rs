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
    todo!()
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkA;
impl CascadiaElkA {
    fn lines(elk: &HashSet<HexCoordinates>) -> Vec<ScoredShape> {
        todo!()
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
        todo!()
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
        todo!()
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
    fn points(animals: &HexBoard<CascadiaAnimal>, elk: HashSet<HexCoordinates>) -> u16 {
        let elk: Vec<_> = elk.into_iter().collect();
        13 - animals
            .all_neighbors(&elk)
            .into_iter()
            .filter(|coord| animals.as_ref().get(coord).is_some())
            .count() as u16
    }

    fn solo_herds(elk: &HashSet<HexCoordinates>) -> Vec<ScoredShape> {
        todo!()
    }
}
impl CascadiaScoringCard for CascadiaElkF {
    fn score(&self, _board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        let elk = elk_positions(animals);
        let shapes = Self::solo_herds(&elk);
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
