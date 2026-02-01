use super::systems::{load_map, spawn_player_at_spawn_point};
use crate::state::GameState;
use bevy::prelude::*;

pub struct TiledPlugin;

impl Plugin for TiledPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_ecs_tiled::prelude::TiledPlugin::default())
            .add_systems(OnEnter(GameState::Playing), load_map)
            .add_systems(Update, spawn_player_at_spawn_point);
    }
}
