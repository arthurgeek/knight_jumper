use super::components::TipText;
use super::systems::spawn_tip_text;
use bevy::prelude::*;

pub struct TipsPlugin;

impl Plugin for TipsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TipText>()
            .add_systems(Update, spawn_tip_text);
    }
}
