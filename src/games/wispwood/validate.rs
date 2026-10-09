use super::WispwoodState;
use super::score::WispwoodScoreBreakdown;
use crate::core::Validate;

impl Validate for WispwoodScoreBreakdown {
    fn validate(&self) -> Result<(), Vec<String>> {
        Ok(())
        // TODO
    }
}

impl Validate for WispwoodState {
    fn validate(&self) -> Result<(), Vec<String>> {
        Ok(())
        // TODO
    }
}
