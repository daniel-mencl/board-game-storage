use crate::boards::HexBoard;

use super::{CascadiaAnimal, CascadiaBoard, CascadiaScoringCard};
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

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearA;
impl CascadiaScoringCard for CascadiaBearA {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearB;
impl CascadiaScoringCard for CascadiaBearB {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearC;
impl CascadiaScoringCard for CascadiaBearC {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearD;
impl CascadiaScoringCard for CascadiaBearD {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearE;
impl CascadiaScoringCard for CascadiaBearE {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearF;
impl CascadiaScoringCard for CascadiaBearF {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CascadiaBearG;
impl CascadiaScoringCard for CascadiaBearG {
    fn score(&self, board: &CascadiaBoard, animals: &HexBoard<CascadiaAnimal>) -> u16 {
        todo!()
    }
}
