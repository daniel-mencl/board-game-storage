use serde::{Serialize, de::DeserializeOwned};

pub struct Score {
    scores: Vec<i16>,
    winners: Vec<u8>,
}

pub trait Game: Send + Sync + 'static {
    type State: Serialize + DeserializeOwned + Clone + Send + Sync + 'static;
    type ScoreBreakdown: Serialize + DeserializeOwned + Clone + Send + Sync + 'static;

    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;

    fn validate_state(state: &Self::State) -> Result<(), String>;
    fn validate_breakdown(breakdown: &Self::ScoreBreakdown) -> Result<(), String>;

    fn score_state(state: &Self::State) -> Self::ScoreBreakdown;
    fn score_breakdown(breakdown: &Self::ScoreBreakdown) -> Score;
}
