use serde::{Deserialize, Serialize};

use super::{WispwoodPlayer, WispwoodScoringCards, WispwoodState};
use crate::core::{PlayerScore, Score, ScoreInto};

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodBoardScore {
    jacks: u16,
    witches: u16,
    orbs: u16,
    hearts: u16,
    trees: u16,
    bonus: u16,
    pub total: u16,
}

impl WispwoodBoardScore {
    pub fn new(
        jacks: u16,
        witches: u16,
        orbs: u16,
        hearts: u16,
        trees: u16,
        bonus: u16,
    ) -> WispwoodBoardScore {
        let total = jacks + witches + orbs + hearts + trees + bonus;
        WispwoodBoardScore {
            jacks,
            witches,
            orbs,
            hearts,
            trees,
            bonus,
            total,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodPlayerScore {
    rounds: [WispwoodBoardScore; 3],
}

impl WispwoodPlayerScore {
    pub fn total(&self) -> u16 {
        self.rounds.iter().map(|round| round.total).sum()
    }
}

impl WispwoodPlayer {
    pub fn score(&self, scoring_cards: &WispwoodScoringCards) -> WispwoodPlayerScore {
        let [b0, b1, b2] = &self.rounds;
        WispwoodPlayerScore {
            rounds: [
                scoring_cards.score(b0),
                scoring_cards.score(b1),
                scoring_cards.score(b2),
            ],
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WispwoodScoreBreakdown {
    players: Vec<WispwoodPlayerScore>,
}

impl ScoreInto<Score> for WispwoodScoreBreakdown {
    fn score(&self) -> Score {
        let scores = self.players.iter().map(|player| player.total()).into_iter();
        let max = scores.clone().max().expect("Should have 1+ players");
        let scores = scores
            .clone()
            .map(|score| PlayerScore::new(score as i16, score == max))
            .collect();
        Score::new(scores)
    }
}

impl ScoreInto<WispwoodScoreBreakdown> for WispwoodState {
    fn score(&self) -> WispwoodScoreBreakdown {
        let players = self
            .players
            .iter()
            .map(|player| player.score(&self.scoring_cards))
            .collect();
        WispwoodScoreBreakdown { players }
    }
}
