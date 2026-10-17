use std::collections::HashMap;

use crate::games::Game;

pub struct GameRepository {
    games: HashMap<String, Game>,
}

impl Default for GameRepository {
    fn default() -> Self {
        Self {
            games: Game::all_games(),
        }
    }
}

impl GameRepository {
    pub fn get_by_id(&self, id: &str) -> Option<Game> {
        self.games.get(id).copied()
    }
}
