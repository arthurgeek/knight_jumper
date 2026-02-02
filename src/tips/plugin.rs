use super::components::{ScoreText, TipText};
use super::systems::{spawn_tip_text, update_score_text};
use crate::core::components::Score;
use bevy::prelude::*;

pub struct TipsPlugin;

impl Plugin for TipsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TipText>()
            .register_type::<ScoreText>()
            .add_systems(Update, spawn_tip_text)
            .add_systems(Update, update_score_text.run_if(resource_changed::<Score>));
    }
}
