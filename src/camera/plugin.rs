use super::systems::{follow_player, reset_camera, spawn_camera};
use crate::state::GameState;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(OnExit(GameState::Playing), reset_camera)
            .add_systems(Update, follow_player);
    }
}
