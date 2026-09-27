use crate::boards::HexBoard;

use super::{CascadiaAnimal, CascadiaBoard, CascadiaScoringCard};
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

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkA;
impl CascadiaScoringCard for CascadiaHawkA {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkB;
impl CascadiaScoringCard for CascadiaHawkB {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkC;
impl CascadiaScoringCard for CascadiaHawkC {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkD;
impl CascadiaScoringCard for CascadiaHawkD {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkE;
impl CascadiaScoringCard for CascadiaHawkE {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkF;
impl CascadiaScoringCard for CascadiaHawkF {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaHawkG;
impl CascadiaScoringCard for CascadiaHawkG {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}
