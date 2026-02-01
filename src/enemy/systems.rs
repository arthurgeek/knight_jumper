use super::components::{Enemy, Slime};
use crate::core::components::Speed;
use crate::state::GameState;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use moonshine_kind::Instance;

/// Spawns slimes at SlimeSpawn points.
pub fn spawn_slime_at_spawn_point(
    mut commands: Commands,
    spawn_points: Query<(Instance<TiledObject>, &TiledName, &Transform), Added<TiledObject>>,
) {
    for (tiled_object, name, transform) in &spawn_points {
        if name.0 == "SlimeSpawn" {
            // Tiled point is bottom-left, offset to center for Bevy (24x24 slime)
            // Anchor is -0.25 Y (6px up), so reduce Y offset
            let mut centered = *transform;
            centered.translation.x += 12.0; // half width
            centered.translation.y += 6.0; // half height minus anchor offset
            let pos = centered.translation.truncate();
            commands.spawn((
                Name::new("Slime"),
                Slime,
                centered,
                Position(pos),
                DespawnOnExit(GameState::Playing),
            ));
            commands.entity(*tiled_object).despawn();
        }
    }
}

/// Moves patrol enemies and flips direction on wall hit.
pub fn update_patrol_movement(
    mut query: Query<
        (
            &RayHits,
            &mut RayCaster,
            &Speed,
            &mut LinearVelocity,
            &mut Sprite,
        ),
        With<Enemy>,
    >,
) {
    for (hits, mut ray_caster, speed, mut velocity, mut sprite) in &mut query {
        // Wall hit - flip direction
        if !hits.is_empty() {
            ray_caster.direction = -ray_caster.direction;
        }

        velocity.x = ray_caster.direction.x * speed.0;
        sprite.flip_x = ray_caster.direction.x < 0.0;
    }
}
