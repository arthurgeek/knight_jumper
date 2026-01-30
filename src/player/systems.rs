use super::components::{AnimationConfig, Grounded, JumpVelocity, Player, Speed};
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

/// System that checks if the player is grounded by examining collision contacts.
/// Grounded = any collision with an upward-facing normal (standing on something).
pub fn update_grounded(
    mut commands: Commands,
    player: Query<(Instance<Player>, &Children)>,
    collider_query: Query<(), With<Collider>>,
    collisions: Collisions,
) {
    for (player_instance, children) in &player {
        // Find the player's collider child
        let Some(collider_entity) = children.iter().find(|c| collider_query.contains(*c)) else {
            continue;
        };

        // Check if any collision has an upward-facing normal (floor)
        let is_grounded = collisions
            .collisions_with(collider_entity)
            .filter_map(|contacts| contacts.manifolds.first())
            .any(|manifold| {
                // Normal pointing up means we're standing on something
                // Use a threshold to allow slight angles
                manifold.normal.y.abs() > 0.7 && manifold.normal.y > 0.0
            });

        if is_grounded {
            commands.entity(*player_instance).insert(Grounded);
        } else {
            commands.entity(*player_instance).remove::<Grounded>();
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
    mut player: Query<(&mut LinearVelocity, &Speed, &JumpVelocity, Has<Grounded>), With<Player>>,
    mut movement_events: MessageWriter<PlayerMovement>,
) {
    for (mut velocity, speed, jump_vel, is_grounded) in &mut player {
        // Handle jumping - only if grounded AND requested
        if input.jump_requested && is_grounded {
            velocity.y = jump_vel.0;
        }
        // Always clear jump request after checking (prevents double jump)
        input.jump_requested = false;

        // Handle horizontal movement
        if input.movement_direction != 0.0 {
            velocity.x = input.movement_direction * speed.0;
        } else {
            // Player released movement keys - apply deceleration
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
