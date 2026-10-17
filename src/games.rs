mod cascadia;
mod wispwood;

use std::collections::HashMap;

use cascadia::Cascadia;
use strum::IntoEnumIterator;
use wispwood::Wispwood;

use crate::core::{EvaluatedMatch, GameEngine, GameUi};
use dioxus::prelude::*;
use strum_macros::EnumIter;

macro_rules! register_games {
    ($( $variant:ident => $game_type:path ),* $(,)?) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter)]
        pub enum Game {
            $( $variant, )*
        }

        impl Game {
            pub fn id(&self) -> &'static str {
                match self {
                    $( Self::$variant => <$game_type as GameEngine>::id(&<$game_type>::default()), )*
                }
            }

            pub fn name(&self) -> &'static str {
                match self {
                    $( Self::$variant => <$game_type as GameEngine>::name(&<$game_type>::default()), )*
                }
            }

            pub fn render_state(
                &self,
                saved_json: Option<&str>,
                player_count: usize,
                on_save: EventHandler<EvaluatedMatch>,
            ) -> Element {
                match self {
                    $(
                        Self::$variant => {
                            let game = <$game_type>::default();

                            let initial_state = saved_json
                                .and_then(|json| GameEngine::load_state(&game, json).ok())
                                .unwrap_or_else(|| GameEngine::default_state(&game, player_count));

                            GameUi::render_state(
                                &game,
                                initial_state,
                                EventHandler::new(move |updated_state| {
                                    let evaluated = GameEngine::evaluate_state(&game, &updated_state);
                                    on_save.call(evaluated);
                                }),
                            )
                        }
                    )*
                }
            }

            pub fn render_score(
                &self,
                saved_json: Option<&str>,
                player_count: usize,
                on_save: EventHandler<EvaluatedMatch>,
            ) -> Element {
                match self {
                    $(
                        Self::$variant => {
                            let game = <$game_type>::default();

                            let initial_score = saved_json
                                .and_then(|json| GameEngine::load_score(&game, json).ok())
                                .unwrap_or_else(|| GameEngine::default_score(&game, player_count));

                            GameUi::render_score(
                                &game,
                                initial_score,
                                EventHandler::new(move |updated_score| {
                                    let evaluated = GameEngine::evaluate_score(&game, &updated_score);
                                    on_save.call(evaluated);
                                }),
                            )
                        }
                    )*
                }
            }
        }
    };
}

register_games! {
    Cascadia => Cascadia,
    Wispwood => Wispwood,
}

impl Game {
    pub fn all_games() -> HashMap<String, Game> {
        Game::iter().map(|game| (game.id().into(), game)).collect()
    }
}
