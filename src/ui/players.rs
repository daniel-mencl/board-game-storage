use std::sync::Arc;

use crate::database::{NewPlayer, Player, PlayerRepository};

use super::components::{Button, ButtonVariant};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
enum Screen {
    List,
    Add,
}

#[component]
pub fn Players(on_back: EventHandler<()>) -> Element {
    let player_repository = use_context::<Arc<dyn PlayerRepository>>();
    let mut players = use_signal(|| player_repository.list_all().unwrap_or_default());

    let mut screen = use_signal(|| Screen::List);

    match screen() {
        Screen::List => rsx! {
            div {
                h1 { "Players" },
                for player in players.read().iter() {
                    "{player.name}"
                }
                Button {
                    on_click: move |_| on_back.call(()),
                    "<- Back"
                }
                Button {
                    variant: ButtonVariant::Secondary,
                    on_click: move |_| screen.set(Screen::Add),
                    "Add player"
                }
            }
        },
        Screen::Add => rsx! {
            AddPlayer {
                on_cancel: move |_| screen.set(Screen::List),
                on_created: move |player: Player| {
                    players.write().push(player);
                    screen.set(Screen::List)
                }
            }
        },
    }
}

#[component]
fn AddPlayer(on_cancel: EventHandler<()>, on_created: EventHandler<Player>) -> Element {
    let player_repository = use_context::<Arc<dyn PlayerRepository>>();
    let mut name = use_signal(String::new);
    let mut error_msg = use_signal(|| None::<String>);

    let handle_save = move |_| {
        let trimmed = name.read().trim().to_string();
        if trimmed.is_empty() {
            error_msg.set(Some("Player name cannot be empty.".to_string()));
            return;
        }

        let new_player = NewPlayer { name: trimmed };
        match player_repository.create(&new_player) {
            Ok(created_player) => on_created.call(created_player),
            Err(_) => error_msg.set(Some("Failed to create player".into())),
        }
    };

    rsx! {
        div {
            h1 { "Add Player" }

            input {
                r#type: "text",
                placeholder: "Enter player name...",
                value: "{name}",
                oninput: move |event| {
                    name.set(event.value());
                    error_msg.set(None);
                }
            }

            if let Some(err) = error_msg.read().as_ref() {
                p { class: "error-text", "{err}" }
            }

            div {
                Button {
                    on_click: move |_| on_cancel.call(()),
                    "Cancel"
                }
                Button {
                    variant: ButtonVariant::Secondary,
                    on_click: handle_save,
                    "Create"
                }
            }
        }
    }
}
