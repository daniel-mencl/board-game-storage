use super::GameEngine;
use dioxus::prelude::*;

pub trait GameUi : GameEngine {
    fn render_score(&self, score: Self::ScoreBreakdown, on_save: EventHandler<Self::ScoreBreakdown>) -> Element;

    fn render_state(&self, state: Self::State, on_save: EventHandler<Self::State>) -> Element;
}