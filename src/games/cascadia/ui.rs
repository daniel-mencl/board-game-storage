use crate::core::GameUi;
use super::Cascadia;
use dioxus::prelude::*;

impl GameUi for Cascadia {
    fn render_score(&self, _score: Self::ScoreBreakdown, _on_save: EventHandler<Self::ScoreBreakdown>) -> Element {
        rsx! {}
    }

    fn render_state(&self, _state: Self::State, _on_save: EventHandler<Self::State>) -> Element {
        rsx! {}
    }
}