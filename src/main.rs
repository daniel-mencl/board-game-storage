use board_game_storage::{
    App, GameRepository, MatchRepository, PlayerRepository, SqliteMatchRepository,
    SqlitePlayerRepository, establish_connection,
};
use dioxus::desktop::Config;

use std::sync::Arc;

fn main() {
    let connection = establish_connection("board_game_storage.db").expect("Should connect to db");

    let player_repository = SqlitePlayerRepository::new(Arc::clone(&connection));
    let player_repository: Arc<dyn PlayerRepository> = Arc::new(player_repository);

    let match_repository =
        SqliteMatchRepository::new(Arc::clone(&connection), GameRepository::default());
    let match_repository: Arc<dyn MatchRepository> = Arc::new(match_repository);

    let config = Config::new().with_menu(None);

    dioxus::LaunchBuilder::new()
        .with_cfg(config)
        .with_context(player_repository)
        .with_context(match_repository)
        .launch(App);
}
