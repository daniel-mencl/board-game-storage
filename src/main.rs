use board_game_storage::{
    App, GameRepository, MatchRepository, PlayerRepository, SqliteMatchRepository,
    SqlitePlayerRepository, establish_connection,
};
use dioxus::desktop::{Config, WindowBuilder, LogicalSize};


use std::sync::Arc;

fn main() {
    let connection = establish_connection("board_game_storage.db").expect("Should connect to db");

    let player_repository = SqlitePlayerRepository::new(Arc::clone(&connection));
    let player_repository: Arc<dyn PlayerRepository> = Arc::new(player_repository);

    let match_repository =
        SqliteMatchRepository::new(Arc::clone(&connection), GameRepository::default());
    let match_repository: Arc<dyn MatchRepository> = Arc::new(match_repository);

    let window = WindowBuilder::new()
        .with_title("Board Game Storage")
        .with_inner_size(LogicalSize::new(900.0, 650.0))
        .with_min_inner_size(LogicalSize::new(400.0, 300.0))
        .with_maximized(false)
        .with_resizable(true);

    let config = Config::new().with_menu(None).with_window(window);

    dioxus::LaunchBuilder::new()
        .with_cfg(config)
        .with_context(player_repository)
        .with_context(match_repository)
        .launch(App);
}
