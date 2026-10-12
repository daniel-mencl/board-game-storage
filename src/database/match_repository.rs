use super::{Player, RepoError};
use crate::Game;

pub struct MatchParticipant {
    player_id: i64,
    seat_order: usize,
    score: i16,
    is_winner: bool,
}

pub struct Match {
    id: i64,
    game_id: String,
    state_json: Option<String>,
    score_json: Option<String>,
    played_at: String,
}

pub struct NewMatch {
    game_id: String,
    state_json: Option<String>,
    score_json: Option<String>,
    participants: Vec<MatchParticipant>,
}

pub struct MatchDetails {
    match_data: Match,
    game: Game,
    participants: Vec<(Player, MatchParticipant)>,
}

pub trait MatchRepository {
    fn create(&self, new_match: &NewMatch) -> Result<MatchDetails, RepoError>;
    fn get_by_id(&self, id: i64) -> Result<Option<MatchDetails>, RepoError>;
    fn list_by_game(&self, game_id: &str) -> Result<Vec<MatchDetails>, RepoError>;
    fn delete(&self, id: i64) -> Result<(), RepoError>;
}
