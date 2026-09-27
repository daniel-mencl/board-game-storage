use crate::boards::HexBoard;

use super::{CascadiaAnimal, CascadiaBoard, CascadiaScoringCard};
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

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkA;
impl CascadiaScoringCard for CascadiaElkA {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkB;
impl CascadiaScoringCard for CascadiaElkB {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkC;
impl CascadiaScoringCard for CascadiaElkC {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkD;
impl CascadiaScoringCard for CascadiaElkD {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkE;
impl CascadiaScoringCard for CascadiaElkE {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkF;
impl CascadiaScoringCard for CascadiaElkF {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaElkG;
impl CascadiaScoringCard for CascadiaElkG {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}
