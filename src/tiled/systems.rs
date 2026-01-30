use crate::player::Player;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    // Spawn a 2D camera with proper scaling
    commands.spawn((Camera2d, Transform::from_scale(Vec3::splat(0.5))));
}

pub fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load a map asset and retrieve its handle
    let map_handle: Handle<TiledMapAsset> = asset_server.load("maps/main.tmx");

    // Spawn the map centered in the view
    commands.spawn((TiledMap(map_handle), TilemapAnchor::Center));
}

pub fn spawn_player_at_spawn_point(
    mut commands: Commands,
    spawn_points: Query<(&TiledName, &Transform), Added<TiledObject>>,
) {
    for (name, transform) in &spawn_points {
        if name.0.as_str() == "PlayerSpawn" {
            commands.spawn((Player, *transform));
        }
    }
}
