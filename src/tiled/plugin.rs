use super::systems::{load_map, spawn_camera, spawn_player_at_spawn_point};
use bevy::prelude::*;

pub struct TiledPlugin;

impl Plugin for TiledPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_ecs_tiled::prelude::TiledPlugin::default())
            .add_systems(Startup, (spawn_camera, load_map))
            .add_systems(Update, spawn_player_at_spawn_point);
    }
}
