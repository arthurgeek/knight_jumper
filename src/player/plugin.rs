use super::components::{CoyoteTimer, JumpBuffer, JumpVelocity, Player, PlayerAnimation};
use super::messages::PlayerMovement;
use super::resources::{KnightAtlas, PlayerInput};
use super::systems::{
    apply_player_movement, clear_coyote_timer, detect_player_input, flip_player_sprite,
    load_knight_atlas, start_coyote_timer, sync_player_animation, tick_coyote_timer,
    tick_jump_buffer, update_grounded, update_platform_velocity, update_player_animation,
    update_wall_contact,
};
use bevy::prelude::*;

/// System sets for player operations with better parallelization
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerSystemSet {
    /// Physics and movement (runs in FixedUpdate)
    Movement,
    /// Animation updates (runs after movement)
    Animation,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<KnightAtlas>()
            .register_type::<Player>()
            .register_type::<PlayerAnimation>()
            .register_type::<JumpVelocity>()
            .register_type::<CoyoteTimer>()
            .register_type::<JumpBuffer>()
            .register_type::<PlayerInput>()
            .init_resource::<PlayerInput>()
            .add_message::<PlayerMovement>()
            // Configure set ordering for FixedUpdate
            .configure_sets(
                FixedUpdate,
                (PlayerSystemSet::Movement, PlayerSystemSet::Animation).chain(),
            )
            // Input detection runs in Update (every frame) for responsive input
            .add_systems(Update, detect_player_input)
            .add_systems(Startup, load_knight_atlas)
            // Movement and animation run in FixedUpdate (synced with physics)
            .add_systems(
                FixedUpdate,
                (
                    (
                        (
                            update_grounded,
                            update_wall_contact,
                            update_platform_velocity,
                        ),
                        (
                            start_coyote_timer,
                            clear_coyote_timer,
                            tick_coyote_timer,
                            tick_jump_buffer,
                        ),
                        apply_player_movement,
                    )
                        .chain()
                        .in_set(PlayerSystemSet::Movement),
                    (
                        update_player_animation,
                        sync_player_animation,
                        flip_player_sprite,
                    )
                        .chain()
                        .in_set(PlayerSystemSet::Animation),
                ),
            );
    }
}
