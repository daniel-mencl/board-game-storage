use crate::core::GameUi;
use super::Wispwood;
use dioxus::prelude::*;

impl GameUi for Wispwood {
    fn render_score(&self, _score: Self::ScoreBreakdown, _on_save: EventHandler<Self::ScoreBreakdown>) -> Element {
        rsx! {}
    }

    fn render_state(&self, _state: Self::State, _on_save: EventHandler<Self::State>) -> Element {
        rsx! {}
    }
}