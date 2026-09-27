use crate::boards::HexBoard;

use super::{CascadiaAnimal, CascadiaBoard, CascadiaScoringCard};
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

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonA;
impl CascadiaScoringCard for CascadiaSalmonA {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonB;
impl CascadiaScoringCard for CascadiaSalmonB {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonC;
impl CascadiaScoringCard for CascadiaSalmonC {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonD;
impl CascadiaScoringCard for CascadiaSalmonD {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonE;
impl CascadiaScoringCard for CascadiaSalmonE {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonF;
impl CascadiaScoringCard for CascadiaSalmonF {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaSalmonG;
impl CascadiaScoringCard for CascadiaSalmonG {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}
