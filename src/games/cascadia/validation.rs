use crate::core::Validate;

use super::{CascadiaScoreBreakdown, CascadiaState};

impl Validate for CascadiaScoreBreakdown {
    fn validate(&self) -> Result<(), Vec<String>> {
        Ok(())
        // TODO
    }
}

impl Validate for CascadiaState {
    fn validate(&self) -> Result<(), Vec<String>> {
        Ok(())
        // TODO
    }
}
