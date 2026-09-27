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
    scores: Vec<PlayerScore>,
}

impl Score {
    pub fn new(scores: Vec<PlayerScore>) -> Score {
        Score { scores }
    }
}

pub trait ScoreInto<T> {
    fn score(&self) -> T;
}

pub trait Validate {
    fn validate(&self) -> Result<(), Vec<String>>;
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
}
