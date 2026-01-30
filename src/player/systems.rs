use super::components::{
    AnimationConfig, Grounded, JumpVelocity, Player, Speed, WallContactLeft, WallContactRight,
};
use super::messages::PlayerMovement;
use super::resources::{KnightAtlas, PlayerInput};
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

/// System that checks if the player is grounded using a short raycast.
/// More reliable than collision contacts when pressed against walls.
pub fn update_grounded(
    mut commands: Commands,
    player: Query<(Instance<Player>, &GlobalTransform, &Children)>,
    collider_query: Query<Entity, With<Collider>>,
    spatial_query: SpatialQuery,
) {
    for (player_instance, transform, children) in &player {
        // Find the player's collider child to exclude from raycast
        let Some(collider_entity) = children.iter().find(|c| collider_query.contains(*c)) else {
            continue;
        };

        // Cast a ray downward from player center, check for ground within capsule reach
        // Capsule bottom is at local y = -7 (child offset) - 5 (half_height) - 3 (radius) = -15
        let player_pos = transform.translation().truncate();
        let ray_origin = player_pos; // Start from player center
        let ray_dir = Dir2::NEG_Y;
        let max_distance = 17.0; // Capsule bottom (-15) + small margin

        let hit = spatial_query.cast_ray(
            ray_origin,
            ray_dir,
            max_distance,
            true,
            &SpatialQueryFilter::default().with_excluded_entities([collider_entity]),
        );

        let is_grounded = hit.map(|h| h.distance < 16.0).unwrap_or(false);

        if is_grounded {
            commands.entity(*player_instance).insert(Grounded);
        } else {
            commands.entity(*player_instance).remove::<Grounded>();
        }
    }
}

/// System that checks if the player is touching a wall by examining collision contacts.
pub fn update_wall_contact(
    mut commands: Commands,
    player: Query<(Instance<Player>, &Children)>,
    collider_query: Query<(), With<Collider>>,
    collisions: Collisions,
) {
    for (player_instance, children) in &player {
        let Some(collider_entity) = children.iter().find(|c| collider_query.contains(*c)) else {
            continue;
        };

        let mut wall_left = false;
        let mut wall_right = false;

        for contacts in collisions.collisions_with(collider_entity) {
            if let Some(manifold) = contacts.manifolds.first() {
                // Normal points from collider1 → collider2, flip if we're collider1
                let normal = if contacts.collider1 == collider_entity {
                    -manifold.normal
                } else {
                    manifold.normal
                };

                // Horizontal normal indicates a wall
                if normal.x.abs() > 0.7 {
                    if normal.x > 0.0 {
                        wall_left = true; // Wall to the left pushes us right
                    } else {
                        wall_right = true; // Wall to the right pushes us left
                    }
                }
            }
        }

        if wall_left {
            commands.entity(*player_instance).insert(WallContactLeft);
        } else {
            commands
                .entity(*player_instance)
                .remove::<WallContactLeft>();
        }

        if wall_right {
            commands.entity(*player_instance).insert(WallContactRight);
        } else {
            commands
                .entity(*player_instance)
                .remove::<WallContactRight>();
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
        ),
        With<Player>,
    >,
    mut movement_events: MessageWriter<PlayerMovement>,
) {
    for (mut velocity, speed, jump_vel, is_grounded, wall_left, wall_right) in &mut player {
        // Handle jumping - only if grounded AND requested
        if input.jump_requested && is_grounded {
            velocity.y = jump_vel.0;
        }
        // Always clear jump request after checking (prevents double jump)
        input.jump_requested = false;

        // Handle horizontal movement (blocked by walls)
        let blocked = (input.movement_direction < 0.0 && wall_left)
            || (input.movement_direction > 0.0 && wall_right);

        if input.movement_direction != 0.0 && !blocked {
            velocity.x = input.movement_direction * speed.0;
        } else {
            // Player released movement keys or blocked by wall - apply deceleration
            velocity.x = move_toward(velocity.x, 0.0, speed.0 * 0.5);
        }

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
