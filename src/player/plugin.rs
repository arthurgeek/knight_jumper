use super::components::{AnimationConfig, AnimationState, Player};
use super::resources::KnightAtlas;
use super::systems::{execute_animations, load_knight_atlas};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<KnightAtlas>()
            .register_type::<Player>()
            .register_type::<AnimationConfig>()
            .register_type::<AnimationState>()
            .add_systems(Startup, load_knight_atlas)
            .add_systems(Update, execute_animations);
    }
}
