use crate::boards::HexBoard;

use super::{CascadiaAnimal, CascadiaBoard, CascadiaScoringCard};
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

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxA;
impl CascadiaScoringCard for CascadiaFoxA {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxB;
impl CascadiaScoringCard for CascadiaFoxB {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxC;
impl CascadiaScoringCard for CascadiaFoxC {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxD;
impl CascadiaScoringCard for CascadiaFoxD {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxE;
impl CascadiaScoringCard for CascadiaFoxE {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxF;
impl CascadiaScoringCard for CascadiaFoxF {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaFoxG;
impl CascadiaScoringCard for CascadiaFoxG {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}
