use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::{WispwoodBoard, WispwoodScoringCard};

#[enum_dispatch(WispwoodScoringCard)]
#[derive(Clone, Serialize, Deserialize)]
pub enum WispwoodTreeScoringCard {
    Largest(WispwoodTreeLargest),
    SecondLargest(WispwoodTreeSecondLargest),
    Groups(WispwoodTreeGroups),
    RowColumn(WispwoodTreeRowColumn),
    Central(WispwoodTreeCentral),
    Diagonal(WispwoodTreeDiagonal),
}

impl Default for WispwoodTreeScoringCard {
    fn default() -> Self {
        Self::Largest(WispwoodTreeLargest)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodTreeLargest;
impl WispwoodScoringCard for WispwoodTreeLargest {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodTreeSecondLargest;
impl WispwoodScoringCard for WispwoodTreeSecondLargest {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodTreeGroups;
impl WispwoodScoringCard for WispwoodTreeGroups {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodTreeRowColumn;
impl WispwoodScoringCard for WispwoodTreeRowColumn {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodTreeCentral;
impl WispwoodScoringCard for WispwoodTreeCentral {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodTreeDiagonal;
impl WispwoodScoringCard for WispwoodTreeDiagonal {
    fn score(&self, board: &WispwoodBoard) -> u16 {
        todo!()
    }
}
