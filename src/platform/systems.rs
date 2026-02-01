use super::components::{MovingPlatform, OneWayPlatform};
use super::resources::PlatformTexture;
use crate::state::GameState;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use moonshine_kind::Instance;

pub fn load_platform_texture(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("sprites/platforms.png");
    commands.insert_resource(PlatformTexture { texture });
}

pub fn spawn_platform_at_spawn_point(
    mut commands: Commands,
    platforms: Query<(Instance<TiledObject>, &TiledName, &Transform), Added<TiledObject>>,
) {
    for (tiled_object, name, transform) in &platforms {
        if name.0 == "PlatformSpawn" {
            // Tiled position is top-left, offset to center for Bevy (32x9 platform)
            let mut centered = *transform;
            centered.translation.x += 16.0; // half width
            centered.translation.y -= 4.5; // half height

            commands.spawn((
                Name::new("OneWayPlatform"),
                OneWayPlatform,
                centered,
                DespawnOnExit(GameState::Playing),
            ));

            // Despawn the spawn marker
            commands.entity(*tiled_object).despawn();
        }
    }
}

/// Initializes MovingPlatform start/end from polyline vertices.
pub fn setup_moving_platforms(
    maps_assets: Res<Assets<TiledMapAsset>>,
    map_query: Query<&TiledMap>,
    mut platforms: Query<
        (
            &mut MovingPlatform,
            &Sprite,
            &TiledObject,
            &TiledMapReference,
            &GlobalTransform,
            &mut Transform,
        ),
        Added<MovingPlatform>,
    >,
) {
    for (mut platform, sprite, tiled_obj, map_ref, global_transform, mut transform) in
        &mut platforms
    {
        let Some(vertices) = map_query
            .get(map_ref.0)
            .ok()
            .and_then(|map_handle| maps_assets.get(&map_handle.0))
            .map(|map_asset| map_asset.object_vertices(tiled_obj, global_transform))
        else {
            continue;
        };

        assert!(
            vertices.len() >= 2,
            "MovingPlatform polyline needs at least 2 points"
        );

        // Get sprite dimensions from rect
        let (width, height) = sprite
            .rect
            .map(|r| (r.width(), r.height()))
            .unwrap_or((32.0, 9.0));

        // First vertex = left edge at start, last = right edge at end
        let first = vertices.first().unwrap();
        let last = vertices.last().unwrap();

        platform.start = Vec2::new(first.x + width / 2.0, first.y - height / 2.0);
        platform.end = Vec2::new(last.x - width / 2.0, last.y - height / 2.0);

        // Set initial position
        transform.translation = platform.start.extend(transform.translation.z);
    }
}

/// Animates moving platforms back and forth using LinearVelocity
pub fn update_moving_platforms(
    mut platforms: Query<(&mut MovingPlatform, &Transform, &mut LinearVelocity)>,
) {
    for (mut platform, transform, mut velocity) in &mut platforms {
        let pos = transform.translation.truncate();
        let target = if platform.direction > 0.0 {
            platform.end
        } else {
            platform.start
        };
        let to_target = target - pos;

        if to_target.length() < 1.0 {
            platform.direction *= -1.0;
            velocity.0 = Vec2::ZERO;
        } else {
            velocity.0 = to_target.normalize() * platform.speed;
        }
    }
}
