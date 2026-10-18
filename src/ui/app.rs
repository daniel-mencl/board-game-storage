use super::Players;
use super::components::Button;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
enum Screen {
    Home,
    Matches,
    Players,
}

#[component]
pub fn App() -> Element {
    let mut screen = use_signal(|| Screen::Home);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }

        match screen() {
            Screen::Home => rsx! {
                div {
                    Button {
                        on_click: move |_| screen.set(Screen::Matches),
                        "Matches"
                    }
                    Button {
                        on_click: move |_| screen.set(Screen::Players),
                        "Players"
                    }
                }
            },
            Screen::Matches => rsx! {
                Matches {
                    on_back: move |_| screen.set(Screen::Home),
                }
            },
            Screen::Players => rsx! {
                Players {
                    on_back: move |_| screen.set(Screen::Home),
                }
            },
        }
    }
}

#[component]
fn Matches(on_back: EventHandler<()>) -> Element {
    rsx! {
        div {
            h1 { "Matches" }
            Button {
                on_click: move |_| on_back.call(()),
                "<- Back"
            }
        }
    }
}
