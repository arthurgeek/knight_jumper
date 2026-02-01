use super::components::{
    AnimationConfig, Grounded, JumpVelocity, PlatformVelocity, Player, Speed, WallContactLeft,
    WallContactRight,
};
use super::messages::PlayerMovement;
use super::resources::{KnightAtlas, PlayerInput};
use crate::killzone::components::DeathTimer;
use avian2d::prelude::*;
use bevy::prelude::*;
use moonshine_kind::Instance;

/// System that loads the knight texture atlas and stores it in a resource
pub fn load_knight_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/knight.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        8,
        8,
        None,
        None,
    ));

    commands.insert_resource(KnightAtlas { texture, layout });
}

/// System that checks if the player is grounded using ShapeCaster hits.
pub fn update_grounded(
    mut commands: Commands,
    mut query: Query<(Instance<Player>, &ShapeHits, &Rotation)>,
    sensors: Query<(), With<Sensor>>,
) {
    for (player, hits, rotation) in &mut query {
        // Grounded if shape caster has a hit with a roughly upward normal (ignoring sensors)
        let is_grounded = hits.iter().any(|hit| {
            // Skip sensors (like coins)
            if sensors.contains(hit.entity) {
                return false;
            }
            (rotation * -hit.normal2).angle_to(Vec2::Y).abs() <= std::f32::consts::FRAC_PI_4
        });

        if is_grounded {
            commands.entity(*player).insert(Grounded);
        } else {
            commands.entity(*player).remove::<Grounded>();
        }
    }
}

/// Tracks velocity from moving platforms the player is standing on.
pub fn update_platform_velocity(
    mut commands: Commands,
    query: Query<(Instance<Player>, &ShapeHits, &Rotation)>,
    sensors: Query<(), With<Sensor>>,
    velocities: Query<&LinearVelocity>,
) {
    for (player, hits, rotation) in &query {
        // Find ground hit
        let ground_hit = hits.iter().find(|hit| {
            if sensors.contains(hit.entity) {
                return false;
            }
            (rotation * -hit.normal2).angle_to(Vec2::Y).abs() <= std::f32::consts::FRAC_PI_4
        });

        if let Some(hit) = ground_hit {
            if let Ok(vel) = velocities.get(hit.entity) {
                commands.entity(*player).insert(PlatformVelocity(vel.0));
            } else {
                commands.entity(*player).remove::<PlatformVelocity>();
            }
        } else {
            commands.entity(*player).remove::<PlatformVelocity>();
        }
    }
}

/// System that checks if the player is touching a wall by examining collision contacts.
pub fn update_wall_contact(
    mut commands: Commands,
    player: Query<Instance<Player>>,
    collisions: Collisions,
    sensors: Query<(), With<Sensor>>,
) {
    for player_instance in &player {
        let player_entity = *player_instance;
        let mut wall_left = false;
        let mut wall_right = false;

        for contacts in collisions.collisions_with(player_entity) {
            // Skip sensors (like coins)
            let other = if contacts.collider1 == player_entity {
                contacts.collider2
            } else {
                contacts.collider1
            };
            if sensors.contains(other) {
                continue;
            }

            if let Some(manifold) = contacts.manifolds.first() {
                // Normal points from collider1 → collider2, flip if we're collider1
                let normal = if contacts.collider1 == player_entity {
                    -manifold.normal
                } else {
                    manifold.normal
                };

                // Horizontal normal indicates a wall
                if normal.x.abs() > 0.7 {
                    if normal.x > 0.0 {
                        wall_left = true;
                    } else {
                        wall_right = true;
                    }
                }
            }
        }

        if wall_left {
            commands.entity(player_entity).insert(WallContactLeft);
        } else {
            commands.entity(player_entity).remove::<WallContactLeft>();
        }

        if wall_right {
            commands.entity(player_entity).insert(WallContactRight);
        } else {
            commands.entity(player_entity).remove::<WallContactRight>();
        }
    }
}

/// System that detects player input and converts it to events
///
/// Runs in InputDetection set and can execute in parallel with other input systems.
/// Only reads input and writes events, enabling better parallelization.
pub fn detect_player_input(keyboard: Res<ButtonInput<KeyCode>>, mut input: ResMut<PlayerInput>) {
    // Movement - overwrite each frame
    input.movement_direction = 0.0;
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        input.movement_direction -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        input.movement_direction += 1.0;
    }

    // Jump - buffer it (don't clear, let movement system clear it)
    if keyboard.just_pressed(KeyCode::Space) {
        input.jump_requested = true;
    }
}

/// System that handles player movement physics based on input resource
///
/// Runs in Movement set after grounded check. Handles all physics calculations
/// and movement execution separately from input detection.
pub fn apply_player_movement(
    mut input: ResMut<PlayerInput>,
    mut player: Query<
        (
            &mut LinearVelocity,
            &Speed,
            &JumpVelocity,
            Has<Grounded>,
            Has<WallContactLeft>,
            Has<WallContactRight>,
            Option<&PlatformVelocity>,
        ),
        (With<Player>, Without<DeathTimer>),
    >,
    mut movement_events: MessageWriter<PlayerMovement>,
) {
    for (mut velocity, speed, jump_vel, is_grounded, wall_left, wall_right, platform_vel) in
        &mut player
    {
        // Handle jumping - only if grounded AND requested
        if input.jump_requested && is_grounded {
            velocity.y = jump_vel.0;
        }
        // Always clear jump request after checking (prevents double jump)
        input.jump_requested = false;

        // Handle horizontal movement (blocked by walls)
        let blocked = (input.movement_direction < 0.0 && wall_left)
            || (input.movement_direction > 0.0 && wall_right);

        // Base velocity from player input
        let player_vel = if input.movement_direction != 0.0 && !blocked {
            input.movement_direction * speed.0
        } else {
            move_toward(velocity.x, 0.0, speed.0 * 0.5)
        };

        // Add platform velocity if standing on a moving platform
        velocity.x = player_vel + platform_vel.map(|p| p.0.x).unwrap_or(0.0);

        // Send movement event for animation system
        movement_events.write(PlayerMovement {
            is_moving: input.movement_direction != 0.0,
            is_grounded,
            facing_left: input.movement_direction < 0.0,
        });
    }
}

fn move_toward(current: f32, target: f32, max_delta: f32) -> f32 {
    if (target - current).abs() <= max_delta {
        target
    } else {
        current + (target - current).signum() * max_delta
    }
}

/// System that updates player animations based on movement events
///
/// Runs in Animation set after movement. Handles all animation state
/// separately from physics and input.
pub fn update_player_animation(
    time: Res<Time>,
    mut player: Query<(&mut AnimationConfig, &mut Sprite), With<Player>>,
    mut movement_events: MessageReader<PlayerMovement>,
) {
    for event in movement_events.read() {
        for (mut config, mut sprite) in &mut player {
            // Flip sprite
            sprite.flip_x = event.facing_left;

            // Change animation based on state
            if !event.is_grounded {
                // *config = AnimationConfig::new(jump_frames...);
            } else if event.is_moving {
                // *config = AnimationConfig::new(run_frames...);
            } else {
                // *config = AnimationConfig::new(idle_frames...);
                // We track how long the current sprite has been displayed for
                config.frame_timer.tick(time.delta());

                // If it has been displayed for the user-defined amount of time (fps)...
                if config.frame_timer.just_finished()
                    && let Some(atlas) = &mut sprite.texture_atlas
                {
                    if atlas.index == config.last_sprite_index {
                        // ...and it IS the last frame, then we move back to the first frame and stop.
                        atlas.index = config.first_sprite_index;
                    } else {
                        // ...and it is NOT the last frame, then we move to the next frame...
                        atlas.index += 1;
                        // ...and reset the frame timer to start counting all over again
                        config.frame_timer =
                            AnimationConfig::timer_from_fps(config.fps, config.looping);
                    }
                }
            }
        }
    }
}
