use super::components::{MovingPlatform, MovingPlatformSpawn, OneWayPlatform};
use super::resources::PlatformTexture;
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

            commands.spawn((Name::new("OneWayPlatform"), OneWayPlatform, centered));

            // Despawn the spawn marker
            commands.entity(*tiled_object).despawn();
        }
    }
}

/// Spawns MovingPlatform entities from MovingPlatformSpawn Tiled objects.
/// The end_point property is an object reference that bevy_ecs_tiled resolves to Entity.
pub fn spawn_moving_platforms(
    mut commands: Commands,
    spawn_configs: Query<(Instance<MovingPlatformSpawn>, &MovingPlatformSpawn, &Transform), Added<MovingPlatformSpawn>>,
    transforms: Query<&Transform>,
) {
    for (spawn_instance, config, transform) in &spawn_configs {
        // Tiled position is top-left, offset to center for Bevy (32x9 platform)
        let mut centered = *transform;
        centered.translation.x += 16.0; // half width
        centered.translation.y -= 4.5; // half height

        let start = centered.translation.truncate();

        // Get end position from the referenced entity
        let end_entity = config
            .end_point
            .expect("MovingPlatformSpawn missing end_point property");
        let end_transform = transforms
            .get(end_entity)
            .expect("MovingPlatform end_point entity has no Transform");
        let end = Vec2::new(
            end_transform.translation.x + 16.0,
            end_transform.translation.y - 4.5,
        );

        let duration = if config.duration > 0.0 {
            config.duration
        } else {
            1.5
        };

        commands.spawn((
            Name::new("MovingPlatform"),
            MovingPlatform::new(start, end, duration),
            centered,
        ));

        // Despawn the spawn marker and end_point marker (no longer needed)
        commands.entity(*spawn_instance).despawn();
        if let Some(end_entity) = config.end_point {
            commands.entity(end_entity).despawn();
        }
    }
}

/// Animates moving platforms back and forth using LinearVelocity
pub fn update_moving_platforms(
    time: Res<Time>,
    mut platforms: Query<(&mut MovingPlatform, &mut LinearVelocity)>,
) {
    for (mut platform, mut velocity) in &mut platforms {
        // Update progress
        platform.progress += platform.direction * time.delta_secs() / platform.duration;

        // Reverse direction at ends
        if platform.progress >= 1.0 {
            platform.progress = 1.0;
            platform.direction = -1.0;
        } else if platform.progress <= 0.0 {
            platform.progress = 0.0;
            platform.direction = 1.0;
        }

        // Calculate velocity to reach target position
        let target = platform.start.lerp(platform.end, platform.progress);
        let next_progress =
            platform.progress + platform.direction * time.delta_secs() / platform.duration;
        let next_target = platform
            .start
            .lerp(platform.end, next_progress.clamp(0.0, 1.0));

        // Set velocity based on where we need to go
        let delta = next_target - target;
        velocity.0 = delta / time.delta_secs();
    }
}
