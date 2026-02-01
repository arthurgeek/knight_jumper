use super::components::{
    CoyoteTimer, Grounded, JumpBuffer, JumpVelocity, PlatformVelocity, Player, WallContactLeft,
    WallContactRight,
};
use super::messages::PlayerMovement;
use super::resources::{KnightAtlas, PlayerInput};
use crate::core::components::Speed;
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
    query: Query<(Instance<Player>, &ShapeHits, &Rotation)>,
    sensors: Query<(), With<Sensor>>,
) {
    for (player, hits, rotation) in &query {
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

/// Starts coyote timer when player leaves ground.
pub fn start_coyote_timer(
    mut commands: Commands,
    mut removed: RemovedComponents<Grounded>,
    players: Query<(), With<Player>>,
) {
    for entity in removed.read() {
        if players.contains(entity) {
            commands.entity(entity).insert(CoyoteTimer::default());
        }
    }
}

/// Clears coyote timer when player lands.
pub fn clear_coyote_timer(
    mut commands: Commands,
    query: Query<Entity, (With<Player>, Added<Grounded>)>,
) {
    for entity in &query {
        commands.entity(entity).remove::<CoyoteTimer>();
    }
}

/// Ticks coyote timer and removes when expired.
pub fn tick_coyote_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CoyoteTimer)>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.entity(entity).remove::<CoyoteTimer>();
        }
    }
}

/// Ticks jump buffer timer and removes when expired.
pub fn tick_jump_buffer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut JumpBuffer)>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.entity(entity).remove::<JumpBuffer>();
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
    mut commands: Commands,
    mut input: ResMut<PlayerInput>,
    mut player: Query<
        (
            Entity,
            &mut LinearVelocity,
            &Speed,
            &JumpVelocity,
            Has<Grounded>,
            Has<WallContactLeft>,
            Has<WallContactRight>,
            Option<&PlatformVelocity>,
            Option<&CoyoteTimer>,
            Option<&JumpBuffer>,
        ),
        (With<Player>, Without<DeathTimer>),
    >,
    mut movement_events: MessageWriter<PlayerMovement>,
) {
    for (
        entity,
        mut velocity,
        speed,
        jump_vel,
        is_grounded,
        wall_left,
        wall_right,
        platform_vel,
        coyote,
        jump_buffer,
    ) in &mut player
    {
        // Can jump if grounded OR within coyote time
        let can_jump = is_grounded || coyote.is_some();

        // Jump requested now, or buffered from earlier
        let wants_jump = input.jump_requested || jump_buffer.is_some();

        if wants_jump && can_jump {
            velocity.y = jump_vel.0;
            // Consume coyote time and jump buffer
            commands
                .entity(entity)
                .remove::<CoyoteTimer>()
                .remove::<JumpBuffer>();
        } else if input.jump_requested && !can_jump {
            // Pressed jump in air without coyote - start buffer
            commands.entity(entity).insert(JumpBuffer::default());
        }

        // Clear raw input after processing
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

        // Send movement event for sprite flipping
        movement_events.write(PlayerMovement {
            is_moving: input.movement_direction != 0.0,
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

/// Flips player sprite based on movement direction.
/// Only updates when moving, so sprite stays in last direction when stopped.
pub fn flip_player_sprite(
    mut player: Query<&mut Sprite, With<Player>>,
    mut movement_events: MessageReader<PlayerMovement>,
) {
    for event in movement_events.read() {
        if event.is_moving {
            for mut sprite in &mut player {
                sprite.flip_x = event.facing_left;
            }
        }
    }
}
