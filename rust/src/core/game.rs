use serde::{Serialize, de::DeserializeOwned};

pub struct PlayerScore {
    score: i16,
    is_winner: bool,
}

impl PlayerScore {
    pub fn new(score: i16, is_winner: bool) -> PlayerScore {
        PlayerScore { score, is_winner }
    }
}

pub struct Score {
    pub(crate) scores: Vec<PlayerScore>,
}

pub trait ScoreInto<T> {
    fn score(&self) -> T;
}

pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}

pub trait Game: Send + Sync + 'static {
    type ScoreBreakdown: ScoreInto<Score>
        + Validate
        + Serialize
        + DeserializeOwned
        + Clone
        + Send
        + Sync
        + 'static;
    type State: ScoreInto<Self::ScoreBreakdown>
        + Validate
        + Serialize
        + DeserializeOwned
        + Clone
        + Send
        + Sync
        + 'static;

    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;

    fn validate_state(state: &Self::State) -> Result<(), String> {
        state.validate()
    }
    fn validate_breakdown(breakdown: &Self::ScoreBreakdown) -> Result<(), String> {
        breakdown.validate()
    }

    fn score_state(state: &Self::State) -> Self::ScoreBreakdown {
        state.score()
    }
    fn score_breakdown(breakdown: &Self::ScoreBreakdown) -> Score {
        breakdown.score()
    }
}
