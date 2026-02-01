use crate::player::Player;
use crate::state::GameState;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load a map asset and retrieve its handle
    let map_handle: Handle<TiledMapAsset> = asset_server.load("maps/main.tmx");

    // Spawn the map centered in the view
    commands.spawn((
        TiledMap(map_handle),
        TilemapAnchor::Center,
        DespawnOnExit(GameState::Playing),
    ));
}

pub fn spawn_player_at_spawn_point(
    mut commands: Commands,
    spawn_points: Query<(&TiledName, &Transform), Added<TiledObject>>,
) {
    for (name, transform) in &spawn_points {
        if name.0.as_str() == "PlayerSpawn" {
            let mut player_transform = *transform;
            player_transform.translation.z = 10.0;
            commands.spawn((Player, player_transform, DespawnOnExit(GameState::Playing)));
        }
    }
}
