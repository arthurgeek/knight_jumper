use super::components::{AnimationConfig, AnimationState, JumpVelocity, Player, Speed};
use super::messages::PlayerMovement;
use super::resources::{KnightAtlas, PlayerInput};
use super::systems::{
    apply_player_movement, detect_player_input, load_knight_atlas, update_grounded,
    update_player_animation,
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
            .register_type::<AnimationConfig>()
            .register_type::<AnimationState>()
            .register_type::<Speed>()
            .register_type::<JumpVelocity>()
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
                    (update_grounded, apply_player_movement)
                        .chain()
                        .in_set(PlayerSystemSet::Movement),
                    update_player_animation.in_set(PlayerSystemSet::Animation),
                ),
            );
    }
}
