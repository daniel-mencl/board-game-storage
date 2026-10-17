use serde::{Serialize, de::DeserializeOwned};

#[derive(Copy, Clone)]
pub struct PlayerScore {
    pub score: Option<i16>,
    pub is_winner: bool,
}

impl PlayerScore {
    pub fn scored(score: i16, is_winner: bool) -> PlayerScore {
        PlayerScore {
            score: Some(score),
            is_winner,
        }
    }

    pub fn unscored(is_winner: bool) -> PlayerScore {
        PlayerScore {
            score: None,
            is_winner,
        }
    }
}

pub struct Score {
    pub scores: Vec<PlayerScore>,
}

impl Score {
    pub fn new(scores: Vec<PlayerScore>) -> Score {
        Score { scores }
    }
}

pub trait ScoreInto<T> {
    fn score(&self) -> T;
}

pub struct EvaluatedMatch {
    state: Option<String>,
    breakdown: Option<String>,
    score: Score,
}

pub trait GameEngine: Send + Sync + 'static {
    type ScoreBreakdown: ScoreInto<Score>
        + From<usize>
        + Serialize
        + DeserializeOwned
        + Clone
        + Send
        + Sync
        + 'static;
    type State: ScoreInto<Self::ScoreBreakdown>
        + From<usize>
        + Serialize
        + DeserializeOwned
        + Clone
        + Send
        + Sync
        + 'static;

    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;

    fn default_state(&self, player_count: usize) -> Self::State {
        Self::State::from(player_count)
    }

    fn default_score(&self, player_count: usize) -> Self::ScoreBreakdown {
        Self::ScoreBreakdown::from(player_count)
    }

    fn load_state(&self, json: &str) -> Result<Self::State, serde_json::Error> {
        serde_json::from_str(json)
    }

    fn load_score(&self, json: &str) -> Result<Self::ScoreBreakdown, serde_json::Error> {
        serde_json::from_str(json)
    }

    fn evaluate_state(&self, state: &Self::State) -> EvaluatedMatch {
        let breakdown = state.score();
        let score = breakdown.score();

        let state = serde_json::to_string(state).expect("State should be serializable");
        let breakdown =
            serde_json::to_string(&breakdown).expect("Breakdown should be serializable");
        EvaluatedMatch {
            state: Some(state),
            breakdown: Some(breakdown),
            score,
        }
    }

    fn evaluate_score(&self, breakdown: &Self::ScoreBreakdown) -> EvaluatedMatch {
        let score = breakdown.score();

        let breakdown = serde_json::to_string(breakdown).expect("Breakdown should be serializable");
        EvaluatedMatch {
            state: None,
            breakdown: Some(breakdown),
            score,
        }
    }
}
